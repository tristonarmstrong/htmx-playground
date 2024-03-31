# How to squash recent (n) commits

## TL;DR
```bash
#change this number -👇🏻- to be the number of commits you want to sqaush starting from top most recent
git reset --soft HEAD~2 && git commit --edit -m"$(git log --format=%B --reverse HEAD..HEAD@{1})"
```

## In Depth breakdown
### git reset
explain this
```bash
git reset --soft
```

### head
explain this
```bash
HEAD~2
```

### commit edit
explain this
```bash
git commit --edit -m
```

### inline command
explain this
```bash
"$(...stuff...)"
```

### git log
explain this
```bash
git log --format=%B --reverse
```

### head stuff
explain this
```bash
HEAD..HEAD@{1}
```
