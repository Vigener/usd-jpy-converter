# usd-jpy-converter

Rustで作成したUSD↔JPYの為替レート変換のCLIアプリ

## 概要

**ujcon** は、最新のドル円為替レートを自動で取得し、  
- ドル→円
- 円→ドル
の相互変換結果を即時表示できるRust製CLIアプリです。

コマンド例:  
```bash
ujcon -d 599     # 599ドルを現在レートで日本円に換算
ujcon -y 1000    # 1000円を現在レートで米ドルに換算
```

実行結果には  
- 現在のUSD/JPYレート
- 換算結果
- レート取得時刻  
を同時に表示します。

## 主な特徴

- コマンドラインでシンプルに為替計算
- 複数の無料APIからリアルタイムレート取得（exchangerate-api.com、open.er-api.com）
- ドル円どちらも相互変換可能
- エラーハンドリング（API障害、入力ミス等に対応）
- 見やすい日本語表示

## 必要環境

- Rust 1.70以降（ビルド時）
- インターネット接続（リアルタイムレート取得のため）

### WSL2 Ubuntu / Linux でのビルドに必要な依存関係

このプロジェクトは `reqwest` クレートを使用しており、内部的に OpenSSL に依存しています。WSL2 Ubuntu や Linux 環境でビルドする場合は、以下のシステムパッケージを事前にインストールする必要があります：

```bash
# Ubuntu / Debian の場合
sudo apt update
sudo apt install pkg-config libssl-dev

# Fedora / CentOS / RHEL の場合
sudo dnf install pkg-config openssl-devel

# Arch Linux の場合
sudo pacman -S pkg-config openssl
```

これらのパッケージがインストールされていないと、ビルド時に以下のようなエラーが発生します：
```
error: failed to run custom build command for `openssl-sys`
Could not find directory of OpenSSL installation
```

## ビルド方法

```bash
# リポジトリをクローン
git clone https://github.com/Vigener/usd-jpy-converter.git
cd usd-jpy-converter

# リリースビルド
cargo build --release

# ビルド後のバイナリは以下に生成されます
# ./target/release/ujcon
```

## インストール方法

ビルドしたバイナリを以下のいずれかの方法でインストールします：

### 方法1: Cargoのインストールコマンドを使用（推奨）

```bash
# リポジトリのルートディレクトリで実行
cargo install --path .

# これにより、バイナリが $HOME/.cargo/bin/ujcon にインストールされます
# $HOME/.cargo/bin が PATH に含まれていることを確認してください
```

PATHの確認と設定：
```bash
# PATHに含まれているか確認
echo $PATH | grep -q ".cargo/bin" && echo "PATHに含まれています" || echo "PATHに含まれていません"

# 含まれていない場合、以下をシェル設定ファイルに追加
# Bashの場合: ~/.bashrc
# Zshの場合: ~/.zshrc
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### 方法2: 手動でバイナリをコピー

```bash
# ユーザー用ディレクトリにコピー
mkdir -p $HOME/.local/bin
cp target/release/ujcon $HOME/.local/bin/

# または、システム全体で使用する場合（sudoが必要）
sudo cp target/release/ujcon /usr/local/bin/

# $HOME/.local/bin をPATHに追加（まだの場合）
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### 方法3: バイナリのディレクトリをPATHに追加

```bash
# 現在のプロジェクトディレクトリのバイナリをそのまま使用
# シェル設定ファイル（~/.bashrc または ~/.zshrc）に以下を追加
echo 'export PATH="'$(pwd)'/target/release:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## 使い方

インストール後、ターミナルのどこからでも `ujcon` コマンドが使用できます。

### 基本的な使い方

```bash
# ヘルプを表示
ujcon --help

# ドルを円に変換（複数のオプション形式をサポート）
ujcon -d 100
ujcon --dollar 100
ujcon --usd 100    # エイリアス
ujcon --USD 100    # エイリアス
ujcon --u 100      # エイリアス

# 円をドルに変換（複数のオプション形式をサポート）
ujcon -y 15000
ujcon --yen 15000
ujcon --jpy 15000  # エイリアス
ujcon --JPY 15000  # エイリアス
ujcon --j 15000    # エイリアス
```

### 実行例

```bash
$ ujcon -d 100
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📊 USD/JPY 為替レート変換
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
💱 現在のレート: 1 USD = 149.50 JPY
🕐 取得時刻: 2024年11月05日 10:30:45
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
💵 100 USD → 💴 14950.00 JPY
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

$ ujcon -y 10000
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📊 USD/JPY 為替レート変換
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
💱 現在のレート: 1 USD = 149.50 JPY
🕐 取得時刻: 2024年11月05日 10:31:12
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
💴 10000 JPY → 💵 66.89 USD
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## トラブルシューティング

### ビルドエラー: `openssl-sys` のビルドに失敗する場合

**症状**:
```
error: failed to run custom build command for `openssl-sys v0.9.110`
Could not find directory of OpenSSL installation
```

**解決方法**:

WSL2 Ubuntu や Linux 環境の場合、OpenSSL の開発パッケージと pkg-config をインストールしてください：

```bash
# Ubuntu / Debian / WSL2 Ubuntu
sudo apt update
sudo apt install pkg-config libssl-dev

# インストール後、再度ビルドを実行
cargo build --release
```

他のディストリビューションの場合は、上記の「必要な依存関係」セクションを参照してください。

### "command not found: ujcon"と表示される場合

- PATHが正しく設定されているか確認：`echo $PATH`
- シェルを再起動または `source ~/.bashrc` を実行
- バイナリの実行権限を確認：`chmod +x $HOME/.cargo/bin/ujcon`

### API接続エラーが発生する場合

- インターネット接続を確認
- ファイアウォールやプロキシ設定を確認
- アプリは複数のAPIエンドポイントを試行します

## 技術スタック

- **言語**: Rust (edition 2021)
- **依存ライブラリ**:
  - `clap` - コマンドライン引数のパース
  - `reqwest` - HTTP通信
  - `serde` / `serde_json` - JSONのシリアライズ/デシリアライズ
  - `chrono` - 日時処理

## ライセンス

このプロジェクトはオープンソースです。

## 貢献

プルリクエストや Issue の作成を歓迎します。
