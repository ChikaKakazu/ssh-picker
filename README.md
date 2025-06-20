# ssh-picker

SSH設定ファイル（`~/.ssh/config`）からホストをインタラクティブに選択してSSH接続するCLIツールです。

## 機能

- ホスト一覧の表示
- 選択したホストへのSSH接続

## 使用方法

### 基本的な使用方法
```bash
ssh-picker
```

### オプション
- `--list` / `-l`: ホスト一覧のみ表示（非インタラクティブ）
- `--config <path>` / `-c <path>`: カスタムSSH設定ファイルのパス指定
- `--help` / `-h`: ヘルプ表示

### 使用例
```bash
# インタラクティブにホストを選択
ssh-picker

# ホスト一覧のみ表示
ssh-picker --list

# カスタム設定ファイルを使用
ssh-picker --config /path/to/custom/ssh_config
```

## インストール

### バイナリダウンロード（推奨）

[GitHub Releases](https://github.com/ChikaKakazu/ssh-picker/releases)から最新版をダウンロード：

```bash
# Linux (x86_64)
curl -L -o ssh-picker https://github.com/ChikaKakazu/ssh-picker/releases/latest/download/ssh-picker-linux-x86_64
chmod +x ssh-picker
sudo mv ssh-picker /usr/local/bin/

# Windows (PowerShell)
Invoke-WebRequest -Uri "https://github.com/ChikaKakazu/ssh-picker/releases/latest/download/ssh-picker-windows-x86_64.exe" -OutFile "ssh-picker.exe"
# パスの通ったディレクトリに移動するか、現在のディレクトリで実行
```

### ソースからビルド

```bash
git clone https://github.com/ChikaKakazu/ssh-picker.git
cd ssh-picker
cargo build --release
```

実行可能ファイル `target/release/ssh-picker` が生成されます。

#### Windows環境でのビルド

Windows環境でRustを使用してビルドする場合：

```powershell
git clone https://github.com/ChikaKakazu/ssh-picker.git
cd ssh-picker
cargo build --release
```

実行可能ファイル `target\release\ssh-picker.exe` が生成されます。

## Windows セットアップ

管理者権限でPowerShellを開いて、順次実行する

```powershell
# 現在のPATHを確認
[Environment]::GetEnvironmentVariable("PATH", "Machine")

# 追加するパス
$newPath = "C:\Program Files\ssh-picker"

# PATHに追加
$currentPath = [Environment]::GetEnvironmentVariable("PATH", "Machine")
[Environment]::SetEnvironmentVariable("PATH", "$currentPath;$newPath", "Machine")

# 追加されたか確認
[Environment]::GetEnvironmentVariable("PATH", "Machine") -split ';' | Where-Object { $_ -like "*ssh-picker*" }
```

新しくPowerShellを開きなおして確認

```powershell
# コマンドが使えるか確認
Get-Command ssh-picker

# 実行テスト
ssh-picker
```

## Rust version

```bash
> rustup -V
rustup 1.28.2 (e4f3ad6f8 2025-04-28)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.87.0 (17067e9ac 2025-05-09)`
```

## SSH設定ファイルの例

```
Host server1
    HostName example.com
    User myuser
    Port 22

Host server2
    HostName test.example.com
    User testuser
    Port 2222
```