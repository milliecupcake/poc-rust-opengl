# 新しい Rust プロジェクトを作る

自分の Rust プロジェクトをゼロから作る手順です。
Rust のインストールがまだの場合は、先に [ターミナル編](setup-terminal.md) か [RustRover 編](setup-rustrover.md) を済ませてください。

## 1. プロジェクトの種類

最初に、どちらの種類を作るかを決めます。迷ったらバイナリを選びます。

| 種類 | 用途 | 入口のファイル | 実行方法 |
|---|---|---|---|
| バイナリ | 実行できるプログラム | `src/main.rs` | `cargo run` |
| ライブラリ | 他のプロジェクトから使う部品 | `src/lib.rs` | `cargo test` で動作を確認 |

## 2. ターミナルで作る

### バイナリを作る

プロジェクトを置きたい場所に移動してから、次を実行します。
`hello-rust` と同じ名前のフォルダが新しく作られます。

```bash
cargo new hello-rust
```

```bash
cd hello-rust
```

```bash
cargo run
```

`Hello, world!` と表示されれば成功です。

### ライブラリを作る

```bash
cargo new my-lib --lib
```

```bash
cd my-lib
```

```bash
cargo test
```

`src/lib.rs` には、サンプルの関数とテストが最初から入っています。

### 既存のフォルダをプロジェクトにする

すでにあるフォルダを使うときは、そのフォルダに移動して `cargo init` を実行します。
フォルダ名がプロジェクト名になります。

```bash
cargo init
```

ライブラリにするときは `cargo init --lib` を使います。

## 3. RustRover で作る

1. **File | New | Project** を選びます（Welcome 画面では **New Project**）
2. 左の一覧で **Rust** を選びます
3. プロジェクトの場所と名前を指定します
4. Toolchain と Standard library が検出されていることを確認します
5. テンプレートを選び、**Create** を押します

| テンプレート | 作られるもの | 自動で作られる実行構成 |
|---|---|---|
| Binary | `src/main.rs` | Run |
| Library | `src/lib.rs` | Test |

RustRover も内部では `cargo new` を使っているので、できあがる構成はターミナルで作った場合と同じです。
ツールチェーンが検出されないときは、[RustRover 編の手順 4](setup-rustrover.md#4-ツールチェーンを確認する) を見てください。

## 4. 作られるファイル

`cargo new hello-rust` を実行すると、次の構成になります。

```text
hello-rust/
├── .git/         # Git リポジトリ
├── .gitignore    # target/ を Git の管理から外す
├── Cargo.toml    # プロジェクトの設定と依存ライブラリ
└── src/
    └── main.rs   # プログラムの入口
```

最初にビルドすると、次の2つが増えます。

| 名前 | 内容 | Git で管理するか |
|---|---|---|
| `Cargo.lock` | 依存ライブラリの正確なバージョン | する（実行するプログラムの場合） |
| `target/` | ビルドの成果物 | しない |

`Cargo.toml` の中身は次のとおりです。

```toml
[package]
name = "hello-rust"   # プロジェクト名
version = "0.1.0"     # プロジェクトのバージョン
edition = "2024"      # 使う Rust のエディション

[dependencies]        # 依存ライブラリをここに書く
```

## 5. ライブラリ（クレート）を追加する

Rust では、ライブラリのことを**クレート**と呼びます。
公開されているクレートは [crates.io](https://crates.io/) で探せます。

例として、乱数を作る `rand` を追加します。

```bash
cargo add rand
```

`Cargo.toml` の `[dependencies]` に、次のような行が追加されます。

```toml
rand = "0.10.3"
```

`src/main.rs` を次のように書き換えます。

```rust
fn main() {
    let dice: u32 = rand::random_range(1..=6);
    println!("サイコロの目: {dice}");
}
```

`cargo run` を実行すると、クレートのダウンロードとビルドの後に結果が表示されます。

クレートの使い方はバージョンで変わることがあります。
コードが動かないときは、[docs.rs](https://docs.rs/) で使っているバージョンのドキュメントを確認してください。

## 6. GitHub に公開する

`cargo new` は Git リポジトリも作るので、`git init` は不要です。
ブランチ名は Git の設定によって `master` になることがあります。`main` に揃える手順も含めて、次のように進めます。

GitHub に空のリポジトリを作ってから、`<ユーザー名>` と `<リポジトリ名>` を置き換えて実行してください。

```bash
git add .
```

```bash
git commit -m "Initial commit"
```

```bash
git branch -M main
```

```bash
git remote add origin git@github.com:<ユーザー名>/<リポジトリ名>.git
```

```bash
git push -u origin main
```

## 7. うまくいかないとき

| 症状 | 原因と対処 |
|---|---|
| `destination ... already exists` | 同じ名前のフォルダがあります。別の名前にするか、そのフォルダで `cargo init` を使ってください |
| `the name cannot start with a digit` | プロジェクト名は数字で始められません。`app-123` のように変えてください |
| `.git` や `.gitignore` が作られない | 既存の Git リポジトリの中で実行すると、新しいリポジトリは作られません。問題なければそのままで大丈夫です |
| ``could not find `Cargo.toml` `` | プロジェクトのフォルダに移動してから実行してください |
| ``cannot find module or crate `rand` `` | `cargo add` でクレートを追加したか確認してください |
