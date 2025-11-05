# usd-jpy-converter

Rustで作成したUSD↔JPYの為替レート変換のCLIアプリ

## 概要

**usd-jpy-converter** は、最新のドル円為替レートを自動で取得し、  
- ドル→円
- 円→ドル
の相互変換結果を即時表示できるRust製CLIアプリです。

コマンド例:  
```bash
usd-jpy-converter -d 599     # 599ドルを現在レートで日本円に換算
usd-jpy-converter -y 1000    # 1000円を現在レートで米ドルに換算
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

## ビルド方法

```bash
# リポジトリをクローン
git clone https://github.com/Vigener/usd-jpy-converter.git
cd usd-jpy-converter

# リリースビルド
cargo build --release

# ビルド後のバイナリは以下に生成されます
# ./target/release/usd-jpy-converter
```

## インストール方法

ビルドしたバイナリを以下のいずれかの方法でインストールします：

### 方法1: Cargoのインストールコマンドを使用（推奨）

```bash
# リポジトリのルートディレクトリで実行
cargo install --path .

# これにより、バイナリが $HOME/.cargo/bin/usd-jpy-converter にインストールされます
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
cp target/release/usd-jpy-converter $HOME/.local/bin/

# または、システム全体で使用する場合（sudoが必要）
sudo cp target/release/usd-jpy-converter /usr/local/bin/

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

インストール後、ターミナルのどこからでも `usd-jpy-converter` コマンドが使用できます。

### 基本的な使い方

```bash
# ヘルプを表示
usd-jpy-converter --help

# ドルを円に変換
usd-jpy-converter -d 100
usd-jpy-converter --dollar 100

# 円をドルに変換
usd-jpy-converter -y 15000
usd-jpy-converter --yen 15000
```

### 実行例

```bash
$ usd-jpy-converter -d 100
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📊 USD/JPY 為替レート変換
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
💱 現在のレート: 1 USD = 149.50 JPY
🕐 取得時刻: 2024年11月05日 10:30:45
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
💵 100 USD → 💴 14950.00 JPY
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

$ usd-jpy-converter -y 10000
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

### "command not found: usd-jpy-converter"と表示される場合

- PATHが正しく設定されているか確認：`echo $PATH`
- シェルを再起動または `source ~/.bashrc` を実行
- バイナリの実行権限を確認：`chmod +x $HOME/.cargo/bin/usd-jpy-converter`

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
