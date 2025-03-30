use reqwest::{self, Result};

fn main() -> Result<()> {
    let body = reqwest::blocking::get("https://google.com")?.text()?;
    println!("body={:#}", body);
    Ok(())
 }
