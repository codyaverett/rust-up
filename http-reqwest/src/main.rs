extern crate reqwest;

use std::collections::HashMap;

fn main() -> Result<(), Box<std::error::Error>> {
    let resp: HashMap<String, String> = reqwest::get(
        "https://support.oneskyapp.com/hc/en-us/article_attachments/202761627/example_1.json",
    )?.json()?;
    println!("{:#?}", resp);
    Ok(())
}
