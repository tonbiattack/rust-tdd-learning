# 10. I/Oと並び替え

## 目的

実ファイルを直接テストせず、`Read` traitの境界へ入力を注入します。読み込みとパースの失敗を分離し、レコードのソート規則をテストで固定します。

## Red → Green → Refactor

`"3 1 2"`の成功と`"x"`の失敗をバイト列から検証します。次に得点降順、同点なら名前昇順という安定した規則を追加します。

完成実装は [`read_numbers`](../src/lib.rs) と [`sorted_records`](../src/lib.rs)です。次は`BufRead`へ拡張し、行単位のエラー位置を返してください。

```bash
cargo test io_sorting_and_cli_are_testable_without_environment
```
