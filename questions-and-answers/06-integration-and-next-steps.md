# 6. 統合テストと次の課題

## 目的

`src/lib.rs`内の単体テストと、`tests/`内の統合テストを比較します。統合テストでは、利用者が見える公開APIだけを呼び、モジュール内部の実装詳細を前提にしません。

## 最初のテスト（Red）

`tests/todo_workflow.rs`でTodoを追加・完了し、未完了一覧を確認するシナリオを書きます。`cargo test --test todo_workflow`で実行できます。

## GreenとRefactor

公開APIが不足していれば、最小限のメソッドだけを公開します。統合テストが通った後、内部のフィールドやアルゴリズムを変更しても同じ振る舞いが保証されるか確認します。Cargoは`tests/`を統合テストの場所として扱います。[1]

## 次の課題

本教材の未着手範囲であるファイルI/O、HTTP、非同期処理、並行性、プロパティベーステストを、同じRed → Green → Refactorの形式で追加してください。外部サービスを使う場合は、まずtrait境界と手書きFakeを設計し、テストをネットワークから分離します。

```bash
cargo test --test todo_workflow
cargo test --doc
```

[1]: https://doc.rust-lang.org/cargo/guide/tests.html "Tests - The Cargo Book"
