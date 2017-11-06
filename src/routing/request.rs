use rocket::request::{FromRequest, Request};
use rocket::{Outcome, http};
use hyper::header::Headers;

#[derive(Debug)]
pub struct ConvertedHeaders {
    pub inner: Headers,
}

impl ConvertedHeaders {
    pub fn as_hyper(self) -> Headers { self.inner }
}

impl <'a, 'req>FromRequest<'a, 'req> for ConvertedHeaders {
    type Error = ();

    fn from_request(request: &'a Request<'req>) -> Outcome<Self, (http::Status, ()), ()> {
        let mut hyper_headers = Headers::new();
        let rocket_headers = request.headers().clone();
        for header in rocket_headers.iter() {
            let header_name = String::from(header.name());
            hyper_headers.append_raw(header_name, String::from(header.value()).as_bytes().to_vec());
        }

        Outcome::Success(ConvertedHeaders {
            inner: hyper_headers
        })
    }
}