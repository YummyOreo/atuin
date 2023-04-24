use std::env;

use clap::Parser;
// use eyre::Result;

pub const ATUIN_INCOGNITO_VAR_NAME: &str = "ATUIN_INCOGNITO";

#[derive(Parser)]
pub struct Cmd {
    command: Option<String>,
}

impl Cmd {
    pub fn run(self) {
        if env::var(ATUIN_INCOGNITO_VAR_NAME).unwrap_or_else(|_| String::from("false")) == *"true" {
            env::set_var(ATUIN_INCOGNITO_VAR_NAME, "false");
            println!("Toggled off incognito");
        } else {
            env::set_var(ATUIN_INCOGNITO_VAR_NAME, "true");
            println!("Toggled on incognito");
        }
        println!("{:?}", env::var(ATUIN_INCOGNITO_VAR_NAME));

        if let Some(_command) = self.command {
            todo!();
        }
    }
}

// async fn run() -> Result<()> {
//     Ok(())
// }
