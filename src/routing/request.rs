use rocket::{self, Outcome, http, Response};
use rocket::request::{FromRequest, Request};
use rocket::http::ContentType;
use hyper::header::Headers;
use std::path::{Path, PathBuf, Component};
use std::io::BufReader;
use std::fs::File;

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

pub struct TypedFile {
    file: File,
    content_type: Option<ContentType>,
    path: PathBuf,
}

impl TypedFile {
    pub fn open<P: AsRef<Path>>(path: P, content_type: Option<rocket::http::ContentType>) -> TypedFile {
        let file = File::open(path.as_ref()).unwrap();
        TypedFile { file, content_type, path: (*path.as_ref()).to_path_buf() }
    }
}

impl rocket::response::Responder<'static> for TypedFile {
    fn respond_to(self, _: &Request) -> Result<Response<'static>, rocket::http::Status> {
        let mut response = Response::new();
        if let Some(content_type) = self.content_type {
            response.set_header(content_type);
        } else {
            response.set_header(ContentType::from_extension(&self.path.extension().unwrap().to_string_lossy()).unwrap());
        }
        response.set_streamed_body(BufReader::new(self.file));
        Ok(response)
    }
}

pub struct RequestPath(pub String);
impl <'a, 'req>FromRequest<'a, 'req> for RequestPath {
    type Error = ();

    fn from_request(request: &'a Request<'req>) -> Outcome<Self, (http::Status, ()), ()> {
        let uri = request.uri();
        Outcome::Success(RequestPath(String::from(uri.path())))
    }
}

pub mod path {
    use std::path as std_path;
    use std::option::Option::*;

    pub struct MatchablePath(pub String);
    pub type PathMatch = (String, String);
    pub type MatchList = Vec<PathMatch>;
    pub type MatchResult = Option<MatchList>;

    impl MatchablePath {
        pub fn from<T>(src: T) -> Self where T: ToString {
            MatchablePath(src.to_string())
        }

        fn path_to_vec<T: ToString>(string: T) -> Vec<String> {
            std_path::PathBuf::from(string.to_string())
                .components()
                .filter_map(|c| match c {
                    std_path::Component::Normal(cmp) => cmp.to_str()
                        .and_then(|s| Some(String::from(s))),
                    _ => None,
                })
                .collect()
        }

        pub fn matches<T>(&self, other: T) -> MatchResult where T: ToString {
            let this_path = MatchablePath::path_to_vec(&self.0);
            let other_path = MatchablePath::path_to_vec(other);

            if this_path.len() == other_path.len() {
                let parts_match = this_path.iter()
                    .zip(other_path.iter())
                    .fold(true, |acc, (this, other)| {
                        acc && (this.starts_with("@") || this == other)
                    });

                if parts_match {
                    Some(this_path.iter()
                        .zip(other_path.iter())
                        .filter_map(|(this, other)| {
                            if this.starts_with("@") {
                                Some((
                                    this.chars().skip(1).collect::<String>(),
                                    other.clone()
                                ))
                            } else {
                                None
                            }
                        })
                        .collect()
                    )
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}
