## ABC 400 C - 2^a b^2

refs: <https://atcoder.jp/contests/abc400/tasks/abc400_c>

各 $a$ について, $b$ の取りうる最大値を求める形式で解答できる.

$\left\lfloor\sqrt{\frac{X}{2^a}}\right\rfloor$ を正確に求められればかなり高速となるが算出誤差が生じうるため, 除算を含まない二分探索を用いるのが最も簡易に済む.

$a = 1$ のとき, 探索される範囲は $X = 2 \times 1^2$, $X = 2 \times 2^2$, $X = 2 \times 3^2$, ... となる.

$a = 2$ のとき, 探索される範囲は $X = 2^2 \times 1^2$, $X = 2^2 \times 2^2$, $X = 2^2 \times 3^2$, ... となる.

$a = 3$ 以上の奇数のとき, $X = 2^3 \times 1^2 = 2 \times (2 \times 1)^2$, $X = 2^3 \times 2^2 = 2 \times (2 \times 2)^2$, ... と, $a = 1$ の探索範囲に含まれる.

$a = 4$ 以上の偶数のとき, $X = 2^4 \times 1^2 = 2^2 \times (2 \times 1)^2$, $X = 2^4 \times 2^2 = 2^2 \times (2 \times 2)^2$, ... と, $a = 2$ の探索範囲に含まれる.
