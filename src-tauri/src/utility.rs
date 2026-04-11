use url::Url;

pub fn normalize_to_url(input: &str) -> Result<String, String> {
    let trimmed = input.trim();

    // If it already parses, return it
    if let Ok(url) = Url::parse(trimmed) {
        return Ok(url.to_string());
    }

    // Try adding https:// only if input is of the form *.*
    if trimmed.contains('.') {
        let with_scheme = format!("https://{}", trimmed);

        match Url::parse(&with_scheme) {
            Ok(url) => Ok(url.to_string()),
            Err(_) => Err("Invalid URL".into()),
        }
    } else {
        Err("Invalid URL".into())
    }
}

pub fn normalize_or_search(input: &str) -> String {
    match normalize_to_url(input) {
        Ok(url) => url,
        Err(_) => {
            format!(
                "https://www.google.com/search?q={}",
                urlencoding::encode(input)
            )
        }
    }
}