# klex プロジェクト仕様書

## プロジェクト概要

**klex (kujira-lexer)** は、Rust用のシンプルなレキサー（トークナイザー）ジェネレーターです。定義ファイルから正規表現パターンを使ってトークンパターンを記述し、`Token`構造体と`Lexer`構造体を含むRustソースコードを出力します。

## 基本情報

- **プロジェクト名**: klex
- **バージョン**: 0.1.0
- **言語**: Rust (Edition 2021)
- **ライセンス**: MIT
- **作者**: kujirahand
- **リポジトリ**: <https://github.com/kujirahand/klex>

## アーキテクチャ

### プロジェクト構造

```text
klex/
├── src/
│   ├── main.rs          # コマンドラインインターフェース
│   ├── lib.rs           # ライブラリのエントリーポイント
│   ├── parser.rs        # 仕様ファイルのパーサー
│   ├── generator.rs     # Rustコード生成器
│   ├── token.rs         # Tokenデータ構造定義
│   └── lexer.rs         # レキサーテンプレート（build.rsで利用）
├── tests/               # テストファイル（.klexファイルとRustテストファイル）
│   ├── example.klex     # サンプル仕様ファイル
│   └── *.klex           # 各種テスト
├── build.rs             # ビルドスクリプト（テンプレート埋め込み）
├── Makefile             # ビルド・テスト自動化
├── Cargo.toml           # プロジェクト設定
└── README.md            # プロジェクト説明書
```

### 主要コンポーネント

#### 1. パーサー (`parser.rs`)

**役割**: `.klex`ファイルを解析し、`LexerSpec`構造体に変換

**主要な構造体**:

- `LexerRule`: 個別のレキサールール（パターン、トークン種別、名前）
- `LexerSpec`: 解析された仕様（プレフィックスコード、ルール、サフィックスコード）
- `ParseError`: パースエラー

**処理フロー**:

1. `%%`区切りで3セクションに分割
2. 中央セクションでルール解析（`pattern -> TOKEN_NAME`形式）
3. 各ルールに連番のトークン種別を割り当て

#### 2. ジェネレーター (`generator.rs`)

**役割**: `LexerSpec`からRustソースコードを生成

**特徴**:

- テンプレートベースのコード生成
- 正規表現キャッシュによる最適化
- トークン定数の自動生成

#### 3. レキサーテンプレート (`lexer.rs`)

**役割**: 生成されるレキサーの基本構造を定義

**主要機能**:

- 正規表現マッチング
- 位置情報追跡（行、列）
- インデント計算
- キャッシュ機能付き正規表現マッチング
- アクションコード実行（カスタムロジック実行）

#### 4. トークン定義 (`token.rs`)

**Token構造体の仕様**:

```rust
pub struct Token {
    pub kind: u32,      // トークン種別（定数として定義）
    pub value: String,  // マッチしたテキスト
    pub row: usize,     // 1ベース行番号
    pub col: usize,     // 1ベース列番号
    pub length: usize,  // トークン長
    pub indent: usize,  // 行頭からのインデント（スペース数）
    pub tag: isize,     // カスタムタグ（デフォルト: 0）
}
```

## 入力ファイル形式

### 3セクション構造

`.klex`ファイルは`%%`で区切られた3つのセクションから構成されます：

```text
(Rustコード - use文など)
%%
(ルール定義 - 正規表現パターン)
%%
(Rustコード - main関数やテストなど)
```

### ルール記述形式

各ルールは以下の形式で記述：

```text
<規則> -> <TOKEN_NAME>
%<TOKEN_NAME> <規則> -> <TOKEN_NAME>
<規則> -> { <ACTION_CODE> }
```

規則は次のいずれかの形式を取ることができます:

```text
'文字'
"文字列"
\+ ← エスケープされた特殊文字
\n ← 改行文字
? ← 任意の単一文字を表す
?+ ← 1回以上の任意の単一文字の連続を表す
[0-9]+ ← 1回以上の単純な文字範囲'A'から'Z'までの連続を表す
[abc]+ ← 1回以上の文字集合'a'、'b'、'c'のいずれかの連続を表す
[0-9]* ← 0回以上の単純な文字範囲'A'から'Z'までの連続を表す
[abc]* ← 0回以上の文字集合'a'、'b'、'c'のいずれかの連続を表す
/正規表現パターン/ ← 正規表現で複雑なパターンを表現
( <規則> | <規則> )  ← 選択肢
```

#### アクションコードルール

アクションコードルールは、マッチしたパターンに対してカスタムなRustコードを実行します。アクションコード内では以下の変数が利用可能です：

- `test_t: Token` - 現在のマッチしたトークン

アクションコードは `Option<Token>` を返す必要があります：

- `Some(token)` - 指定されたトークンを返す
- `None` - トークンをスキップして次のマッチングを継続

アクションルールは通常のトークンルールより高い優先度で処理されます。

コンテキスト依存ルール（`%`で始まる）は、指定されたトークンの直後にのみマッチします。空白やニューライントークンはコンテキストを更新しないため、意味のあるトークン間でのコンテキスト依存マッチングが可能です。

**例**:

```text
[0-9]+ -> NUMBER
/[a-zA-Z_][a-zA-Z0-9_]*/ -> IDENTIFIER
'+' -> PLUS
'-' -> MINUS
'*' -> MULTIPLY
'/' -> DIVIDE
'(' -> LPAREN
')' -> RPAREN
/[ \t]+/ -> WHITESPACE
\n -> NEWLINE
%IDENTIFIER /[0-9]+/ -> INDEXED_NUMBER
%PLUS /[0-9]+/ -> POSITIVE_NUMBER
// アクションコード例
"debug" -> { println!("Debug mode activated!"); None }
/[0-9]+\.[0-9]+/ -> { 
    let float_val: f64 = matched.parse().unwrap();
    Some(Token::new(FLOAT_TOKEN, matched.clone(), _start_row, _start_col, matched.len(), _indent))
}
```

この例では：

- `INDEXED_NUMBER`は`IDENTIFIER`トークンの直後にのみマッチし、`POSITIVE_NUMBER`は`PLUS`トークンの直後にのみマッチします
- `"debug"`はデバッグメッセージを出力してトークンをスキップします
- 浮動小数点数は値を解析してカスタムトークンを返します

## 使用方法

### ライブラリとして使用

```rust
use klex::{generate_lexer, parse_spec};
use std::fs;

// 入力ファイル読み込み
let input = fs::read_to_string("tests/example.klex").unwrap();

// 仕様を解析
let spec = parse_spec(&input).unwrap();

// Rustコード生成
let output = generate_lexer(&spec, "tests/example.klex");

// 出力ファイル書き込み
fs::write("output.rs", output).unwrap();
```

### コマンドラインツールとして使用

```bash
# 基本的な使用法
cargo run -- <入力ファイル> [出力ファイル]

# 例
cargo run -- tests/example.klex generated_lexer.rs
```

### 生成されたレキサーの使用

```rust
let input = "123 + abc".to_string();
let mut lexer = Lexer::new(input);

while let Some(token) = lexer.next_token() {
    println!("{:?}", token);
}
```

## 技術仕様

### 依存関係

- `regex = "1"`: 正規表現マッチング

### ビルド時処理

`build.rs`がビルド時に以下を実行：

1. `src/lexer.rs`テンプレートファイルを読み込み
2. 文字列リテラルとしてエスケープ処理
3. `OUT_DIR/template.rs`として埋め込み用コードを生成

### コンテキスト依存機能

- **前トークン追跡**: 直前の意味のあるトークンを記録
- **選択的コンテキスト更新**: `WHITESPACE`や`NEWLINE`はコンテキストを更新しない
- **優先度制御**: コンテキスト依存ルールは通常ルールより優先

### パフォーマンス最適化

- **正規表現キャッシュ**: コンパイル済み正規表現をHashMapでキャッシュ
- **効率的な文字列処理**: UTF-8対応の文字単位処理
- **メモリ効率**: 必要最小限のメモリ使用

### エラー処理

- 未知のトークンは`UNKNOWN_TOKEN (u32::MAX)`として処理
- パースエラーは`ParseError`型で詳細情報を提供
- ファイルI/Oエラーは標準的なエラー処理

## 制限事項と注意事項

1. **正規表現の制約**: Rustの`regex`クレートの制約に従う
2. **トークン優先度**: コンテキスト依存ルールが通常ルールより優先される
3. **コンテキスト管理**: 現在は直前の1トークンのみを追跡（深いコンテキストは未サポート）
4. **メモリ使用量**: 大きな入力に対してはメモリ使用量に注意
5. **Unicode対応**: UTF-8文字列の適切な処理をサポート

## 拡張可能性

### 将来の機能追加候補

1. **拡張コンテキスト**: 複数トークンの履歴追跡
2. **カスタムトークン処理**: トークン後処理のフック機能
3. **エラーリカバリ**: より詳細なエラー情報と回復機能
4. **パフォーマンス向上**: さらなる最適化オプション
5. **IDEサポート**: 構文ハイライトや自動補完
6. **デバッグ機能**: トレース機能やデバッグ出力
7. **状態機械**: より複雑なコンテキスト依存ルール

### アーキテクチャ拡張

- プラグインシステムの導入
- 複数バックエンド対応（他言語での出力）
- 並列処理サポート

## テスト戦略

- **単体テスト**: 各モジュールの機能テスト（`cargo test`）
- **統合テスト**: エンドツーエンドのファイル処理テスト（`tests/`ディレクトリ）
- **サンプルテスト**: `tests/example.klex`と`tests/test_context.klex`を使った実際の使用例テスト
- **自動化**: `Makefile`による統合されたビルド・テストプロセス

### Makefileターゲット

- `make test` - 全テストの実行
- `make generate-lexers` - 全テストファイルからレキサー生成
- `make test-example` - example.klexの個別テスト
- `make test-context` - test_context.klexの個別テスト  
- `make list-tests` - 利用可能なテストファイル一覧

## ドキュメント

- **README.md**: 基本的な使用方法と概要
- **README-ja.md**: 日本語版ドキュメント
- **rustdoc**: コード内ドキュメント（`///`コメント）
- **cargo doc**: API ドキュメント生成

この仕様書は、klexプロジェクトの開発、保守、拡張における指針として活用されることを想定しています。
