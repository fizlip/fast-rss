use std::fs::File;
use rss::Channel;
use rss::Item;
use std::io::{BufRead, BufReader};

async fn read_rss(url: &str) -> Option<Vec<Item>>{
  let content = reqwest::get(url)
        .await.ok()?
        .bytes()
        .await.ok()?;

  let channel = Channel::read_from(&content[..]).ok()?;
  Some(channel.items)
}

fn print_items(items: Vec<Item>) {
  for item in items {
    println!("{:?}", item.title.unwrap());
    println!("{:?}", item.link.unwrap());
    println!("\n")
  }
}

async fn consume_feed(url:&str) -> Result<(), &str>{
  let items = read_rss(url).await.ok_or("Could not fetch rss feed")?;
  print_items(items);
  Ok(())
}

fn get_feeds() -> std::io::Result<Vec<String>> {
  let file = File::open("feeds.txt")?;
  let reader = BufReader::new(file);
  reader.lines().collect()
} 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let feeds:Vec<String> = get_feeds()?;  
  for f in feeds {
    consume_feed(&f).await;
  }
  Ok(())
}
