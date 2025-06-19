# ssh-picker

SSH設定ファイル（`~/.ssh/config`）からホストをインタラクティブに選択してSSH接続するCLIツールです。

## 機能

- SSH設定ファイルの解析
- ホスト一覧の表示
- インタラクティブなホスト選択UI
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

# macOS (Intel)
curl -L -o ssh-picker https://github.com/ChikaKakazu/ssh-picker/releases/latest/download/ssh-picker-darwin-x86_64
chmod +x ssh-picker
sudo mv ssh-picker /usr/local/bin/

# macOS (Apple Silicon)
curl -L -o ssh-picker https://github.com/ChikaKakazu/ssh-picker/releases/latest/download/ssh-picker-darwin-arm64
chmod +x ssh-picker
sudo mv ssh-picker /usr/local/bin/
```

### ソースからビルド

```bash
git clone https://github.com/ChikaKakazu/ssh-picker.git
cd ssh-picker
cargo build --release
```

実行可能ファイルは `target/release/ssh-picker` に生成されます。

## 必要条件

- Rust 2024 edition
- SSH設定ファイル（`~/.ssh/config`）

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