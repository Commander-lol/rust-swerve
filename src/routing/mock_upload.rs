use rocket::Data;
use formdata::{read_formdata};
use routing::request::ConvertedHeaders;
use hyper::header::{Headers, ContentDisposition};
use rocket::request::FromRequest;
use std::io::{Read, Write, copy};
use std::io::{BufReader, BufWriter};
use std::fs::{OpenOptions, File};

#[post(path = "/", data = "<upload>")]
pub fn to_file(headers: ConvertedHeaders, upload: Data) -> String {
    let formdata = read_formdata(&mut upload.open(), &headers.as_hyper());
    if let Ok(data) = formdata {
        for file in data.files {
            let (fieldname, fieldvalue) = file;
            if fieldname == String::from("upload") {
                let mut input = File::open(fieldvalue.path.clone()).unwrap();
                let mut output = OpenOptions::new().write(true).open("upload_data").unwrap();
                copy(&mut input, &mut output).unwrap();
            }
        }
        String::from("Complete")
    } else {
        String::from("Failed")
    }
}