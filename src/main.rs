use std::thread;
use std::time::Duration;

struct Crawler {
    found_nodes: Vec<String>,
}

impl Crawler {
    fn init() -> Crawler {
        Crawler {
            found_nodes: vec![],
        }
    }

    fn crawl(&self, source_nodes: Vec<&'static str>, depth: usize) -> () {
        if source_nodes.len() == 1 {
            // crawl from 1
        } else {
            // spawn threads and crawl
            let mut threads = vec![];

            for source in source_nodes {
                threads.push(thread::spawn(move || {
                    thread::sleep(Duration::from_secs(2));
                    println!("{}", source);
                }));
            }

            for thread in threads {
                thread.join().unwrap();
            }
        }
    }
}

fn main() {
    let crawler = Crawler::init();
    crawler.crawl(vec!["test", "test1"], 1);
}
