use rss::Item;
use rss::Channel;
use std::cmp::Ordering;
use chrono::DateTime;

pub fn generate_channels_css(channels: &Vec<Channel>) -> String {
        // generate a line like this for every channel
        //#xiokka:checked ~ .container .xiokka,
        let mut channel_css = String::new();
        for channel in channels {
                channel_css.push_str(&format!(
"#{}:checked ~ .container .{},",to_id(&channel.title),to_id(&channel.title
                )));
        }
        channel_css.pop(); // remove last comma
        return channel_css;
}

pub fn generate_channels_radio(channels: &Vec<Channel>) -> String {
        let mut channel_bar = String::new();
        channel_bar.push_str(&format!(
"<input type=\"radio\" name=\"author\" id=\"all\" class=\"author-radio\" checked>"));

        for channel in channels {
                channel_bar.push_str(&format!(
"<input type=\"radio\" name=\"author\" id=\"{}\" class=\"author-radio\">"
, to_id(&channel.title)));
        }
        return channel_bar;
}

pub fn generate_labels(channels: &Vec<Channel>) -> String {
        let mut channel_labels = String::new();
        channel_labels.push_str(&format!("<label for=\"all\">All</label>"));

        for channel in channels {
                channel_labels.push_str(&format!("<label for=\"{}\">{}</label>"
, to_id(&channel.title), channel.title));
        }
        return channel_labels;
}

pub fn items_to_html(items: Vec<Item>) -> String {
    let mut feed = String::new();

    // Default values
    let default_title = "Untitled".to_string();
    let default_item_title = "No Title".to_string();
    let default_pub_date = "No Date".to_string();
    let default_link = "".to_string();
        for i in 0..items.len() {
            let item = &items[i]; // Access the item by index
            let item_author = item.author.as_ref().unwrap_or(&default_item_title);
            let item_title = item.title.as_ref().unwrap_or(&default_item_title);
            let pub_date = item.pub_date.as_ref().unwrap_or(&default_pub_date);
            let link = item.link.as_ref().unwrap_or(&default_link);
	
            let mut this_item = format!("<div class=\"item all {}\">
                <h3><a href=\"{}\" target=\"view\">{}</a></h3>
		<h2>{}</h2>
		<p><i>{}</i></p>
		",
               to_id(&item_author), link, item_title, item_author , pub_date
            );

            if let Some(enclosure) = &item.enclosure {
                let mut length: u64 = enclosure.length.parse().unwrap_or(0); // bytes
                length /= 1024; // Convert to KB
                let mut unit = "KB".to_string();
                if length > 1024 {
                    length /= 1024; // Convert to MB
                    unit = "MB".to_string();
                }
                this_item.push_str(&format!(
                    "<a href=\"{}\" target=\"view\">📎See attached media [{} {}]</a>",
                    enclosure.url,
                    length,
                    unit
                ));
            }
            this_item.push_str("</div>");
            feed.push_str(&this_item);
        }
    feed
}

pub fn merge_and_sort(feeds: &mut Vec<Channel>) -> String {
        let mut all_entries:Vec<Item> = vec![];
        for mut channel in feeds {
            for i in 0..channel.items.len() {
                channel.items[i].author = Some(format!("{}", channel.title.to_string()));
            }


                if let Some(ref image) = channel.image {
                        for item in &mut channel.items {
                                let image = format!("<img src=\"{}\" width=\"64\" height=\"64\" style=\"border-radius: 50%; float: left; padding: 10px\">", channel.image.clone().unwrap().url);
                                item.comments = Some(image);
                        }
                }
                all_entries.extend(channel.items.clone());
        }


        // Sort items by pub_date in descending order
        all_entries.sort_unstable_by(|a, b| {
                let date_a = a.pub_date.as_ref().and_then(|date| DateTime::parse_from_rfc2822(date).ok());
                let date_b = b.pub_date.as_ref().and_then(|date| DateTime::parse_from_rfc2822(date).ok());
                match (date_a, date_b) {
                        (Some(d_a), Some(d_b)) => d_b.cmp(&d_a), // Reverse the order for descending sort
                        (None, Some(_)) => std::cmp::Ordering::Less, // `None` comes before `Some`
                        (Some(_), None) => std::cmp::Ordering::Greater, // `Some` comes after `None`
                        (None, None) => std::cmp::Ordering::Equal, // Both are `None`
                }
        });
        return items_to_html(all_entries);
}

pub fn to_id(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .map(|c| c.to_ascii_lowercase())
        .collect()
}
