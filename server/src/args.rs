use std::collections::HashMap;
use std::path::PathBuf;
use clap::{Args, Parser};
use jsonrpsee_http_client::{HttpClient, HttpClientBuilder};
use url::Url;
use std::str::FromStr;
use std::sync::Arc;
use dojo_world::manifest::Manifest;
use keiko_api::server_state;

const LOCAL_KATANA: &str = "http://0.0.0.0:5050";
const LOCAL_TORII: &str = "http://0.0.0.0:8080";

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct KeikoArgs {
    #[command(flatten)]
    #[command(next_help_heading = "Server options")]
    pub server: ServerOptions,

    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    pub starknet: StarknetOptions,

    #[command(flatten)]
    #[command(next_help_heading = "Slot options")]
    pub slot: SlotOptions,

    #[command(flatten)]
    #[command(next_help_heading = "World options")]
    pub world: WorldOptions,
}

#[derive(Debug, Args, Clone)]
pub struct SlotOptions {
    #[arg(long)]
    #[arg(help = "the url to the deployed slot katana")]
    #[arg(env = "SLOT_KATANA")]
    pub katana: Option<Url>,

    #[arg(long)]
    #[arg(help = "the url to the deployed slot torii")]
    #[arg(env = "SLOT_TORII")]
    pub torii: Option<Url>,
}

#[derive(Debug, Args, Clone)]
pub struct WorldOptions {
    #[arg(long)]
    #[arg(help = "the world address")]
    #[arg(env = "WORLD_ADDRESS")]
    pub address: Option<String>,

    #[arg(long)]
    #[arg(help = "the world salt")]
    #[arg(env = "WORLD_NAME")]
    pub name: Option<String>,
}

#[derive(Debug, Args, Clone)]
pub struct ServerOptions {
    #[arg(long)]
    #[arg(default_value = "3000")]
    #[arg(help = "Port number to listen on.")]
    #[arg(env = "SERVER_PORT")]
    pub port: u16,

    #[arg(short, long)]
    #[arg(default_value = "contracts")]
    #[arg(value_parser = PathBuf::from_str)]
    #[arg(help = "Path to the contracts directory")]
    #[arg(env = "CONTRACT_PATH")]
    pub contract_path: PathBuf,

    #[arg(long)]
    #[arg(default_value = "static")]
    #[arg(value_parser = PathBuf::from_str)]
    #[arg(help = "Path to the static directory")]
    #[arg(env = "STATIC_PATH")]
    pub static_path: PathBuf,

    #[arg(long)]
    #[arg(default_value = "false")]
    #[arg(help = "Builds the dashboard if set to true")]
    #[arg(env = "PROD")]
    pub prod: bool,
}

#[derive(Debug, Args, Clone)]
pub struct StarknetOptions {
    #[arg(long)]
    #[arg(default_value = "0")]
    #[arg(help = "Specify the seed for randomness of accounts to be predeployed.")]
    #[arg(env = "SEED")]
    pub seed: String,

    #[arg(long = "accounts")]
    #[arg(value_name = "NUM")]
    #[arg(default_value = "10")]
    #[arg(help = "Number of pre-funded accounts to generate.")]
    #[arg(env = "TOTAL_ACCOUNTS")]
    pub total_accounts: u8,
}

impl KeikoArgs {

    /**
     * checks if keiko should run katana
     */
    pub fn can_run_katana(&self) -> bool {
        self.slot.katana.is_none()
    }

    /**
     * checks if keiko should run torii
     */
    pub fn can_run_torii(&self) -> bool {
        match self.slot.torii {
            None => self.can_run_katana() || self.world.address.is_some(),
            Some(_) => false
        }
    }

    /**
     * creates a json_rpc_client from katana
     */
    pub fn json_rpc_client(&self) -> HttpClient {
        HttpClientBuilder::default()
            .build(self.rpc_url())
            .unwrap()
    }

    /**
     * creates the rpc_url
     */
    pub fn rpc_url(&self) -> Url {
        self.slot.katana.clone().unwrap_or(Url::parse(LOCAL_KATANA).unwrap())
    }

    /*
    *    creates the torii_url
    */
    pub fn torii_url(&self) -> Url {
        self.slot.torii.clone().unwrap_or(Url::parse(LOCAL_TORII).unwrap())
    }

    /*
    *    gets the server state
    */
    pub fn server_state(&self) -> server_state::ServerState {
        server_state::ServerState {
            json_rpc_client: self.json_rpc_client(),
            rpc_url: self.rpc_url(),
            store: Arc::new(tokio::sync::Mutex::new(HashMap::<String, Manifest>::new())),
            torii_url: self.torii_url(),
            starknet: server_state::StarknetOptions {
                seed: self.starknet.seed.clone(),
                total_accounts: self.starknet.total_accounts.clone(),
            },
        }
    }

}