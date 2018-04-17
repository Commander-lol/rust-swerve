use cli;
use std::fs;
use std::path;
use rocket::{self, http::ContentType};
use routing::request::TypedFile;

#[get("/")]
fn serve_root(args: rocket::State<cli::Args>) -> Option<TypedFile> {
    serve_files(None, args)
}

#[get("/<file..>")]
fn serve_files(file: Option<path::PathBuf>, args: rocket::State<cli::Args>) -> Option<TypedFile> {
    let stub = match file {
        Some(path) => path,
        None => path::PathBuf::from(""),
    };

    let path = args.get_dir().join(stub);

    let meta = match fs::metadata(&path) {
        Ok(metadata) => metadata,
        _ => return None,
    };

    if meta.is_dir() && !args.flag_no_index {
        Some(TypedFile::open(path.join("index.html"), None))
    } else {
        if &path.extension().unwrap().to_string_lossy() == "wasm" {
           Some( TypedFile::open(path, Some(ContentType::new("application", "wasm"))))
        } else {
            Some(TypedFile::open(path, None))
        }
    }
}
