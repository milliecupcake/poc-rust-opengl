# Rust の構文入門

C#、Java、C/C++、Python、Go のどれかを知っている人向けに、Rust の基本構文をまとめます。
例はできるだけ、このリポジトリの [src/main.rs](../src/main.rs) から取っています。

## 早見表

| 書き方 | ひとことで言うと | 解説 |
|---|---|---|
| `let mut` | 値を変更できる変数を宣言する | [1](#1-let-mut-変更できる変数) |
| `struct` | データの型を定義する | [2](#2-struct-データの型を定義する) |
| `App { ... }` | 構造体の値を作る | [3](#3-app----構造体の値を作る) |
| `impl` | 型に関数やメソッドを追加する | [4](#4-impl-型にメソッドを追加する) |
| `impl A for B` | 型 B がトレイト A を実装する | [5](#5-impl-a-for-b-トレイトを実装する) |
| `Option<T>` | 値があるか、ないかを表す型 | [6](#6-optiont-値がないかもしれない型) |
| `while let` | パターンに一致する間、繰り返す | [7](#7-while-let-パターンに一致する間繰り返す) |
| `match` | 値の形で処理を分ける | [8](#8-match-値の形で処理を分ける) |
| `&mut` | 値を変更できる参照を渡す | [9](#9-mut-変更できる参照を渡す) |
| `write!` | 整形した文字列を書き込むマクロ | [10](#10-write-マクロで書き込む) |
| `unsafe` | 安全性の確認を書き手が引き受ける | [11](#11-unsafe-安全性を自分で保証する) |

## 1. `let mut`: 変更できる変数

Rust の変数は、既定では変更できません。変更したい変数にだけ `mut` を付けます。

```rust
fn main() {
    let a = 1; // 変更できない
    let mut b = 2; // 変更できる
    b += a;
    println!("{b}");
}
```

`mut` のない変数に再代入すると、コンパイルエラーになります。

```rust
// コンパイルエラーになる例
fn main() {
    let a = 1;
    a = 2;
    println!("{a}");
}
```

他の言語と比べると、既定値が逆です。

| 言語 | 変更できない | 変更できる |
|---|---|---|
| Rust | `let x = 1;` | `let mut x = 1;` |
| Java | `final int x = 1;` | `int x = 1;` |
| C++ | `const int x = 1;` | `int x = 1;` |
| C# | `const int x = 1;`（定数のみ） | `int x = 1;` |

Java の `final` との大きな違いは、**中身の変更も禁止される**ことです。
`final List` には要素を追加できますが、Rust では `mut` のない `Vec` に要素を追加できません。
C++ の `const std::vector` に近い動きです。

`src/main.rs` の次のコードでは、`next()` が内部の位置を進めるので、`args` に `mut` が必要です。

```rust
// src/main.rs
let mut args = std::env::args().skip(1);
```

## 2. `struct`: データの型を定義する

`struct` はデータの型を定義します。各フィールドは `名前: 型` の順に書きます。

```rust
// src/main.rs
struct App {
    scene: Option<Scene>,
    screenshot_path: Option<PathBuf>,
    screenshot_frame: u32,
}
```

クラスとの主な違いは次のとおりです。

- メソッドは `struct` の中に書かず、`impl` に分けて書きます（[4](#4-impl-型にメソッドを追加する)）
- 継承はありません。共通の振る舞いはトレイトで表します（[5](#5-impl-a-for-b-トレイトを実装する)）
- フィールドは既定で非公開です。外のモジュールに公開するには `pub` を付けます

Go の `struct` にメソッドを別に定義する形が、いちばん近いです。

## 3. `App { ... }`: 構造体の値を作る

`src/main.rs` の次のコードは、`App` の値を作って、変更できる変数 `app` に入れています。

```rust
// src/main.rs
let mut app = App {
    scene: None,
    screenshot_path: None,
    screenshot_frame: DEFAULT_SCREENSHOT_FRAME,
};
```

これは関数呼び出しではなく、構造体の値を直接書く構文です。Go の `App{Scene: nil}` とほぼ同じです。

- Rust には `new` 演算子もコンストラクタ構文もありません
- すべてのフィールドに値を指定する必要があります。指定を忘れるとコンパイルエラーになります
- `mut` が必要なのは、後で `app.screenshot_path` などを書き換えるためです

フィールド名と同じ名前の変数があれば、`名前: 名前` を `名前` だけに省略できます。

```rust
// src/main.rs
Self {
    gl_window,
    cube,
    started_at: Instant::now(),
    frame: 0,
}
```

コンストラクタの代わりに、`new` という名前の関数を作るのが慣習です。
`new` は予約語ではなく、ただの関数名です。

## 4. `impl`: 型にメソッドを追加する

`impl 型名 { ... }` の中に、その型の関数を書きます。

```rust
struct Counter {
    value: u32,
}

impl Counter {
    // self がない関数。Java や C# の static メソッドにあたる
    fn new() -> Self {
        Self { value: 0 }
    }

    // 読み取るだけのメソッド
    fn get(&self) -> u32 {
        self.value
    }

    // 値を変更するメソッド
    fn increment(&mut self) {
        self.value += 1;
    }
}

fn main() {
    let mut counter = Counter::new();
    counter.increment();
    println!("{}", counter.get());
}
```

第1引数の `self` の書き方で、メソッドが値をどう扱うかが決まります。

| 第1引数 | 意味 | 似ているもの |
|---|---|---|
| なし | `型名::関数()` で呼ぶ関数 | Java や C# の `static` |
| `&self` | 読み取るだけ | C++ の `const` メソッド、Go の値レシーバーに近い |
| `&mut self` | 値を変更する | C++ の通常のメソッド、Go のポインタレシーバー |
| `self` | 値を受け取って消費する | 近いものなし |

Python の `self` のように明示して、`this` のように暗黙にはしません。
`Self`（大文字）は、`impl` の対象の型を指します。ここでは `Counter` と同じ意味です。

`get` の `self.value` にセミコロンがないことにも注目してください。
関数の最後の式にセミコロンを付けないと、それが戻り値になります。`return` は早く抜けるときに使います。

## 5. `impl A for B`: トレイトを実装する

**トレイト**は、Java や C# のインターフェースにあたるものです。
`impl A for B` は「型 B にトレイト A を実装する」という意味です。

```rust
// 「挨拶できる」という機能を定義する
trait Greet {
    fn greet(&self) -> String;

    // 既定の実装を持てる
    fn greet_twice(&self) -> String {
        format!("{0} {0}", self.greet())
    }
}

struct Dog;

// Dog に Greet を実装する
impl Greet for Dog {
    fn greet(&self) -> String {
        String::from("ワン")
    }
}

fn main() {
    println!("{}", Dog.greet_twice());
}
```

他の言語で書くと、次の形にあたります。

| 言語 | 書き方 |
|---|---|
| Rust | `impl Greet for Dog { ... }` |
| Java | `class Dog implements Greet { ... }` |
| C# | `class Dog : IGreet { ... }` |
| C++ | `class Dog : public Greet { ... }`（純粋仮想関数を実装） |
| Python | `class Dog(Greet): ...`（抽象基底クラスの場合） |
| Go | 同じメソッドを定義すれば、宣言なしで実装扱いになる |

Java や C# との違いは、型の定義とトレイトの実装が別々に書けることです。
Go との違いは、実装を `impl` で明示する必要があることです。

`src/main.rs` の次のコードは、winit の `ApplicationHandler` トレイトを `App` に実装しています。

```rust
// src/main.rs
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.scene.is_none() {
            self.scene = Some(Scene::new(event_loop));
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        // ...
    }
}
```

`resumed` や `window_event` を自分で呼んでいる場所はありません。
winit がイベントの発生時に呼び出します。GUI フレームワークのイベントハンドラと同じ仕組みです。

`ApplicationHandler` にはほかのメソッドもありますが、既定の実装があるので、必須の2つだけを書いています。

## 6. `Option<T>`: 値がないかもしれない型

Rust には `null` がありません。値がないかもしれないときは `Option<T>` を使います。
`Option<T>` は、標準ライブラリに次のように定義された列挙型です。

```rust
enum Option<T> {
    None,    // 値がない
    Some(T), // T 型の値がある
}

fn main() {
    let value: Option<u32> = Option::Some(1);
    if let Option::Some(number) = value {
        println!("{number}");
    }
    let _empty: Option<u32> = Option::None;
}
```

`<T>` はジェネリクスで、`Option<Scene>` なら「`Scene` があるか、ないか」を表します。
実際には、`Some` と `None` は `Option::` なしで書けます。

| 言語 | 近いもの |
|---|---|
| Java | `Optional<T>` |
| C++ | `std::optional<T>` |
| C# | `T?` |
| Python | `Optional[T]`（`T \| None`） |
| Go | `nil` になるポインタ、`(値, ok)` の戻り値 |

`null` との違いは、**中身を確認しないと値を使えない**ことです。
`Option<Scene>` を `Scene` として使うとコンパイルエラーになるので、ヌルポインタの実行時エラーが起きません。

`App` の `scene` が `Option` なのは、起動の直後はまだウィンドウがなく、`Scene` を作れないからです。

```rust
// src/main.rs
let mut app = App {
    scene: None,
```

```rust
// src/main.rs
self.scene = Some(Scene::new(event_loop));
```

中身の取り出し方は、状況で使い分けます。

```rust
fn main() {
    let value: Option<u32> = Some(10);

    // あるときだけ処理する
    if let Some(number) = value {
        println!("{number}");
    }

    // ないときは既定値を使う
    let number = value.unwrap_or(0);
    println!("{number}");

    // ないときはエラーで停止する
    let number = value.expect("値が必要です");
    println!("{number}");
}
```

ないときに関数を抜ける `let ... else` もあります。Go の早期リターンに近い書き方です。

```rust
// src/main.rs
let Some(scene) = self.scene.as_mut() else {
    return;
};
```

`as_mut()` は、中身を取り出さずに、変更できる参照だけを借りるメソッドです。

## 7. `while let`: パターンに一致する間、繰り返す

`while let Some(arg) = args.next()` は、次の処理を繰り返します。

1. `args.next()` を呼びます。戻り値は `Option<String>` です
2. `Some(文字列)` なら、中身を `arg` に入れてループの中を実行します
3. `None` なら、ループを終わります

`let` の左側の `Some(arg)` が**パターン**です。右側の値の形を確認して、一致すれば中身を取り出します。

```rust
fn main() {
    let mut args = vec!["a", "b"].into_iter();
    while let Some(arg) = args.next() {
        println!("{arg}");
    }
}
```

他の言語で書くと、次の形にあたります。

```java
// Java
while (args.hasNext()) {
    String arg = args.next();
}
```

```c
// C
while ((arg = next_arg()) != NULL) {
}
```

ふつうは `for arg in args` と書きます。`src/main.rs` が `while let` を使うのは、ループの中でも `args.next()` を呼ぶからです。

```rust
// src/main.rs
while let Some(arg) = args.next() {
    match arg.as_str() {
        "--screenshot" => {
            let path = args.next().expect("--screenshot には保存先が必要です");
            // ...
        }
        // ...
    }
}
```

`--screenshot target/cube.ppm` では、`--screenshot` の次の値も読み進める必要があります。
`for` は `args` を受け取ってしまうので、ループの中で `args.next()` を呼べません。

## 8. `match`: 値の形で処理を分ける

`match` は、`switch` を強力にしたものです。

```rust
// src/main.rs
match arg.as_str() {
    "--screenshot" => {
        // ...
    }
    "--frame" => {
        // ...
    }
    other => panic!("不明な引数です: {other}"),
}
```

- `パターン => 処理` の形で書き、上から順に比べます
- `break` は不要で、次の分岐に流れ落ちません
- `other` は、それまでに一致しなかった値を受ける変数です。値を使わないときは `_` と書きます
- **すべての場合を書かないとコンパイルエラーになります**

C# の `switch` 式、Java 21 の `switch`、Python の `match` が近い書き方です。
C/C++ や Go の `switch` と違い、値を返せます。

```rust
fn main() {
    let number = 2;
    let text = match number {
        1 => "one",
        2 | 3 => "two or three",
        _ => "many",
    };
    println!("{text}");
}
```

### `arg.as_str()` が必要な理由

`arg` は `String` 型で、`"--frame"` のような文字列リテラルは `&str` 型です。
`match` で文字列リテラルと比べるために、`as_str()` で `String` を `&str` に変換しています。

| Rust の型 | 意味 | C++ で近いもの |
|---|---|---|
| `String` | 文字列を所有し、変更できる | `std::string` |
| `&str` | 文字列の一部を借りて読む | `std::string_view` |

### 構造の中まで調べる

`match` は値の中身も分解できます。
次のコードは「ウィンドウが閉じられた」または「Esc キーが押された」を、1つの分岐で判定しています。
`..` は残りのフィールドを無視するという意味です。

```rust
// src/main.rs
match event {
    WindowEvent::CloseRequested
    | WindowEvent::KeyboardInput {
        event:
            KeyEvent {
                logical_key: Key::Named(NamedKey::Escape),
                state: ElementState::Pressed,
                ..
            },
        ..
    } => event_loop.exit(),
    WindowEvent::Resized(size) => scene.gl_window.resize(size),
    // ...
    _ => {}
}
```

`_ => {}` は「それ以外は何もしない」という分岐です。

## 9. `&mut`: 変更できる参照を渡す

`src/main.rs` の最後の部分です。

```rust
// src/main.rs
event_loop
    .run_app(&mut app)
    .expect("実行中にエラーが発生しました");
```

`&mut app` は、`app` を変更できる参照を作ります。`run_app` は `app` を借りて、`resumed` などのメソッドを呼びます。
これらのメソッドは `&mut self` を取るので、`&mut` が必要です。

Rust の引数の渡し方は3種類です。

| 渡し方 | 意味 | 呼び出し後の元の変数 |
|---|---|---|
| `f(x)` | 所有権を渡す（ムーブ） | 使えなくなる（`Copy` の型を除く） |
| `f(&x)` | 読み取り用に貸す | 使える |
| `f(&mut x)` | 変更用に貸す | 使える |

```rust
fn read(text: &String) {
    println!("{text}");
}

fn change(text: &mut String) {
    text.push_str(" world");
}

fn take(text: String) {
    println!("{text}");
}

fn main() {
    let mut text = String::from("hello");
    read(&text);
    change(&mut text);
    take(text); // ここで所有権が移る
}
```

`take(text)` の後に `text` を使うと、コンパイルエラーになります。

```rust
// コンパイルエラーになる例
fn take(text: String) {
    println!("{text}");
}

fn main() {
    let text = String::from("hello");
    take(text);
    println!("{text}");
}
```

| 言語 | `&mut app` に近いもの |
|---|---|
| C++ | `App&` 引数。ただし Rust は呼び出す側にも `&mut` を書く |
| C / Go | `&app` でポインタを渡す |
| C# | `ref app` |
| Java / Python | オブジェクトの参照渡しが標準なので、近い動きになる |

ポインタとの大きな違いは、コンパイラが次の規則を確認することです。

- ある値に対して、`&mut` の参照は同時に1つだけ作れます
- `&mut` の参照がある間は、`&` の参照も作れません
- 読み取り用の `&` の参照は、同時にいくつでも作れます

また、`&mut app` を作るには、変数も `let mut app` で宣言されている必要があります。

## 10. `write!`: マクロで書き込む

名前の後ろの `!` は、関数ではなく**マクロ**を呼び出す印です。
マクロはコンパイル時に別のコードへ展開されます。

```rust
// src/main.rs
write!(file, "P6\n{width} {height}\n255\n")?;
```

このコードは、`file` に整形した文字列を書き込みます。

| 部分 | 意味 |
|---|---|
| `file` | 書き込み先 |
| `"...{width}..."` | 書式の文字列。`{width}` は変数 `width` の値に置き換わる |
| `?` | エラーなら、この関数からエラーを返す |

他の言語で書くと、次の形にあたります。

| 言語 | 書き方 |
|---|---|
| C | `fprintf(file, "%d", width);` |
| Go | `fmt.Fprintf(file, "%d", width)` |
| Python | `print(f"{width}", file=file)` |
| C# | `writer.Write($"{width}");` |

`{width}` の書き方は、Python の f-string や C# の `$"..."` に近いです。

マクロになっているのは、Rust の関数が可変長引数を持てないことと、書式の間違いをコンパイル時に見つけるためです。
`write!` を使うには、`std::io::Write` トレイトの読み込みが必要です。`src/main.rs` の先頭の `use` がそれにあたります。

```rust
// src/main.rs
use std::io::{BufWriter, Write};
```

このリポジトリでは、ほかに次のマクロも使っています。

| マクロ | 役割 |
|---|---|
| `println!` | 標準出力に1行出力する |
| `panic!` | エラーでプログラムを停止する |
| `vec!` | `Vec` を作る |
| `include_str!` | ファイルの中身をコンパイル時に埋め込む |

マクロの定義の仕方や、実際の展開結果は [Rust のマクロ](macros.md) にまとめています。

### `?` 演算子

`Result<T, E>` は、成功の `Ok(値)` か失敗の `Err(エラー)` を表す型です。`Option` のエラー版にあたります。
`?` は、`Ok` なら中身を取り出し、`Err` なら関数からそのエラーを返します。

```rust
use std::fs::File;
use std::io::Write;

fn save(path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    write!(file, "hello")?;
    Ok(())
}

fn main() {
    if let Err(error) = save("/no/such/folder/file.txt") {
        println!("保存に失敗しました: {error}");
    }
}
```

Go の次のコードを短く書いたものと考えると分かりやすいです。

```go
file, err := os.Create(path)
if err != nil {
    return err
}
```

## 11. `unsafe`: 安全性を自分で保証する

`unsafe { ... }` は、「コンパイラが確認できない安全性を、書いた人が保証する」という宣言です。
ブロックの中では、通常は禁止された操作ができます。主に次の2つです。

- `unsafe fn` を呼び出す
- 生ポインタの参照先を読み書きする

```rust
// src/main.rs
unsafe {
    println!("OpenGL: {}", gl.get_parameter_string(glow::VERSION));
    println!("GPU:    {}", gl.get_parameter_string(glow::RENDERER));
    gl.enable(glow::DEPTH_TEST);
    gl.enable(glow::CULL_FACE);
    gl.clear_color(0.08, 0.09, 0.12, 1.0);
}
```

ここで `unsafe` が必要なのは、OpenGL を呼び出す glow の関数が `unsafe fn` だからです。
OpenGL は C の API なので、渡した ID が有効かどうかなどを、Rust のコンパイラでは確認できません。

```rust
fn main() {
    let value = 10;
    let pointer = &value as *const i32; // 生ポインタの作成は unsafe なしでできる

    // 生ポインタの参照先を読むには unsafe が必要
    let read = unsafe { *pointer };
    println!("{read}");
}
```

注意点は次のとおりです。

- `unsafe` は、すべてのチェックを止めるものではありません。型の確認や借用の規則は、参照に対しては有効なままです
- 間違えると、C/C++ と同じようにメモリ破壊やクラッシュが起きることがあります
- 問題が起きたときに `unsafe` の部分から調べられるように、範囲はできるだけ小さくします

| 言語 | 近いもの |
|---|---|
| C/C++ | すべてのコードが `unsafe` の中にあるような状態 |
| C# | `unsafe` ブロック |
| Go | `unsafe` パッケージ |
| Java / Python | JNI や C 拡張でネイティブコードを呼ぶ部分 |

## 次に読む

- [The Rust Programming Language](https://doc.rust-lang.org/book/): 公式の入門書。特に第4章「所有権」が重要です
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/): 実行できる例で学ぶ公式の教材です
