# simplang-rust

## 使用言語

Rust

## 実装方針

コンパイルパイプラインを段階ごとに独立した関数・モジュールに分割する。

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

## 依存関係

- **実行時**: `gcc` がシステムに必要（アセンブリのコンパイルに使用）
- **Rust クレート**: `clap` v4（CLI）、`tempfile` v3（テスト用）

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

## アプリケーション仕様

各ステップのアプリケーション仕様は `docs/stepNNN.md` を参照。現時点の最新仕様: [docs/step000.md](docs/step000.md)
