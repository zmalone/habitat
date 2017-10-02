// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;
use std::io::Read;
use std::time::{UNIX_EPOCH, Duration, SystemTime};

use hyper::{self, Url};
use hyper::status::StatusCode;
use hyper::header::{Authorization, Accept, Bearer, UserAgent, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::net::HttpsConnector;
use hyper_openssl::OpensslClient;
use jwt;
use serde_json;

use config::GitHubCfg;
use error::{HubError, HubResult};
use types::*;

const USER_AGENT: &'static str = "Habitat-Builder";
const HTTP_TIMEOUT: u64 = 3_000;

#[derive(Clone)]
pub struct GitHubClient {
    pub url: String,
    pub web_url: String,
    pub client_id: String,
    pub client_secret: String,
    app_private_key: String,
}

impl GitHubClient {
    pub fn new(config: GitHubCfg) -> Self {
        GitHubClient {
            url: config.url,
            web_url: config.web_url,
            client_id: config.client_id,
            client_secret: config.client_secret,
            app_private_key: config.app_private_key,
        }
    }

    pub fn app(&self) -> HubResult<App> {
        let app_token = generate_app_token(&self.app_private_key);
        let url = Url::parse(&format!("{}/app", self.url)).map_err(
            HubError::HttpClientParse,
        )?;
        let mut rep = http_get_preview(url, app_token)?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = serde_json::from_str(&body)?;
            return Err(HubError::ApiError(rep.status, err));
        }
        let contents = serde_json::from_str::<App>(&body)?;
        Ok(contents)
    }

    pub fn app_installation_token(&self, installation_id: u32) -> HubResult<String> {
        let app_token = generate_app_token(&self.app_private_key);
        let url = Url::parse(&format!(
            "{}/installations/{}/access_tokens",
            self.url,
            installation_id
        )).map_err(HubError::HttpClientParse)?;
        let mut rep = http_post_preview(url, app_token)?;
        let mut encoded = String::new();
        rep.read_to_string(&mut encoded)?;
        match serde_json::from_str::<AppInstallationToken>(&encoded) {
            Ok(msg) => Ok(msg.token),
            Err(_) => {
                let err = serde_json::from_str::<AppAuthErr>(&encoded)?;
                Err(HubError::AppAuth(err))
            }
        }
    }

    pub fn authenticate(&self, code: &str) -> HubResult<String> {
        let url = Url::parse(&format!(
            "{}/login/oauth/access_token?\
                                client_id={}&client_secret={}&code={}",
            self.web_url,
            self.client_id,
            self.client_secret,
            code
        )).map_err(HubError::HttpClientParse)?;
        let mut rep = http_post(url)?;
        if rep.status.is_success() {
            let mut encoded = String::new();
            rep.read_to_string(&mut encoded)?;
            match serde_json::from_str::<AuthOk>(&encoded) {
                Ok(msg) => {
                    let missing = msg.missing_auth_scopes();
                    if missing.is_empty() {
                        Ok(msg.access_token)
                    } else {
                        Err(HubError::AuthScope(missing))
                    }
                }
                Err(_) => {
                    let err = serde_json::from_str::<AuthErr>(&encoded)?;
                    Err(HubError::Auth(err))
                }
            }
        } else {
            Err(HubError::HttpResponse(rep.status))
        }
    }

    /// Returns the contents of a file or directory in a repository.
    pub fn contents(
        &self,
        token: &str,
        owner: &str,
        repo: &str,
        path: &str,
    ) -> HubResult<Contents> {
        let url = Url::parse(&format!(
            "{}/repos/{}/{}/contents/{}",
            self.url,
            owner,
            repo,
            path
        )).map_err(HubError::HttpClientParse)?;
        let mut rep = http_get(url, token)?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = serde_json::from_str(&body)?;
            return Err(HubError::ApiError(rep.status, err));
        }
        let mut contents: Contents = serde_json::from_str(&body)?;

        // We need to strip line feeds as the Github API has started to return
        // base64 content with line feeds.
        if contents.encoding == "base64" {
            contents.content = contents.content.replace("\n", "");
        }

        Ok(contents)
    }

    pub fn repo(&self, token: &str, owner: &str, repo: &str) -> HubResult<Repository> {
        let url = Url::parse(&format!("{}/repos/{}/{}", self.url, owner, repo)).unwrap();
        let mut rep = http_get(url, token)?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = serde_json::from_str(&body)?;
            return Err(HubError::ApiError(rep.status, err));
        }

        let repo = match serde_json::from_str::<Repository>(&body) {
            Ok(r) => r,
            Err(e) => {
                debug!("github repo decode failed: {}. response body: {}", e, body);
                return Err(HubError::from(e));
            }
        };

        Ok(repo)
    }

    pub fn user(&self, token: &str) -> HubResult<User> {
        let url = Url::parse(&format!("{}/user", self.url)).unwrap();
        let mut rep = http_get(url, token)?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = serde_json::from_str(&body)?;
            return Err(HubError::ApiError(rep.status, err));
        }
        let user: User = serde_json::from_str(&body)?;
        Ok(user)
    }

    pub fn other_user(&self, token: &str, username: &str) -> HubResult<User> {
        let url = Url::parse(&format!("{}/users/{}", self.url, username)).unwrap();
        let mut rep = http_get(url, token)?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = serde_json::from_str(&body)?;
            return Err(HubError::ApiError(rep.status, err));
        }
        let user: User = serde_json::from_str(&body)?;
        Ok(user)
    }

    pub fn emails(&self, token: &str) -> HubResult<Vec<Email>> {
        let url = Url::parse(&format!("{}/user/emails", self.url)).unwrap();
        let mut rep = http_get(url, token)?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = serde_json::from_str(&body)?;
            return Err(HubError::ApiError(rep.status, err));
        }
        let emails: Vec<Email> = serde_json::from_str(&body)?;
        Ok(emails)
    }

    pub fn orgs(&self, token: &str) -> HubResult<Vec<Organization>> {
        let url = Url::parse(&format!("{}/user/orgs", self.url)).map_err(
            HubError::HttpClientParse,
        )?;
        let mut rep = http_get(url, token)?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = serde_json::from_str(&body)?;
            return Err(HubError::ApiError(rep.status, err));
        }
        let orgs: Vec<Organization> = serde_json::from_str(&body)?;
        Ok(orgs)
    }

    pub fn search_file(&self, token: &str, repo: &str, file: &str) -> HubResult<Search> {
        let url = Url::parse(&format!(
            "{}/search/code?q={}+in:path+repo:{}",
            self.url,
            file,
            repo
        )).map_err(HubError::HttpClientParse)?;
        let mut rep = http_get_preview(url, token)?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = serde_json::from_str(&body)?;
            return Err(HubError::ApiError(rep.status, err));
        }
        let search = serde_json::from_str::<Search>(&body)?;
        Ok(search)
    }

    pub fn teams(&self, token: &str) -> HubResult<Vec<Team>> {
        let url = Url::parse(&format!("{}/user/teams", self.url)).map_err(
            HubError::HttpClientParse,
        )?;
        let mut rep = http_get(url, token)?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = serde_json::from_str(&body)?;
            return Err(HubError::ApiError(rep.status, err));
        }
        let teams: Vec<Team> = serde_json::from_str(&body)?;
        Ok(teams)
    }
}

fn generate_app_token<T>(key_path: T) -> String
where
    T: ToString,
{
    let mut payload = jwt::Payload::new();
    let header = jwt::Header::new(jwt::Algorithm::RS256);
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let expiration = now + Duration::from_secs(10 * 60);
    payload.insert("iat".to_string(), now.as_secs().to_string());
    payload.insert("exp".to_string(), expiration.as_secs().to_string());
    payload.insert("iss".to_string(), 5565.to_string());
    jwt::encode(header, key_path.to_string(), payload)
}

fn http_get<T>(url: Url, token: T) -> HubResult<hyper::client::response::Response>
where
    T: ToString,
{
    hyper_client()
        .get(url)
        .header(Accept(vec![
            qitem(
                Mime(TopLevel::Application, SubLevel::Json, vec![])
            ),
        ]))
        .header(Authorization(Bearer { token: token.to_string() }))
        .header(UserAgent(USER_AGENT.to_string()))
        .send()
        .map_err(HubError::HttpClient)
}

fn http_get_preview<T>(url: Url, token: T) -> HubResult<hyper::client::response::Response>
where
    T: ToString,
{
    hyper_client()
        .get(url)
        .header(Accept(vec![
            qitem(
                Mime(TopLevel::Application, SubLevel::Json, vec![])
            ),
            qitem(
                "application/vnd.github.machine-man-preview+json"
                    .parse()
                    .unwrap()
            ),
        ]))
        .header(Authorization(Bearer { token: token.to_string() }))
        .header(UserAgent(USER_AGENT.to_string()))
        .send()
        .map_err(HubError::HttpClient)
}

fn http_post(url: Url) -> HubResult<hyper::client::response::Response> {
    hyper_client()
        .post(url)
        .header(Accept(vec![
            qitem(
                Mime(TopLevel::Application, SubLevel::Json, vec![])
            ),
        ]))
        .header(UserAgent(USER_AGENT.to_string()))
        .send()
        .map_err(HubError::HttpClient)
}

fn http_post_preview<T>(url: Url, token: T) -> HubResult<hyper::client::response::Response>
where
    T: ToString,
{
    hyper_client()
        .post(url)
        .header(Accept(vec![
            qitem(
                Mime(TopLevel::Application, SubLevel::Json, vec![])
            ),
            qitem(
                "application/vnd.github.machine-man-preview+json"
                    .parse()
                    .unwrap()
            ),
        ]))
        .header(Authorization(Bearer { token: token.to_string() }))
        .header(UserAgent(USER_AGENT.to_string()))
        .send()
        .map_err(HubError::HttpClient)
}

fn hyper_client() -> hyper::Client {
    let ssl = OpensslClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let mut client = hyper::Client::with_connector(connector);
    client.set_read_timeout(Some(Duration::from_millis(HTTP_TIMEOUT)));
    client.set_write_timeout(Some(Duration::from_millis(HTTP_TIMEOUT)));
    client
}