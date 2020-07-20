extern crate wallpaper;
extern crate chrono;
extern crate regex;

#[macro_use] extern crate lazy_static;

use colored::*;
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut now: DateTime<Utc> = Utc::now();
    let mut max = 10;
    let client = reqwest::blocking::Client::new();
    let mut image = format!("");


    loop {
        max -= 1;

        // Build the date and the url
        let date = now.format("%Y-%m-%d").to_string();
        let url = format!("https://api.nasa.gov/planetary/apod?hd=1&api_key={key}&date={date}",
                    date = date,
                    key = "iRvIUhR0nzV2AhaTmptIthCF2psE7dv7Hkx7fVfa");


        // Query the nasa api
        let resp = client.get(&url).send()?.json::<HashMap<String, String>>()?;

        // Handle video types
        if resp["media_type"] == "video" {
            let ref url = resp["url"];
            image = format!("{}", thumbnail_url(url));
        }

        // Handle image types
        if resp["media_type"] == "image" {
            image = format!("{}", resp["hdurl"]);
        }


        // If we have an image, report it and break
        if image != "" {
            println!("Using {}", date.green().bold());
            println!("\n{} - {}\n{}\n", 
                resp["title"].green().bold(),
                image.bold().italic(),
                resp["explanation"]);

            break;
        }

        // Report what we tried
        println!("Tried {}...", date.blue());

        // Make sure we haven't checked too many
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

    for cap in RE.captures_iter(url) {
        return format!("https://img.youtube.com/vi/{}/maxresdefault.jpg", &cap[1]);
    }

    return format!("");
}