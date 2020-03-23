use reqwest::Url;


/// Add an extra subdirectory to the end of the url. This utilizes
/// Hyper's more generic Uri type. We've set it up to act as a Url.
pub fn url_join(url: &Url, path: &str) -> Result<Url, Box<dyn std::error::Error + Send + Sync + 'static>> {
    Ok(url.join(path)?)
}
