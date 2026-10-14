## Code 1

```text
dfs(i, k):
    if i == N:
        k が N で割り切れれば最後の項を決定して出力
        return

    x = 0 .. floor(k / i):
        A_i = x
        dfs(i + 1, k - i*x)
````
