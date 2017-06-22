extern crate serde_json;
#[macro_use]
extern crate error_chain;
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

mod error {
    use std;
    use serde_json;

    error_chain!{
        foreign_links {
            IoError(std::io::Error);
            JsonError(serde_json::error::Error);
        }
    }
}
use error::*;

#[derive(PartialEq, Eq)]
enum ParameterType {
    Input,
    Output
}

fn json_from_path(path: String) -> Result<Value> {
   let mut file = File::open(path)?;
   let mut contents = String::new();
   file.read_to_string(&mut contents)?;
   let json = serde_json::from_str(&contents)?;
   Ok(json)
}

fn operations(service: &Value) -> Result<Vec<String>> {
    let mut rc: Vec<String> = Vec::new();
    for (key, _) in service["operations"].as_object().unwrap() {
        rc.push(key.to_string());
    }
    Ok(rc)
}

fn parameters(service: &Value, operation: String,
              f: &Fn(ParameterType, &str, &Value)) {
    let operation = service["operations"][operation].as_object().unwrap();
    let ref shapes = service["shapes"];
    if operation.contains_key("input") {
        let input = operation["input"]["shape"].as_str().unwrap();
        f(ParameterType::Input, input, &shapes[input]);
    }
    // let errors = operation["errors"].as_array().unwrap(); // ignore errors for now
    if operation.contains_key("output") {
        let output = operation["output"]["shape"].as_str().unwrap();
        f(ParameterType::Output, output, &shapes[output]);
    }
}

fn print_parameters(ptype: ParameterType, _: &str, parameters: &Value) {
    if ptype == ParameterType::Input {
        for (key, _) in parameters["members"].as_object().unwrap() {
            println!("    {}", key);
        }
    }
    // TODO: Determine if we want to check outputs as well
}

/// Crawls a standard botocore fs hierarchy. Returns map of service/service file path
///
/// # Arguments
///
/// * `basepath` - base path for boto. Default:
///                /usr/lib/python3/dist-packages/botocore/data/
fn service_files(basepath: Option<&str>) -> Result<HashMap<String, String>> {
    let mut rc = HashMap::new();
    let path = match basepath {
        Some(x) => x.to_string(),
        None => "/usr/lib/python3/dist-packages/botocore/data".to_string(),
    };
    for entry in std::fs::read_dir(&path)? {
        let entry = entry?;
        let entrypath = entry.path();
        if entrypath.is_dir() {
            let servicename = entry.file_name();
            let mut newest = std::ffi::OsString::from("0");
            for serviceentry in std::fs::read_dir(entry.path())? {
                let serviceentry = serviceentry?;
                let current = serviceentry.file_name();
                if current > newest {
                    newest = current;
                }
            }
            let newestserviceentry = entrypath.join(newest).join("service-2.json");
            rc.insert(servicename.into_string().unwrap(), newestserviceentry.to_str().unwrap().to_string());
        }
    }
    //rc.insert("kms".to_string(), path + &("/kms/2014-11-01/service-2.json".to_string()));
    Ok(rc)
}

/**********************************************************************
 * Process (not all of that necessarily here)
 *
 * 1. Crawl through /usr/lib/python3/dist-packages/botocore/data/*
 * 2. For each service, find latest rev of files
 * 3. Process 'service-2.json' in each directory (can we ignore paginators/waiters?)
 *********************************************************************/
*/
fn main() {
    for (service, file) in service_files(None).unwrap() {
        println!("{}", service);
        //println!("  {}", file);
        let j: Value = json_from_path(file).unwrap();
        for operation in operations(&j).unwrap() {
            println!("  {}", operation);
            parameters(&j, operation, &print_parameters);
        }
    }
}
