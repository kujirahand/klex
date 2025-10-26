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
パターン -> トークン名
```

サポートされているパターン形式：

- `'c'` - 単一文字リテラル
- `"文字列"` - 文字列リテラル
- `[0-9]+` - 文字範囲と量詞
- `[abc]+` - 文字集合と量詞
- `/正規表現/` - 正規表現パターン
- `( パターン1 | パターン2 )` - パターンの選択肢
- `\+` - エスケープされた特殊文字（`\+`、`\*`、`\n`、`\t`など）
- `?` - 任意の単一文字
- `?+` - 1回以上の任意文字

例：

```text
[0-9]+ -> NUMBER
[a-zA-Z_][a-zA-Z0-9_]* -> IDENTIFIER
\+ -> PLUS
\- -> MINUS
\n -> NEWLINE
\t -> TAB
? -> ANY_CHAR
?+ -> ANY_CHAR_PLUS
"hello" -> HELLO
/[0-9]+\.[0-9]+/ -> FLOAT
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

## 高度な機能

### エスケープ文字

klexではエスケープされた特殊文字をサポートしています：

```text
\+ -> PLUS_ESCAPED    # リテラル '+'にマッチ
\* -> MULTIPLY        # リテラル '*'にマッチ
\n -> NEWLINE         # 改行文字にマッチ
\t -> TAB             # タブ文字にマッチ
```

### ワイルドカードパターン

柔軟なマッチングのためにワイルドカードパターンを使用できます：

```text
? -> ANY_CHAR         # 任意の単一文字にマッチ
?+ -> ANY_CHAR_PLUS   # 1文字以上の任意文字にマッチ
```

### コンテキスト依存ルール

ルールは直前のトークンに依存することができます：

```text
%IDENTIFIER [0-9]+ -> INDEXED_NUMBER   # IDENTIFIERの後でのみ
```

### アクションコード

パターンがマッチしたときにカスタムRustコードを実行できます：

```text
"debug" -> { println!("Debug mode!"); None }
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

すべてのテストを実行：

```bash
cargo test
```

テストファイルには以下が含まれます：

- `tests/example.klex` - 基本的なレキサーの例
- `tests/test_context.klex` - コンテキスト依存ルール
- `tests/test_new_patterns.klex` - 様々なパターンタイプ
- `tests/test_escaped_chars.klex` - エスケープ文字パターン
- `tests/test_any_chars.klex` - ワイルドカードパターン

## ライセンス

MITライセンスの下で公開されています。
