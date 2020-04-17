use hyper::header::{HeaderName, HeaderValue};
use hyper::{HeaderMap, StatusCode};

use reqwest::blocking::{Client, Request};
use reqwest::{Url, Method};
use serde::de::DeserializeOwned;
use serde::Serialize;

use std::cell::RefCell;
use std::rc::Rc;

use crate::util::url_join;

#[derive(Deserialize, Serialize, Clone, Debug)]
struct User {
    id: i32,
    name: String,
}

// A simple type alias so as to DRY.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

pub struct Github {
    token: String,
    client: Rc<Client>,
}

impl Clone for Github {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            client: Rc::clone(&self.client),
        }
    }
}

new_type!(GetQueryBuilder);
new_type!(PostQueryBuilder);
new_type!(PutQueryBuilder);
new_type!(CustomQuery);
exec!(CustomQuery);

pub trait Executor {
    fn execute<T>(self) -> Result<(HeaderMap, StatusCode, Option<T>)>
    where
        T: DeserializeOwned;
}

impl Github {
    pub fn new<T>(token: T) -> Self
    where
        T: ToString,
    {
        let client = Client::new();
        Self {
            token: token.to_string(),
            client: Rc::new(client),
        }
    }

    /// Get the currently set Authorization Token
    pub fn get_token(&self) -> &str {
        &self.token
    }

    /// Change the currently set Authorization Token using a type that can turn
    /// into an &str. Must be a valid API Token for requests to work.
    pub fn set_token<T>(&mut self, token: T)
    where
        T: ToString,
    {
        self.token = token.to_string();
    }

    /// Begin building up a GET request to GitHub
    pub fn get(&self) -> GetQueryBuilder {
        self.into()
    }

    pub fn post<T>(&self, body: T) -> PostQueryBuilder
    where
        T: Serialize,
    {
        let mut qb: PostQueryBuilder = self.into();
        if let Ok(mut qbr) = qb.request {
            let serialized = serde_json::to_vec(&body);
            match serialized {
                Ok(json) => {
                    let json_str: Vec<u8> = json.into();
                    let body = reqwest::blocking::Body::from(json_str);
                    *qbr.get_mut().body_mut() = Some(body);
                    qb.request = Ok(qbr);
                }
                Err(_) => {
                    qb.request = Err("Unable to serialize data to JSON".into());
                }
            }
        }

        qb
    }

    pub fn put<T>(&self, body: T) -> PutQueryBuilder
    where
        T: Serialize,
    {
        let mut qb: PutQueryBuilder = self.into();
        if let Ok(mut qbr) = qb.request {
            let serialized = serde_json::to_vec(&body);
            match serialized {
                Ok(json) => {
                    let json_str: Vec<u8> = json.into();
                    let body = reqwest::blocking::Body::from(json_str);
                    *qbr.get_mut().body_mut() = Some(body);
                    qb.request = Ok(qbr);
                }
                Err(_) => {
                    qb.request = Err("Unable to serialize data to JSON".into());
                }
            }
        }

        qb
    }
}

impl<'g> GetQueryBuilder<'g> {
    func_client!(custom_endpoint, CustomQuery, endpoint_str);

    /// Query the user endpoint
    func_client!(user, crate::users::get::User<'g>);

    /// Query the repos endpoint
    func_client!(repos, crate::repos::get::Repos<'g>);

    /// Query the issues endpoint
    func_client!(issues, crate::issues::get::Issues<'g>);

    pub fn set_header(
        mut self,
        header_name: impl Into<HeaderName>,
        accept_header: impl Into<HeaderValue>,
    ) -> Self {
        match self.request {
            Ok(mut req) => {
                req.get_mut()
                    .headers_mut()
                    .insert(header_name.into(), accept_header.into());
                self.request = Ok(req);
                self
            }
            Err(_) => self,
        }
    }
}

impl<'g> PostQueryBuilder<'g> {
    func_client!(custom_endpoint, CustomQuery, endpoint_str);
    func_client!(repos, crate::repos::post::Repos<'g>);
}

impl<'g> PutQueryBuilder<'g> {
    func_client!(custom_endpoint, CustomQuery, endpoint_str);
    func_client!(repos, crate::repos::put::Repos<'g>);
}

// exec!(Github);

from!(
    @GetQueryBuilder
        => "GET"
    @PostQueryBuilder
        => "POST"
    @PutQueryBuilder
        => "PUT"
);

from!(
    @GetQueryBuilder
        => CustomQuery
    @PostQueryBuilder
        => CustomQuery
    @PutQueryBuilder
        => CustomQuery
);

impl<'a> CustomQuery<'a> {
    /// Set custom header for request.
    /// Useful for custom headers (sometimes using in api preview).
    pub fn set_header(
        mut self,
        header_name: impl Into<HeaderName>,
        accept_header: impl Into<HeaderValue>,
    ) -> Self {
        match self.request {
            Ok(mut req) => {
                req.get_mut()
                    .headers_mut()
                    .insert(header_name.into(), accept_header.into());
                self.request = Ok(req);
                self
            }
            Err(_) => self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn auth_token() -> Result<String> {
        Ok("test_token".to_string())
    }

    fn setup_github_connection() -> Github {
        Github::new(ghauto_config::credentials::access_token().unwrap().to_string())
    }

    #[test]
    fn set_and_load_token() {
        let g = setup_github_connection()
            .get()
            .custom_endpoint("users")
            .execute::<serde_json::Value>()
            //.execute::<Vec<User>>()
            .unwrap();
        // println!("response {:#?}", g);
        //.expect("Connection failed");

        // let response = CustomQuery::execute::<Vec<User>>().await;
        // let (_, _, users) = response.ok();

        // let u = match g {
        //     Ok(u) => println!("success: {:#?}", u),
        //     Err(e) => {
        //         println!("error: {:#?}", e);
        //     }
        // };
    }

 #[test]
    fn users() {
        let g = setup_github_connection()
            .get()
            .user()
            .emails()
            .execute::<serde_json::Value>()
            //.execute::<Vec<User>>()
            .unwrap();
        println!("response {:#?}", g);
    }

    // #[test]
    // fn auth() {
    //     use crate::gh_auth::*;
    //     github_authorize();
    // }
}
