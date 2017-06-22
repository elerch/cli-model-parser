extern crate serde_json;
#[macro_use]
extern crate error_chain;
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;

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

fn json_from_path(path: &str) -> Result<Value> {
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
              f: &Fn(ParameterType, &str, &Value)) -> Result<()> {
    let operation = service["operations"][operation].as_object().unwrap();
    let input = operation["input"]["shape"].as_str().unwrap();
    // let errors = operation["errors"].as_array().unwrap(); // ignore errors for now
    let ref shapes = service["shapes"];
    f(ParameterType::Input, input, &shapes[input]);
    if operation.contains_key("output") {
        let output = operation["output"]["shape"].as_str().unwrap();
        f(ParameterType::Output, output, &shapes[output]);
    }
    Ok(())
}

fn print_parameters(ptype: ParameterType, _: &str, parameters: &Value) {
    if ptype == ParameterType::Input {
        for (key, _) in parameters["members"].as_object().unwrap() {
            println!("  {}", key);
        }
    }
    // TODO: Determine if we want to check outputs as well
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
    let j: Value = json_from_path("/usr/lib/python3/dist-packages/botocore/data/kms/2014-11-01/service-2.json").unwrap();
    for operation in operations(&j).unwrap() {
        println!("{}", operation);
        parameters(&j, operation, &print_parameters).unwrap();
    }
}
