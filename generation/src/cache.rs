use reqwest::get;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{
    fs::{create_dir_all, read, write},
    path::{Path, PathBuf},
};
use url::Url;

fn hash_url(url: &Url) -> u64 {
    let mut hasher = DefaultHasher::new();
    Hash::hash_slice(url.as_str().as_bytes(), &mut hasher);
    hasher.finish()
}

pub struct Cache {
    path: PathBuf,
}

impl Cache {
    #[must_use]
    pub fn new(path: &str) -> Self {
        create_dir_all(path).unwrap();
        create_dir_all(Path::new(path).join("error")).unwrap();
        Self {
            path: Path::new(path).to_path_buf(),
        }
    }

    pub async fn get(&self, url: Url) -> Option<Vec<u8>> {
        let hash = hash_url(&url).to_string();
        let path = self.path.join(hash.clone());
        let error_path = self.path.join("error").join(hash);

        if error_path.exists() {
            None
        } else if path.exists() {
            println!("hit");
            Some(read(path).unwrap())
        } else {
            println!("miss");
            let response = get(url.clone()).await.unwrap();
            let status = response.status();
            if status.is_client_error() {
                write(error_path, status.as_str()).unwrap();
                None
            } else {
                let body = response.bytes().await.unwrap();
                println!("{:?}", path);
                write(path, body.clone()).unwrap();
                Some(Vec::from(body))
            }
        }
    }
}
