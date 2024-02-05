use regex::Regex;

pub fn extract_code_block(text: &str) -> Result<Option<String>, String> {
    let re = Regex::new(r"```(?s)(.*?)```");
    let re = match re {
        Ok(re) => re,
        Err(_) => return Err("Failed to create regex".to_string()),
    };
    let result = re
        .captures(text)
        .and_then(|cap| cap.get(1))
        .map(|match_| match_.as_str().trim().to_string());
    Ok(result)
}
