use serde::Deserialize;
use reqwest::Error;
use reqwest::header::USER_AGENT;



#[derive(Deserialize, Debug)]
struct Repo {
    name: String,
    owner: Owner,
    description: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Owner {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let owner = "kamlesh-Sahani";
    let repo = "Apis-Master";
    let request_api = format!("https://api.github.com/repos/{owner}/{repo}");
    println!("Requesting API: {request_api}");

    let client = reqwest::Client::new();
    let response = client
        .get(&request_api)
        .header(USER_AGENT, "Rust web-api-client demo")
        .send()
        .await?;

    let repo_date:Repo = response.json().await?;
    println!("Repo Data {:?}",repo_date);
    Ok(())
}
