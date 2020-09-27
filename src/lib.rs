mod err;
mod ua;
mod url_build;

pub use ua::Agent;
pub use url_build::UrlBuilder;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
