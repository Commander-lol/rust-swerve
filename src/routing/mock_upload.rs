use rocket::{Data, State};
use formdata::{read_formdata, FilePart};
use routing::request::ConvertedHeaders;
use hyper::header::{ContentDisposition, DispositionParam};
use std::io::{copy};
use std::fs::{OpenOptions, File, create_dir};
use cli::{HandlerMethod, SwerveConfig};
use std::path::{PathBuf};
use std::collections::HashMap;

#[post(path = "/", data = "<upload>")]
pub fn to_file(headers: ConvertedHeaders, conf: State<SwerveConfig>, upload: Data) -> Result<String, String> {
    let formdata = read_formdata(&mut upload.open(), &headers.as_hyper());
    if let Ok(data) = formdata {
        let fields = collect_fields(data.fields);
        match conf.field_handling {
            HandlerMethod::Log => println!("{:?}", fields),
            HandlerMethod::File => println!("{:?}", fields),
        }

        match create_dir("uploads") {
			Ok(_) => {},
			Err(err) => {
				return Err(format!("Could not create uploads directory:\n{}", err));
			}
		}

        for file in data.files {
            match conf.file_handling {
                HandlerMethod::Log => log_file(file),
                HandlerMethod::File => upload_file(file),
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

type Upload = (String, FilePart);

fn upload_file(file: Upload) {
    let (name, value) = file;
    let content_disposition = value.headers.get::<ContentDisposition>().unwrap();
    let file_name = filename_from_disposition(content_disposition).unwrap_or(name);

    let mut input = File::open(value.path.clone()).unwrap();
    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .open(PathBuf::from("uploads").join(file_name.clone()))
        .unwrap();

    copy(&mut input, &mut output).unwrap();
    println!("File written to {}", file_name);
}

fn log_file(file: Upload) {
    let (name, value) = file;
    println!("[UPLOAD] From field: {}. Size: {}", name, value.size.unwrap_or(0));
    for header in value.headers.iter() {
        println!("[UPLOAD] :: {}; {}", header.name(), header.value_string());
    }
}

fn filename_from_disposition(dispo: &ContentDisposition) -> Option<String> {
    for param in dispo.parameters.iter() {
        if let &DispositionParam::Filename(_, _, ref name_vec) = param {
            return Some(String::from_utf8(name_vec.to_vec()).unwrap_or(String::from("bad_filename.bin")));
        }
    }
    None
}