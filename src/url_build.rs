pub struct UrlBuilder {
    protocol: String,
    domain: String,
    results_path: String,
    repo_name: String,
    check_path: String,
}

impl UrlBuilder {
    pub fn new() -> Self {
        UrlBuilder {
            protocol: "https".to_owned(),
            domain: "gentooqa.levelnine.at".to_owned(),
            results_path: "results".to_owned(),
            repo_name: "gentoo".to_owned(),
            check_path: "stats/EAC-STA-ebuild_cleanup_candidates".to_owned(),
        }
    }
    pub fn protocol(mut self, protocol: &str) -> Self {
        self.protocol = protocol.to_owned();
        self
    }
    pub fn domain(mut self, domain: &str) -> Self {
        self.domain = domain.to_owned();
        self
    }
    pub fn results_path(mut self, results_path: &str) -> Self {
        self.results_path = results_path.to_owned();
        self
    }
    pub fn repo_name(mut self, repo_name: &str) -> Self {
        self.repo_name = repo_name.to_owned();
        self
    }
    pub fn check_path(mut self, check_path: &str) -> Self {
        self.check_path = check_path.to_owned();
        self
    }
    pub fn build_string(self) -> String {
        self.protocol
            + "://"
            + &self.domain
            + "/"
            + &self.results_path
            + "/"
            + &self.repo_name
            + "/"
            + &self.check_path
            + "/full.txt"
    }
}
