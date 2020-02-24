// Tokio/Future Imports
use futures::future::ok;
use futures::{Future, Stream};
use tokio_core::reactor::Core;

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

use std::cell::RefCell;
use std::rc::Rc;

pub struct Github {
    token: String,
    core: Rc<RefCell<Core>>,
    client: Rc<Client<HttpsConnector>>,
}

impl Clone for Github {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            core: Rc::clone(&self.core),
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
        let core = Core::new()?;
        #[cfg(feature = "rustls")]
        let client = Client::builder().build(HttpsConnector::new());
        #[cfg(feature = "rust-native-tls")]
        let client = Client::builder().build(HttpsConnector::new());
        Ok(Self {
            token: token.to_string(),
            core: Rc::new(RefCell::new(core)),
            client: Rc::new(client),
        })
    }

    pub fn get_core(&self) -> &Rc<RefCell<Core>> {
        &self.core
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
