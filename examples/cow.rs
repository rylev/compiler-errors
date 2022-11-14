fn main() {
    get("https://www.example.com");
}

fn get(url: &str) {
    let url = if url.starts_with("https://") {
        url
    } else {
        format!("https://example.com{url}")
    };
    make_request(Method::Get, url);
}

fn make_request(method: Method, url: String) {
    todo!("Make HTTP Request")
}

enum Method {
    Get,
    Post,
    Put,
    Delete,
}
