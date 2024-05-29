use anyhow::{Error,Result,Context};
use serde::Deserialize;
use std::env;
use std::io::{self, Write};
use base64::encode;
use reqwest::blocking::Client;
use serde_json::Value;

#[derive(Deserialize, Debug)]
struct Company {
    title: String,
    company_number: String,
    company_status: String,
    date_of_creation: String,
    company_type: String,
    address_snippet: String,
}

fn get_request



fn main() -> Result<()> {
    // Retrieve the API key from environment variables
    let api_key = env::var("COMPANIES_HOUSE_API_KEY").expect("COMPANIES_HOUSE_API_KEY not set");

    //company name prompt
    print!("Enter company name: ");
    io::stdout().flush().unwrap();
    
    let mut company_name = String::new();
    io::stdin().read_line(&mut company_name).unwrap();
    let company_name = company_name.trim();

    // Encode the API key for Basic Authentication
    let encoded_key = encode(format!("{}:", api_key));

    // API request
    let url = format!(
        "https://api.company-information.service.gov.uk/search/companies?q={}",
        company_name
    );
    let client = Client::new();
    let res = client
        .get(&url)
        .header("Authorization", format!("Basic {}", encoded_key))
        .send().context("failed to send")?;

    let status: reqwest::StatusCode = res.status();

    if !status.is_success() {
        let error_message = format!("?Unable to send request due: {status}");
        return Err(Error::msg(error_message));
    }

    let body: Value = res.json().context("failed to turn into json")?;
    


    // Extract the company details from the response
    if let Some(items) = body["items"].as_array() {
        if !items.is_empty() {
            // Attempt to parse the first item as a Company
            let company_info: Company = serde_json::from_value(items[0].clone()).unwrap();
            println!("Company Information:");
            println!("Title: {}", company_info.title);
            println!("Number: {}", company_info.company_number);
            println!("Status: {}", company_info.company_status);
            println!("Date of Creation: {}", company_info.date_of_creation);
            println!("Type: {}", company_info.company_type);
            println!("Address: {}", company_info.address_snippet);
        } else {
            println!("No company found with the name '{}'", company_name);
        }
    } else {
        println!("Error retrieving company details");
    }

    Ok(())
}
