use std::sync::Arc;
use std::slice::Iter;

use figment::{Provider, Metadata, Profile, Error};
use figment::value::{Map, Dict, Value, Tag};
use serde::Deserialize;

/// A provider that fetches its data from a given URL.
pub struct CliProvider {
    /// The profile to emit data to if nesting is disabled.
    profile: Option<Profile>,
    args: Vec<std::string::String>,
}

impl CliProvider {
    pub fn new() -> CliProvider {
        CliProvider {
            profile: None,
            args: wild::args().collect(),
        }
    }
}

impl Provider for CliProvider {
    /// Returns metadata with kind `Network`, custom source `self.url`,
    /// and interpolator that returns a URL of `url/a/b/c` for key `a.b.c`.
    fn metadata(&self) -> Metadata {
        let args = &self.args;
        Metadata::named("CLI Flags")
            .source(args.join(" "))
            //.source(args.map(|args| args.collect::<Vec<_>>().join(" ")).unwrap_or(String::default()))
            /* .interpolater(move |profile, keys| match profile.is_custom() {
                true => format!("{}/{}/{}", url, profile, keys.join("/")),
                false => format!("{}/{}", url, keys.join("/")),
            }) */
    }

    /// Fetches the data from `self.url`. Note that `Dict`, `Map`, and
    /// `Profile` are `Deserialize`, so we can deserialized to them.
    fn data(&self) -> Result<Map<Profile, Dict>, Error> {
        // Parse a `Value` from a `String` 
        fn parse_from_string(string: &String) -> Value {
            // TODO: Other integer types
            match string.parse::<i32>() {
                Ok(i) => Value::Num(Tag::Default, figment::value::Num::I32(i)),
                Err(_) => match string.parse::<bool>() {
                    Ok(b) => Value::Bool(Tag::Default, b),
                    Err(_) => Value::from(string.to_owned()),
                },
            }
        }

        fn parse_keys(keys: &mut Iter<&str>, dict: &Dict, vals: &Vec<String>) -> Value {
            let key = keys.next();

            match key {
                None => {
                    if vals.len() == 1 {
                        parse_from_string(&vals[0])
                    } else {
                        let mut values = Vec::new();
                        for val in vals.iter() {
                            values.push(parse_from_string(val));
                        }
    
                        Value::Array(Tag::Default, values)
                    }
                },
                Some(key) => {
                    let key = key.to_string();
                    println!("Key is {}", key);

                    println!("Dict is {:?}", dict);

                    match dict.get(&key) {
                        Some(val) => {
                            println!("Val is {:?}", val);

                            match val.as_dict() {
                                Some(dict) => parse_keys(keys, &dict, vals),
                                None => panic!("Expected a `Dict`, got some other value"),
                            }
                            //parse_keys(keys, &dict, vals)
                        },
                        None => {
                            let mut current_dict = Dict::new();
                            let val = parse_keys(keys, &current_dict, vals);

                            current_dict.insert(key.to_string(), val);

                            Value::from(current_dict)
                        }
                    }
                    /* let mut current_dict = Dict::new();
                    let val = parse_keys(keys, &current_dict, vals);

                    current_dict.insert(key.to_string(), val);

                    Value::from(current_dict) */
                }
            }
        }

        fn parse_cli(args: &Vec<std::string::String>)-> Result<Dict, Error> {
            let (args, argv) = argmap::parse(args.iter());

            let mut dict = Dict::new();

            for (key, vals) in argv {
                let len = vals.len();
                if len == 0 {
                    continue;
                }

                let key_vec = key.split(".").collect::<Vec<_>>();
                if key_vec.len() > 1 {
                    let mut key_iter = key_vec.iter();

                    //let key = key_iter.next();
                    let key = key_vec.first();
                    let val = parse_keys(&mut key_iter, &dict, &vals);

                    println!("Final val is {:?}", val);

                    dict.insert(key.unwrap().to_string(), val);
                } else {
                    if len == 1 {
                        dict.insert(key, parse_from_string(&vals[0]));
                    } else {
                        let mut values = Vec::new();
                        for val in &vals {
                            values.push(parse_from_string(val));
                        }
    
                        dict.insert(key, Value::Array(Tag::Default, values));
                    }
                }

                println!("Dict: {:?}", dict);
            }

            

            Ok(dict)
        }

        match &self.profile {
            // Don't nest: `fetch` into a `Dict`.
            Some(profile) => Ok(profile.collect(parse_cli(&self.args)?)),
            None => {
                let mut map = Map::new();
                map.insert(Profile::default(), parse_cli(&self.args)?);
                Ok(map)   
            }
        }
    }
}