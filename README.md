# klex (kujira-lexer)

シンプルなLexer（字句解析器）ジェネレーターです。

## 概要

klexは、定義ファイルからRustのLexerコードを自動生成するツールです。正規表現でトークンのパターンを記述でき、自動的にToken構造体とLexer構造体を含むRustコードを出力します。

## インストール

```bash
cargo build --release
```

## 使い方

### 基本的な使い方

```bash
cargo run -- <入力ファイル> [出力ファイル]
```

### 入力ファイルの形式

入力ファイルは3つのセクションから構成され、`%%`で区切ります：

```
(ここにRustのコード - use文など)
%%
(ここにルール - 正規表現でトークンパターンを記述)
%%
(ここにRustのコード - main関数やテストなど)
```

### ルールの記述方法

各ルールは1行に1つ記述します：

```
正規表現パターン -> トークン名
```

例：
```
[0-9]+ -> NUMBER
[a-zA-Z_][a-zA-Z0-9_]* -> IDENTIFIER
\+ -> PLUS
\- -> MINUS
```

### 生成されるToken構造体

生成されるLexerは以下のToken構造体を出力します：

```rust
struct Token {
    kind: u32,        // トークンの種類（定数で定義される）
    value: String,    // トークンの文字列値
    row: usize,       // 行番号（1から開始）
    col: usize,       // 列番号（1から開始）
    length: usize,    // トークンの長さ
    indent: usize,    // インデント（行頭の空白数）
    tag: isize,       // カスタムタグ（デフォルトは0）
}
```

## 例

`example.klex`ファイルを参照してください。

### Lexerの生成

```bash
cargo run -- example.klex generated_lexer.rs
```

### 生成されたLexerの使用

生成されたファイルには、`Lexer`構造体と関連する定数が含まれています：

```rust
let input = "123 + abc".to_string();
let mut lexer = Lexer::new(input);

while let Some(token) = lexer.next_token() {
    println!("{:?}", token);
}
```

## テスト

```bash
cargo test
```

## ライセンス

MITライセンスの下で公開されています。

