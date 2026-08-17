# 4. 依存性注入

## 目的

外部通知のような境界を`Notifier` traitで抽象化し、実ネットワークを使わずにテストします。Rustではtrait boundを使うことで、依存の契約をコンパイル時に確認できます。

## 最初のテスト（Red）

`Spy`をテスト内に定義し、`publish_result`が送ったメッセージを記録するテストを書きます。最初は`Notifier`も関数も存在しないため、コンパイルエラーを観測します。

## Green

`Notifier::send`を1メソッドの契約として定義し、`publish_result`へ`&mut N`を注入します。成功結果の文面だけを検証し、Spyの内部実装はテストの対象にしません。

## Refactor

本番用Notifierを作る場合も、`publish_result`へ標準出力やHTTPクライアントを直接渡さない設計を維持します。実装は [`src/lib.rs`](../src/lib.rs) の`Notifier`と`publish_result`です。

## 次の一歩

失敗通知を扱う`Result`返却へAPIを拡張し、Spyが送信エラーを返せるようにしてください。

```bash
cargo test publishing_uses_injected_notifier
```
