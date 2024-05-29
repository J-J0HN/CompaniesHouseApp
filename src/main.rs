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

impl Company {
    fn display(&self) {
        println!("
            \ncompany_title: {},
            \ncompany_number: {},
            \ncompany_status: {},
            \ncompany_creation: {},
            \ncompany_type: {},
            \ncompany_address: {}
        ",self.title,self.company_number,self.company_status,self.date_of_creation,self.company_type,self.address_snippet)
    }
}



fn get_user_input(prompt: &str) -> Result<String> {
    let mut return_string = String::new();
    println!("{prompt}: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut return_string).context("unable to read user input")?;
    Ok(return_string.trim().to_owned())
}
// --------------------------------------------------------------------------------------------------
fn get_request(url: &str, api_key: &str) -> Result<Value> {
    // Encode the API key for Basic Authentication
    let encoded_key = encode(format!("{}:", api_key));
    let client = Client::new();
    let res = client
        .get(url)
        .header("Authorization", format!("Basic {}", encoded_key))
        .send().context("failed to send")?;

    let status: reqwest::StatusCode = res.status();

    if !status.is_success() {
        let error_message = format!("?Unable to send request due: {status}");
        return Err(Error::msg(error_message));
    }

    let body: Value = res.json().context("failed to turn into json")?;
    
    Ok(body)
}

fn turn_json_to_vec(body: Value, number_of_items: i32) -> Result<Vec<Company>> {
    let mut companies_vec: Vec<Company> = Vec::new();
    if let Some(items) = body["items"].as_array() {
        if items.is_empty() {
            return Err(Error::msg("No companies found with this name"))
            
        } else {
            for company_json in items {
                if companies_vec.len() > number_of_items as usize {
                    return Ok(companies_vec);
                }
                let company_info: Company = serde_json::from_value(company_json.clone()).context("Could not parse company")?;
                companies_vec.push(company_info);
            }
           
        }
    } else {
        return Err(Error::msg("Company data is not an array"))
    }
    Ok(companies_vec)
}

fn display_vector_of_companies(company_vector: Vec<Company>) {
    for company in company_vector {
        println!("----------------------");
        company.display();
    }
}



fn main() -> Result<()> {
    // Retrieve the API key from environment variables
    let api_key = env::var("COMPANIES_HOUSE_API_KEY").expect("COMPANIES_HOUSE_API_KEY not set");

    let company_name = get_user_input("type in a company")?;
    let company_amount = get_user_input("type in the maximum amount of companies to show")?;
    let company_amount_as_i32 = company_amount.parse::<i32>()?;

    let url = format!(
        "https://api.company-information.service.gov.uk/search/companies?q={}",
        company_name
    );

    let body = get_request(&url, &api_key)?;

    let vector_of_companies = turn_json_to_vec(body,company_amount_as_i32)?;

    display_vector_of_companies(vector_of_companies);
   
    Ok(())
}
