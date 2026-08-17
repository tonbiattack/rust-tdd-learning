# Coverage Matrix

指定資料の章構成を、Rustで学ぶべき設計概念へ対応付けています。原典のコードを複製せず、各章に説明・振る舞いテスト・完成コードを用意しています。

| 指定資料の主題 | Rust版の章 | 実装・テスト | 状態 |
|---|---|---|---|
| Hello World | 1. Hello, Rust | `greeting` | 実装済み |
| 整数 | 2. 整数と純粋関数 | `add_two` | 実装済み |
| 反復、配列、スライス | 3-4. コレクション | `repeat`、`above`、`sum` | 実装済み |
| マップ | 4. 配列・スライス・マップ | `word_counts` | 実装済み |
| 構造体、メソッド、エラー | 5. 値オブジェクトとResult | `Score`、`Wallet` | 実装済み |
| DI、スタブ、モック | 6. DIとテストダブル | `Notifier`、Spy、Clock | 実装済み |
| 並行性、選択、コンテキスト | 7. 並行性とキャンセル | thread、event、atomic | 実装済み |
| リフレクション、同期、property-based testing | 8. enum・同期・不変条件 | enum、Roman numeral | 部分実装 |
| アプリケーション状態管理 | 9. Todoサービス | `TodoList` | 実装済み |
| I/O、並び替え | 10. I/Oと並び替え | `read_numbers`、`sorted_records` | 実装済み |
| CLI、パッケージ構造 | 11. コマンドラインと構造 | `parse_command` | 部分実装 |
| 時間 | 12. 時間の注入 | `Clock`、Fake Clock | 実装済み |
| 数学 | 13. 数学とSVG | `render_clock_svg` | 実装済み |
| エラー型、統合テスト | 14-15. 補足 | `WalletError`、統合テスト | 実装済み |
| HTTP、JSON、WebSocket | 発展課題 | 外部crateを使うサーバー | 未着手 |
| OS実行、Context-aware Reader | 発展課題 | 外部境界の追加設計 | 未着手 |

「実装済み」は章ガイド・振る舞いテスト・完成実装が揃った章だけを指します。「部分実装」は概念をRust標準ライブラリで再表現したものの、原典の全ての応用範囲を収録していない章です。

## 参照

- [日本語版 Learn Go with Tests][1]
- [日本語版 GitHub リポジトリ][2]
- [原典リポジトリ][3]
- [Rustの自動テスト][4]

[1]: https://andmorefine.gitbook.io/learn-go-with-tests "テスト駆動開発でGO言語を学びましょう"
[2]: https://github.com/andmorefine/learn-go-with-tests "andmorefine/learn-go-with-tests"
[3]: https://github.com/quii/learn-go-with-tests "quii/learn-go-with-tests"
[4]: https://doc.rust-lang.org/book/ch11-00-testing.html "Writing Automated Tests - The Rust Programming Language"
