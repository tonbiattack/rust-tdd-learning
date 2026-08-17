# 12. 時間の注入

## 目的

実時間を直接読むとテストが不安定になるため、`Clock` traitで現在時刻を注入します。Fake Clockを使うと、朝・昼・夜の境界を高速かつ決定的に検証できます。

## Red → Green → Refactor

8時と20時のFake Clockを用意し、挨拶の結果をテストします。次に11時59分、12時、17時59分、18時の境界を追加し、時間帯の仕様を明示します。

完成実装は [`Clock`](../src/lib.rs) と [`greeting_for_clock`](../src/lib.rs)です。次は`Duration`を注入し、期限判定のテストへ発展させてください。

```bash
cargo test time_is_injected
```
