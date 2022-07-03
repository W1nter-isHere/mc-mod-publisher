use core::panicking::panic;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use crate::get_str_from_json;

#[derive(Serialize, Deserialize)]
pub struct PublishingInformation {
    pub name: String,
    pub slug: String,
    pub summary: String,
    pub description: String,
    pub links: LinkInformation
}

impl PublishingInformation {
    pub fn from_cf_json(mod_data: &Value, description: &Value) -> Self {
        let links = LinkInformation::from_cf_json(&mod_data["links"]);
        
        PublishingInformation {
            name: get_str_from_json(&mod_data, "name"),
            slug: get_str_from_json(&mod_data, "slug"),
            summary: get_str_from_json(&mod_data, "summary"),
            description: get_str_from_json(&description, "data"),
            links
        }
    }

    pub fn new(name: &str, slug: &str, summary: &str, body: &str, links: &LinkInformation) -> Self {
        PublishingInformation {
            name: name.clone().to_owned(),
            slug: slug.clone().to_owned(),
            summary: summary.clone().to_owned(),
            description: body.clone().to_owned(),
            links: links.clone().to_owned()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LinkInformation {
    pub web_url: String,
    pub wiki_url: String,
    pub issues_url: String,
    pub source_url: String,
    pub dc_url: String
}

impl LinkInformation {
    pub fn from_cf_json(data: &Value) -> Self{
        LinkInformation::new(
            get_str_from_json(&data, "websiteUrl").as_str(), 
            get_str_from_json(&data, "wikiUrl").as_str(), 
            get_str_from_json(&data, "issuesUrl").as_str(), 
            get_str_from_json(&data, "sourceUrl").as_str(), 
            ""
        )
    }

    pub fn new(web_url: &str, wiki_url: &str, issues_url: &str, source_url: &str, dc_url: &str) -> Self {
        LinkInformation {
            web_url: web_url.clone().to_owned(),
            wiki_url: wiki_url.clone().to_owned(),
            issues_url: issues_url.clone().to_owned(),
            source_url: source_url.clone().to_owned(),
            dc_url: dc_url.clone().to_owned()
        }
    }
}

impl Clone for LinkInformation {
    fn clone(&self) -> Self {
        LinkInformation {
            web_url: self.web_url.clone(),
            wiki_url: self.wiki_url.clone(),
            issues_url: self.issues_url.clone(),
            source_url: self.source_url.clone(),
            dc_url: self.dc_url.clone()
        }
    }

    fn clone_from(&mut self, _source: &Self) {
        panic!("Why are you doing this?");
    }
}