## ABC 394 D - Colorful Bracket Sequence

refs: <https://atcoder.jp/contests/abc394/tasks/abc394_d>

Stack を使ってやると, `()`, `[]`, `<>`の部分文字列の削除を簡易に実行できる.
マッチするカッコを随時削除していき, 最終的にすべてのカッコがマッチして消滅していればカラフル括弧列になる.
