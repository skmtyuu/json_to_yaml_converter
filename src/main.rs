use serde::{Deserialize, Serialize};
use serde_json;
// use serde_yaml;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct MyData {
    name: String,
    data_type: String,
    description: String,
}

fn main() {
    // JSONファイルを読み込みます。
    let mut file = File::open("data.json").expect("Unable to open file");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)
        .expect("Unable to read file");

    // JSONをデシリアライズしてRustの構造体に変換します。
    let deserialized_data: Vec<MyData> =
        serde_json::from_str(&json_data).expect("Unable to deserialize JSON");

    // YAML形式の文字列を構築します。
    let mut yaml_string = String::new();
    for data in &deserialized_data {
        yaml_string.push_str(&format!(
            "- name: {}\n  data_type: {}\n  description: {}\n\n",
            data.name, data.data_type, data.description
        ));
    }

    // YAML形式の文字列をファイルに書き出します。
    let mut output_file = File::create("output.yaml").expect("Unable to create output.yaml file");
    output_file
        .write_all(yaml_string.as_bytes())
        .expect("Unable to write to output.yaml file");
    println!("YAML data has been written to output.yaml");
}
