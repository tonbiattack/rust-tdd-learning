# Rustをテストから学ぶ

**Rustを学びながら、Red → Green → Refactorの小さなサイクルを実践する日本語教材**です。指定された日本語版 [Learn Go with Tests][1] の「基礎をテストで探索し、その後にアプリケーションを段階的に拡張する」構成を参考に、Goのコードを翻訳するのではなく、設計概念をRust標準ライブラリへ再表現しています。

各章は、最初に振る舞いを表すテストを書き、コンパイラやテストの失敗を観測し、最小実装で通し、最後にRustらしい所有権・`Result`・trait・iteratorへ整理します。テストは正しさの検証だけでなく、学習した機能の実行可能なドキュメントです。[2]

## 対象と開始方法

プログラミングの基本概念とターミナル操作を理解している人を対象にします。Rust **1.75以上**を用意し、リポジトリのルートで次を実行してください。

```bash
cargo test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo doc --no-deps --open
```

Cargoはソース内の単体テストと`tests/`内の統合テストを実行します。[3] 完成実装は [`src/lib.rs`](src/lib.rs)、利用者視点の統合テストは [`tests/todo_workflow.rs`](tests/todo_workflow.rs)にあります。

## 学習の進め方

章の「最初のテスト」を自分で追加し、意図的にRedを確認してください。次に、エラーメッセージを設計情報として読み、Greenとなる最小のAPIを実装します。最後に、重複の除去、所有権の境界、エラー型、依存性の注入を見直します。テストを先に書くことで、Rustの文法を断片的に暗記するのではなく、振る舞いを実現するために必要な機能として学べます。

## 目次

| 区分 | 章 | 主な概念 | 状態 |
|---|---|---|---|
| 基礎 | 1. Hello, Rust | 関数、`String`、境界値 | 実装済み |
| 基礎 | 2. 整数と純粋関数 | 型、算術、テストの最小単位 | 実装済み |
| 基礎 | 3. 反復とコレクション | `for`、iterator、`Option` | 実装済み |
| 基礎 | 4. 配列・スライス・マップ | `Vec`、slice、`HashMap` | 実装済み |
| 基礎 | 5. 値オブジェクトとResult | struct、enum、不変条件 | 実装済み |
| 基礎 | 6. 依存性注入とテストダブル | trait、Spy、Fake | 実装済み |
| 基礎 | 7. 並行性とキャンセル | thread、atomic、キャンセル | 実装済み |
| 基礎 | 8. enum・同期・不変条件 | 状態の表現、Roman numeral | 実装済み |
| アプリ | 9. Todoサービス | 状態遷移、失敗時の状態 | 実装済み |
| アプリ | 10. I/Oと並び替え | `Read`注入、sort | 実装済み |
| アプリ | 11. コマンドラインと構造 | 引数解析、境界 | 実装済み |
| アプリ | 12. 時間の注入 | Clock trait、決定性 | 実装済み |
| アプリ | 13. 数学とSVG | 純粋な描画関数 | 実装済み |
| 補足 | 14. エラー型と公開境界 | エラー契約 | 実装済み |
| 補足 | 15. 統合テストと次の課題 | 公開API、統合テスト | 実装済み |

章の詳細は [`SUMMARY.md`](SUMMARY.md)、Rust固有の置換判断は [`DESIGN.md`](DESIGN.md)、指定資料との比較は [`research/andmorefine-findings.md`](research/andmorefine-findings.md)、実装範囲は [`coverage-matrix.md`](coverage-matrix.md)にまとめています。

## 設計上の対応

Goのgoroutine/selectは`std::thread`とメッセージのenum、contextは`Arc<AtomicBool>`によるキャンセル契約、mockingは手書きSpyとtrait、reflectionは型安全なenum、timeはFake Clock trait、I/Oは`Read`境界へ置き換えました。外部依存を増やさず、テストをネットワークや実時間から分離することを優先しています。

## ライセンスと帰属

本リポジトリの新規コードと日本語文書はMIT Licenseで提供します。設計上の着想を得た資料は、[日本語版リポジトリ][4]および原典 [quii/learn-go-with-tests][5]です。原典の文章・コードを大量に複製せず、章名と学習概念を参照してRust向けに新規作成しています。

## References

[1]: https://andmorefine.gitbook.io/learn-go-with-tests "テスト駆動開発でGO言語を学びましょう"
[2]: https://doc.rust-lang.org/book/ch11-00-testing.html "Writing Automated Tests - The Rust Programming Language"
[3]: https://doc.rust-lang.org/cargo/guide/tests.html "Tests - The Cargo Book"
[4]: https://github.com/andmorefine/learn-go-with-tests "andmorefine/learn-go-with-tests"
[5]: https://github.com/quii/learn-go-with-tests "quii/learn-go-with-tests"
