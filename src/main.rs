use reqwest;
use reqwest::header::USER_AGENT;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use tokio;
use std::mem;

mod pcgamestorrents;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
	let m = pcgamestorrents::DownloadHandler::new(String::from("Minecraft"));
	let info = m.get_info().await?;
	println!("{:?}",info);

	let string_arr :[String;2] = ["Hello".to_string(),"GoodBye".to_string()];
	
	Ok(())
}
