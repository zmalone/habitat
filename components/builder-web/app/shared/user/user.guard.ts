import { Injectable } from "@angular/core";
import { CanActivate, Router } from "@angular/router";
import { AppStore } from "../../AppStore";
import config from "../../config";

@Injectable()
export class UserLoggedInGuard implements CanActivate {

  constructor(private store: AppStore, private router: Router) { }

  canActivate() {
    const qs = window.location.search;
    const hasCode = !!qs.match(/[\?&]code=[\w-]+/);
    const hasToken = !!this.store.getState().gitHub.authToken;

    if (hasCode || hasToken) {
      return true;
    }

    window.location.href = config["www_url"];
    return false;
  }
}

@Injectable()
export class UserLoggedOutGuard implements CanActivate {

  constructor(private store: AppStore, private router: Router) { }

  canActivate() {
    const qs = window.location.search;
    const hasCode = !!qs.match(/[\?&]code=[\w-]+/);
    const hasToken = !!this.store.getState().gitHub.authToken;

    if (!hasCode || !hasToken) {
      return true;
    }

    window.location.href = config["www_url"];
    return false;
  }
}
