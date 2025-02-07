use std::convert::Infallible;

use crate::helpers::{create_array, push_item_to_array};
use crate::merge::merge;
#[cfg(feature = "regex1")]
use lazy_static::lazy_static;
use percent_encoding::percent_decode;
#[cfg(feature = "regex1")]
use regex::Regex;
use serde_json::{Map, Number, Value};
use thiserror::Error;

#[cfg(feature = "regex1")]
lazy_static! {
    static ref PARENT_REGEX: Regex = Regex::new(r"^([^\]\[]+)").unwrap();
    static ref CHILD_REGEX: Regex = Regex::new(r"(\[[^\]\[]*\])").unwrap();
}

type Object = Map<String, Value>;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Failed to decode: {0}")]
    DecodingError(String),
    #[error("Other")]
    Other,
}

pub type ParseResult<T> = Result<T, ParseError>;

pub fn decode_component(source: &str) -> Result<String, String> {
    let result = percent_decode(source.as_bytes())
        .decode_utf8_lossy()
        .to_string();
    Ok(result)
}

fn parse_pair(part: &str) -> (&str, Option<&str>) {
    let separator = part
        .find("]=")
        .map(|pos| pos + 1)
        .or_else(|| part.find('='));
    match separator {
        None => (part, None),
        Some(pos) => {
            let key = &part[..pos];
            let val = &part[(pos + 1)..];
            (key, Some(val))
        }
    }
}

fn parse_pairs(body: &str) -> Vec<(&str, Option<&str>)> {
    let mut pairs = vec![];
    for part in body.split('&') {
        pairs.push(parse_pair(part));
    }
    pairs
}

#[cfg(feature = "regex1")]
fn parse_key(key: &str) -> ParseResult<Vec<String>> {
    let mut keys: Vec<String> = vec![];

    if let Some(captures) = PARENT_REGEX.captures(key) {
        match decode_component(captures.get(1).unwrap().as_str()) {
            Ok(decoded_key) => keys.push(decoded_key),
            Err(err_msg) => return Err(ParseError::DecodingError(err_msg)),
        }
    };

    for captures in CHILD_REGEX.captures_iter(key) {
        match decode_component(captures.get(1).unwrap().as_str()) {
            Ok(decoded_key) => keys.push(decoded_key),
            Err(err_msg) => return Err(ParseError::DecodingError(err_msg)),
        }
    }

    Ok(keys)
}

#[cfg(not(feature = "regex1"))]
fn parse_key(key: &str) -> ParseResult<Vec<String>> {
    let mut keys: Vec<String> = vec![];

    match key.split(|c| c == '[' || c == ']').next() {
        Some(parent) if !parent.is_empty() => match decode_component(parent) {
            Ok(decoded_key) => keys.push(decoded_key),
            Err(err_msg) => return Err(ParseError::DecodingError(err_msg)),
        },
        _ => (),
    }

    let mut prev_bracket = None;
    for (idx, ch) in key.char_indices() {
        match ch {
            '[' => prev_bracket = Some(idx),
            ']' => {
                if let Some(prev_idx) = prev_bracket {
                    prev_bracket = None;
                    let child = &key[prev_idx..=idx];
                    match decode_component(child) {
                        Ok(decoded_key) => keys.push(decoded_key),
                        Err(err_msg) => {
                            return Err(ParseError {
                                kind: ParseErrorKind::DecodingError,
                                message: err_msg,
                            })
                        }
                    }
                }
            }
            _ => (),
        }
    }

    Ok(keys)
}

fn cleanup_key(key: &str) -> &str {
    if key.starts_with('[') && key.ends_with(']') {
        &key[1..(key.len() - 1)]
    } else {
        key
    }
}

fn create_idx_merger(idx: u64, obj: Value) -> Value {
    let mut tree = Object::new();
    tree.insert("__idx".to_string(), Value::Number(Number::from(idx)));
    tree.insert("__object".to_string(), obj);
    Value::Object(tree)
}

fn create_object_with_key(key: String, obj: Value) -> Value {
    let mut tree = Object::new();
    tree.insert(key, obj);
    Value::Object(tree)
}

fn apply_object(keys: &[String], val: Value) -> Value {
    if !keys.is_empty() {
        let key = keys.get(0).unwrap();
        if key == "[]" {
            let mut new_array = create_array();
            let item = apply_object(&keys[1..], val);
            push_item_to_array(&mut new_array, item);
            new_array
        } else {
            let key = cleanup_key(key);
            let array_index = key.parse();

            match array_index {
                Ok(idx) => {
                    let result = apply_object(&keys[1..], val);
                    create_idx_merger(idx, result)
                }
                Err(_) => create_object_with_key(key.to_string(), apply_object(&keys[1..], val)),
            }
        }
    } else {
        val
    }
}

pub fn parse(params: &str) -> ParseResult<Value> {
    let tree = Object::new();
    let mut obj = Value::Object(tree);
    let decoded_params = match decode_component(&params.replace('+', " ")) {
        Ok(val) => val,
        Err(err) => return Err(ParseError::DecodingError(err)),
    };
    let pairs = parse_pairs(&decoded_params);
    for &(key, value) in pairs.iter() {
        let parse_key_res = parse_key(key)?;
        let key_chain = &parse_key_res[0..];
        let decoded_value = match value {
            None => Value::default(),
            Some(val) => match decode_component(val) {
                Ok(v) => {
                    let n = v.parse::<i64>();
                    let f = v.parse::<f64>();
                    let b = v.parse::<bool>();
                    let null = Ok::<_, Infallible>(v == "null");
                    match (n, f, b, null) {
                        (Ok(n), _, _, _) => Value::Number(Number::from(n)),
                        (_, Ok(f), _, _) => Value::Number(Number::from_f64(f).unwrap()),
                        (_, _, Ok(b), _) => Value::Bool(b),
                        (_, _, _, Ok(true)) => Value::Null,
                        _ => Value::String(v),
                    }
                }
                Err(err) => return Err(ParseError::DecodingError(err)),
            },
        };
        let partial = apply_object(key_chain, decoded_value);
        merge(&mut obj, &partial);
    }

    Ok(obj)
}

#[cfg(test)]
mod tests {
    use super::{parse, parse_pair};
    use serde_json::{json, to_string, Value};

    fn eq_str(value: Value, string: &str) {
        assert_eq!(&to_string(&value).unwrap(), string)
    }

    #[test]
    fn test_parse_pair() {
        assert_eq!(parse_pair("foo=1"), ("foo", Some("1")));
        assert_eq!(parse_pair("empty="), ("empty", Some("")));
        assert_eq!(parse_pair("noval"), ("noval", None));
    }

    #[test]
    fn it_parses_simple_string() {
        eq_str(parse("0=foo").unwrap(), r#"{"0":"foo"}"#);
        eq_str(parse("a[<=>]==23").unwrap(), r#"{"a":{"<=>":"=23"}}"#);
        eq_str(
            parse(" foo = bar = baz ").unwrap(),
            r#"{" foo ":" bar = baz "}"#,
        );
    }

    #[test]
    fn it_parses_nested_string() {
        eq_str(
            parse("a[b][c][d][e][f][g][h]=i").unwrap(),
            r#"{"a":{"b":{"c":{"d":{"e":{"f":{"g":{"h":"i"}}}}}}}}"#,
        );
    }

    #[test]
    fn it_parses_simple_array() {
        eq_str(
            parse("a=b&a=c&a=d&a=e").unwrap(),
            r#"{"a":["b","c","d","e"]}"#,
        );
    }

    #[test]
    fn it_parses_explicit_array() {
        eq_str(
            parse("a[]=b&a[]=c&a[]=d").unwrap(),
            r#"{"a":["b","c","d"]}"#,
        );
    }

    #[test]
    fn it_parses_nested_array() {
        eq_str(
            parse("a[b][]=c&a[b][]=d").unwrap(),
            r#"{"a":{"b":["c","d"]}}"#,
        );
    }

    #[test]
    fn it_allows_to_specify_array_indexes() {
        eq_str(
            parse("a[0][]=c&a[1][]=d").unwrap(),
            r#"{"a":[["c"],["d"]]}"#,
        );
    }

    #[test]
    fn it_transforms_arrays_to_object() {
        eq_str(
            parse("foo[0]=bar&foo[bad]=baz").unwrap(),
            r#"{"foo":{"0":"bar","bad":"baz"}}"#,
        );

        eq_str(
            parse("foo[0][a]=a&foo[0][b]=b&foo[1][a]=aa&foo[1][b]=bb").unwrap(),
            r#"{"foo":[{"a":"a","b":"b"},{"a":"aa","b":"bb"}]}"#,
        );
    }

    #[test]
    fn it_transforms_standalone_keys() {
        eq_str(parse("foo=bar&baz").unwrap(), r#"{"baz":null,"foo":"bar"}"#);
    }

    #[test]
    fn it_doesnt_produce_empty_keys() {
        assert_eq!(parse("_r=1&").unwrap(), json!({"_r": 1}));
    }

    #[test]
    fn it_supports_encoded_strings() {
        eq_str(parse("a[b%20c]=c%20d").unwrap(), r#"{"a":{"b c":"c d"}}"#);
    }

    #[test]
    fn it_parses_explicit_encoded_array() {
        eq_str(
            parse("a%5B%5D=b&a%5B%5D=c&a%5B%5D=d").unwrap(),
            r#"{"a":["b","c","d"]}"#,
        );
    }
    #[test]
    fn it_parses_plus_sign() {
        eq_str(parse("a=b%20c+d%2B").unwrap(), r#"{"a":"b c d+"}"#);
    }

    #[test]
    fn it_parses_numbers() {
        assert_eq!(parse("a=1").unwrap(), json!({"a": 1}));
        assert_eq!(parse("a=1.1").unwrap(), json!({"a": 1.1}));
        assert_eq!(parse("a=1.1e1").unwrap(), json!({"a": 11.0}));
        assert_eq!(parse("a=1.1e-1").unwrap(), json!({"a": 0.11}));
    }

    #[test]
    fn it_parses_booleans() {
        assert_eq!(parse("a=true").unwrap(), json!({"a": true}));
        assert_eq!(parse("a=false").unwrap(), json!({"a": false}));
    }

    #[test]
    fn it_parses_null() {
        assert_eq!(parse("a=null").unwrap(), json!({ "a": null }));
    }

    #[test]
    fn it_parses_empty_string() {
        assert_eq!(parse("a=").unwrap(), json!({ "a": "" }));
    }

    #[test]
    fn it_parses_array_integer() {
        assert_eq!(parse("a[]=1&a[]=2").unwrap(), json!({ "a": [1, 2] }));
    }

    #[test]
    fn it_parses_nested_array_integer() {
        assert_eq!(
            parse("a[b]=null&a[b]=2").unwrap(),
            json!({ "a": {"b": [null, 2] }})
        );

        assert_eq!(
            parse("a[b]=1&a[b]=2").unwrap(),
            json!({ "a": {"b": [1, 2] }})
        );
    }
}
