use std::{path::Path, thread};
use thirtyfour::{self, error::WebDriverResult, By, DesiredCapabilities, WebDriver};

struct Post {
    site: String,
    author: String,
    title: String,
    content: String,
}

struct Crawler {
    found_nodes: Vec<String>,
    found_posts: Vec<Post>,
    driver: WebDriver,
}

impl Crawler {
    async fn init() -> WebDriverResult<Crawler> {
        let mut init_caps = DesiredCapabilities::firefox();
        init_caps.set_firefox_binary(Path::new(
            "/mnt/c/Users/Diego\\ Andrade/Desktop/Tor\\ Browser/Browser/firefox.exe",
        ))?;

        Ok(Crawler {
            found_nodes: vec![],
            found_posts: vec![],
            driver: WebDriver::new("http://localhost:4444", init_caps).await?,
        })
    }

    async fn scrape(&self, url: &str, class_name: &str) -> WebDriverResult<()> {
        let mut posts: Vec<Post> = vec![];

        self.driver.goto(url).await?;
        self.driver.find(By::ClassName(class_name)).await?;

        Ok(())
    }

    async fn crawl(&self, source_nodes: Vec<&'static str>, depth: usize) -> WebDriverResult<()> {
        if source_nodes.len() == 1 {
            self.scrape(source_nodes[0], "postMain").await?;
        } else {
            // spawn threads and crawl
            let mut threads = vec![];

            for source in source_nodes {
                threads.push(thread::spawn(move || {}));
            }

            for thread in threads {
                thread.join().unwrap();
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let crawler = Crawler::init().await?;
    crawler.crawl(vec!["test", "test1"], 1).await?;

    Ok(())
}
