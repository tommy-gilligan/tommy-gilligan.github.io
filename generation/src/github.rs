use regex::Regex;
use std::fmt;
use url::Url;

pub struct Remote { username: String }

#[derive(Debug, Clone)]
pub struct DoubleError;

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

fn try_remote_from_ssh(url: &str) -> Option<Remote> {
    let re = Regex::new(r"\Agit@github.com:([^/]+)/(.+)\z").unwrap();
    let matcher = re.captures_iter(url);
    let x = match matcher.map(|c| c.extract()).next() {
        Some((_, [username, _])) => Some(Remote {
            username: username.to_string(),
        }),
        None => None,
    };
    x
}

fn try_remote_from_https(url: &str) -> Option<Remote> {
    let re = Regex::new(r"\Ahttps://github.com/([^/]+)/(.+)\z").unwrap();
    let matcher = re.captures_iter(url);
    let x = match matcher.map(|c| c.extract()).next() {
        Some((_, [username, _])) => Some(Remote {
            username: username.to_string(),
        }),
        None => None,
    };
    x
}

impl std::convert::TryFrom<git2::Remote<'_>> for Remote {
    type Error = DoubleError;

    fn try_from(value: git2::Remote) -> Result<Self, Self::Error> {
        let url = value.url().unwrap();
        Ok(try_remote_from_ssh(url).unwrap_or_else(|| try_remote_from_https(url).unwrap()))
    }
}

impl Remote {
    pub fn user(&self) -> User {
        User(self.username.clone())
    }
}

pub struct User(String);

impl User {
    pub fn avatar(&self, size: u16) -> Url {
        format!("https://github.com/{}.png?size={}", self.0, size)
            .parse()
            .unwrap()
    }
}
