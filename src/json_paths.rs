use std::{collections::{HashMap, VecDeque}, ops::Not};
use serde_json::Value;

/// Returns a [Vec] containing [String]s representing possible paths
/// on JSON data
/// 
/// Examples:
/// ```rust
/// use serde_json::Value;
/// use nestac::json_get_paths;
/// 
/// fn main() {
///     let json_str = r#"
///     {
///         "foo": {
///             "bar": "bingo!"
///         },
///         "hello": {
///             "world": "!"
///         }
///     }
///     "#;
///     let json_data: Result<Value, _> = serde_json::from_str(json_str);
///     let paths: Vec<String> = json_get_paths(
///         json_data.as_ref().unwrap(),
///         None,
///     );
///     assert_eq!(paths.len(), 5);
///     assert_eq!(paths[0], "$");
///     assert_eq!(paths[1], "$.foo");
///     assert_eq!(paths[2], "$.foo.bar");
///     assert_eq!(paths[3], "$.hello");
///     assert_eq!(paths[4], "$.hello.world");
/// }
/// ```


/*

        if data.is_object() {
            ret.push(symbol.clone());
            for key_s in data.as_object().unwrap().keys() {
                let child = data.as_object().unwrap().get(key_s).unwrap();
                for path in json_get_paths(child, Some(key_s.to_string())) {
                    ret.push([symbol.clone(), path].join("."));
                }
            }
        }
        else if data.is_array() {
            ret.push(symbol.clone());
            for (i, child) in data.as_array().unwrap().iter().enumerate() {
                for path in json_get_paths(&child, Some(i.to_string())) {
                    ret.push([symbol.clone(), path].join("."));
                }
            }
        }
        else {
            ret.push(symbol.clone());
        }
*/

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Result;

    #[test]
    fn key_path_interpolation() {
        let json_str = r#"
            {
                "foo": {
                    "bar": {
                        "hello": "world!"
                    }
                },
                "one": {
                    "two": {
                        "three": {
                            "four": "five"
                        }
                    }
                }
            }
        "#;
        println!("{}", json_str);
        let json_data: Result<Value> = serde_json::from_str(json_str);
        let paths: Vec<String> = json_get_paths(
            json_data.as_ref().unwrap(),
            None,
            None,
        );
        assert_eq!(paths.len(), 8);
        assert_eq!(paths[0], "$");
        assert_eq!(paths[1], "$.foo");
        assert_eq!(paths[2], "$.foo.bar");
        assert_eq!(paths[3], "$.foo.bar.hello");
        assert_eq!(paths[4], "$.one");
        assert_eq!(paths[5], "$.one.two");
        assert_eq!(paths[6], "$.one.two.three");
        assert_eq!(paths[7], "$.one.two.three.four");

        let json_str = r#"{
            "medabots": {
                "hokusho": {
                    "medal": "not kabuto"
                },
                "metabee": {
                    "medal": "kabuto"
                }
            }
        }"#;
        println!("{}", json_str);
        let json_data: Result<Value> = serde_json::from_str(json_str);
        let paths: Vec<String> = json_get_paths(
            json_data.as_ref().unwrap(),
            None,
            None,
        );
        assert_eq!(paths.len(), 5);
        assert_eq!(paths[0], "$.medabots");
        assert_eq!(paths[1], "$.medabots.hokusho");
        assert_eq!(paths[2], "$.medabots.hokusho.medal");
        assert_eq!(paths[3], "$.medabots.metabee");
        assert_eq!(paths[4], "$.medabots.metabee.medal");

        /*
        let json_str = r#"
            {
                "foo": {
                    "bar": ["bingo!"]
                },
                "hello": [
                    "world",
                    "!"
                ]
            }
        "#;
        let json_data: Result<Value> = serde_json::from_str(json_str);
        let paths: Vec<String> = json_get_paths(
            json_data.as_ref().unwrap(), None);
        assert_eq!(paths.len(), 7);
        assert_eq!(paths[0], "$");
        assert_eq!(paths[1], "$.foo");
        assert_eq!(paths[2], "$.foo.bar");
        assert_eq!(paths[3], "$.foo.bar.0");
        assert_eq!(paths[4], "$.hello");
        assert_eq!(paths[5], "$.hello.0");
        assert_eq!(paths[6], "$.hello.1");
        */
    }
}
