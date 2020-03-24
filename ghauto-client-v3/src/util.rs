use reqwest::Url;


/// Add an extra subdirectory to the end of the url. This utilizes
/// Hyper's more generic Uri type. We've set it up to act as a Url.
pub fn url_join(url: &Url, path: &str) -> Result<Url, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let url_str = url.to_string();
    if url_str.ends_with("/") {
        return Ok(url.join(path)?);
    } else {
        let u = url_str + "/" + path;
        return Ok(reqwest::Url::parse(&u)?);
    }
}
