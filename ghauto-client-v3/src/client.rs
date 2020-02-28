use async_trait::async_trait;
use hyper::header::{HeaderName, HeaderValue, IF_NONE_MATCH};
use hyper::StatusCode;
use hyper::{Body, Client, HeaderMap};

#[cfg(feature = "rustls")]
type HttpsConnector = hyper_rustls::HttpsConnector<hyper::client::HttpConnector>;
#[cfg(feature = "rust-native-tls")]
use hyper_tls;
#[cfg(feature = "rust-native-tls")]
type HttpsConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;

use crate::bytes::Buf;
use bytes::buf::ext::BufExt;
use serde::de::DeserializeOwned;

// pub struct Github {
//     token: String,
//     client: Rc<Client<HttpsConnector>>,
// }

// impl Clone for Github {
//     fn clone(&self) -> Self {
//         Self {
//             token: self.token.clone(),
//             client: Rc::clone(&self.client),
//         }
//     }
// }

// impl Github {
//     pub fn new<T>(token: T) -> Result<Self>
//     where
//         T: ToString,
//     {
//         #[cfg(feature = "rustls")]
//         let client = Client::builder().build(HttpsConnector::new());
//         #[cfg(feature = "rust-native-tls")]
//         let client = Client::builder().build(HttpsConnector::new());
//         Ok(Self {
//             token: token.to_string(),
//             client: Rc::new(client),
//         })
//     }

//     /// Get the currently set Authorization Token
//     pub fn get_token(&self) -> &str {
//         &self.token
//     }

//     pub fn get(&self) -> GetQueryBuilder {
//         self.into()
//     }
// }
//

// #[derive(Deserialize, Serialize, Clone, Debug)]
// pub struct Response {
//     pub headers: HeaderMap,
//     pub status_code: StatusCode,
//     pub body: Option<User>,
// }

#[derive(Deserialize, Serialize, Clone, Debug)]
struct User {
    id: i32,
    name: String,
}

// impl From<(HeaderMap, StatusCode, Option<User>)> for Response {
//     fn from(x: (HeaderMap, StatusCode, Option<User>)) -> Self {
//         let (headers, status_code, body) = x;
//         Self {
//             headers: headers,
//             status_code: status_code,
//             body: body,
//         }
//     }
// }
//

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

macro_rules! exec {
    ($t: ident) => {
        #[async_trait]
        impl Service for $t {
            async fn execute<T>() -> Result<(HeaderMap, StatusCode, Option<T>)>
            where
                T: DeserializeOwned,
            {
                let url_str = "http://jsonplaceholder.typicode.com/usersr2";
                let url = url_str.parse().expect("Failed to parse URL");
                let client = Client::builder().build::<_, Body>(HttpsConnector::new());
                let res = client.get(url).await?;
                let headers = res.headers().clone();
                let status = res.status();
                let bytes = res.into_bytes();
                println!("body: {:?}", res.body());

                let body = hyper::body::aggregate(res).await?;
                println!("request finished-- returning response");
                println!("Response: {}", status);

                let res = serde_json::from_reader(body.reader())?;

                Ok((headers, status, res))
            }
        }
    };
}

struct Svc;

#[async_trait]
pub trait Service {
    async fn execute<T>() -> Result<(HeaderMap, StatusCode, Option<T>)>
    where
        T: DeserializeOwned;
}

exec!(Svc);

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn set_and_load_token() {
        let response = Svc::execute::<Vec<User>>().await;
        // let (_, _, users) = response.ok();
        println!("data: {:#?}", response.ok());
    }
}
