extern crate serde;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    pub name: String,
    pub age: u8,
}

pub fn get_response<'a>() -> Result<String, Box<std::error::Error>> {
    let tarou = Person { name: "太郎".to_string(), age: 18 };
    // let mut json = serde_json::to_string(&tarou)?;
    // result = &'a json;

    Ok(serde_json::to_string(&tarou)?)
}
