#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rand;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::collections::HashMap;
use std::sync::Mutex;

use rand::Rng;
use rocket_contrib::{Json, Value};
use rocket::{Outcome, State};
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

// Authentication
struct ApiKey(String);

fn is_valid(key: &str) -> bool {
    let parts: Vec<&str> = key.split(' ').collect();
    parts.len() == 2 && parts[0] == "Bearer" && parts[1] == "bobo"
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();

        if keys.len() != 1 {
            return Outcome::Failure((Status::Forbidden, ()));
        }

        let key = keys[0];

        if !is_valid(key) {
            return Outcome::Failure((Status::Forbidden, ()));
        }

        return Outcome::Success(ApiKey(key.to_string()));
    }
}

// this is our temp db while we get a handle on what it means to write stuff in rocket
type Db = Mutex<HashMap<String, Origin>>;

#[derive(Clone, Serialize, Deserialize, Hash, PartialEq)]
struct Origin {
    id: u64,
    name: String,
    visibility: String,
}

#[derive(Serialize, Deserialize)]
struct OriginCreate {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct OriginUpdate {
    visibility: String,
}

#[get("/origins/<name>", format = "application/json")]
fn show(name: String, map: State<Db>) -> Option<Json<Origin>> {
    let db = map.lock().expect("map lock is poisoned");
    db.get(&name).map(|contents| Json(contents.clone()))
}

// these 2 require auth, probably implemented as request guards
#[post("/origins", format = "application/json", data = "<origin>")]
fn create(origin: Json<OriginCreate>, map: State<Db>, _key: ApiKey) -> Json<Value> {
    let mut rng = rand::thread_rng();
    let id = rng.gen::<u64>();
    let name = origin.0.name;

    let origin = Origin {
        id: id,
        name: name.clone(),
        visibility: "public".to_string(),
    };

    let mut db = map.lock().expect("map lock is poisoned");

    if db.contains_key(&name) {
        Json(json!({"status": "error", "reason": "ID exists. Try put."}))
    } else {
        let json = json!(&origin);
        db.insert(name, origin);
        Json(json)
    }
}

#[put("/origins/<name>", format = "application/json", data = "<origin>")]
fn update(
    name: String,
    origin: Json<OriginUpdate>,
    map: State<Db>,
    _key: ApiKey,
) -> Option<Json<Origin>> {
    let mut db = map.lock().expect("map lock is poisoned");

    if db.contains_key(&name) {
        let o = db.get_mut(&name).unwrap();
        o.visibility = origin.0.visibility;
        Some(Json(o.clone()))
    } else {
        None
    }
}

#[error(404)]
fn not_found() -> Json<Value> {
    Json(
        json!({"status": "error", "reason": "Resource was not found."}),
    )
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/v2", routes![show, create, update])
        .catch(errors![not_found])
        .manage(Mutex::new(HashMap::<String, Origin>::new()))
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::{Accept, ContentType, Header, Status};
    use serde_json;

    #[test]
    fn test_the_entire_api() {
        // First let's create an origin
        let client = Client::new(rocket()).unwrap();
        let bobo = Header::new("Authorization", "Bearer bobo");
        let creation_response = client
            .post("/v2/origins")
            .header(ContentType::JSON)
            .header(Accept::JSON)
            .header(bobo.clone())
            .body(r#"{"name":"haha"}"#)
            .dispatch();
        assert_eq!(creation_response.status(), Status::Ok);

        // Now let's update it
        let update_response = client
            .put("/v2/origins/haha")
            .header(ContentType::JSON)
            .header(Accept::JSON)
            .header(bobo)
            .body(r#"{"visibility":"private"}"#)
            .dispatch();
        assert_eq!(update_response.status(), Status::Ok);

        // Finally let's fetch it and make sure it matches
        let mut get_response = client.get("/v2/origins/haha").dispatch();
        assert_eq!(get_response.status(), Status::Ok);
        assert_eq!(get_response.content_type(), Some(ContentType::JSON));

        let body_string = get_response.body_string().unwrap();
        let v: serde_json::Value = serde_json::from_str(&body_string).unwrap();
        assert_eq!(v["name"], "haha");
        assert_eq!(v["visibility"], "private");
    }
}
