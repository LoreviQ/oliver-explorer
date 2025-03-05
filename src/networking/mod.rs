use std::error::Error;

pub fn fetch_url(url: &str) -> Result<String, Box<dyn Error>> {
    let body = reqwest::blocking::get(url)?.text()?;
    Ok(body)
}
