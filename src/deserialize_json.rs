//use serde::Deserialize;
use std::fs::File;
use std::path::Path;

/*#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Poems {
    poems: (u32, String, Vec<String>),
}*/

pub struct Deserializer;

impl Deserializer  {
    pub fn deserialize_poem(path: String) {
        let json_file_path = Path::new(&path);
        let json_file = File::open(json_file_path).expect("file not found");
        let poems: serde_json::Value =
        serde_json::from_reader(json_file).expect("error while reading json");
        println!("{:?}", poems)
    }
    pub fn deserialize_poem_test() {
        let the_file = r#"{
            "FirstName": "John",
            "LastName": "Doe",
            "Age": 43,
            "Address": {
                "Street": "Downing Street 10",
                "City": "London",
                "Country": "Great Britain"
            },
            "PhoneNumbers": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
        let json: serde_json::Value =
        serde_json::from_str(the_file).expect("JSON was not well-formatted");
        println!("{:?}", json)
    }
}