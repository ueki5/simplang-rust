# simplang-rust

四則演算式を書いたソースファイルをネイティブバイナリ（x86-64）にコンパイルする玩具コンパイラ。

## コンパイルパイプライン

```
ソースファイル
  → tokenize()   → Vec<Token>    字句解析
  → parse()      → Expr (AST)   構文解析
  → compile()    → Vec<Instr>   スタック VM IR へ変換
  → codegen()    → x86-64 AT&T アセンブリ文字列
  → gcc          → ネイティブバイナリ
```

## ソースファイル構成

| ファイル | 役割 |
|---|---|
| `src/main.rs` | CLI エントリポイント（clap）、ファイル読み込み、gcc 呼び出し |
| `src/lib.rs` | `codegen` / `compiler` モジュールの再エクスポート |
| `src/compiler.rs` | 字句解析・構文解析・AST (`Expr`)・IR (`Instr`)・コンパイラ・スタック VM |
| `src/codegen.rs` | x86-64 アセンブリコード生成 |
| `tests/integration_test.rs` | gcc を使った実行ベースの統合テスト |

## サポートする言語構文

| 構文 | 例 |
|---|---|
| 整数リテラル（i64） | `42` |
| 加算 | `1 + 2` |
| 減算 | `5 - 3` |
| 乗算 | `2 * 6` |
| 除算 | `6 / 3` |
| 単項マイナス | `-5`, `-(1 + 2)` |
| 括弧によるグループ化 | `(1 + 2) * 3` |

演算子優先順位: `*` `/` > `+` `-`

## ビルド

```bash
cargo build --release
# バイナリ: ./target/release/simplang
```

## 実行

```bash
echo "1 + 2 * 3" > expr.simp
./target/release/simplang expr.simp   # バイナリ ./out を生成
./out                                  # 7

# 出力ファイル名を指定
./target/release/simplang expr.simp -o myprogram

# アセンブリを保存
./target/release/simplang expr.simp -S out.s
```

## テスト

```bash
cargo test               # 全テスト（ユニット + 統合）
cargo test --lib         # ユニットテストのみ
cargo test --test integration_test   # 統合テストのみ
```

## 依存関係

- **実行時**: `gcc` がシステムに必要（アセンブリのコンパイルに使用）
- **Rust クレート**: `clap` v4（CLI）、`tempfile` v3（テスト用）
