use std::io::{self, Write};

use colored::Colorize;
use reqwest::header::{HeaderMap, USER_AGENT};
use serde_json;
use serde::{Deserialize, Serialize};

fn error_msg(message: String) -> String {
    return message.red().to_string();
}

pub fn get_username() {

    let username_msg: String = ">> Enter your username: ".yellow().to_string();
    print!("{}", username_msg);
    io::stdout().flush().unwrap();

    let mut mut_github_username = String::new();
    io::stdin().read_line(&mut mut_github_username).ok().expect(&error_msg("[x] - Couldn't get username".to_string()));

    let github_username: String = mut_github_username.trim().parse().expect(&error_msg("[x] - Expected a string".to_string()));
    api_connect(github_username);
}

fn api_connect(github_username: String) {
    let github_api_url = format!("https://api.github.com/users/{}", github_username);
    let request_msg: String = format!(">> Requesting info from: {}", github_api_url).bright_green().to_string();
    println!("{}", request_msg);

    let user_agent: String = "Mozilla/5.0 (X11; Fedora; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/110.0".to_string();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, user_agent.parse().unwrap());

    let request_client = reqwest::blocking::Client::builder()
    .default_headers(headers)
    .build()
    .unwrap();

    match request_client.get(&github_api_url).send() {
        Ok(response) => {
            let response_body: String = response.text().unwrap_or_else(|_| "Empty response".to_string());
            get_information(response_body);
        },
        Err(e) => println!("[x] - Error trying to get API body response: {}", e)
    }
}

fn get_information(response_body: String) {

    #[derive(Serialize, Deserialize)]
    struct User {
        login: String,
        id: u32,
        public_repos: u32,
        followers: u32,
        following: u32,
    }

    let user: User = serde_json::from_str(&response_body).unwrap();

    let return_message: String = format!("
* Username: {}
* User ID: {}
* Public Repos: {}
* Followers: {}
* Following: {}
    ", user.login, user.id, user.public_repos, user.followers, user.following);

    println!("{}", return_message.yellow().bold().to_string());
}