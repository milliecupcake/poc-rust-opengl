# セットアップ: RustRover から開発する

JetBrains の Rust 用 IDE、RustRover でこのプロジェクトを開発する手順です。
ターミナルだけで動かす場合は [ターミナルから実行する](setup-terminal.md) を見てください。

メニュー名は英語表示のものです。バージョンによって表記が変わることがあります。

## 1. Rust をインストールする

RustRover にも Rust 本体が必要です。
先に [ターミナル編の手順 1〜2](setup-terminal.md#1-必要なもの) を済ませておくのがおすすめです。

Rust が入っていない場合は、RustRover が `rustup` のダウンロードを提案します。

## 2. RustRover をインストールする

次のどれかの方法で入れます。

- [JetBrains Toolbox App](https://www.jetbrains.com/toolbox-app/) から入れる（公式のおすすめ）
- [公式サイト](https://www.jetbrains.com/rust/download/) から `.dmg` をダウンロードして、Applications フォルダにドラッグする
- Homebrew で入れる

```bash
brew install --cask rustrover
```

`.dmg` は Apple Silicon 用と Intel 用で分かれています。お使いの Mac に合うほうを選んでください。

### ライセンス

学習など非商用の利用なら無料です。初回起動時のライセンス画面で非商用利用を選び、JetBrains アカウントでログインします。

- 有効化にはインターネット接続が必要です
- 仕事などで収入を得る開発には、有料ライセンスが必要です
- 無料ライセンスでは、利用状況の統計データが JetBrains に送信され、オフにできません
- 送信されるデータには、編集履歴などコードに関する情報が含まれる場合があります

条件は変わることがあるので、[非商用ライセンスの FAQ](https://sales.jetbrains.com/hc/en-gb/articles/18950890312210-The-free-non-commercial-licensing-FAQ) と[料金・ライセンスのページ](https://www.jetbrains.com/rust/buy/)で最新の内容を確認してください。

## 3. プロジェクトを開く

1. **File | Open** を選びます（Welcome 画面では **Open**）
2. リポジトリのルートフォルダ（`Cargo.toml` があるフォルダ）を選びます
3. プロジェクトを信頼するか聞かれたら、**Trust Project** を選びます

開くと、RustRover が `Cargo.toml` を読み込み、依存ライブラリを取得します。
右下の進捗表示が消えるまで待ってください。

## 4. ツールチェーンを確認する

**Settings | Rust** を開き、Toolchain location にパスが入っていることを確認します。
macOS では **RustRover | Settings**（`Cmd + ,`）で開けます。

通常は自動で検出されます。検出されないときは、インストール方法に合わせて次のパスを指定します。

| インストール方法 | Toolchain location |
|---|---|
| 公式インストーラ | `~/.cargo/bin`（`~` は自分のホームフォルダに置き換える） |
| Homebrew（Apple Silicon） | `/opt/homebrew/opt/rustup/bin` |
| Homebrew（Intel Mac） | `/usr/local/opt/rustup/bin` |

### ツールチェーンとは

Rust のプログラムを作るための道具一式のことです。コンパイラの `rustc` やビルドツールの `cargo` が含まれます。
RustRover には Rust が付属していないので、これらの場所を指定する必要があります。

### Homebrew で入れた場合

Homebrew で入れた Rust は、`~/.zshrc` に PATH を設定していても自動検出されないことがあります。
新規プロジェクトの画面で「ツールチェーンの場所」が空になり、「ツールチェーンバージョン」に「該当なし」と表示されます。

この場合、Rust は入っているので、「Rustup のインストール」は押さないでください。Rust の環境が二重になります。
代わりに、「ツールチェーンの場所」に上の表のパスを入力します（Apple Silicon で動作確認済み）。
バージョンが表示されれば設定できています。

「標準ライブラリ」が空のままなら、次のパスを入力します。`<ユーザー名>` は自分のものに置き換えてください。

```text
/Users/<ユーザー名>/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library
```

### 設定はプロジェクトごとに保存される

入力したパスは、プロジェクトの `.idea/workspace.xml` に保存されます。
そのため、別のプロジェクトを作るときに、もう一度入力が必要になることがあります。

毎回の入力を省くには、**File | New Projects Setup | Settings for New Projects** の Rust の設定に、同じパスを入れる方法があります。
この方法で既定値にできるかは未検証です。効かないときは、「ツールチェーンの場所」の `∨` に候補が出ていないか確認してください。

## 5. 実行する

いちばん簡単なのは、`src/main.rs` を開いて `fn main()` の左にある緑の ▶ をクリックし、**Run** を選ぶ方法です。

回転する立方体のウィンドウが開き、下の Run ウィンドウに OpenGL と GPU の情報が出れば成功です。
終了するには、`Esc` キーを押すか、ウィンドウを閉じます。

一度実行すると、右上のツールバーに実行構成が追加されます。次からはそこの ▶ で実行できます。

## 6. 引数を付けて実行する

スクリーンショットの保存など、引数を付けて実行するときは実行構成を作ります。

1. **Run | Edit Configurations** を開きます
2. **+** を押して **Cargo** を選びます
3. 次のように入力して **OK** を押します

| 項目 | 値 |
|---|---|
| Name | `Screenshot` |
| Command | `run -- --screenshot target/cube.ppm --frame 120` |
| Working directory | リポジトリのルート |

Command 欄の書式は `[コマンド] [ビルドオプション] -- [プログラムの引数]` です。
`--` より後ろがプログラムに渡されます。引数の意味は[ターミナル編の手順 4](setup-terminal.md#4-プログラムに引数を渡す) にまとめています。

`target/cube.ppm` は Working directory からの相対パスです。

## 7. デバッガを使う

Rust の学習では、変数の中身を止めて見られるデバッガが役に立ちます。

1. `src/main.rs` の `render` メソッド内の行で、行番号をクリックしてブレークポイントを置きます
2. `fn main()` の左の ▶ から **Debug** を選びます
3. 止まったら、Debug ウィンドウで変数の値を確認します
4. `F8`（Step Over）や `F7`（Step Into）で 1 行ずつ進めます

`render` は毎フレーム呼ばれるので、再開するとすぐ同じ場所で止まります。
確認が終わったら、ブレークポイントを外してください。

## 8. 便利な機能

| やりたいこと | 操作 |
|---|---|
| `cargo` のコマンドを実行する | **View \| Tool Windows \| Cargo** を開く |
| ターミナルを開く | **View \| Tool Windows \| Terminal** を開く |
| 定義へ移動する | `Cmd` を押しながらクリック |
| 型の情報を見る | 変数にマウスを乗せる |
| コードを整形する | `Cmd + Option + L` |

内蔵のターミナルでは、[ターミナル編](setup-terminal.md) のコマンドをそのまま使えます。

## 9. うまくいかないとき

| 症状 | 原因と対処 |
|---|---|
| Toolchain が見つからないと表示される | 手順 4 のパスを指定してください |
| 補完が効かない、コードが赤くなる | 読み込みが終わるまで待ちます。直らなければ Cargo ツールウィンドウの更新ボタンを押してください |
| 標準ライブラリの定義に移動できない | ターミナルで `rustup component add rust-src` を実行してください |
| 引数がプログラムに渡らない | Command 欄に `--` があるか確認してください |
| シェーダーを直しても画面が変わらない | もう一度実行してください。シェーダーは実行ファイルに埋め込まれているので、再ビルドが必要です |

## 次に読む

- 自分のプロジェクトを作るときは、[新しい Rust プロジェクトを作る](new-project.md) を見てください
- コードの読み方は、[Rust の構文入門](syntax-basics.md) にまとめています

## 参考

- [Quick start guide](https://www.jetbrains.com/help/rust/quick-start-guide-rustrover.html)
- [Install RustRover](https://www.jetbrains.com/help/rust/installation-guide.html)
- [Cargo run/debug configuration](https://www.jetbrains.com/help/rust/cargo-run-debug-configuration.html)
