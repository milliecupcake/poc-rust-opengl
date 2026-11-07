# セットアップ: ターミナルから実行する

エディタに依存せず、ターミナルだけでこのプロジェクトをビルドして動かす手順です。
IDE を使う場合は [RustRover から開発する](setup-rustrover.md) を見てください。

手順は macOS（Apple Silicon）で確認しています。

## 1. 必要なもの

| 項目 | 内容 |
|---|---|
| Xcode Command Line Tools | リンカなど、ビルドに必要なツール |
| Rust 1.88 以上 | edition 2024 と `if` 内の `let` 連結を使っているため |

Command Line Tools が入っていなければ、次のコマンドで入れます。

```bash
xcode-select --install
```

## 2. Rust をインストールする

Rust 本体は `rustup` というツールで管理します。次のどちらかを選んでください。迷ったら A です。

### A. 公式インストーラ（おすすめ）

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

途中で選択肢が出たら、標準のインストールを選びます。完了したら、ターミナルを開き直してください。

### B. Homebrew

```bash
brew install rustup
```

```bash
rustup default stable
```

Homebrew の `rustup` は、`cargo` や `rustc` を PATH の通らない場所に置きます。
そのままだと `cargo not found` になるので、PATH を追加します。

```bash
echo 'export PATH="/opt/homebrew/opt/rustup/bin:$PATH"' >> ~/.zshrc
```

```bash
source ~/.zshrc
```

Intel Mac の場合は、`/opt/homebrew` を `/usr/local` に読み替えてください。

### インストールの確認

```bash
cargo --version
```

```bash
rustc --version
```

どちらもバージョンが表示されれば成功です。

## 3. プロジェクトを実行する

リポジトリのルート（`Cargo.toml` があるフォルダ）に移動して、次を実行します。

```bash
cargo run
```

初回は依存ライブラリのダウンロードとビルドがあるので、少し時間がかかります。
回転する立方体のウィンドウが開き、ターミナルに次のような情報が出れば成功です。
内容は環境によって変わります。

```text
OpenGL: 4.1 Metal - 90.5
GPU:    Apple M5
```

終了するには、`Esc` キーを押すか、ウィンドウを閉じます。

## 4. プログラムに引数を渡す

`cargo run` の後ろに `--` を書くと、それより後ろの引数はプログラムに渡されます。
`--` より前は `cargo` 自身のオプションです。

このプログラムは、指定したフレームを画像に保存して終了できます。

```bash
cargo run -- --screenshot target/cube.ppm
```

| 引数 | 意味 | 既定値 |
|---|---|---|
| `--screenshot <パス>` | PPM 形式の画像を保存して終了する | なし |
| `--frame <番号>` | 保存するフレームの番号 | 30 |

```bash
cargo run -- --screenshot target/cube.ppm --frame 120
```

macOS では、`sips` で PNG に変換できます。

```bash
sips -s format png target/cube.ppm --out target/cube.png
```

## 5. よく使うコマンド

| コマンド | 役割 |
|---|---|
| `cargo check` | 実行ファイルを作らず、コンパイルエラーだけを確認する（速い） |
| `cargo build` | デバッグ用にビルドする（`target/debug/`） |
| `cargo build --release` | 最適化してビルドする（`target/release/`） |
| `cargo run --release` | 最適化した状態で実行する |
| `cargo test` | テストを実行する（現在テストはありません） |
| `cargo fmt` | コードを整形する |
| `cargo clippy` | よくない書き方を指摘する |
| `cargo add <名前>` | 依存ライブラリを追加する |
| `cargo doc --open` | 依存ライブラリも含めた API ドキュメントをブラウザで開く |

コードを書いている間は `cargo check` でエラーを確認し、動きを見たいときに `cargo run` を使うと効率的です。

## 6. うまくいかないとき

| 症状 | 原因と対処 |
|---|---|
| `cargo not found` | PATH が通っていません。手順 2 を確認し、ターミナルを開き直してください |
| `rustup could not choose a version` | 既定のツールチェーンが未設定です。`rustup default stable` を実行してください |
| `edition 2024` や `let` に関するコンパイルエラー | Rust が古い可能性があります。`rustup update` を実行してください |
| `linker 'cc' not found` や `xcrun` のエラー | Command Line Tools がありません。手順 1 を実行してください |
| シェーダーを直しても画面が変わらない | `cargo run` で再ビルドしてください。シェーダーは `include_str!` で実行ファイルに埋め込まれています |

## 次に読む

自分のプロジェクトを作るときは、[新しい Rust プロジェクトを作る](new-project.md) を見てください。
