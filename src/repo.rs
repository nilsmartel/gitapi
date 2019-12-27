use serde_json::Value;
use std::error::Error;

///
/// Print all of the users Repositories to the stdout
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
    let url = format!("https://api.github.com/users/{}/repos", username);

    let resp: Value = collect_repos(&url).unwrap();

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

fn collect_repos(url: &str) -> Result<Value, Box<dyn Error>> {
    let mut resp = reqwest::get(url)?;

    let json: Value = resp.json()?;

    let link = resp
        .headers()
        .get("link")
        .unwrap()
        .to_str()
        .unwrap()
        .clone();
    let (_, (next, last)) = parse_link(&link).unwrap();

    if next != last {
        let next = collect_repos(next)?;
        match (json, next) {
            (Value::Array(mut a), Value::Array(b)) => {
                a.extend(b.into_iter());
                return Ok(Value::Array(a));
            }
            _ => unreachable!(),
        };
    } else {
        Ok(json)
    }
}

fn parse_link(input: &str) -> nom::IResult<&str, (&str, &str)> {
    // Sample input
    // let x = "<https://api.github.com/user/28377948/repos?page=2>; rel=\"next\", <https://api.github.com/user/28377948/repos?page=2>; rel=\"last\"
    use nom::{
        bytes::complete::{tag, take_until},
        character::complete::char,
        sequence::{preceded, tuple},
    };

    tuple((
        preceded(char('<'), take_until(">; rel=\"next\", ")),
        preceded(
            tag(">; rel=\"next\", "),
            preceded(char('<'), take_until(">; rel=\"last\"")),
        ),
    ))(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_link() {
        let input = "<https://api.github.com/user/12345678/repos?page=2>; rel=\"next\", <https://api.github.com/user/12345678/repos?page=2>; rel=\"last\"";

        let (rest, result) = super::parse_link(input).unwrap();

        assert_eq!(
            "https://api.github.com/user/12345678/repos?page=2",
            result.0
        );
        assert_eq!(
            "https://api.github.com/user/12345678/repos?page=2",
            result.1
        );
    }
}
