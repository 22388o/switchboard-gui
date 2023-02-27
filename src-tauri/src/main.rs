#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use anyhow::Result;
use clap::Parser;
use futures::executor::block_on;
use serde_json::Value;
use std::path::PathBuf;
use switchboard::{config::Config, launcher};
use tauri::{RunEvent, WindowEvent};
use web3::Transport;

#[tauri::command]
fn get_geth_console_command(command: tauri::State<'_, GethConsole>) -> String {
    command.0.clone()
}

struct Main(ureq_jsonrpc::Client);
struct Testchain(ureq_jsonrpc::Client);
struct BitAssets(ureq_jsonrpc::Client);
struct Zcash(ureq_jsonrpc::Client);
struct Web3(web3::transports::Http);

#[tauri::command]
async fn spawn_mainchain(
    datadir: tauri::State<'_, PathBuf>,
    config: tauri::State<'_, Config>,
    client: tauri::State<'_, Main>,
) -> Result<(), String> {
    let main_datadir = datadir.join("data/main");
    let first_launch = !main_datadir.exists();
    let mut main = launcher::spawn_main_qt(&datadir, &config).map_err(|err| format!("{}", err))?;
    if config.switchboard.regtest && first_launch {
        while client
            .0
            .send_request::<ureq_jsonrpc::Value>("getblockcount", &[])
            .is_err()
        {
            println!("waiting");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
        println!("activating sidechains");
        launcher::activate_sidechains(&client.0).map_err(|err| format!("{}", err))?;
    }
    main.wait();
    Ok(())
}

#[tauri::command]
async fn spawn_testchain(
    datadir: tauri::State<'_, PathBuf>,
    config: tauri::State<'_, Config>,
) -> Result<(), String> {
    launcher::spawn_testchain_qt(&datadir, &config)
        .map_err(|err| format!("{}", err))?
        .wait();
    Ok(())
}

#[tauri::command]
async fn spawn_bitassets(
    datadir: tauri::State<'_, PathBuf>,
    config: tauri::State<'_, Config>,
) -> Result<(), String> {
    launcher::spawn_bitassets_qt(&datadir, &config)
        .map_err(|err| format!("{}", err))?
        .wait();
    Ok(())
}

#[tauri::command]
async fn spawn_zcash(
    datadir: tauri::State<'_, PathBuf>,
    config: tauri::State<'_, Config>,
) -> Result<(), String> {
    let home_dir = dirs::home_dir().unwrap();
    if !home_dir.join(".zcash-params").exists() {
        tokio::process::Command::new(datadir.join("bin/fetch-params.sh"))
            .spawn()
            .map_err(|err| format!("{}", err))?;
    }
    launcher::spawn_zcash(&datadir, &config)
        .map_err(|err| format!("{}", err))?
        .wait();
    Ok(())
}

#[tauri::command]
async fn spawn_ethereum(
    datadir: tauri::State<'_, PathBuf>,
    config: tauri::State<'_, Config>,
) -> Result<(), String> {
    if config.switchboard.regtest && !datadir.join("data/ethereum").exists() {
        launcher::ethereum_regtest_setup(&datadir).map_err(|err| format!("{}", err))?;
    }
    let mut ethereum =
        launcher::spawn_ethereum(&datadir, &config).map_err(|err| format!("{}", err))?;
    ethereum.wait();
    Ok(())
}

#[tauri::command]
fn kill_ethereum() -> Result<(), String> {
    println!("killing ethereum process");
    let pid = std::process::Command::new("pgrep")
        .args(["--oldest", "geth"])
        .output()
        .map_err(|err| format!("{}", err))?;
    let pid = std::str::from_utf8(&pid.stdout)
        .map_err(|err| format!("{}", err))?
        .trim();
    let pid: u32 = pid.parse().map_err(|err| format!("{}", err))?;
    std::process::Command::new("kill")
        .args(["-s", "INT", &pid.to_string()])
        .spawn()
        .map_err(|err| format!("{}", err))?
        .wait()
        .map_err(|err| format!("{}", err))?;
    Ok(())
}

#[tauri::command]
async fn mainchain(
    client: tauri::State<'_, Main>,
    method: &str,
    params: Vec<ureq_jsonrpc::Value>,
) -> Result<Value, String> {
    client
        .0
        .send_request(method, &params)
        .map_err(|err| format!("{}", err))
}

#[tauri::command]
async fn testchain(
    client: tauri::State<'_, Testchain>,
    method: &str,
    params: Vec<ureq_jsonrpc::Value>,
) -> Result<Value, String> {
    client
        .0
        .send_request(method, &params)
        .map_err(|err| format!("{}", err))
}

#[tauri::command]
async fn bitassets(
    client: tauri::State<'_, BitAssets>,
    method: &str,
    params: Vec<ureq_jsonrpc::Value>,
) -> Result<Value, String> {
    client
        .0
        .send_request(method, &params)
        .map_err(|err| format!("{}", err))
}

#[tauri::command]
async fn zcash(
    client: tauri::State<'_, Zcash>,
    method: &str,
    params: Vec<ureq_jsonrpc::Value>,
) -> Result<Value, String> {
    client
        .0
        .send_request(method, &params)
        .map_err(|err| format!("{}", err))
}

#[tauri::command]
async fn web3(
    client: tauri::State<'_, Web3>,
    method: &str,
    params: Vec<ureq_jsonrpc::Value>,
) -> Result<Value, String> {
    client
        .0
        .execute(method, params)
        .await
        .map_err(|err| format!("{}", err))
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let datadir = {
        let home_dir = dirs::home_dir().unwrap();
        let sb_dir = home_dir.join(".switchboard");
        args.datadir.unwrap_or(sb_dir)
    };
    let config: Config = confy::load_path(datadir.join("config.toml"))?;
    let geth_console = {
        let ipc_file = datadir.join("data/ethereum/geth.ipc");
        let geth_bin = datadir.join("bin/geth");
        format!("{} attach {}", geth_bin.display(), ipc_file.display())
    };
    let url = args
        .bin_download_url
        .unwrap_or("http://drivechain.info/releases/bin/bin-gui.tar.gz".to_string());
    if !datadir.join("bin").exists() {
        const DIGEST: &str = "2e85f9b54f7ab79780018dda3df670428b4c6289dce6913f383d5c2584a3c48b";
        launcher::download_binaries(&datadir, &url, DIGEST)?;
    }
    let main_client = ureq_jsonrpc::Client {
        host: "localhost".to_string(),
        port: config.main.port,
        user: config.switchboard.rpcuser.clone(),
        password: config.switchboard.rpcpassword.clone(),
        id: "switchboard-gui".to_string(),
    };
    let testchain_client = ureq_jsonrpc::Client {
        host: "localhost".to_string(),
        port: config.testchain.port,
        user: config.switchboard.rpcuser.clone(),
        password: config.switchboard.rpcpassword.clone(),
        id: "switchboard-gui".to_string(),
    };
    let bitassets_client = ureq_jsonrpc::Client {
        host: "localhost".to_string(),
        port: config.bitassets.port,
        user: config.switchboard.rpcuser.clone(),
        password: config.switchboard.rpcpassword.clone(),
        id: "switchboard-gui".to_string(),
    };
    let zcash_client = ureq_jsonrpc::Client {
        host: "localhost".to_string(),
        port: config.zcash.port,
        user: config.switchboard.rpcuser.clone(),
        password: config.switchboard.rpcpassword.clone(),
        id: "switchboard-gui".to_string(),
    };
    let web3_client =
        web3::transports::Http::new(&format!("http://localhost:{}", config.ethereum.port))?;
    // let mut daemons = Daemons::start(&url, &datadir, &config)?;
    let app = tauri::Builder::default()
        .manage(config)
        .manage(datadir)
        .manage(Main(main_client.clone()))
        .manage(Testchain(testchain_client.clone()))
        .manage(BitAssets(bitassets_client.clone()))
        .manage(Zcash(zcash_client.clone()))
        .manage(Web3(web3_client))
        .manage(GethConsole(geth_console))
        .invoke_handler(tauri::generate_handler![
            spawn_mainchain,
            spawn_testchain,
            spawn_bitassets,
            spawn_zcash,
            spawn_ethereum,
            kill_ethereum,
            get_geth_console_command,
            mainchain,
            testchain,
            bitassets,
            zcash,
            web3
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");
    app.run(move |_app_handle, event| match event {
        tauri::RunEvent::Exit => {
            kill_ethereum();
            testchain_client.send_request::<ureq_jsonrpc::Value>("stop", &[]);
            bitassets_client.send_request::<ureq_jsonrpc::Value>("stop", &[]);
            zcash_client.send_request::<ureq_jsonrpc::Value>("stop", &[]);
            main_client.send_request::<ureq_jsonrpc::Value>("stop", &[]);
        }
        _ => {}
    });
    Ok(())
}

struct GethConsole(String);

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    datadir: Option<PathBuf>,
    #[arg(short, long)]
    bin_download_url: Option<String>,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("ureq jsonrpc error")]
    Ureq(#[from] ureq_jsonrpc::Error),
    #[error("web3 error")]
    Web3(#[from] web3::Error),
}
