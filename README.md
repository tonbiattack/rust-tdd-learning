# Rustをテストから学ぶ

**Rustを学びながら、Red → Green → Refactor の小さなサイクルを実践する日本語教材**です。各章は、最初に振る舞いを表すテストを書き、最小実装で通し、その後にRustらしい所有権・`Result`・trait・コレクションへ整理する構成です。

この教材は [Learn Go with Tests][1] の「テストを書きながら言語の基礎を学び、最後にアプリケーションへ進む」という学習設計に着想を得ています。ただし、文章・コード・テストはRust向けに新規作成しており、原典のコードや文章を翻訳したものではありません。Rustの標準テスト機構については [The Rust Programming Language][2] と [The Cargo Book][3] を参照しています。

## 対象と開始方法

プログラミングの基本概念とターミナル操作を理解している人を対象にします。Rust **1.75以上**を用意し、リポジトリのルートで次を実行してください。

```bash
cargo test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo doc --no-deps --open
```

`src/lib.rs`には各章の完成コードと単体テストがあり、`tests/`には公開APIを使う統合テストがあります。Cargoはソース内の単体テストと`tests/`内の統合テストを実行できます。[3]

## 学習の進め方

各章のガイドにある「最初のテスト」を先に自分で書き、意図的に失敗させます。次に、コンパイラのエラーとテストの失敗を観測し、通る最小実装を作ります。最後に完成コードと比較し、命名・所有権・エラー境界をリファクタリングしてください。Rustの型検査は重要な誤りを捕捉しますが、意図した振る舞いそのものはテストで確認する必要があります。[2]

## 目次

章の順序と実装状態は [`SUMMARY.md`](SUMMARY.md)、原典との対応と範囲は [`coverage-matrix.md`](coverage-matrix.md)、Rust固有の置換判断は [`DESIGN.md`](DESIGN.md)にまとめています。

| 区分 | 章 | 主な概念 | 状態 |
|---|---|---|---|
| 基礎 | [1. Hello, Rust](rust-fundamentals/01-hello-rust.md) | 関数、`String`、境界値 | 実装済み |
| 基礎 | [2. 反復とコレクション](rust-fundamentals/02-iteration-and-collections.md) | スライス、iterator、`Option` | 実装済み |
| 基礎 | [3. 値オブジェクトとResult](rust-fundamentals/03-value-objects-and-result.md) | struct、enum、不変条件 | 実装済み |
| アプリ | [4. 依存性注入](build-an-application/04-dependency-injection.md) | trait、テストダブル、借用 | 実装済み |
| アプリ | [5. Todoサービス](build-an-application/05-todo-service.md) | 状態、`Vec`、失敗時の状態 | 実装済み |
| 補足 | [6. 統合テストと次の課題](questions-and-answers/06-integration-and-next-steps.md) | 公開API、統合テスト、拡張 | 実装済み |

## ライセンスと帰属

本リポジトリの新規コードと日本語文書はMIT Licenseで提供します。設計上の着想を得た原典は [quii/learn-go-with-tests][1] です。原典のライセンスと最新の章構成は、利用時点で必ず原典を確認してください。

## References

[1]: https://github.com/quii/learn-go-with-tests "quii/learn-go-with-tests"
[2]: https://doc.rust-lang.org/book/ch11-00-testing.html "Writing Automated Tests - The Rust Programming Language"
[3]: https://doc.rust-lang.org/cargo/guide/tests.html "Tests - The Cargo Book"
