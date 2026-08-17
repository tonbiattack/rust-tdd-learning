# 7. 並行性とキャンセル

## 目的

Goのgoroutine/select/contextに対応する設計課題を、Rustの`std::thread`、`Arc<AtomicBool>`、明示的なイベント型で学びます。外部ランタイムや実時間を使わず、観測可能な契約に絞ります。

## Red → Green → Refactor

まず入力を左右に分けて並列合計するテストを書きます。次に、キャンセル済みなら処理件数が0になるテストを追加します。最後にpanic・キャンセル・共有状態の扱いを検討します。

完成実装は [`parallel_sum`](../src/lib.rs)、[`count_until_cancel`](../src/lib.rs)、[`Event`](../src/lib.rs)です。次はチャネルを注入し、送信側終了とデータ到着の順序をテストしてください。

```bash
cargo test concurrency_and_cancellation_are_observable
```
