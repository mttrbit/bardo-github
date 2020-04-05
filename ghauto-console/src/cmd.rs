use client::client::Result;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use std::collections::HashMap;

// A simple type alias so as to DRY.
pub type HttpResponse<T> = (HeaderMap, StatusCode, Option<T>);

pub trait Command<T> {
    fn execute(&self) -> Result<(HeaderMap, StatusCode, Option<T>)>;
}

pub trait FetchAll<T> {
    fn fetch_all(&self, vec: & mut T);

    fn read_page_from_link_header(&self, headers: &HeaderMap) -> Option<String> {
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
}
