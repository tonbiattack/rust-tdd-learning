# 5. 値オブジェクトとResult

## 目的

`struct`で不変条件を閉じ込め、`Result<T, E>`で不正入力を明示的に扱います。Scoreは0〜100だけを受け付け、60以上を合格とします。

## Red → Green → Refactor

100、59、101のケースをテストします。生成失敗をenumで表し、`Score`の内部値を非公開にした後、`passed`というドメイン語彙へリファクタリングします。

完成実装は [`Score`](../src/lib.rs) と [`ScoreError`](../src/lib.rs)です。Walletのような別の不変条件へ一般化してください。

```bash
cargo test score_enforces_range_and_pass_mark
```
