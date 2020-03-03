use async_trait::async_trait;
use hyper::header::{HeaderName, HeaderValue, IF_NONE_MATCH};
use hyper::{Response, Request, Body, Client, HeaderMap, StatusCode};

#[cfg(feature = "rustls")]
type HttpsConnector = hyper_rustls::HttpsConnector<hyper::client::HttpConnector>;
#[cfg(feature = "rust-native-tls")]
use hyper_tls;
#[cfg(feature = "rust-native-tls")]
type HttpsConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;

// use crate::futures::TryFutureExt;
use bytes::buf::ext::BufExt;
use serde::de::DeserializeOwned;

use std::cell::RefCell;
use std::rc::Rc;

use crate::util::url_join;

#[derive(Deserialize, Serialize, Clone, Debug)]
struct User {
    id: i32,
    name: String,
}

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

struct Github {
    token: String,
    client: Rc<Client<HttpsConnector>>,
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
new_type!(CustomQuery);

impl Github {
    pub fn new<T>(token: T) -> Result<Self>
    where
        T: ToString,
    {
        #[cfg(feature = "rustls")]
        let client = Client::builder().build(HttpsConnector::new());
        #[cfg(feature = "rust-native-tls")]
        let client = Client::builder().build(HttpsConnector::new()?);
        Ok(Self {
            token: token.to_string(),
            client: Rc::new(client),
        })
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
}

impl<'g> GetQueryBuilder<'g> {
     func_client!(custom_endpoint, CustomQuery, endpoint_str);
}


#[async_trait]
pub trait Executor {
    async fn execute<T>() -> Result<(HeaderMap, StatusCode, Option<T>)>
    where
        T: DeserializeOwned;
}

//exec!(Github);

exec!(CustomQuery);

from!(
    @GetQueryBuilder => "GET"
);

from!(
    @GetQueryBuilder => CustomQuery
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

    #[tokio::test]
    async fn set_and_load_token() {
        let response = CustomQuery::execute::<Vec<User>>().await;
        // let (_, _, users) = response.ok();

        let u = match response {
            Ok(u) => println!("success: {:#?}", u),
            Err(e) => {
                println!("error: {:#?}", e);
            },
        };
    }
}
