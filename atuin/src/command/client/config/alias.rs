use clap::Subcommand;
use eyre::{Context, ContextCompat, Result};

use atuin_client::{encryption, record::sqlite_store::SqliteStore, settings::Settings};

use atuin_config::store::AliasStore;

#[derive(Subcommand, Debug)]
#[command(infer_subcommands = true)]
pub enum Cmd {
    Set { name: String, value: String },
    Delete { name: String },
    Get { name: Option<String> },
}

impl Cmd {
    async fn set(&self, store: AliasStore, name: String, value: String) -> Result<()> {
        store.set(&name, &value).await?;

        Ok(())
    }

    async fn delete(&self, store: AliasStore, name: String) -> Result<()> {
        store.delete(&name).await?;

        Ok(())
    }

    async fn get(&self, store: AliasStore, name: Option<String>) -> Result<()> {
        let aliases = store.aliases().await?;
        if let Some(name) = name {
            let pos = aliases
                .iter()
                .position(|x| x.name == name)
                .wrap_err("No alias with name: ".to_string() + &name)?;
            let alias = aliases.get(pos).expect("Should exist");
            println!("{}: {}", alias.name, alias.value);
        } else {
            for alias in aliases {
                println!("{}: {}", alias.name, alias.value);
            }
        }
        Ok(())
    }

    pub async fn run(&self, settings: &Settings, store: SqliteStore) -> Result<()> {
        let encryption_key: [u8; 32] = encryption::load_key(settings)
            .context("could not load encryption key")?
            .into();
        let host_id = Settings::host_id().expect("failed to get host_id");

        let alias_store = AliasStore::new(store, host_id, encryption_key);

        match self {
            Self::Set { name, value } => self.set(alias_store, name.clone(), value.clone()).await,

            Self::Delete { name } => self.delete(alias_store, name.clone()).await,

            Self::Get { name } => self.get(alias_store, name.clone()).await,
        }
    }
}
