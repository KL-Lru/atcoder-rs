## ABC 399 C - Make it Forest

refs: <https://atcoder.jp/contests/abc399/tasks/abc399_c>

与えられたグラフが森となる = 各連結成分の辺の数が, ちょうど頂点の数 - 1 となる, と読み替えて良い.

連結している頂点がどこかだけ識別できれば良いため, Union Find でさくっと対応が効く.


