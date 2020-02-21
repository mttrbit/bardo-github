use tokio_core::reactor::Core;

use hyper::{self, Body, HeaderMap};
use hyper::{Client, Request};

#[cfg(feature = "rustls")]
type HttpsConnector = hyper_rustls::HttpConnector<hyper::client::HttpConnector>;

#[cfg(feature = "rust-native-tls")]
use hyper_tls;

#[cfg(feature = "rust-native-tls")]
type HttpsConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;

use std::cell::RefCell;
use std::rc::Rc;

struct BaseClient {
    token: String,
    core: Rc<RefCell<Core>>,
    client: Rc<Client<HttpsConnector>>,
}

new_type!(GetQueryBuilder);

impl Github {
    pub fn new<T>(token: T) -> Result<Self>
    where
        T: ToString,
    {
        let core = Core::new()?;
        #[cfg(feature = "rustls")]
        let client = Client::builder().build(HttpsConnector::new(4));
        #[cfg(feature = "rust-native-tls")]
        let client = Client::builder().build(HttpsConnector::new(4));
        Ok(Self {
            token: token.to_string(),
            core: Rc::new(RefCell::new(core)),
            client: Rc::new(client),
        })
    }

    /// Get the currently set Authorization Token
    pub fn get_token(&self) -> &str {
        &self.token
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn setAndLoad_token() {
        let client = Github::new("123456789");
        assert_eq!("123456789", client.get_token());
    }
}
