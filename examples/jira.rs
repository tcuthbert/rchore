use dotenv::dotenv;
use gouqi::{Credentials, Jira};
use std::env;
use tracing::error;

extern crate rchore;

fn main() {
    dotenv().ok();

    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    if let (Ok(host), Ok(token)) = (env::var("JIRA_HOST"), env::var("JIRA_TOKEN")) {
        let query = env::args()
            .nth(1)
            .unwrap_or("assignee = currentUser() AND resolution = Unresolved order by updated DESC".to_owned());
        let jira = Jira::new(host, Credentials::Bearer(token)).expect("Error initializing Jira");

        match jira.search().iter(query, &Default::default()) {
            Ok(results) => {
                for issue in results {
                    println!("{:#?}", issue);
                }
            }
            Err(err) => panic!("{:#?}", err),
        }
    } else {
        error!("Missing environment variable JIRA_HOST or JIRA_TOKEN!");
    }
}
