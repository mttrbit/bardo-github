// Tokio/Future Imports
use futures::future::ok;
use futures::{Future, Stream};
use hyper::header::{HeaderName, HeaderValue, IF_NONE_MATCH};
use hyper::StatusCode;
use hyper::{self, Body, HeaderMap};
use hyper::{Client, Request};

#[cfg(feature = "rustls")]
type HttpsConnector = hyper_rustls::HttpsConnector<hyper::client::HttpConnector>;
#[cfg(feature = "rust-native-tls")]
use hyper_tls;
#[cfg(feature = "rust-native-tls")]
type HttpsConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;


use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;

use crate::errors::*;
use crate::util::url_join;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Github {
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
exec!(CustomQuery);

pub trait Executor {
    fn execute<T>(self) -> Result<(HeaderMap, StatusCode, Option<T>)>
    where
        T: DeserializeOwned;
}

impl Github {
    pub fn new<T>(token: T) -> Result<Self>
    where
        T: ToString,
    {
        #[cfg(feature = "rustls")]
        let client = Client::builder().build(HttpsConnector::new());
        #[cfg(feature = "rust-native-tls")]
        let client = Client::builder().build(HttpsConnector::new());
        Ok(Self {
            token: token.to_string(),
            client: Rc::new(client),
        })
    }

    /// Get the currently set Authorization Token
    pub fn get_token(&self) -> &str {
        &self.token
    }

    pub fn get(&self) -> GetQueryBuilder {
        self.into()
    }
}

impl<'g> GetQueryBuilder<'g> {
    func_client!(custom_endpoint, CustomQuery, endpoint_str);
}

from!(
    @GetQueryBuilder => "GET"
);

from!(
    @GetQueryBuilder => CustomQuery
);

impl<'a> CustomQuery<'a> {
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
    #[test]
    fn setAndLoad_token() {
        //let client = Github::new("123456789");
        //assert_eq!("123456789", client.get_token());
    }
}
