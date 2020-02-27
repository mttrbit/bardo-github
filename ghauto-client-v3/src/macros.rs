/// asasa
macro_rules! from {
    ($(@$f: ident
       $( ?> $i1: ident = $e1: tt )*
       $( => $t: ident )*
       $( -> $i2: ident = $e2: tt )*
    )* ) => (
        $($(
            impl <'g> From<$f<'g>> for $t<'g> {
                fn from(f: $f<'g>) -> Self {
                    Self {
                        request: f.request,
                        client: f.client,
                        parameter: None,
                    }
                }
            }
        )*$(
            impl <'g> From<$f<'g>> for $i1<'g> {
                fn from(mut f: $f<'g>) -> Self {
                    use std::str::FromStr;

                    // borrow checking abuse
                    if f.request.is_ok() {
                        // why doe sthis work?
                        let mut req = f.request.unwrap();
                        let url = f.parameter
                            .ok_or("Expecting parameter".into())
                            .and_then(|param| {
                                let sep = if req.get_mut().uri().query().is_some() { "&" } else { "?" };
                                hyper::Uri::from_str(&format!("{}{}{}={}",
                                                              req.get_mut().uri(),
                                                              sep,
                                                              $e1,
                                                              param)
                                ).chain_err(|| "Failed to parse Url")
                            });
                        match url {
                            Ok(u) => {
                                *req.get_mut().uri_mut() = u;
                                f.request = Ok(req);
                            },
                            Err(e) => {
                                f.request = Err(e);
                            }
                        }

                        Self {
                            request: f.request,
                            client: f.client,
                            parameter: None,
                        }
                    } else {
                        Self {
                            request: f.request,
                            client: f.client,
                            parameter: None,
                        }
                    }
                }
            }
        )*$(
            impl <'g> From<$f<'g>> for $i2<'g> {
            fn from(mut f: $f<'g>) -> Self {
                // This is borrow checking abuse and about the only
                // time I'd do is_ok(). Essentially this allows us
                // to either pass the error message along or update
                // the url
                if f.request.is_ok() {
                    // We've checked that this works
                    let mut req = f.request.unwrap();
                    let url = url_join(req.borrow().uri(), $e2);
                    match url {
                        Ok(u) => {
                            *req.get_mut().uri_mut() = u;
                            f.request = Ok(req);
                        },
                        Err(e) => {
                            f.request = Err(e.into());
                        }
                    }

                    Self {
                        request: f.request,
                        client: f.client,
                        parameter: None,
                    }

                } else {

                    Self {
                        request: f.request,
                        client: f.client,
                        parameter: None,
                    }

                }
            }
        }
        )*)*
    );
    ($(@$t: ident => $p: expr)*) => (
        $(
        impl <'g> From<&'g Github> for $t<'g> {
            fn from(gh: &'g Github) -> Self {
                use hyper::header::{ ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT };
                let res = Request::builder().method($p)
                    .uri("https://api.github.com")
                    .body(hyper::Body::empty())
                    .map_err(From::from)
                    .and_then(|req| {
                        let token = String::from("token ") + &gh.token;
                        HeaderValue::from_str(&token).map(|token| (req, token))
                            .map_err(From::from)
                    });
                match res {
                    Ok((mut req, token)) => {
                        {
                            let headers = req.headers_mut();
                            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                            headers.insert(USER_AGENT, HeaderValue::from_static("ghauto"));
                            headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.github.v3+json"));
                            headers.insert(AUTHORIZATION, token);
                        }
                        Self {
                            request: Ok(RefCell::new(req)),
                            client: &gh.client,
                            parameter: None,
                        }
                    }
                    Err(err) => {
                        Self {
                            request: Err(err),
                            client: &gh.client,
                            parameter: None,
                        }
                    }
                }
            }
        }
    )*
    );
}

/// Used to identify a new type used in a query pipeline. The types are
/// consistent between each one in terms of transforming one to another.
/// This helps reduce boiler plate code and makes it easy to exapnd and
/// maintain code in the future by simply adding a new field here if needed
macro_rules! new_type {
    ($($i: ident)*) => (
        $(
            pub struct $i<'g> {
                pub(crate) request: Result<RefCell<Request<Body>>>,
                pub(crate) client: &'g Rc<Client<HttpsConnector>>,
                pub(crate) parameter: Option<String>,
            }
        )*
    );
}


/// Used to generate an execute function for a terminal type in a query
/// pipeline. If passed a type it creates the impl as well as it needs
/// no extra functions.
macro_rules! exec {
    ($t: ident) => {
        impl<'a> Executor for $t<'a> {
            /// Execute the query by sending the built up request to GitHub.
            /// The value returned is either an error or the Status Code and
            /// Json after it has been deserialized. Please take a look at
            /// the GitHub documentation to see what value you should receive
            /// back for good or bad requests.
            fn execute<T>(self) -> Result<(HeaderMap, StatusCode, Option<T>)>
            where
                T: DeserializeOwned,
            {
                let client = self.client;
                let request = self.request?.into_inner();
                    let join = tokio::task::block_in_place(|| async {
                        let response = client.request(request).await;
                        response.and_then(|res| {
                            let header = res.headers().clone();
                            let status = res.status();
                            Ok((header, status, None))
                        });
                        //let str = String::from_utf8(bytes.to_vec());
                    });


                join
            }
        }
    };
}

macro_rules! func_client {
    ($i: ident, $t: ty) => (
        pub fn $i(self) -> $t {
            self.into()
        }
    );
    ($i: ident, $t: ident, $e: ident) => (
        pub fn $i(mut self, $e: &str) -> $t<'g> {
            if self.request.is_ok() {
                let mut req = self.request.unwrap();
                let url = url_join(req.borrow().uri(), $e);
                match url {
                    Ok(u) => {
                        *req.get_mut().uri_mut() = u;
                        self.request = Ok(req);
                    },
                    Err(e) => {
                        self.request = Err(e.into());
                    }
                }
            }
            self.into()
        }
    );
}
