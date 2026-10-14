## Code 1

```text
0, 1, 2, ..., floor(N/2), ..., 2, 1
```


## Code 2

```text
zero_i で add を +1
zero_i で sub を -1
```


## Code 3

```text
if N is even:
  max_i で反転
else:
  max_i と max_i + 1 に分けて反転
```


## Code 4

```text
ans = score

for x in 1..N:
  score = score + add - sub
  ans = min(ans, score)

  add += add_diff[x]
  sub += sub_diff[x]
```
