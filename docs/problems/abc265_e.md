## Code 1

```text
HashMap<(x, y)> → 経路数
```


## Code 2

```text
HashSet<(x, y)>
```


## Code 3

```text
for ((x, y), count) in dp:
    for (dx, dy) in dirs:
        next = (x + dx, y + dy)
        if next is not obstacle:
            new[next] += count
```
