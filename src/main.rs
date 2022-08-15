use curl::easy::{Easy, List};
use std::error::Error;

pub fn post_json_to(url: &str, payload: &str) -> Result<(), Box<dyn Error>> {
    let payload_bytes = payload.as_bytes();

    let mut easy = Easy::new();
    easy.url(&url)?;

    let mut headers = List::new();
    headers.append("Accept: application/json")?;
    headers.append("Content-Type:application/json")?;
    easy.http_headers(headers)?;

    easy.post(true)?;
    easy.post_field_size(payload_bytes.len() as u64)?;
    easy.post_fields_copy(payload_bytes)?;
    let mut response_buf = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|buf| {
            response_buf.extend_from_slice(buf);
            Ok(buf.len())
        })?;
        transfer.perform()?;
    }
    let code = easy.response_code()?;
    if code != 200 && code != 204 {
        let response = String::from_utf8(response_buf)?;
        println!("{}", response);
    }
    Ok(())
}

fn main() {
    post_json_to("https://discord.com/api/webhooks/1008828275591741561/mMQSFo48LWqBKC1bn4cBAyxftDfQ_ZOih_WVevZB5zusPdmL5rYnErsmh1tr6rzSS6Ur", r#"{ }"#).unwrap();
}