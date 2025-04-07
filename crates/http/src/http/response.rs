use std::{collections::HashMap, time::SystemTime};

use jiff::{fmt::strtime::format, Timestamp};
use uuid::Uuid;

use super::{Protocol, Status};

#[derive(Debug)]
pub struct Response {
    pub protocol: Protocol,
    pub status: Status,
    pub headers: HashMap<String, String>,
    pub cache_options: Option<CacheOptions>,
    pub cookies: Option<Cookies>,
    pub body: Vec<u8>,
}

const HEADER_LIST: [&'static str; 11] = [
    "accept-ranges",
    "age",
    "cache-control",
    "content-length",
    "content-type",
    "date",
    "proxy-authenticate",
    "server",
    "transfer-encoding",
    "vary",
    "www-authenticate",
];

#[derive(Debug)]
pub struct CacheOptions {
    max_age: Option<u32>,
    no_cache: bool,
    no_store: bool,
    immutable: bool,
    private: bool,
    public: bool,
    last_modified: Timestamp,
    etag: String,
}

impl CacheOptions {
    pub fn new(
        max_age: Option<u32>,
        no_cache: bool,
        no_store: bool,
        immutable: bool,
        private: bool,
        public: bool,
        last_modified: Timestamp,
        etag: String,
    ) -> Self {
        Self {
            max_age,
            no_cache,
            no_store,
            immutable,
            private,
            public,
            last_modified,
            etag,
        }
    }

    pub fn new_default() -> Self {
        Self {
            max_age: None,
            no_cache: true,
            no_store: false,
            immutable: false,
            private: false,
            public: false,
            last_modified: Timestamp::now(),
            etag: Uuid::new_v4().to_string(), // depending on the service provider - here we simply provide uuid4
        }
    }

    pub fn to_cache_related_header(&self) -> String {
        let mut cache_control_header = "Cache-Control: ".to_string();

        if let Some(max_age) = self.max_age {
            cache_control_header.push_str(format!("max_age={}", max_age).as_str());
        }

        if self.no_cache {
            cache_control_header.push_str("no-cache");
        }

        if self.no_store {
            cache_control_header.push_str("no-strore");
        }

        if self.immutable {
            cache_control_header.push_str("immutable");
        }

        if self.private {
            cache_control_header.push_str("private");
        }

        // if public and private are set at the same time, we consider private first
        if self.public && !self.private {
            cache_control_header.push_str("public");
        }

        let last_modified = self.last_modified.strftime("%a, %e %b %Y %I:%M:%S GMT");
        let last_modified_header = format!("Last-Modified: {}", last_modified);

        let etag_header = format!("ETag: {}", self.etag);

        format!(
            "{}\n{}\n{}",
            cache_control_header, last_modified_header, etag_header
        )
    }
}

#[derive(Debug)]
pub struct Cookies {
    values: HashMap<String, String>,
    max_age: Option<u32>,
    secure: bool,
    http_only: bool,
    domain: Option<String>,
    path: Option<String>,
    same_site: Option<String>,
}

impl Cookies {
    pub fn new(
        values: HashMap<String, String>,
        max_age: Option<u32>,
        secure: bool,
        http_only: bool,
        domain: Option<String>,
        path: Option<String>,
        same_site: Option<String>,
    ) -> Self {
        Self {
            values,
            max_age,
            secure,
            http_only,
            domain,
            path,
            same_site,
        }
    }

    pub fn to_set_cookie_header(self) -> String {
        let mut values = self
            .values
            .into_iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<_>>()
            .join("; ");

        if let Some(max_age) = self.max_age {
            values.push_str(format!("; Max-Age={}", max_age).as_str());
        }

        if self.secure {
            values.push_str("; Secure");
        }

        if self.http_only {
            values.push_str("; HttpOnly");
        }

        if let Some(domain) = self.domain {
            values.push_str(format!("; Domain={}", domain).as_str());
        }

        if let Some(path) = self.path {
            values.push_str(format!("; Path={}", path).as_str());
        }

        if let Some(same_site) = self.same_site {
            if let Some(same_site_value) = match same_site.to_lowercase().as_str() {
                "strict" => Some("Strict"),
                "lax" => Some("Lax"),
                "none" => Some("None"),
                _ => None,
            } {
                values.push_str(format!("; SameSite={}", same_site_value).as_str());

                if same_site_value == "Lax" && !self.secure {
                    values.push_str("; Secure");
                }
            }
        }

        format!("Set-Cookie: {}", values)
    }
}

impl Response {
    pub fn new(
        protocol: Protocol,
        status: Status,
        headers: HashMap<String, String>,
        cookies: Option<Cookies>,
        cache_options: Option<CacheOptions>,
        body: Option<Vec<u8>>,
    ) -> Self {
        Self {
            protocol,
            status,
            headers,
            cookies,
            cache_options,
            body: body.unwrap_or(vec![]),
        }
    }

    pub fn encode(self) -> Vec<u8> {
        let (status_code, message) = match self.status {
            Status::OK200(message) => (200, message),
            _ => (400, "bad request".into()),
        };

        let mut headers = self
            .headers
            .into_iter()
            .fold(String::new(), |mut acc, (key, value)| {
                acc.push_str(format!("{}: {}\n", key, value).as_str());
                acc
            });

        if let Some(cache_options) = self.cache_options {
            headers.push_str(cache_options.to_cache_related_header().as_str());
            headers.push('\n');
        }

        if let Some(cookies) = self.cookies {
            headers.push_str(cookies.to_set_cookie_header().as_str());
            headers.push('\n');
        }

        let mut encode_str = format!(
            "{} {} {}\n{}\n",
            self.protocol, status_code, message, headers
        );

        // todo!("learn how to send body");
        // if !self.body.is_empty() {
        // }

        let metadata = encode_str.as_bytes();
        let body = self.body;

        [metadata, &body].concat()
    }
}
