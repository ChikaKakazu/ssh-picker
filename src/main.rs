use anyhow::Result;
use clap::{Parser, command};
use dialoguer::Select;
use std::process::Command;
use std::result::Result::Ok;

#[derive(Parser)]
#[command(name = "ssh-picker")]
#[command(about = "Interactive SSH host picker from ~/.ssh/config")]
struct Args {
    #[arg(short, long)]
    config: Option<String>,

    #[arg(short, long)]
    list: bool,
}

#[derive(Debug)]
struct SshHost {
    name: String,
    hostname: Option<String>,
    user: Option<String>,
    port: Option<u16>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // SSH設定ファイルパスの取得
    let config_path = get_config_path(args.config)?;
    println!("Config path: {}", config_path);

    // SSH設定ファイルを解析
    let hosts = parse_ssh_config(&config_path)?;

    if args.list {
        // ホスト一覧表示
        list_hosts(&hosts);
    } else {
        // ホスト選択
        let selected_host = interactive_select(&hosts)?;

        // ssh接続
        connect_to_host(&selected_host)?;
    }

    Ok(())
}

// .ssh/config のパスを返す
fn get_config_path(file_path: Option<String>) -> Result<String> {
    let config_path = match file_path {
        Some(path) => path,
        None => {
            let home = if cfg!(windows) {
                std::env::var("USERPROFILE")
                    .map_err(|_| anyhow::anyhow!("USERPROFILE環境変数が設定されていません"))?
            } else {
                std::env::var("HOME")
                    .map_err(|_| anyhow::anyhow!("HOME環境変数が設定されていません"))?
            };

            let ssh_path = if cfg!(windows) {
                format!("{}/.ssh/config", home)
            } else {
                format!("{}/.ssh/config", home)
            };
            ssh_path
        }
    };

    // ファイルの存在確認
    if !std::path::Path::new(&config_path).exists() {
        return Err(anyhow::anyhow!(
            "SSH設定ファイルが見つかりません: {}",
            config_path
        ));
    }

    // ファイルが読み取り可能か確認
    if let Err(e) = std::fs::File::open(&config_path) {
        return Err(anyhow::anyhow!(
            "SSH設定ファイルを開けません: {} ({})",
            config_path,
            e
        ));
    }

    Ok(config_path)
}

// ssh configを読み込み、SshHostを作成する
fn parse_ssh_config(config_path: &str) -> Result<Vec<SshHost>> {
    let config = std::fs::read_to_string(config_path).map_err(|e| {
        anyhow::anyhow!(
            "SSH設定ファイルの読み込みに失敗しました: {} ({})",
            config_path,
            e
        )
    })?;
    let mut hosts = Vec::new();
    let mut current_host: Option<SshHost> = None;

    // 1行ずつ読み込み、SshHostを作成する
    for line in config.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with("Host ") {
            // 直前のホストがあればpush
            if let Some(host) = current_host.take() {
                hosts.push(host);
            }
            // Host ○○ からHostを取り除いてHost名を取り出す
            let name = line["Host ".len()..].trim().to_string();
            current_host = Some(SshHost {
                name,
                hostname: None,
                user: None,
                port: None,
            });
        } else if let Some(ref mut host) = current_host {
            if let Some(rest) = line.strip_prefix("HostName ") {
                host.hostname = Some(rest.trim().to_string());
            } else if let Some(rest) = line.strip_prefix("User ") {
                host.user = Some(rest.trim().to_string());
            } else if let Some(rest) = line.strip_prefix("Port ") {
                if let Ok(port) = rest.trim().parse() {
                    host.port = Some(port);
                }
            }
        }
    }

    // ループ終了後の最後のホストもpush
    if let Some(host) = current_host {
        hosts.push(host);
    }

    // ホストが見つからない場合の警告
    if hosts.is_empty() {
        return Err(anyhow::anyhow!(
            "SSH設定ファイルにHostが定義されていません: {}",
            config_path
        ));
    }

    Ok(hosts)
}

// ホストを一覧表示
fn list_hosts(hosts: &[SshHost]) {
    println!("Available SSH hosts:");
    for (i, host) in hosts.iter().enumerate() {
        println!(
            "{}. {} ({}@{}:{})",
            i + 1,
            host.name,
            host.user.as_deref().unwrap_or("?"),
            host.hostname.as_deref().unwrap_or("?"),
            host.port.unwrap_or(22)
        );
    }
}

// ホスト選択
fn interactive_select(hosts: &[SshHost]) -> Result<&SshHost> {
    if hosts.is_empty() {
        return Err(anyhow::anyhow!("No hosts available"));
    }

    // ホスト選択肢を作成
    let items: Vec<String> = hosts
        .iter()
        .map(|host| {
            format!(
                "{} ({}@{}:{})",
                host.name,
                host.user.as_deref().unwrap_or("?"),
                host.hostname.as_deref().unwrap_or("?"),
                host.port.unwrap_or(22)
            )
        })
        .collect();

    // 選択UI表示
    let selection = Select::new()
        .with_prompt("SSH接続先を選択してください")
        .items(&items)
        .default(0)
        .interact()?;

    Ok(&hosts[selection])
}

// ホストに接続
fn connect_to_host(host: &SshHost) -> Result<()> {
    println!("Connecting to: {}", host.name);

    // SSH接続を実行
    let status = Command::new("ssh").arg(&host.name).status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("SSH connection failed"));
    }

    Ok(())
}
