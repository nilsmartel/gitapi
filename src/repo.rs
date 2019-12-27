use serde_json::Value;
use std::error::Error;

pub fn print_repos(user: &str) {
    match get_repos(user) {
        Err(e) => eprintln!("{}", e),
        Ok(list) => {
            for f in list {
                println!("{}", f);
            }
        }
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
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            })
            .collect(),
        _ => unreachable!(),
    };

    Ok(res)
}
