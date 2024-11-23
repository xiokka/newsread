use std::error::Error;
use rss::Channel;
use std::time;
use std::sync::{Arc, Mutex};
use std::thread;

fn open_feed(url: String) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::blocking::get(&url)?
        .bytes()?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

pub fn fetch_feeds_concurrently(url_vec: Vec<String>) -> Vec<Channel> {
    let feeds_vec = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for url in url_vec {
        let feeds_vec = Arc::clone(&feeds_vec);  // Clone the Arc pointer for thread safety
        let url_clone = url.clone();  // Clone the URL here to move the clone into the thread
        println!("Fetching: {}", url_clone);
        let handle = thread::spawn(move || {
            match open_feed(url_clone) {
                Ok(mut channel) => {
                    let mut feeds = feeds_vec.lock().unwrap();
                    println!("Fetched: {}", channel.title);
                    feeds.push(channel);  // Safely push to shared vector
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            };
        });
        handles.push(handle); // Collect thread handles
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Now that all threads have finished, we can safely collect the Vec<Channel> from the Mutex
    let feeds = feeds_vec.lock().unwrap();  // Lock the Mutex to access the vector
    feeds.clone()  // Return a clone of the vector (so it's no longer behind a Mutex)
}
