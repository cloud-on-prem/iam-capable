use colored::*;
use serde_json::Value;

pub fn compare_and_display_json(json1: Value, json2: Value) {
    compare_values(&json1, &json2, "".to_string());
}

fn compare_values(value1: &Value, value2: &Value, key_prefix: String) {
    match (value1, value2) {
        (Value::Object(map1), Value::Object(map2)) => {
            for (key, value) in map1.iter() {
                let full_key = format!("{}{}-{}", key_prefix, key, value);

                if let Some(value2) = map2.get(key) {
                    println!(
                        "{}",
                        format!("{}-{} - Only in JSON 1", full_key, value2).red()
                    );
                }
            }

            for key in map2.keys() {
                if !map1.contains_key(key) {
                    let full_key = format!("{}{}", key_prefix, key);
                    println!("{}", format!("{} - Only in JSON 2", full_key).green());
                }
            }
        }
        (Value::Array(arr1), Value::Array(arr2)) => {
            let len1 = arr1.len();
            let len2 = arr2.len();
            let min_len = std::cmp::min(len1, len2);

            for index in 0..min_len {
                let full_key = format!("{}[{}]", key_prefix, index);
                compare_values(&arr1[index], &arr2[index], format!("{}: ", full_key));
            }

            if len1 > len2 {
                for index in len2..len1 {
                    let full_key = format!("{}[{}]", key_prefix, index);
                    println!("{}", format!("{} - Only in JSON 1", full_key).red());
                }
            } else if len2 > len1 {
                for index in len1..len2 {
                    let full_key = format!("{}[{}]", key_prefix, index);
                    println!("{}", format!("{} - Only in JSON 2", full_key).green());
                }
            }
        }
        _ => {
            if value1 != value2 {
                println!(
                    "{}",
                    format!(
                        "{}: {} {}",
                        key_prefix.strip_suffix(": ").unwrap_or(&key_prefix),
                        value1.to_string().red(),
                        value2.to_string().green()
                    )
                );
            }
        }
    }
}
