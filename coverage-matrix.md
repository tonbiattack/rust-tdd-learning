# Coverage Matrix

| 原典の設計上の主題 | Rust教材の章 | 対応する実装・テスト | 状態 |
|---|---|---|---|
| Hello world、最初のテスト | 1. Hello, Rust | `greeting`、単体テスト | 実装済み |
| iteration、配列・スライス | 2. 反復とコレクション | `repeat`、`sum`、`average` | 実装済み |
| structs、methods、errors | 3. 値オブジェクトとResult | `Score`、`ScoreError` | 実装済み |
| dependency injection、mocking | 4. 依存性注入 | `Notifier`、Spy | 実装済み |
| application iteration、状態管理 | 5. Todoサービス | `TodoList`、失敗時状態 | 実装済み |
| acceptance / integration test | 6. 統合テストと次の課題 | `tests/todo_workflow.rs` | 実装済み |
| concurrency、HTTP、WebSocket、外部I/O | 発展課題 | 未収録 | 未着手 |
| generics、reflection、property-based testing | 発展課題 | 未収録 | 未着手 |

## 解釈

原典の全章をRustの機能へ1対1変換することは目的にしていません。表の「実装済み」は、説明・振る舞いテスト・完成コードが一組で存在する章だけを指します。「未着手」は欠落ではなく、初版の標準ライブラリ中心という範囲を示します。

## 参照

原典の章構成は [Learn Go with Tests][1]、Rustのテストの位置付けは [Rust Book Chapter 11][2]、Cargoの単体・統合テストの配置は [Cargo Testing Guide][3] に基づきます。

[1]: https://github.com/quii/learn-go-with-tests "Learn Go with Tests"
[2]: https://doc.rust-lang.org/book/ch11-00-testing.html "Writing Automated Tests - The Rust Programming Language"
[3]: https://doc.rust-lang.org/cargo/guide/tests.html "Tests - The Cargo Book"
