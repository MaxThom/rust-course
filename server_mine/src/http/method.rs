pub enum Method {
    GET(String),
    DELETE(u64),
    PUT,
    POST,
    PATCH,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
}

