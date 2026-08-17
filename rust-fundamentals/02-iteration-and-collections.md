# 2. 反復とコレクション

## 目的

`usize`、スライス、iterator、`Option`を、正常系と空入力のテストから学びます。Rustでは「値がない」ことを`Option`で表すことで、呼び出し側に分岐を要求できます。

## 最初のテスト（Red）

`repeat`、`sum`、`average`のテストを一つずつ追加し、`cargo test average_returns_none_for_empty_input`で空入力の失敗を確認します。

## Green

反復は標準の`str::repeat`、集計は`iter().copied().sum()`で実装します。平均は空スライスなら`None`、それ以外は`Some`とします。

## Refactor

インデックス操作を避けてiteratorへ寄せ、整数の合計と浮動小数点への変換を別々の意図として読めるか確認します。実装は [`src/lib.rs`](../src/lib.rs) の`repeat`、`sum`、`average`です。

## 次の一歩

負数を含む入力、大きな値、平均の丸め方を仕様に追加してください。

```bash
cargo test average_returns_none_for_empty_input
```
