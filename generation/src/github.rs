use git2::Commit;
use regex::Regex;
use std::fmt;
use url::Url;

pub struct Remote {
    username: String,
    repo_name: String,
}

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
    let x = matcher
        .map(|c| c.extract())
        .next()
        .map(|(_, [username, repo_name])| Remote {
            username: username.to_string(),
            repo_name: repo_name.to_string(),
        });
    x
}

fn try_remote_from_https(url: &str) -> Option<Remote> {
    let re = Regex::new(r"\Ahttps://github.com/([^/]+)/(.+)\z").unwrap();
    let matcher = re.captures_iter(url);
    let x = matcher
        .map(|c| c.extract())
        .next()
        .map(|(_, [username, repo_name])| Remote {
            username: username.to_string(),
            repo_name: repo_name.to_string(),
        });
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
    #[must_use]
    pub fn user(&self) -> User {
        User(self.username.clone())
    }

    fn webpage(&self) -> Url {
        format!("https://github.com/{}/{}", self.username, self.repo_name)
            .parse()
            .unwrap()
    }

    #[must_use]
    pub fn page_for(&self, commit: &Commit) -> Url {
        let mut url = self.webpage();
        url.path_segments_mut()
            .unwrap()
            .push("commits")
            .push(&commit.id().to_string());
        url
    }
}

pub struct User(String);

impl User {
    #[must_use]
    pub fn avatar(&self, size: u16) -> Url {
        format!("https://github.com/{}.png?size={}", self.0, size)
            .parse()
            .unwrap()
    }
}
