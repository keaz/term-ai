use regex::Regex;

pub fn extract_code_block(text: &str) -> Option<String> {
    let re = Regex::new(r"```(?s)(.*?)```").unwrap();
    re.captures(text)
        .and_then(|cap| cap.get(1))
        .map(|match_| match_.as_str().trim().to_string())
}
