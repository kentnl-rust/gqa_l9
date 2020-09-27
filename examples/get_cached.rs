use gqa_l9::{Agent, UrlBuilder};

fn main() {
    let mut agent = Agent::default().unwrap();
    let url = UrlBuilder::new().build_string();
    let resp = agent.get_url_content(&url).unwrap();
    for line in resp.lines() {
        let cells = line.split('|');
        for cell in cells {
            eprint!("{}\t", cell);
        }
        eprintln!();
    }
}
