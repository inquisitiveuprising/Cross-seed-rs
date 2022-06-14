use std::sync::Arc;

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

        fn fetch<'a, T: Deserialize<'a>>(args: &Vec<std::string::String>) -> Result<T, Error> {
            let (args, argv) = argmap::parse(args.iter());

            let mut dict = Dict::new();

            for (key, vals) in argv {
                let len = vals.len();
                if len == 0 {
                    continue;
                }

                let key_vec: Vec<&str> = key.split(".").collect();
                for key in key_vec.iter() {
                    dict.insert(key.to_owned(), Value::from(key.to_owned()));
                }

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

            Ok(T::deserialize(dict).unwrap())

            //Ok(T::deserialize(args.unwrap_or(&std::env::args()))?)

            //Profile::default()
        }

        match &self.profile {
            // Don't nest: `fetch` into a `Dict`.
            Some(profile) => Ok(profile.collect(fetch(&self.args)?)),
            // Nest: `fetch` into a `Map<Profile, Dict>`.
            None => fetch(&self.args),
        }
    }
}