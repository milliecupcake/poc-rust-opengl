# Rust のマクロ

マクロの仕組み、定義の仕方、このリポジトリのマクロが実際に展開されるコードをまとめます。
基本構文は [Rust の構文入門](syntax-basics.md) を先に読んでください。

## 1. マクロとは

マクロは、**コードを生成するコード**です。コンパイルの途中で、呼び出した部分が別のコードに置き換わります。

```text
ソースコード
  ↓ 字句解析・構文解析
マクロの展開        ← ここで write! などが置き換わる
  ↓ 名前解決・型チェック
機械語
```

型チェックより前に展開されるので、次のような特徴があります。

- 関数にはできないことができます。可変長の引数、構造体や `impl` の生成などです
- 生成されたコードは、手で書いたコードと同じように型チェックされます
- マクロは、型の情報を見て処理を変えることはできません

### 他の言語との比較

| 言語 | 近いもの | Rust との違い |
|---|---|---|
| C/C++ | `#define` | C は文字列に近い置換で、Rust は構文を理解して置換する |
| C++ | テンプレート | テンプレートは型に基づいて生成する。マクロは型を見ない |
| Java | アノテーションプロセッサ | Rust の `derive` に近い。コンパイル時にコードを生成する |
| C# | ソースジェネレーター | Rust の `derive` に近い |
| Python | デコレータ | デコレータは実行時に動く。マクロはコンパイル時に動く |
| Go | `go generate` | 別のコマンドで生成する。マクロはコンパイルの一部として動く |

### マクロの種類

| 種類 | 呼び出し方 | 定義の方法 | このリポジトリの例 |
|---|---|---|---|
| 宣言的マクロ | `name!(...)` | `macro_rules!` | `vec!`、`write!`、`println!` |
| 関数風の手続き的マクロ | `name!(...)` | Rust の関数 | なし（[手順 5](#5-手続き的マクロを定義する) の `quote!` がこの種類） |
| derive マクロ | `#[derive(Name)]` | Rust の関数 | `Pod`、`Zeroable` |
| 属性マクロ | `#[name]` | Rust の関数 | なし |
| コンパイラ組み込み | `name!(...)` | コンパイラが実装 | `format_args!`、`include_str!`、`offset_of!` |

## 2. 展開結果を確認する

実際の展開結果は、次のコマンドで見られます。リポジトリのルートで実行してください。

```bash
RUSTC_BOOTSTRAP=1 cargo rustc --quiet -- -Zunpretty=expanded > /tmp/expanded.rs
```

`-Zunpretty` は本来 nightly 版の機能です。`RUSTC_BOOTSTRAP=1` を付けると、stable 版のまま使えます。
調査用の方法なので、通常のビルドには使わないでください。

RustRover では、マクロの呼び出しにカーソルを置いて `Option + Enter` を押し、**Show recursive macro expansion** を選ぶと確認できます。
1段階だけ展開する **Show single-step macro expansion** もあります。
この画面操作は JetBrains の情報に基づくもので、このリポジトリでは試していません。

## 3. このリポジトリのマクロの展開結果

以下は Rust 1.99.1 での実際の出力です。読みやすいように改行を調整しています。
`::std::io::_print` などは内部用の名前で、Rust のバージョンで変わることがあります。自分のコードから直接呼ばないでください。

| マクロ | 使用箇所 | 展開先 |
|---|---|---|
| `println!` | `src/main.rs` | `::std::io::_print(format_args!(...))` |
| `eprintln!` | `src/gl_window.rs` | `::std::io::_eprint(format_args!(...))` |
| `write!` | `src/main.rs` | `file.write_fmt(format_args!(...))` |
| `format!` | `src/shader.rs` | `::alloc::fmt::format(format_args!(...))` |
| `panic!` | `src/main.rs` | `::core::panicking::panic_fmt(format_args!(...))` |
| `vec!` | `src/main.rs` | `::alloc::vec::from_elem(...)` |
| `include_str!` | `src/cube.rs` | ファイルの中身の文字列リテラル |
| `offset_of!` | `src/cube.rs` | コンパイラ組み込みの `builtin # offset_of(...)` |
| `#[derive(...)]` | `src/cube.rs` | `impl` ブロックとコンパイル時のチェック |

文字列を整形するマクロは、どれも `format_args!` を使っていることが分かります。

### `write!`

```rust
// src/main.rs
write!(file, "P6\n{width} {height}\n255\n")?;
```

展開すると、`file` の `write_fmt` メソッドの呼び出しになります。

```rust
// 展開後
file.write_fmt(format_args!("P6\n{0} {1}\n255\n", width, height))?;
```

`write!` は標準ライブラリで次のように定義されています。

```rust
// 標準ライブラリの定義
macro_rules! write {
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt($crate::format_args!($($arg)*))
    };
    ($($arg:tt)*) => {
        compile_error!("requires a destination and format arguments, like `write!(dest, \"format string\", args...)`")
    };
}
```

最初の引数を `$dst`、残りを `$arg` に受け取って、`$dst.write_fmt(...)` を生成しています。
`write_fmt` は `std::io::Write` トレイトのメソッドです。`use std::io::Write` が必要なのはこのためです。

`?` はマクロではないので、この段階ではそのまま残ります。

### `println!`

```rust
// src/main.rs
println!("OpenGL: {}", gl.get_parameter_string(glow::VERSION));
```

```rust
// 展開後
::std::io::_print(format_args!("OpenGL: {0}\n",
        gl.get_parameter_string(glow::VERSION)));
```

改行の `\n` は、マクロが書式の文字列に追加します。

### `format_args!`

ここまでの展開結果に `format_args!` が残っているのは、これがコンパイラ組み込みのマクロだからです。
`format_args!` は書式の文字列をコンパイル時に解析して、次のことを行います。

- `{width}` のような名前を、`{0}` と引数 `width` の形に変換します
- 引数の数や、表示できる型かどうかを確認します

そのため、書式の間違いがコンパイルエラーになります。C の `printf` のように実行時まで分からない、ということがありません。

```rust
// コンパイルエラーになる例
fn main() {
    println!("{} {}", 1);
}
```

### `panic!`

```rust
// src/main.rs
other => panic!("不明な引数です: {other}"),
```

```rust
// 展開後
other => {
    ::core::panicking::panic_fmt(format_args!("不明な引数です: {0}",
            other));
}
```

### `vec!`

```rust
// src/main.rs
let mut pixels = vec![0u8; row_size * height as usize];
```

```rust
// 展開後
let mut pixels = ::alloc::vec::from_elem(0u8, row_size * height as usize);
```

`vec!` は引数の書き方で展開先が変わります。標準ライブラリの定義の一部は次のとおりです。

```rust
// 標準ライブラリの定義（一部）
macro_rules! vec {
    () => (
        $crate::vec::Vec::new()
    );
    ($elem:expr; $n:expr) => (
        $crate::vec::from_elem($elem, $n)
    );
    // ...
}
```

`vec![0u8; 100]` は `$elem:expr; $n:expr` の形に一致します。
`$crate` は、マクロを定義したクレートを指します。

### `include_str!`

```rust
// src/cube.rs
include_str!("../shaders/cube.vert"),
```

ファイルの中身が、文字列リテラルとして実行ファイルに埋め込まれます。

```text
"#version 410 core\n\nlayout(location = 0) in vec3 a_position;\n..."
```

実行時にシェーダーのファイルを読みに行かないので、実行ファイルだけで動きます。
パスは、マクロを書いたソースファイルからの相対パスです。

### `offset_of!`

```rust
// src/cube.rs
std::mem::offset_of!(Vertex, position),
```

構造体の中でのフィールドの位置を、バイト単位で返します。C の `offsetof` と同じ役割です。
展開するとコンパイラ組み込みの構文になります。

```text
builtin # offset_of(Vertex, position)
```

### `#[derive(...)]`

`derive` は、構造体の定義からトレイトの実装を自動で生成します。

```rust
// src/cube.rs
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    color: [f32; 3],
}
```

`#[repr(C)]` はメモリ配置を C と同じにする指定で、マクロではありません。

`Clone` と `Copy` は次のように展開されます。

```rust
// 展開後
#[automatically_derived]
impl ::core::clone::Clone for Vertex {
    #[inline]
    fn clone(&self) -> Vertex {
        let _: ::core::clone::AssertParamIsClone<[f32; 3]>;
        let _: ::core::clone::AssertParamIsClone<[f32; 3]>;
        let _: ::core::clone::AssertParamIsClone<[f32; 3]>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Vertex { }
```

`Pod` と `Zeroable` は bytemuck クレートの derive マクロです。
`Vertex` をバイト列として GPU に送っても安全かを、コンパイル時に確認するコードを生成します。

```rust
// 展開後
const _: () =
    {
        if !(::core::mem::size_of::<Vertex>() ==
                    (::core::mem::size_of::<[f32; 3]>() +
                                ::core::mem::size_of::<[f32; 3]>() +
                            ::core::mem::size_of::<[f32; 3]>())) {
            {
                ::std::rt::begin_panic("derive(Pod) was applied to a type with padding");
            }
        };
    };
// ...
unsafe impl ::bytemuck::Pod for Vertex {}
```

構造体の大きさが、フィールドの大きさの合計と等しいことを確認しています。
等しくなければ、間に隙間（パディング）があるので、コンパイルエラーになります。
実際には、各フィールドの型が `Pod` かどうかを確認するコードも生成されます。

## 4. 宣言的マクロを定義する

`macro_rules!` で、自分のマクロを定義できます。
`match` のように、入力のパターンと、生成するコードの組を書きます。

```rust
macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

fn main() {
    println!("{}", square!(2 + 3)); // 25
}
```

- `$x` はマクロの引数の名前です
- `expr` は、引数の種類が「式」であることを表します
- `=>` の右側が生成するコードです

C の `#define SQUARE(x) x * x` では、`SQUARE(2 + 3)` が `2 + 3 * 2 + 3` に展開されて 11 になります。
Rust のマクロは `2 + 3` を1つの式として扱うので、正しく 25 になります。

### 引数の種類

よく使うものは次のとおりです。

| 指定 | 受け取るもの | 例 |
|---|---|---|
| `expr` | 式 | `1 + 2`、`foo()` |
| `ident` | 名前 | `value`、`Player` |
| `ty` | 型 | `u32`、`Vec<String>` |
| `pat` | パターン | `Some(x)` |
| `literal` | リテラル | `1`、`"text"` |
| `block` | ブロック | `{ ... }` |
| `tt` | 任意のトークン | 何でも |

`ident` と `ty` を使うと、構造体やメソッドも生成できます。

```rust
macro_rules! make_struct {
    ($name:ident, $field:ident: $field_type:ty) => {
        struct $name {
            $field: $field_type,
        }

        impl $name {
            fn $field(&self) -> &$field_type {
                &self.$field
            }
        }
    };
}

make_struct!(Player, score: u32);

fn main() {
    let player = Player { score: 10 };
    println!("{}", player.score());
}
```

### 繰り返し

`$( ... )` で囲むと、同じ形の繰り返しを受け取れます。

| 書き方 | 意味 |
|---|---|
| `$( ... ),*` | カンマ区切りで0回以上 |
| `$( ... ),+` | カンマ区切りで1回以上 |
| `$( ... )?` | 0回か1回 |

生成するコードの側でも、同じ書き方で繰り返します。

```rust
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($($item:expr),+ $(,)?) => {{
        let mut values = Vec::new();
        $(
            values.push($item);
        )+
        values
    }};
}

fn main() {
    let empty: Vec<u32> = my_vec![];
    let numbers = my_vec![1, 2, 3];
    println!("{empty:?} {numbers:?}");
}
```

- `$(,)?` は、最後のカンマがあってもなくてもよいという意味です
- `{{ ... }}` の外側はマクロの構文で、内側は生成するブロックです
- パターンは上から順に比べられ、最初に一致したものが使われます

### 再帰

マクロは自分自身を呼び出せます。

```rust
macro_rules! max {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($rest:expr),+) => {{
        let first = $x;
        let rest = max!($($rest),+);
        if first > rest { first } else { rest }
    }};
}

fn main() {
    println!("{}", max!(3, 9, 5));
}
```

### 衛生性（hygiene）

マクロの中で作った変数は、呼び出し側の同じ名前の変数と衝突しません。これを**衛生性**と呼びます。

```rust
macro_rules! double {
    ($x:expr) => {{
        let value = $x;
        value * 2
    }};
}

fn main() {
    let value = 10;
    println!("{}", double!(value + 1)); // 22
    println!("{value}"); // 10
}
```

逆に、マクロの中から呼び出し側の変数を勝手に使うこともできません。使いたい変数は引数で渡します。

```rust
// コンパイルエラーになる例
macro_rules! print_value {
    () => {
        println!("{}", value)
    };
}

fn main() {
    let value = 1;
    print_value!();
}
```

### 定義の順序と公開

`macro_rules!` のマクロは、定義より後の行でしか使えません。関数とは違うので注意してください。

```rust
// コンパイルエラーになる例
fn main() {
    hello!();
}

macro_rules! hello {
    () => {
        println!("hello")
    };
}
```

他のクレートから使えるようにするには、`#[macro_export]` を付けます。

```rust
#[macro_export]
macro_rules! hello {
    () => {
        println!("hello")
    };
}

fn main() {
    hello!();
}
```

## 5. 手続き的マクロを定義する

`derive` のように構文を解析してコードを作るマクロは、Rust の関数として書きます。
入力のトークン列を受け取り、新しいトークン列を返す関数です。

手続き的マクロは、**専用のクレート**に分ける必要があります。

```text
hello-derive/         # マクロを定義するクレート
├── Cargo.toml
└── src/lib.rs
hello-app/            # マクロを使うクレート
├── Cargo.toml
└── src/main.rs
```

クレートを作って、必要なライブラリを追加します。

```bash
cargo new hello-derive --lib
```

```bash
cargo new hello-app
```

```bash
cargo add syn quote --manifest-path hello-derive/Cargo.toml
```

```bash
cargo add --path hello-derive --manifest-path hello-app/Cargo.toml
```

`hello-derive/Cargo.toml` に、次の設定を追加します。

```toml
[lib]
proc-macro = true
```

`hello-derive/src/lib.rs` にマクロを書きます。

```rust
// hello-derive/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Hello)]
pub fn derive_hello(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl #name {
            fn hello(&self) {
                println!("Hello from {}", stringify!(#name));
            }
        }
    };

    expanded.into()
}
```

| 名前 | 役割 |
|---|---|
| `syn` | トークン列を解析して、構造体の名前などを取り出す |
| `quote!` | Rust のコードを書く形で、トークン列を作る。`#name` は変数の値に置き換わる |
| `TokenStream` | マクロの入力と出力になるトークン列 |

`hello-app/src/main.rs` で使います。

```rust
// hello-app/src/main.rs
use hello_derive::Hello;

#[derive(Hello)]
struct App;

fn main() {
    App.hello();
}
```

`cargo run` を実行すると `Hello from App` と表示されます。展開結果は次のとおりです。

```rust
// hello-app の展開後
struct App;
impl App {
    fn hello(&self) {
        { ::std::io::_print(format_args!("Hello from {0}\n", "App")); };
    }
}
```

## 6. マクロを使うべきとき

まず関数やジェネリクスで書けないかを考えます。
マクロは、書く側も読む側も難しくなり、エラーメッセージも分かりにくくなりがちです。

| やりたいこと | 向いている方法 |
|---|---|
| 処理を共通化する | 関数 |
| 複数の型で同じ処理を使う | ジェネリクス、トレイト |
| 引数の数を自由にする | マクロ |
| 似た `struct` や `impl` を繰り返し書く | 宣言的マクロ |
| 型の定義からコードを自動で作る | derive マクロ |

## 次に読む

- [The Rust Programming Language: Macros](https://doc.rust-lang.org/book/ch20-05-macros.html): 公式の入門書のマクロの章
- [The Rust Reference: Macros By Example](https://doc.rust-lang.org/reference/macros-by-example.html): `macro_rules!` の仕様
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/): マクロを詳しく解説した資料
