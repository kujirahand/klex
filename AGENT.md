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
├── build.rs             # ビルドスクリプト（テンプレート埋め込み）
├── example.klex         # サンプル仕様ファイル
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
<正規表現パターン> -> <TOKEN_NAME>
```

**例**:

```text
[0-9]+ -> NUMBER
[a-zA-Z_][a-zA-Z0-9_]* -> IDENTIFIER
\+ -> PLUS
\- -> MINUS
\* -> MULTIPLY
/ -> DIVIDE
\( -> LPAREN
\) -> RPAREN
[ \t]+ -> WHITESPACE
\n -> NEWLINE
```

## 使用方法

### ライブラリとして使用

```rust
use klex::{generate_lexer, parse_spec};
use std::fs;

// 入力ファイル読み込み
let input = fs::read_to_string("example.klex").unwrap();

// 仕様を解析
let spec = parse_spec(&input).unwrap();

// Rustコード生成
let output = generate_lexer(&spec, "example.klex");

// 出力ファイル書き込み
fs::write("output.rs", output).unwrap();
```

### コマンドラインツールとして使用

```bash
# 基本的な使用法
cargo run -- <入力ファイル> [出力ファイル]

# 例
cargo run -- example.klex generated_lexer.rs
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
2. **トークン優先度**: ファイル内の記述順序でマッチング優先度が決定
3. **メモリ使用量**: 大きな入力に対してはメモリ使用量に注意
4. **Unicode対応**: UTF-8文字列の適切な処理をサポート

## 拡張可能性

### 将来の機能追加候補

1. **カスタムトークン処理**: トークン後処理のフック機能
2. **エラーリカバリ**: より詳細なエラー情報と回復機能
3. **パフォーマンス向上**: さらなる最適化オプション
4. **IDEサポート**: 構文ハイライトや自動補完
5. **デバッグ機能**: トレース機能やデバッグ出力

### アーキテクチャ拡張

- プラグインシステムの導入
- 複数バックエンド対応（他言語での出力）
- 並列処理サポート

## テスト戦略

- 単体テスト: 各モジュールの機能テスト
- 統合テスト: エンドツーエンドのファイル処理テスト
- サンプルテスト: `example.klex`を使った実際の使用例テスト

## ドキュメント

- **README.md**: 基本的な使用方法と概要
- **README-ja.md**: 日本語版ドキュメント
- **rustdoc**: コード内ドキュメント（`///`コメント）
- **cargo doc**: API ドキュメント生成

この仕様書は、klexプロジェクトの開発、保守、拡張における指針として活用されることを想定しています。
