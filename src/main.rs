use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::error::Error;
use std::fs;
use dirs;

use rss::Channel;
use rss::Item;

use webui_rs::webui;
use webui_rs::webui::WebUIBrowser;
use webui_rs::webui::wait;

mod html;
use crate::html::microblog_html;
use crate::html::default_css;

mod generate;
use crate::generate::*;

mod fetch;
use crate::fetch::*;

pub fn main() -> io::Result<()>{
	// Locate the home directory
	let home_dir = dirs::home_dir().expect("Failed to find home directory");

	// Locate the .newsread directory. If it doesn't exist, create it.
	let newsread_dir = home_dir.join(".newsread");
	if !newsread_dir.exists() {
		match fs::create_dir_all(&newsread_dir) {
			Ok(_) => println!(".newsread directory created successfully."),
			Err(e) => eprintln!("Failed to create .newsread directory: {}", e),
		}
	} else {
		println!(".newsread directory already exists.");
	}

	// Check and create 'urls' file if it doesn't exist
	let file_path = newsread_dir.join("urls");
	if !file_path.exists() {
		let file = File::create(&file_path)?;
		println!("File created at: {:?}", file_path);
	} else {
		println!("File already exists at: {:?}", file_path);
	}

	// Check and create 'newsread.css' file if it doesn't exist
	let css_file_path = newsread_dir.join("newsread.css");
	if !css_file_path.exists() {
		let mut file = File::create(&css_file_path)?;
		file.write_all(default_css.as_bytes())?;
		println!("File created at: {:?}", css_file_path);
	} else {
		println!("File already exists at: {:?}", css_file_path);
	}

	// Print the paths
	println!("Home directory: {:?}", home_dir);
	println!(".newsread directory: {:?}", newsread_dir);
	println!("CSS file path: {:?}", css_file_path);

	// Open the URL file
	let file = File::open(&file_path).map_err(|e| {
		io::Error::new(
			e.kind(),
			format!("Failed to open the file at {}: {}.", file_path.display(), e),
		)
	})?;

	// Open the CSS file
	let css_content = fs::read_to_string(css_file_path)?;

	// Create a buffered reader for efficient reading
	let reader = io::BufReader::new(file);

	// Initialize a Vec<&str> to store each line from the file
	let mut url_vec: Vec<String> = Vec::new();

	// Iterate over each line in the file
	for line in reader.lines() {
		// Unwrap the line and push it to the vector
		match line {
			Ok(line_str) => url_vec.push(line_str),
			Err(e) => eprintln!("Error reading line: {}", e),
		}
	}

	let mut feeds = fetch_feeds_concurrently(url_vec);
	println!("Sorting channels. ");
	feeds.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
        let win = webui::Window::new();
	println!("Generating radio input.");
	let channel_radio = generate_channels_radio(&feeds);
        println!("Generating labels.");
        let labels = generate_labels(&feeds);
        println!("Generating channels CSS.");
	let channel_css = generate_channels_css(&feeds);
        println!("Sorting entries.");
	let latest_entries = merge_and_sort(&mut feeds);
	let mut base_html = microblog_html.to_string();
        println!("Generating HTML.");
	base_html = base_html.replace("{CONTENT}", &latest_entries);
	base_html = base_html.replace("{RADIO}", &channel_radio);
        base_html = base_html.replace("{CHANNELS_CSS}", &channel_css);
        base_html = base_html.replace("{LABELS}", &labels);
	base_html = base_html.replace("{CUSTOM_CSS}", &css_content);
        println!("Showing window.");
	win.show_browser(&base_html, WebUIBrowser::AnyBrowser);
	wait();
	Ok(())
}
