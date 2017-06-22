extern crate serde_json;
#[macro_use]
extern crate error_chain;
use serde_json::{Value, Error};
use std::fs::File;
use std::io::prelude::*;

fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"{
                    "name": "John Doe",
                    "age": 43,
                    "phones": [
                      "+44 1234567",
                      "+44 2345678"
                    ]
                  }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}

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

fn jsonFromPath(path: &str) -> Result<Value> {
   let mut file = File::open(path)?;
   let mut contents = String::new();
   file.read_to_string(&mut contents)?;
   let json = serde_json::from_str(&contents)?;
   Ok(json)
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
    let j: Value = jsonFromPath("/usr/lib/python3/dist-packages/botocore/data/kms/2014-11-01/service-2.json").unwrap();
    println!("done");
}
