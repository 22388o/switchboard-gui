#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use anyhow::Result;
use clap::Parser;
use futures::executor::block_on;
use serde_json::Value;
use std::path::PathBuf;
use switchboard::{
    api::{get_message, Balances, BlockCounts, Chain, Sidechain, SidechainClient},
    config::Config,
    launcher::*,
};
use tauri::{RunEvent, WindowEvent};

#[tauri::command]
async fn main_request(
    client: tauri::State<'_, SidechainClient>,
    method: String,
    params: Option<Vec<Value>>,
) -> Result<Value, String> {
    client
        .main_request(method, params)
        .await
        .map_err(|err| get_message(err))
}

#[tauri::command]
async fn zcash_request(
    client: tauri::State<'_, SidechainClient>,
    method: String,
    params: Option<Vec<Value>>,
) -> Result<Value, String> {
    client
        .zcash_request(method, params)
        .await
        .map_err(|err| get_message(err))
}

#[tauri::command]
async fn ethereum_request(
    client: tauri::State<'_, SidechainClient>,
    method: String,
    params: Option<Vec<Value>>,
) -> Result<Value, String> {
    client
        .ethereum_request(method, params)
        .await
        .map_err(|err| get_message(err))
}

#[tauri::command]
async fn generate(
    client: tauri::State<'_, SidechainClient>,
    amount: u64,
) -> Result<String, String> {
    client
        .generate(1, amount)
        .await
        .map(|hashes| hashes.first().unwrap().to_string())
        .map_err(|err| format!("{:#?}", err))
}

#[tauri::command]
async fn deposit(
    client: tauri::State<'_, SidechainClient>,
    sidechain: Sidechain,
    amount: u64,
    fee: u64,
) -> Result<(), String> {
    client
        .deposit(sidechain, amount, fee)
        .await
        .map_err(|err| format!("{:#?}", err))?;
    Ok(())
}

#[tauri::command]
async fn withdraw(
    client: tauri::State<'_, SidechainClient>,
    sidechain: Sidechain,
    amount: u64,
    fee: u64,
) -> Result<(), String> {
    client
        .withdraw(sidechain, amount, fee)
        .await
        .map_err(|err| format!("{:#?}", err))?;
    Ok(())
}

#[tauri::command]
async fn refund(
    client: tauri::State<'_, SidechainClient>,
    sidechain: Sidechain,
    amount: u64,
    fee: u64,
) -> Result<(), String> {
    client
        .refund(sidechain, amount, fee)
        .await
        .map_err(|err| format!("{:#?}", err))?;
    Ok(())
}

#[tauri::command]
async fn get_balances(client: tauri::State<'_, SidechainClient>) -> Result<Balances, String> {
    let balances = client
        .get_balances()
        .await
        .map_err(|err| format!("{:#?}", err))?;
    Ok(balances)
}

#[tauri::command]
async fn get_block_counts(
    client: tauri::State<'_, SidechainClient>,
) -> Result<BlockCounts, String> {
    client
        .get_block_counts()
        .await
        .map_err(|err| format!("{:#?}", err))
}

#[tauri::command]
async fn get_new_address(
    client: tauri::State<'_, SidechainClient>,
    chain: Chain,
) -> Result<String, String> {
    client
        .get_new_address(chain)
        .await
        .map_err(|err| format!("{:#?}", err))
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let home_dir = dirs::home_dir().unwrap();
    // let sb_dir = home_dir.join(".switchboard");
    let sb_dir = home_dir.join(".switchboard");
    let datadir = args.datadir.unwrap_or(sb_dir);
    let config: Config = confy::load_path(datadir.join("config.toml"))?;
    let mut first_launch = false;
    if !datadir.join("bin").exists() {
        let url = args
            .bin_download_url
            .unwrap_or("http://localhost:8080/bin.tar.gz".into());
        download_binaries(&datadir, &url).await?;
        if config.switchboard.regtest {
            ethereum_regtest_setup(&datadir).await?;
        }
        first_launch = true;
    }
    if !home_dir.join(".zcash-params").exists() {
        zcash_fetch_params(&datadir).await?;
    }
    let client = SidechainClient::new(&config)?;
    let Daemons { mut ethereum, .. } = spawn_daemons(&datadir, &config).await?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    if config.switchboard.regtest && first_launch {
        client.activate_sidechains().await?;
    }
    let app = tauri::Builder::default()
        .manage(client.clone())
        .invoke_handler(tauri::generate_handler![
            main_request,
            zcash_request,
            ethereum_request,
            get_new_address,
            deposit,
            withdraw,
            refund,
            generate,
            get_balances,
            get_block_counts
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");
    app.run(move |_app_handle, event| match event {
        tauri::RunEvent::Exit => {
            let mut kill = tokio::process::Command::new("kill")
                .args(["-s", "INT", &ethereum.id().unwrap().to_string()])
                .spawn()
                .unwrap();
            block_on(client.stop()).unwrap();
        }
        _ => {}
    });
    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    datadir: Option<PathBuf>,
    #[arg(short, long)]
    bin_download_url: Option<String>,
}
