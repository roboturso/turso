# Reproducer: #8715 — A correlated subquery over a hash join drops its rowid correlation

https://github.com/tursodatabase/turso/issues/8715

## Environment

- Commit: 7139413538e84324c70bad16bbddec2b9dfb2884
- Platform: linux x86_64
- Reproduced at: 2026-09-22T11:59:53Z

## Reproduce

```sh
cd /home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8715 && cargo build -q --bin tursodb && printf "CREATE TABLE t(a INTEGER PRIMARY KEY, g, n);\nCREATE TABLE u(g);\nCREATE TABLE r(k);\nINSERT INTO t VALUES(1,'p',1),(2,'q',2);\nINSERT INTO u VALUES('p'),('q');\nINSERT INTO r VALUES(1),(2),(3);\nSELECT k, (SELECT count(*) FROM t JOIN u ON u.g=t.g WHERE t.a=r.k) FROM r ORDER BY k;\nDELETE FROM r WHERE (SELECT count(*) FROM t JOIN u ON u.g=t.g WHERE t.a=r.k) = 0;\nSELECT k FROM r ORDER BY k;\n" | ./target/debug/tursodb -q -m list :memory: | diff - <(printf "1|1\n2|1\n3|0\n1\n2\n")
```

Fails on the current tree; passes once the bug is fixed.

## What happens

On current main the correlated subquery returns 1|1, 2|1, 3|0 and the DELETE leaves rows 1 and 2, matching SQLite for every variant in the report (including partial indexes, larger tables, and forced join orders); the planner now keeps the rowid seek on t and uses an automatic index on u, so this appears to have been fixed by the recent hash join optimizer changes.

## Observed failure

RoboTurso ran the command above and observed:

```text
0a1,5
> 1|1
> 2|1
> 3|0
> 1
> 2
```

## Files

- (none)