extern crate slack_api as slack;

use dotenv::dotenv;
use slack::requests::{Error, Client};

fn main() {
    dotenv().ok();

    let token = std::env::var("AUTH_TOKEN").expect("AUTH_TOKEN not set in .env");
    let client = slack::default_client().unwrap();

    let response = slack::users::list(&client,
                                      &token,
                                      &slack::users::ListRequest::default());

    if let Ok(response) = response {
        if let Some(users) = response.members {
            println!("Got {} users", users.len());
            for user in users {
                println!("{:?}", user.name);
            }
        }
    } else {
        println!("{:?}", response);
    }
}
