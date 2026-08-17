# 3. 反復とコレクション

## 目的

`for`、iterator、スライス、`Option`を、正常系と空入力のテストから学びます。値が存在しない状態を`None`で返し、呼び出し側に分岐を要求します。

## Red → Green → Refactor

`repeat`の0回、`sum`の空入力、`average`の空入力を順にテストします。最小実装を通した後、インデックス操作をiteratorへ寄せます。

完成実装は [`repeat`](../src/lib.rs)、[`sum`](../src/lib.rs)、[`average`](../src/lib.rs)です。次は遅延iteratorを返すAPIと、ベンチマーク可能な集計を比較してください。

```bash
cargo test repeat_and_average_handle_boundaries
```
