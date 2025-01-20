use reqwest::Client;
use tokio;
use std::io;
use serde_json::Value;
use colored::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("
    
     ____    _                              ___            __         
    |  _ \\  | |__     ___    _ __     ___  |_ _|  _ __    / _|   ___  
    | |_) | | '_ \\   / _ \\  | '_ \\   / _ \\  | |  | '_ \\  | |_   / _ \\ 
    |  __/  | | | | | (_) | | | | | |  __/  | |  | | | | |  _| | (_) |
    |_|     |_| |_|  \\___/  |_| |_|  \\___| |___| |_| |_| |_|    \\___/ 
    
    OpenSource Project KodakSec
    ");
    println!("Enter the phone number you want to search:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let phone_number = input.trim();
    let formatted_phone_number: String = phone_number.chars()
        .filter(|c| c.is_numeric())
        .collect();
    let country_code = "33";
    let full_phone_number = if formatted_phone_number.starts_with(country_code) {
        formatted_phone_number.clone()
    } else {
        format!("{}{}", country_code, formatted_phone_number)
    };
    let url = "https://api.apilayer.com/number_verification/validate";
    let params = [
        ("number", &full_phone_number),
    ];
    let api_key = "9N8LbOSbAGHTIiVGAz6CXXl0KvwvOR5l"; // Go to https://apilayer.com/marketplace/number_verification-api --> free api 200 request just register on website
    let client = Client::new();
    let response = client.get(url)
        .query(&params)
        .header("apikey", api_key)
        .send()
        .await?;
    if response.status().as_u16() == 200 {
        let data: Value = response.json().await?;
        println!("{}", "Phone Number Details:".red().bold());

        for (key, value) in data.as_object().unwrap() {
            match value {
                serde_json::Value::String(s) => {
                    println!("{}: {}", key.red(), s);
                }
                serde_json::Value::Bool(b) => {
                    println!("{}: {}", key.red(), b);
                }
                _ => {
                    println!("{}: {:?}", key.red(), value);
                }
            }
        }
    } else {
        println!("Failed to retrieve the page. Status code: {}", response.status().to_string().red());
    }
    Ok(())
}
