use serde_json::Value;
use std::error::Error;

fn main() {
    let username = std::env::args().nth(1).expect("specify username");

    for f in get_repos(&username).unwrap() {
        println!("{}", f);
    }
}

fn get_repos(username: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let resp: Value =
        reqwest::get(&format!("https://api.github.com/users/{}/repos", username))?.json()?;

    let res: Vec<String> = match resp {
        Value::Array(a) => a
            .into_iter()
            .map(|i| match i {
                Value::Object(o) => match &o["name"] {
                    Value::String(s) => s.to_owned(),
                    _ => panic!("no property `name`"),
                },
                _ => panic!("please dont do this"),
            })
            .collect(),
        _ => panic!("resp must be of type array"),
    };

    Ok(res)
}
