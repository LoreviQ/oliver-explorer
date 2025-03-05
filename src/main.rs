use std::error::Error;

fn main() {
    match fetch_url("https://www.oliver.tj/") {
        Ok(html) => println!("Fetched HTML: {}", html),
        Err(e) => println!("Error: {}", e),
    }
}

fn fetch_url(url: &str) -> Result<String, Box<dyn Error>> {
    let body = reqwest::blocking::get(url)?.text()?;
    Ok(body)
}
