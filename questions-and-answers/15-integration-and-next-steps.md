# 15. 統合テストと次の課題

## 目的

`src/lib.rs`内の単体テストと、`tests/`内の統合テストを比較します。統合テストでは利用者が見える公開APIだけを呼び、内部フィールドやアルゴリズムを前提にしません。

## Red → Green → Refactor

`tests/todo_workflow.rs`でTodoを追加・完了し、公開APIだけで未完了一覧を確認します。`cargo test --test todo_workflow`で単独実行し、ライブラリ内部のリファクタリング後も通ることを確認してください。

## 発展課題

指定資料にあるHTTP、JSON、WebSocket、外部プロセス、より本格的なプロパティベーステストは、まずtrait境界とFakeを設計してから追加します。外部依存を増やす場合は、依存の理由とテスト戦略を`DESIGN.md`へ記録してください。

```bash
cargo test --test todo_workflow
cargo test --doc
```
