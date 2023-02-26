use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::ApiRequest;

#[derive(Deserialize, Serialize, Debug)]
pub struct Users {
    username: String,
    password: String,
}

impl Users {
    fn validate(value: &Value) -> Result<Self, String> {
        let username = match value.get("username") {
            Some(username) => match username.as_str() {
                Some(username) => username.to_string(),
                None => return Err("Username is not a string".to_string()),
            },
            None => return Err("Username is not given".to_string()),
        };
        let password = match value.get("password") {
            Some(password) => match password.as_str() {
                Some(password) => password.to_string(),
                None => return Err("Password is not a string".to_string()),
            },
            None => return Err("Password is not given".to_string()),
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
                error: None,
                data,
            },
            Err(err) => Self {
                _raw: basic._raw,
                model: basic.model,
                action: basic.action,
                method: basic.method,
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
            return HttpResponse::BadRequest().body(err);
        }
        HttpResponse::Ok().body(format!(
            "called method {} with action {}. Data: {:?}",
            self.model, self.action, self.data
        ))
    }
}
