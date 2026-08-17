# 6. 依存性注入とテストダブル

## 目的

通知や時計のような外部境界をtraitで表し、テストではSpyやFakeを注入します。Rustではtrait boundが依存の契約をコンパイル時に検査します。

## Red → Green → Refactor

通知されたメッセージを記録するSpyをテスト内に作り、`publish_result`へ渡します。次に`Clock`をFake実装し、実時間に依存しない朝・昼・夜のテストを追加します。

完成実装は [`Notifier`](../src/lib.rs)、[`publish_result`](../src/lib.rs)、[`Clock`](../src/lib.rs)です。次は送信失敗を`Result`で表現し、呼び出し側がリトライ方針を選べるようにしてください。

```bash
cargo test publishing_uses_injected_notifier time_is_injected
```
