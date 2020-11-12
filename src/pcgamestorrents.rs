use select;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use reqwest;
use scraper::{Selector, Html};

use reqwest::header::USER_AGENT;
use tokio;

pub const URL:&str = "https://pcgamestorrents.com/";



pub struct DownloadHandler {
	pub search_query:String,
	pub client:reqwest::Client,
}

impl DownloadHandler {

	pub fn new(search_query: String) -> Self {
		 Self {
				search_query,
				client:reqwest::Client::new()
			}
	}
	pub async fn get_html(&self,url:&str) -> Result<String,reqwest::Error> {
		

		////println!("{}",url_search);

		let res: reqwest::Response = self.client
		.get(url)
		.header(USER_AGENT, "Mozilla/5.0")
		.send()
		.await?;
		Ok(res.text().await?)
	}

	/// will get all of the links to the download page 
	/// NOTE: this page still not contain the magnet link
	pub async fn get_info(&self)-> Result<Vec<[String;2]>,reqwest::Error> {
		let mut url_search:String= URL.clone().to_string();
		url_search.push_str("?s=");
		url_search.push_str(&self.search_query.replace(" ", "+"));

		let html :&str = &self.get_html(&url_search).await?;
		let document = Document::from(html);
		let mut link_info :Vec<[String;2]> = Vec::new();

		for node in document.find(Name("a")).into_iter() {
			if node.text().contains(&self.search_query) {
				if let Some(href) = node.attr("href") {
					//println!("{} ({:?})", node.text().trim(), href);
					link_info.push([node.text().trim().to_string(),href.to_string()]);
				}
			}
		}
		Ok(link_info)
	}

	pub async fn get_magnet(&self,index:usize) ->Result<String,reqwest::Error>{
		
		let info = self.get_info().await?;
		let link = info[index][1];

		let html = self.get_html(&link).await?;



		
		// Need to put here the Vec of string 
		// the strings are the magnets link
		Ok(())
	}
}

