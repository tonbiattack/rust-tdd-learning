# 9. Todoサービス

## 目的

追加・完了・未完了一覧を持つ状態オブジェクトを、成功と失敗時の状態までテストします。`Vec<Todo>`と`&mut self`を通して所有権と状態変更の境界を確認します。

## Red → Green → Refactor

空タイトル、2件の追加、0番目の完了、存在しない番号の完了を一つのシナリオにします。失敗後も件数が変わらないことを観測し、`get_mut`と`Result`で最小実装します。

完成実装は [`TodoList`](../src/lib.rs)です。次はインデックスではなくIDを導入し、削除・タイトル変更・永続化を追加してください。

```bash
cargo test todo_list_rejects_empty_and_tracks_completion
```
