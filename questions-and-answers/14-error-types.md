# 14. エラー型と公開境界

## 目的

文字列エラーから構造化されたenumエラーへ進み、呼び出し側が失敗理由を分岐できるAPIを設計します。失敗時に状態を変更しないことも振る舞いとして固定します。

## Red → Green → Refactor

Walletの残高以上の引き出し、Scoreの範囲外、数値I/Oの不正入力をテストします。エラーの値と、失敗後の状態の両方を検証してください。

完成実装は [`WalletError`](../src/lib.rs)、[`ScoreError`](../src/lib.rs)、[`read_numbers`](../src/lib.rs)です。次はエラーに入力位置や元エラーを含め、利用者が回復方法を選べる設計にします。

```bash
cargo test wallet_preserves_state_after_failed_withdrawal
```
