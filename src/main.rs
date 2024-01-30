mod utils;

use std::io::{self, Error};
use std::env;
use std::collections::HashMap;
use walkdir::WalkDir;


fn main() {
    let args : Vec<String>= env::args().collect();
    let mut arguments_with_values = HashMap::new();
    arguments_with_values = extract_arguments(args, arguments_with_values.clone());

    let files = get_files(arguments_with_values.get("p").expect("Invalid path"));
    
}

fn extract_arguments(mut arg: Vec<String>, mut hashmap: HashMap<String, String>)-> HashMap<String,String>{
    //get rid of the first argument as it is the binary name
    arg.remove(0);
    for flag in arg.clone(){
        if utils::does_flag_expect_a_value(flag.clone()) && arg.len() > 1{
            //remove the - prefix for ease in usage
            let mut flag = flag.replace("-","");

            hashmap.insert(String::from(flag), arg.get(1).unwrap().clone());
            arg.drain(0..2);
        }
    }
    return hashmap;
}


fn get_files(path:&String) -> Result<(), io::Error>{
    for entry in WalkDir::new(path) {
        println!("{}", entry?.path().display());
    }
    Ok(())
}