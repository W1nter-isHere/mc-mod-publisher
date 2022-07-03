extern crate core;

mod publishing_info;

use std::io::{stdin, Stdin};
use std::path::Path;
use console::style;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::__private::de::IdentifierDeserializer;
use serde::de::Unexpected::Option;
use serde_json::Value;
use publishing_info::PublishingInformation;
use crate::publishing_info::LinkInformation;

static API_KEY: &'static str = "$2a$10$nUcmykmfhF225Wnjw1i7d.HIDTX6qqLRKrR62eD2WYofvZCk4KBei";
static CF_API: &'static str = "https://api.curseforge.com/v1/mods/";

static CACHE_PATH: &'static str = "./mmp_cache";
static PROJECT_PATH: &'static str = "./mmp_cache/project.json";

#[tokio::main]
async fn main() {
    println!("Welcome to MC Mod Publisher, make sure this executable is in the root directory of the mod dev environment");
    let stdin = stdin();
    
    println!("{}", style("Initializing web client").dim());
    let web_client = Client::new();
    let cache_path = Path::new(PROJECT_PATH);

    let mut cf_header = HeaderMap::new();
    cf_header.insert("Accept", HeaderValue::from_str("application/json").unwrap());
    cf_header.insert("x-api-key", HeaderValue::from_str(&API_KEY).unwrap());

    let mut project_info= None;
    
    if !cache_path.exists() {
        while project_info.is_none() {
            println!("Please configure your project\n1. Import project from curseforge\n2. Create new project");

            let mode_str = user_input(&stdin);

            if mode_str.contains("1") {
                project_info = Some(retrieve_cf_project_info(&stdin, &web_client, &cf_header, &cache_path).await);
            } else if mode_str.contains("2") {
                project_info = Some(create_info(&stdin, &cache_path));
            }
        }
    } else { 
        project_info = Some(retrieve_cache_info(&cache_path));
    }
    
    if project_info.is_none() { 
        panic!("This should not happen, project should be configured!");
    }

    println!("Upload to (12 for both)\n1. Curseforge\n2. Modrinth");
    let upload_destination = user_input(&stdin);
    
    if upload_destination.contains("1") { 
        
    }
    
    if upload_destination.contains("2") {
        
    }
}

async fn retrieve_cf_project_info(stdin: &Stdin, client: &Client, cf_header: &HeaderMap, cache_path: &Path) -> PublishingInformation {
    println!("Please enter the cf project ID");
    let cf_project_id_str = user_input(&stdin);
    
    println!("{} Retrieving mod information from curseforge api", style("[1/4]").dim().bold());
    let mod_body = client
        .get(CF_API.clone().to_owned() + cf_project_id_str.as_str())
        .headers(cf_header.clone())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{} Retrieving mod description from curseforge api", style("[2/4]").dim().bold());
    let description_body = client
        .get(CF_API.clone().to_owned() + cf_project_id_str.as_str() + "/description")
        .headers(cf_header.clone())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    
    let body: &Value = &serde_json::from_str::<Value>(&mod_body).unwrap()["data"];
    let description: Value = serde_json::from_str(&description_body).unwrap();
    
    let publishing_info = PublishingInformation::from_cf_json(body, &description);
    let info_json = serde_json::to_string(&publishing_info).unwrap_or("{}".to_string());
    std::fs::write(cache_path, info_json);
    
    return publishing_info;
}

fn create_info(stdin: &Stdin, cache_path: &Path) -> PublishingInformation {
    println!("Please enter desired mod name");
    let mod_name = user_input(&stdin);
    
    let default_slug = mod_name.clone().to_lowercase().replace(" ", "_");
    println!("Please enter desired slug ({}?)", default_slug);
    
    let mut slug = user_input(&stdin);
    if slug.is_empty() { 
        slug = default_slug;
    }

    println!("Please enter a short mod summary");
    let summary = user_input(&stdin);
    
    println!("Please enter a mod description (can point to file)");
    let mut body = user_input(&stdin);
    let path = Path::new(&body);
    
    if path.exists() {
        body = std::fs::read_to_string(&path).unwrap();
    }

    println!("Please enter a website link");
    let website = user_input(&stdin);

    println!("Please enter a wiki link");
    let wiki = user_input(&stdin);

    println!("Please enter a issues report link");
    let issues = user_input(&stdin);

    println!("Please enter a source code link");
    let source = user_input(&stdin);

    println!("Please enter a discord link");
    let discord = user_input(&stdin);

    let info = PublishingInformation::new(
        mod_name.as_str(),
        slug.as_str(),
        summary.as_str(),
        body.as_str(),
        &LinkInformation::new(
            website.as_str(),
            wiki.as_str(),
            issues.as_str(),
            source.as_str(),
            discord.as_str()
        )
    );

    let info_json = serde_json::to_string(&info).unwrap_or("{}".to_string());
    std::fs::write(cache_path, info_json);

    return info;
}

fn retrieve_cache_info(cache_path: &Path) -> PublishingInformation {
    serde_json::from_str(std::fs::read_to_string(cache_path).unwrap().as_str()).unwrap()
}

fn user_input(stdin: &Stdin) -> String {
    let mut value = String::new();
    stdin.read_line(&mut value);
    value
}

pub fn get_str_from_json(body: &Value, key: &str) -> String {
    body[key].as_str().unwrap_or("").clone().to_string()
}