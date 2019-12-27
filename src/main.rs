use std::error::Error;

fn main() {
    let username = std::env::args().nth(1).expect("specify username");
}

fn get_repos(username: &str) -> Result<Vec<String>, Box<Error>> {
    use std::collections::HashMap;
    let resp = reqwest::get(&format!("https://api.github.com/users/{}/repos", username))?;

    dbg!(resp);
    Ok(vec![])
}
