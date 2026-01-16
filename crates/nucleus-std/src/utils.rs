pub fn url_encode(s: &str) -> String {
    urlencoding::encode(s).to_string()
}
