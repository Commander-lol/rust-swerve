//use dyon::{Runtime,Module,load_str,Variable,Dfn,Type};
//use dyon::ast::convert;
use std::convert::AsRef;
use std::path::Path;
use std::fs::File;
use std::sync::Arc;
use std::io::Read;
use std::collections::HashMap;

const SCRIPT_FOOTER: &'static str = "
fn main() {
    println(\"Don't directly execute the module, nimrod\")
}";


pub fn run_script<P: AsRef<Path>>(path: P) -> Option<String> {
//    let mut resolution: Option<Variable> = None;
//    dyon_fn!(fn resolve(val: Variable) {
//           resolution = Some(val); // if let Ok(val) = runtime.pop() { Some(val) } else { None };
//    });
//
//    println!("{:?}", resolution);

    let mut file = File::open(&path).unwrap();
    let mut buf = String::new();

    file.read_to_string(&mut buf);
    buf.push_str(SCRIPT_FOOTER);
//    let mut script_module = Module::new();
//
//    {
//        load_str(path.as_ref().to_str().unwrap(), Arc::new(buf), &mut script_module);
//    }
    Some(buf)
//    script_module.add(Arc::new("resolve".into()), resolve, Dfn {
//        lts: vec![],
//        tys: vec![Type::Object],
//        ret: Type::Void,
//    });
//
//    let mut hashmap: HashMap<Arc<String>, Variable> = HashMap::new();

//    runtime.call_str("handle", &[Variable::f64(123f64)], &Arc::new(script_module));

//    None
}