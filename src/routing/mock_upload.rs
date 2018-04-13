use rocket::{Data, State};
use formdata::{read_formdata};
use routing::request::ConvertedHeaders;
use hyper::header::{Headers, ContentDisposition, DispositionParam};
use rocket::request::FromRequest;
use std::io::{Read, Write, copy};
use std::io::{BufReader, BufWriter};
use std::fs::{OpenOptions, File, create_dir};
use cli::{HandlerMethod, SwerveConfig};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use rand::{Rng, StdRng};

#[post(path = "/", data = "<upload>")]
pub fn to_file(headers: ConvertedHeaders, conf: State<SwerveConfig>, upload: Data) -> Result<String, String> {
    let formdata = read_formdata(&mut upload.open(), &headers.as_hyper());
    if let Ok(data) = formdata {
        let fields = collect_fields(data.fields);
        match conf.field_handling {
            HandlerMethod::Log => println!("{:?}", fields),
            HandlerMethod::File => println!("{:?}", fields),
        }

        create_dir("uploads");

        for file in data.files {
            let (name, value) = file;
            if name == String::from("upload") {
                let content_disposition = value.headers.get::<ContentDisposition>().unwrap();
                let file_name = filename_from_disposition(content_disposition);

                let mut input = File::open(value.path.clone()).unwrap();
                let mut output = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(PathBuf::from("uploads").join(file_name.clone().unwrap_or(String::from("upload_data"))))
                    .unwrap();

                copy(&mut input, &mut output).unwrap();
                println!("File written to {}", file_name.unwrap());
            }
        }
        Ok(String::from("Complete"))
    } else {
        Err(String::from("Failed"))
    }
}

fn collect_fields(fields: Vec<(String, String)>) -> HashMap<String, String> {
    let mut map = HashMap::with_capacity(fields.len());
    'collector: for (key, value) in fields {
        if value == String::from("undefined") { continue 'collector };
        map.insert(key.clone(), value.clone());
    }
    map
}

fn filename_from_disposition(dispo: &ContentDisposition) -> Option<String> {
    for param in dispo.parameters.iter() {
        if let &DispositionParam::Filename(_, _, ref name_vec) = param {
            return Some(String::from_utf8(name_vec.to_vec()).unwrap_or(String::from("bad_filename.bin")));
        }
    }
    None
}