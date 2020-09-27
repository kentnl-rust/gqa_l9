use crate::err::ErrorKind;
use reqwest::{Client, Url};
use static_http_cache::Cache;

pub struct Agent {
    cache: Cache<Client>,
}

impl Agent {
    pub fn default() -> Result<Agent, ErrorKind> {
        use directories::ProjectDirs;

        let dir = ProjectDirs::from("org", "kentfredric.github", "GQA_L9")
            .ok_or_else(|| ErrorKind::NoProjectDir)?;
        let client = Client::builder().gzip(true).build()?;
        let cache = Cache::new(dir.cache_dir().to_path_buf(), client)?;
        Ok(Agent { cache: cache })
    }
    pub fn get(&mut self, u: Url) -> Result<std::fs::File, ErrorKind> {
        Ok(self.cache.get(u)?)
    }
    pub fn get_url(&mut self, u: &str) -> Result<std::fs::File, ErrorKind> {
        let uri = Url::parse(u)?;
        self.get(uri)
    }
    pub fn get_url_content<'a>(&'a mut self, u: &str) -> Result<String, ErrorKind> {
        use std::io::Read;
        let mut buf = String::new();
        let mut resp = self.get_url(u)?;
        resp.read_to_string(&mut buf)?;
        Ok(buf)
    }
}
