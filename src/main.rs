use std::net::IpAddr;

use dialoguer::{Select, theme::ColorfulTheme, Error, Input};
use local_ip_address::local_ip;
use once_cell::sync::Lazy;
use coloriz::*;


static PRIMARY_COLOR: (u8, u8,u8) = (127, 0, 255);
static DEVICE_ID: Lazy<IpAddr> = Lazy::new(|| local_ip().unwrap());

fn main() {
    let prompts = vec!["Create a budget", "Connect to budget", "List budgets"];

    let option = prompter(prompts).unwrap();

    match option {
        0 => {
            create_budget()
        },
        1 => {
            println!("Poooorly");
        },
        2 => {
            println!("Poooorly");
        },
        default => {
            println!("Wiiiisely {}", default);
        }
    }
}

fn prompter(prompts: Vec<&str>) -> Result<usize, Error> {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you choose?")
        .items(&prompts)
        .interact()
}

fn inputer(input: &str) -> Result<String, Error> {
    Input::<String>::new()
        .with_prompt(input)
        .interact_text()
}

fn create_budget() {
    let budget_name = inputer("Enter budget name").unwrap();
    println!("You ({}) created budget: {}", DEVICE_ID.to_string(), primary_color(&budget_name));
}

fn primary_color(text: &str) -> StyledText {
    text.fg(PRIMARY_COLOR)
}