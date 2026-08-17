# 8. enum・同期・不変条件

## 目的

reflectionを無理に再現せず、Rustのenumとmatchで許可された状態を網羅的に扱います。Roman numeralの範囲テストを通して、単一例ではなく不変条件を確認します。

## Red → Green → Refactor

`None`を閉じたイベント、`Plain`と`Uppercase`を形式として表すテストを書きます。続いて1〜3999だけをRoman numeralへ変換し、範囲外は`None`にします。

完成実装は [`Event`](../src/lib.rs)、[`Format`](../src/lib.rs)、[`roman`](../src/lib.rs)です。次は変換結果を逆変換する関数を追加し、`roman(parse(roman(n))) == roman(n)`をテストしてください。

```bash
cargo test enum_boundary_is_exhaustive roman_and_clock_have_invariants
```
