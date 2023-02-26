use actix_web::{http::Method, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use ts_rs::TS;

use super::ApiRequest;

#[derive(Deserialize, Serialize, Debug, TS)]
#[ts(export, export_to = "../app/src/types/api/users.ts")]
pub struct Users {
    username: String,
    password: String,
}

impl Users {
    fn validate(value: &Value) -> Result<Self, (String, u16)> {
        let username = match value.get("username") {
            Some(username) => match username.as_str() {
                Some(username) => {
                    if username.len() == 0 {
                        return Err(("Username can't be empty".to_string(), 400));
                    }
                    username.to_string()
                }
                None => return Err(("Username is not a string".to_string(), 400)),
            },
            None => return Err(("Username is not given".to_string(), 400)),
        };
        let password = match value.get("password") {
            Some(password) => match password.as_str() {
                Some(password) => {
                    if password.len() == 0 {
                        return Err(("Password can't be empty".to_string(), 400));
                    }
                    password.to_string()
                }
                None => return Err(("Password is not a string".to_string(), 400)),
            },
            None => return Err(("Password is not given".to_string(), 400)),
        };
        Ok(Self { username, password })
    }
}

impl ApiRequest<Users> {
    pub fn from_basic(basic: ApiRequest) -> Self {
        match Users::validate(&basic.data) {
            Ok(data) => Self {
                _raw: basic._raw,
                model: basic.model,
                action: basic.action,
                method: basic.method,
                path: basic.path,
                error: None,
                data,
            },
            Err(err) => Self {
                _raw: basic._raw,
                model: basic.model,
                action: basic.action,
                method: basic.method,
                path: basic.path,
                error: Some(err),
                data: Users {
                    username: "".to_string(),
                    password: "".to_string(),
                },
            },
        }
    }
    pub fn evaluate(self) -> HttpResponse {
        if self.error.is_some() {
            let err = self.error.unwrap();
            return super::respond(Err(err));
        }
        match self.method {
            Method::GET => match self.action.as_str() {
                "login" => super::respond(Ok(json!(format!(
                    "called endpoint \"{}/{}\" with data: {:?}",
                    self.model, self.action, self.data
                )))),
                _ => super::respond(Err((format!("Unknown action \"{}\"", self.action), 404))),
            },
            Method::POST => match self.action.as_str() {
                "login" => super::respond(Ok(json!(format!(
                    "called endpoint \"{}/{}\" with data: {:?}",
                    self.model, self.action, self.data
                )))),
                _ => super::respond(Err((format!("Unknown action \"{}\"", self.action), 404))),
            },
            _ => super::respond(Err((
                format!(
                    "Method \"{}\" not allowed on endpoint\"{}/{}\"",
                    self.method, self.model, self.action
                ),
                405,
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    #[test]
    fn validate() {
        let successful_user_list = vec![
            json!({
                "username": "someUser",
                "password": "somePassword",
            }),
            json!({
                "username": "test",
                "password": "test",
            }),
        ];
        for user in successful_user_list {
            assert!(Users::validate(&user).is_ok());
        }

        let failing_user_list = vec![
            json!({}),
            json!({
                "username": "someUser"
            }),
            json!({
                "username": ""
            }),
            json!({
                "username": 32
            }),
            json!({
                "password": "somePassword"
            }),
            json!({
                "password": ""
            }),
            json!({
                "password": false
            }),
            json!({
                "username": "test",
                "password": [],
            }),
            json!({
                "username": "test",
                "password": "",
            }),
            json!({
                "username": "",
                "password": "",
            }),
            json!({
                "username": "",
                "password": "test",
            }),
        ];
        for user in failing_user_list {
            assert!(Users::validate(&user).is_err());
        }
    }
}
