use std::fs::File;
use std::io::BufReader;
use rss::Channel;
use rss::Item;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let feeds = vec![
    "https://www.reutersagency.com/feed/?best-topics=political-general&post_type=best",
    "https://techcrunch.com/feed",
    "https://hnrss.org/frontpage",
    "https://feeds.bloomberg.com/markets/news.rss",
    "https://feeds.bloomberg.com/politics/news.rss"
  ]; 
  
  for f in feeds {
    consume_feed(f).await;
  }
  Ok(())
}
