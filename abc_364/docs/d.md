## ABC 364 D - K-th Nearest

refs: <https://atcoder.jp/contests/abc364/tasks/abc364_d>

点 $B_j$ からの距離が $d$ 以内の中にある点 $A_i$ の個数が $K$ 以上となるような最小の $d$ を求めればよい.
最小の $d$ を求める二分探索と, 座標が $B_j - d$ 以上かつ $B_j + d$ 以下になる点 $A_i$ の個数を数える二分探索を組み合わせることで解くことができる.
`upper_bound`, `lower_bound` が便利.
