# 11. コマンドラインと構造

## 目的

CLIの引数をアプリケーションの内部状態へ変換します。`std::env::args`を関数の内部へ隠さず、`&[&str]`を受け取る純粋なパーサとしてテストします。

## Red → Green → Refactor

`add <title>`と`list`の成功、空タイトルと未知のコマンドの失敗を定義します。完成実装は [`parse_command`](../src/lib.rs) と [`Command`](../src/lib.rs)です。

次に、引数の取得とドメイン処理を別モジュールへ分け、CLIからTodoサービスへ依存を注入してください。

```bash
cargo test io_sorting_and_cli_are_testable_without_environment
```
