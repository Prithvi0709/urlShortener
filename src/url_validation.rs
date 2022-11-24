/*
    Backend Guard for URL validation
    Author: Sam Ul Haque <sam.ulhaque@gwu.edu>
    Rust Group for ASP Fall 2022
*/

use url::{Url};

pub fn period_in_url (url: &str) -> bool {
    let mut period_count = 0;
    for c in url.chars() {
        if c == '.' {
            period_count += 1;
        }
    }
    if period_count > 0 {
        return true;
    }
    false
}

pub fn add_protocol(_url: &str) -> String {
    let url = Url::parse(_url);
    match url {
        Ok(url) => {
            if url.scheme() == "http" || url.scheme() == "https" {
                url.to_string()
            } else {
                format!("http://{}", url)
            }
        }
        Err(_) => format!("http://{}", _url ),
    }
}