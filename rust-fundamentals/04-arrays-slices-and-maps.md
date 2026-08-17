# 4. 配列・スライス・マップ

## 目的

`&[i32]`を受け取ることで呼び出し側のコレクションを借用し、結果だけを`Vec<i32>`として所有する設計を学びます。続いて`HashMap`で単語頻度を集計します。

## Red → Green → Refactor

しきい値より大きい値だけを残すテストと、大文字小文字を正規化した頻度テストを書きます。空入力、重複、順序を確認し、iteratorと`entry` APIへリファクタリングします。

完成実装は [`above`](../src/lib.rs) と [`word_counts`](../src/lib.rs)です。次は、並び順が必要な集計結果を`BTreeMap`で返す仕様を追加してください。

```bash
cargo test collections_keep_behavior_explicit
```
