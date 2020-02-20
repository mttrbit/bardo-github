use hyper::{Client, Request};

#[cfg(feature = "rustls")]
type HttpsConnector = hyper_rustls::HttpConnector<hyper::client::HttpConnector>;

#[cfg(feature = "rust-native-tls")]
use hyper_tls;

#[cfg(feature = "rust-native-tls")]
type HttpsConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;

use std::cell::RefCell;
use std::rc::Rc;

struct ClientOptions {
    auth_token: &str,
}

struct BaseClient {
    opts: ClientOptions,
    client: Rc<Client<HttpsConnector>>,
}
