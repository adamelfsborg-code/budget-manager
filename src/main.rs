use dialoguer::{Select, theme::ColorfulTheme, Error, Input};
pub mod color;

use coloriz::*;

const PRIMARY_COLOR: (u8, u8,u8) = (127, 0, 255);

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
    let budget_name = inputer("Enter budget name").unwrap().fg(PRIMARY_COLOR);
    println!("YOu budget will be called: {}", budget_name);
}