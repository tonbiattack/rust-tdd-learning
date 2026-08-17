# 1. Hello, Rust

## 目的

関数、`&str`、`String`、`match`をテストから学びます。入力が空白だけの場合に既定の挨拶を返すという、小さな境界値を最初から振る舞いとして定義します。

## 最初のテスト（Red）

`src/lib.rs`の`tests::greeting_uses_name_and_default`を先に書き、`greeting`が未定義の状態で`cargo test greeting_uses_name_and_default`を実行します。コンパイルエラーは、次に必要な関数のシグネチャを教えてくれます。

## Green

`&str`を受け取り、`String`を返す関数を作ります。名前が空なら`Hello, Rust!`、それ以外なら名前を埋め込んだ文字列を返すだけにします。

## Refactor

`trim`をどの境界で適用するかを確認し、返却値が所有権を持つ理由を説明できるようにします。実装は [`src/lib.rs`](../src/lib.rs) の`greeting`です。

## 次の一歩

改行を含む入力、名前に記号を含む入力を追加し、仕様として許容する文字列境界を考えてください。

```bash
cargo test greeting_uses_name_and_default
```
