use client::client:: Result;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use std::collections::HashMap;

// A simple type alias so as to DRY.
pub type HttpResponse<T> = (HeaderMap, StatusCode, Option<T>);

pub trait Command<T> {
    fn execute(&self) -> Result<HttpResponse<T>>;
}

pub trait IterableCommand<T> {
    fn execute_iter(&self) -> ResultIterator<T>;
}

pub struct ResultIterator<'a, T> {
    service_call: Box<dyn Fn(&str) -> Result<HttpResponse<T>> + 'a>,
    page: Option<String>,
}

impl<'a, T> ResultIterator<'a, T> {
    pub fn new(
        service_call: Box<dyn Fn(&str) -> Result<HttpResponse<T>> + 'a>,
        page: Option<String>,
    ) -> Self {
        Self {
            service_call: service_call,
            page: page,
        }
    }

    pub fn service_call(
        &self,
    ) -> &Box<dyn Fn(&str) -> Result<HttpResponse<T>> + 'a> {
        &self.service_call
    }

    pub fn page(&self) -> Option<&String> {
        self.page.as_ref()
    }

    pub fn page_mut(&mut self) -> &mut Option<String> {
        &mut self.page
    }
}

impl<'a, T> Iterator for ResultIterator<'a, T> {
    type Item = Result<HttpResponse<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.page() {
            Some(num) => {
                let result = match self.service_call()(num) {
                    Ok(res) => {
                        let headers = res.0.clone();
                        *self.page_mut() = crate::cmd::read_page_from_link_header(&headers);
                        Ok(res)
                    }
                    Err(e) => {
                        *self.page_mut() = None;
                        Err(e)
                    }
                };

                Some(result)
            }
            _ => None,
        }
    }
}

pub fn read_page_from_link_header(headers: &HeaderMap) -> Option<String> {
    fn get_key_next(links: client::headers::Links) -> Option<HashMap<String, String>> {
        links.get("next").cloned()
    }

    fn get_key_page(next: HashMap<String, String>) -> Option<String> {
        next.get("page").cloned()
    }

    client::headers::link(&headers)
        .and_then(get_key_next)
        .and_then(get_key_page)
}

pub trait PrintStd {
    fn to_std_out(&self);
}

pub trait CommandExecutor {

    fn execute(&self, args: &Vec<Vec<&str>>);
}
