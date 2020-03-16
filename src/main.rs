use clap::{App,Clap};
use std::env;
use reqwest::{header, Client};
use serde::Deserialize;

#[derive(Clap,Debug)]
#[clap(version = "1.0", author = "Baspar")]
struct Opts {
    #[clap(short = "a", long = "author")]
    authors: Vec<String>,
    #[clap(short = "o", long = "organization")]
    organization: String
}

#[derive(Deserialize,Debug)]
struct Repo {
    name: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = env::var("GITHUB_API_KEY").expect("GITHUB_API_KEY should be defined");
    let opts: Opts = Opts::parse();

    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("token {}", api_key).parse().unwrap());
    let client = Client::builder()
        .default_headers(headers)
        .user_agent("My Rust Program 1.0")
        .build()?;

    let repos = client.get(&format!("https://api.github.com/orgs/{}/repos?per_page=100", opts.organization))
        .send().await?
        .json::<Vec<Repo>>().await?;
    println!("{:?}: {:?} ", repos.len(), repos);

    println!("{:?}", opts);
    println!("Hello, world!");
    Ok(())
}
