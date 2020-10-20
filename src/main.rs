extern crate wallpaper;
extern crate chrono;
extern crate regex;

#[macro_use] extern crate lazy_static;

use colored::*;
use chrono::{Date, Local, Duration};
use std::collections::HashMap;
use regex::Regex;
use reqwest::blocking::{Client, Response};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut now: Date<Local> = Local::today();
    let mut max = 10;
    let mut image = format!("");
    let mut url_string = format!("");

    loop {
        max -= 1;

        // Build the date and the url
        let date = now.format("%Y-%m-%d").to_string();
        let url = format!("https://api.nasa.gov/planetary/apod?hd=1&api_key={key}&date={date}",
                    date = date,
                    key = "iRvIUhR0nzV2AhaTmptIthCF2psE7dv7Hkx7fVfa");


        // Query the nasa api
        let resp = Client::new().get(&url).send()?.json::<HashMap<String, String>>()?;

        // Handle video types
        if resp["media_type"] == "video" {
            let ref url = resp["url"];
            image = format!("{}", thumbnail_url(url));
            url_string = resp["url"].clone();
        }

        // Handle image types
        if resp["media_type"] == "image" {
            image = format!("{}", resp["hdurl"]);
            url_string = image.clone();
        }

        // If we have an image, report it and break
        if image != "" {
            println!("Using {}", date.green().bold());
            println!("\n{} - {}\n{}\n",
                     resp["title"].green().bold(),
                     url_string.bold().italic(),
                     resp["explanation"]);

            break;
        }

        // Report what we tried
        println!("Tried {}...", date.blue());

        // Make sure we haven"t checked too many
        now = now - Duration::days(1);
        if max == 0 {
            println!("{}", "No new wallpaper found.".red());
            break;
        }
    }

    if image != "" {
        println!("{}", "Setting wallpaper...".bold());
        let result = wallpaper::set_from_url(&*image);
        match result {
            Ok(_) => {
                println!("Done!");
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

fn thumbnail_url(url: &String) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"/embed/(.+?)(?:\?rel=0|$)").unwrap();
    }

    let mut options: [String; 6] = [String::default(), String::default(), String::default(), String::default(), String::default(), String::default()];
    for cap in RE.captures_iter(url) {
        options = [
            format!("https://img.youtube.com/vi/{}/maxresdefault.jpg", &cap[1]),
            format!("https://img.youtube.com/vi/{}/sddefault.jpg", &cap[1]),
            format!("https://img.youtube.com/vi/{}/hqdefault.jpg", &cap[1]),
            format!("https://img.youtube.com/vi/{}/mqdefault.jpg", &cap[1]),
            format!("https://img.youtube.com/vi/{}/default.jpg", &cap[1]),
            format!("https://img.youtube.com/vi/{}/0.jpg", &cap[1]),
        ];
        break;
    }

    let client: Client = Client::new();

    // Loop over each option until we have one that isn't 1097 bytes. This rules out default thumbnails
    for option in options.iter() {
        let response: Response = client.get(&option.to_string()).send().unwrap();

        // If content length is exactly 1097 it means we have the default thumbnail
        if response.content_length().unwrap() == 1097 {
            continue;
        }

        return option.to_string();
    }

    return format!("");
}