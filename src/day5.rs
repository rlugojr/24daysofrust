extern crate hyper;
extern crate serialize;
extern crate url;

use hyper::{HttpError, HttpResult, Url};
use hyper::client::Request;
use hyper::header::ContentLength;
use serialize::{Encodable, json};
use std::io::IoError;
use url::form_urlencoded;

fn get_content(url: &str) -> HttpResult<String> {
    let url = match Url::parse(url) {
        Ok(url) => url,
        Err(_) => return Err(HttpError::HttpUriError),
    };
    let fresh_request = try!(Request::get(url));
    let streaming_request = try!(fresh_request.start());
    let mut response = try!(streaming_request.send());
    Ok(try!(response.read_to_string()))
}

type Query<'a> = Vec<(&'a str, &'a str)>;

fn post_query(url: &str, query: Query) -> HttpResult<String> {
    let url = match Url::parse(url) {
        Ok(url) => url,
        Err(_) => return Err(HttpError::HttpUriError),
    };
    let body = form_urlencoded::serialize(query.into_iter());
    let mut fresh_request = try!(Request::post(url));
    fresh_request.headers_mut().set(ContentLength(body.len()));
    let mut streaming_request = try!(fresh_request.start());
    try!(streaming_request.write_str(body[]));
    let mut response = try!(streaming_request.send());
    Ok(try!(response.read_to_string()))
}

fn post_json<'a, T: Encodable<json::Encoder<'a>, IoError>>(url: &str, payload: &T) -> HttpResult<String> {
    let body = json::encode(payload);
    let url = match Url::parse(url) {
        Ok(url) => url,
        Err(_) => return Err(HttpError::HttpUriError),
    };
    let mut fresh_request = try!(Request::post(url));
    fresh_request.headers_mut().set(ContentLength(body.len()));
    let mut streaming_request = try!(fresh_request.start());
    try!(streaming_request.write_str(body[]));
    let mut response = try!(streaming_request.send());
    Ok(try!(response.read_to_string()))
}

#[deriving(Decodable, Encodable)]
struct Movie {
    title: String,
    bad_guy: String,
    pub_year: uint,
}

fn main() {
    println!("24 days of Rust - hyper (day 5)");
    println!("{}", get_content("http://httpbin.org/status/200"));
    let query = vec![("key", "value"), ("foo", "bar")];
    println!("{}", post_query("http://httpbin.org/post", query));
    let movie = Movie {
        title: "You Only Live Twice".to_string(),
        bad_guy: "Blofeld".to_string(),
        pub_year: 1967,
    };
    println!("{}", post_json("http://httpbin.org/post", &movie));
}
