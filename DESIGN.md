# DESIGN

## 方針

指定された日本語版 [Learn Go with Tests][1] の章構成とTDDの導線を参考にしています。ただし、GoのコードをRustへ逐語的に移植せず、同じ設計上の問いをRustの型・所有権・標準ライブラリで再表現します。外部依存を増やさず、`cargo test`一回で全テストが実行できることを優先しました。

| Go教材の主題 | Rustでの選択 | 置換理由 |
|---|---|---|
| Hello world、整数、反復 | `String`、`i32`、iterator | 借用・所有、純粋関数、境界値を最初に学ぶ |
| 配列・スライス、マップ | `&[T]`、`Vec<T>`、`HashMap` | 所有権を奪わない入力とコレクションAPIを明示する |
| ポインタ・エラー | 非公開struct、`Result<T,E>`、enum | 不変条件を生成時に保証し、失敗理由を型で表す |
| インターフェース、DI、mock | trait、手書きSpy・Fake | 動的モックに頼らず、契約と依存注入を見える化する |
| goroutine、select、context | `std::thread`、イベントenum、`Arc<AtomicBool>` | 並行性とキャンセルを標準ライブラリで決定的に観測する |
| reflection | enumと網羅的match | 実行時型検査ではなく、Rustの型安全な状態表現を学ぶ |
| sync | 不変条件と共有状態の境界 | mutexのAPI暗記より、状態遷移とデータ競合の設計を優先する |
| property-based testing | Roman numeralの範囲・出力不変条件 | 初版は外部依存なしで、性質を複数境界値として固定する |
| HTTP、JSON、WebSocket | 初版では未収録 | 標準ライブラリのみの範囲を維持し、trait境界を先に設計する |
| I/O、CLI、時間、数学 | `Read`、引数スライス、Clock trait、SVG文字列 | 実環境・実時間・ブラウザからテストを分離する |

## TDDサイクル

各章は「最初のテスト」「Redで観測すること」「Greenの最小方針」「Refactorの観点」「次の振る舞い」を記録します。完成版ではすべてのテストが通りますが、学習者は関数とテストを一度消してサイクルを再現できます。

## 範囲外と次の拡張

HTTPサーバー、JSON、WebSocket、外部プロセス、async runtime、外部crateを使う本格的なproperty-based testingは未収録です。次の拡張では、まずWire-likeなtrait境界とFakeを追加し、統合テストでプロトコルを検証します。

[1]: https://andmorefine.gitbook.io/learn-go-with-tests "テスト駆動開発でGO言語を学びましょう"
