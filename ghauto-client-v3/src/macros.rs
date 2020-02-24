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
                        core: f.core,
                        client: f.client,
                        parameter: None,
                    }
                }
            }
        )*$(
            impl <'g> From<$f<'g>> for $i1<'g> {
                fn from(mut f: $f<'g>) -> Self {
                    use std::str::FromStr;
                }
            }
        ))
    )
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
                pub(crate) core: &'g Rc<RefCell<Core>>,
                pub(crate) client: &'g Rc<Client<HttpsConnector>>,
                pub(crate) parameter: Option<String>,
            }
        )*
    );
}

macro_rules! exec {
    ($t: ident) => {
        impl<'a> Executor for $t<'a> {
            fn execute<T>(self) -> Result<(HeaderMap, StatusCode, Option<T>)>
            where
                T: DeserializeOwned,
            {
                let mut core_ref = self.core.try_borrow_mut()?;
                let client = self.client;
                let work = client.request(self.request?.into_inner()).and_then(|res| {
                    let header = res.headers().clone();
                    let status = res.status();
                    res.into_body()
                       .fold(Vec::new(), |mut v, chunk| {
                           v.extend(&chunk[..]);
                           ok::<_, hyper::Error>(v)
                       })
                       .map(move |chunks|{
                           if chunks.is_empty() {
                               Ok((header, status, None))
                           } else {
                               Ok((header, status, Some(serde_json::from_slice(&chunks)?)))
                           }
                       })
                });
                core_ref.run(work)?
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
                let url = url_join(req.borrow().url(), $e);
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
