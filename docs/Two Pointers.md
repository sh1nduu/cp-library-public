## Code 1

```text
r = 0
for l = 0..N-1
    while r < N and 区間[l, r+1) が条件OK
        r++
    ans = max(ans, r - l)
```


## Code 2

```text
i = 下限, j = 上限
while i <= j
    if f(i, j) が条件OK
        答え候補を更新
        j--
    else
        i++
```
