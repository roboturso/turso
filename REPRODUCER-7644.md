# Reproducer: #7644 — ALTER TABLE RENAME COLUMN fails with "trigger table not found" when a TEMP trigger exists on the table

https://github.com/tursodatabase/turso/issues/7644

## Environment

- Commit: d4d6777a2685ab9a3ffc8733329743fdfe000c45
- Platform: linux x86_64
- Reproduced at: 2026-06-26T18:01:01Z

## Reproduce

```sh
cd testing/sqltests && cargo run -q --bin test-runner -- run tests --backend rust --snapshot-filter __never__ tests/alter-rename-column-temp-trigger.sqltest
```

Fails on the current tree; passes once the bug is fixed.

## What happens

A TEMP trigger on a main-database table makes ALTER TABLE ... RENAME COLUMN fail with 'trigger table not found' because the trigger-rewrite resolves the owning table in the trigger's temp database instead of the database where the table actually lives.

## Observed failure

RoboTurso ran the command above and observed:

```text
Generating integrity-check fixtures...
Generating default databases...
[1mtests/affinity.sqltest[0m
  [[32mPASS[0m] affinity-compound-subquery-three-arms-mixed [2m(2.74ms)[0m
  [[32mPASS[0m] affinity-compound-subquery-text-col-plus-numeric-literal [2m(5.44ms)[0m
  [[32mPASS[0m] affinity-compound-subquery-numeric-literal-first-then-col [2m(7.21ms)[0m
  [[32mPASS[0m] affinity-compound-subquery-numeric-col-plus-numeric-literal [2m(23.90ms)[0m
  [[32mPASS[0m] affinity-compound-subquery-all-numeric   [2m(2.47ms)[0m
  [[32mPASS[0m] affinity-compound-subquery-all-text      [2m(2.40ms)[0m
  [[32mPASS[0m] affinity-compound-subquery-order-independent [2m(1.96ms)[0m
  [[32mPASS[0m] affinity-compound-subquery-text-numeric-no-affinity [2m(2.20ms)[0m
  [[32mPASS[0m] cast-text-to-numeric-integral-float      [2m(3.11ms)[0m
  [[32mPASS[0m] affinity-any-strict                      [2m(26.36ms)[0m
  [[32mPASS[0m] affinity-any-non-strict                  [2m(11.89ms)[0m
  [[32mPASS[0m] affinity-real-mixed-signs                [2m(9.28ms)[0m
  [[32mPASS[0m] affinity-real-leading-plus-sign          [2m(12.07ms)[0m
  [[32mPASS[0m] affinity-ascii-whitespace-1_1            [2m(3.93ms)[0m
  [[32mPASS[0m] affinity-original-issue-2                [2m(4.95ms)[0m
  [[32mPASS[0m] affinity-mixed-columns-update-2          [2m(31.06ms)[0m
  [[32mPASS[0m] affinity-in-integer-column-2             [2m(15.86ms)[0m
  [[32mPASS[0m] affinity-in-text-column-with-index-2     [2m(7.88ms)[0m
  [[32mPASS[0m] affinity-in-text-column-2                [2m(13.04ms)[0m
  [[32mPASS[0m] affinity-upsert-text-2                   [2m(9.94ms)[0m
  [[32mPASS[0m] affinity-update-text-with-index-2        [2m(25.27ms)[0m
  [[32mPASS[0m] affinity-insert-text-with-index-2        [2m(18.25ms)[0m
  [[32mPASS[0m] affinity-insert-text-from-integer-2      [2m(9.67ms)[0m
  [[32mPASS[0m] affinity-insert-text-from-real-2         [2m(4.23ms)[0m
  [[32mPASS[0m] affinity-original-issue                  [2m(13.61ms)[0m
  [[32mPASS[0m] affinity-mixed-columns-update            [2m(37.66ms)[0m
  [[32mPASS[0m] affinity-mixed-columns-insert            [2m(5.16ms)[0m
  [[32mPASS[0m] affinity-scalar-subquery-text-delete-predicate [2m(34.72ms)[0m
  [[32mPASS[0m] affinity-scalar-subquery-text-comparison [2m(12.72ms)[0m
  [[32mPASS[0m] affinity-in-subquery-union-text-rhs      [2m(30.43ms)[0m
  [[32mPASS[0m] affinity-in-subquery-text-lhs-integer-rhs [2m(9.39ms)[0m
  [[32mPASS[0m] affinity-in-subquery-group-concat        [2m(29.97ms)[0m
  [[32mPASS[0m] affinity-in-subquery-cast-text           [2m(21.28ms)[0m
  [[32mPASS[0m] affinity-in-integer-column               [2m(26.43ms)[0m
  [[32mPASS[0m] affinity-in-text-column-with-index       [2m(15.18ms)[0m
  [[32mPASS[0m] affinity-in-text-column                  [2m(34.65ms)[0m
  [[32mPASS[0m] affinity-real-non-numeric-text           [2m(21.91ms)[0m
  [[32mPASS[0m] affinity-upsert-text                     [2m(26.13ms)[0m
  [[32mPASS[0m] affinity-update-text-with-index          [2m(17.20ms)[0m
  [[32mPASS[0m] affinity-update-text-from-real           [2m(30.92ms)[0m
  [[32mPASS[0m] affinity-insert-text-with-index          [2m(11.82ms)[0m
  [[32mPASS[0m] affinity-insert-text-from-integer        [2m(27.16ms)[0m
  [[32mPASS[0m] affinity-insert-text-from-real           [2m(4.74ms)[0m
  [[32mPASS[0m] affinity-join-blob-vs-text-column        [2m(37.09ms)[0m
  [[32mPASS[0m] affinity-rowid                           [2m(30.54ms)[0m
  [[32mPASS[0m] affinity-nan-inf-integer                 [2m(30.36ms)[0m
  [[32mPASS[0m] affinity                                 [2m(26.41ms)[0m

[1mtests/agg-functions/avg-large-integers.sqltest[0m
  [[32mPASS[0m] avg_infinite_cancellation_returns_null   [2m(2.64ms)[0m
  [[32mPASS[0m] avg_large_integers_grouped               [2m(2.14ms)[0m
  [[32mPASS[0m] avg_large_integer_mixed_with_real        [2m(2.11ms)[0m
  [[32mPASS[0m] avg_i64_edge_pair_negative               [2m(2.60ms)[0m
  [[32mPASS[0m] avg_i64_edge_pair_positive               [2m(1.88ms)[0m
  [[32mPASS[0m] avg_large_integers_ignore_nulls          [2m(2.04ms)[0m
  [[32mPASS[0m] avg_large_integers_rounds_like_sqlite    [2m(9.39ms)[0m

[1mtests/agg-functions/avg-text-types.sqltest[0m
  [[32mPASS[0m] avg_partial_integer_text_grouped         [2m(2.68ms)[0m
  [[32mPASS[0m] avg_partial_integer_text_rounds_like_sqlite [2m(3.22ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] jsonb-group-object-empty                 [2m(698.26µs)[0m
  [[32mPASS[0m] select-agg-unary-negative                [2m(46.59ms)[0m
  [[32mPASS[0m] select-string-agg-with-column-delimiter-group-by [2m(19.12ms)[0m
  [[32mPASS[0m] select-count-constant-false              [2m(604.77µs)[0m
  [[32mPASS[0m] select-count-constant-true               [2m(34.61ms)[0m
  [[32mPASS[0m] select-avg                               [2m(47.31ms)[0m
  [[32mPASS[0m] json_group_array_arrow_operator_repro    [2m(498.08µs)[0m
  [[32mPASS[0m] json_group_array_extract_repro           [2m(553.71µs)[0m
  [[32mPASS[0m] format-19029102                          [2m(356.86µs)[0m
  [[32mPASS[0m] printf-19029102                          [2m(406.36µs)[0m
  [[32mPASS[0m] select-nested-agg-func-in-expression     [2m(28.40ms)[0m
  [[32mPASS[0m] select-string-agg-with-column-delimiter-group-by [2m(989.00µs)[0m
  [[32mPASS[0m] select-string-agg-with-column-delimiter  [2m(693.20µs)[0m
  [[32mPASS[0m] select-string-agg-with-delimiter-expression [2m(727.14µs)[0m
  [[32mPASS[0m] select-string-agg-with-delimiter         [2m(686.14µs)[0m
  [[32mPASS[0m] select-group-concat-with-column-delimiter-group-by [2m(1.01ms)[0m
  [[32mPASS[0m] select-group-concat-with-column-delimiter [2m(684.12µs)[0m
  [[32mPASS[0m] select-group-concat-with-delimiter-expression [2m(785.23µs)[0m
  [[32mPASS[0m] select-group-concat-with-delimiter       [2m(1.56ms)[0m
  [[32mPASS[0m] select-group-concat                      [2m(29.74ms)[0m
  [[32mPASS[0m] select-avg                               [2m(53.54ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] filter-with-having                       [2m(5.41ms)[0m
  [[32mPASS[0m] filter-null-in-condition                 [2m(9.04ms)[0m
  [[32mPASS[0m] filter-on-empty-table                    [2m(8.27ms)[0m
  [[32mPASS[0m] filter-all-rows-filtered-out             [2m(11.95ms)[0m
  [[32mPASS[0m] filter-multiple-different-conditions     [2m(12.26ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter_table_drop_column_with_single_quote_in_name [2m(20.02ms)[0m
  [[32mPASS[0m] alter_table_add_column_with_single_quote_in_name [2m(24.26ms)[0m
  [[32mPASS[0m] alter-table-rename-column-view-mutual-cycle [2m(14.63ms)[0m
  [[32mPASS[0m] alter-table-rename-column-view-self-cycle [2m(8.37ms)[0m
  [[32mPASS[0m] alter-table-rename-column-view-tablecall-branch [2m(21.44ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-rename-table-updates-sequence-entry [2m(13.03ms)[0m
  [[32mPASS[0m] autoinc-drop-mixed-case-table-removes-sequence-entry [2m(11.36ms)[0m
  [[32mPASS[0m] autoinc-drop-last-table-empties-sequence [2m(12.67ms)[0m

[1mtests/char.sqltest[0m
  [[32mPASS[0m] char-null-in-middle                      [2m(2.51ms)[0m
  [[32mPASS[0m] char-null-produces-nul                   [2m(2.56ms)[0m
  [[32mPASS[0m] char-over-max-replacement                [2m(2.73ms)[0m
  [[32mPASS[0m] char-max-codepoint                       [2m(2.74ms)[0m
  [[32mPASS[0m] char-negative-replacement                [2m(2.86ms)[0m
  [[32mPASS[0m] char-nul                                 [2m(2.87ms)[0m
  [[32mPASS[0m] char-unicode-emoji                       [2m(2.80ms)[0m
  [[32mPASS[0m] char-unicode-mixed                       [2m(2.82ms)[0m
  [[32mPASS[0m] char-unicode-cjk                         [2m(2.88ms)[0m
  [[32mPASS[0m] char-mixed-case                          [2m(2.85ms)[0m
  [[32mPASS[0m] char-basic-ascii                         [2m(2.79ms)[0m
  [[32mPASS[0m] char-unicode-256                         [2m(2.79ms)[0m
  [[32mPASS[0m] char-non-integer                         [2m(2.76ms)[0m
  [[32mPASS[0m] char-null                                [2m(2.76ms)[0m
  [[32mPASS[0m] char-empty                               [2m(2.72ms)[0m
  [[32mPASS[0m] char-nested                              [2m(2.83ms)[0m
  [[32mPASS[0m] char                                     [2m(2.79ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_valid_function          [2m(8.58ms)[0m
  [[32mPASS[0m] check_constraint_alter_add_column_aggregate [2m(14.97ms)[0m
  [[32mPASS[0m] check_constraint_alter_add_column_nonexistent_column [2m(5.58ms)[0m
  [[32mPASS[0m] check_constraint_subquery                [2m(2.51ms)[0m
  [[32mPASS[0m] check_constraint_bind_parameter          [2m(2.51ms)[0m
  [[32mPASS[0m] check_constraint_aggregate_function_sum  [2m(2.52ms)[0m
  [[32mPASS[0m] check_constraint_aggregate_function      [2m(2.52ms)[0m
  [[32mPASS[0m] check_constraint_nonexistent_function    [2m(2.52ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_group_by_implicit_rtrim          [2m(23.96ms)[0m
  [[32mPASS[0m] collate_group_by_implicit_nocase         [2m(5.05ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-lte-null-null                    [2m(3.34ms)[0m
  [[32mPASS[0m] compare-lte-null-text                    [2m(3.13ms)[0m
  [[32mPASS[0m] compare-lte-null-float                   [2m(2.76ms)[0m
  [[32mPASS[0m] compare-lte-null-int                     [2m(3.01ms)[0m
  [[32mPASS[0m] compare-lte-text-null                    [2m(2.77ms)[0m
  [[32mPASS[0m] compare-lte-text-text-3                  [2m(2.89ms)[0m
  [[32mPASS[0m] compare-lte-text-text-2                  [2m(2.86ms)[0m
  [[32mPASS[0m] compare-lte-text-text-1                  [2m(2.83ms)[0m
  [[32mPASS[0m] compare-lte-float-null                   [2m(2.83ms)[0m
  [[32mPASS[0m] compare-lte-float-float-3                [2m(5.93ms)[0m
  [[32mPASS[0m] compare-lte-float-float-2                [2m(5.79ms)[0m

[1mtests/correlated-subquery-aggregate-groupby.sqltest[0m
  [[32mPASS[0m] correlated-subquery-in-min-max-group-by  [2m(7.56ms)[0m

[1mtests/correlated-subquery-hash-join.sqltest[0m
  [[32mPASS[0m] correlated-subquery-with-join-no-order   [2m(12.89ms)[0m
  [[32mPASS[0m] correlated-subquery-with-join-basic      [2m(8.66ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-mixed-types                   [2m(19.95ms)[0m
  [[32mPASS[0m] cross-join-text-values                   [2m(14.68ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-unreferenced-column-mismatch-delete  [2m(16.98ms)[0m
  [[32mPASS[0m] cte-explicit-columns-chain               [2m(3.13ms)[0m
  [[32mPASS[0m] cte-explicit-columns-multi               [2m(2.97ms)[0m
  [[32mPASS[0m] cte-explicit-columns-expr                [2m(2.89ms)[0m
  [[32mPASS[0m] cte-explicit-columns-rename              [2m(2.95ms)[0m

[1mtests/cte_expressions.sqltest[0m
  [[32mPASS[0m] cte-cross-join-values-duplicate-keys     [2m(6.27ms)[0m
  [[32mPASS[0m] cte-join-values-duplicate-keys-with-data [2m(7.33ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-is-int-int-1                     [2m(2.99ms)[0m
  [[32mPASS[0m] compare-lte-float-float-1                [2m(2.80ms)[0m

[1mtests/char.sqltest[0m
  [[32mPASS[0m] char-multiple-nulls                      [2m(2.96ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-max                               [2m(47.21ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] filter-same-agg-different-filters        [2m(14.32ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-unreferenced-column-mismatch-update  [2m(14.74ms)[0m

[1mtests/default_value.sqltest[0m
  [[32mPASS[0m] default-value-text                       [2m(29.39ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-is-int-int-2                     [2m(4.83ms)[0m

[1mtests/affinity.sqltest[0m
  [[32mPASS[0m] affinity-mixed-columns-insert-2          [2m(19.23ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-alter-column-clears-sequence-row [2m(18.64ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-update-child-noop-ok                  [2m(18.04ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-expressions                   [2m(12.75ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_ipk_update_violation    [2m(12.22ms)[0m

[1mtests/affinity.sqltest[0m
  [[32mPASS[0m] affinity-update-text-from-real-2         [2m(11.80ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_group_by_explicit_override       [2m(22.05ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-agg-unary-negative                [2m(52.71ms)[0m

[1mtests/correlated-subquery-hash-join.sqltest[0m
  [[32mPASS[0m] correlated-subquery-with-join-missing-rows [2m(20.38ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter_table_add_column_with_space_in_name_keeps_quoted_schema_sql [2m(14.27ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-is-float-float-1                 [2m(3.87ms)[0m
  [[32mPASS[0m] compare-is-float-float-2                 [2m(2.68ms)[0m
  [[32mPASS[0m] compare-is-text-text-1                   [2m(1.89ms)[0m

[1mtests/default_value.sqltest[0m
  [[32mPASS[0m] default-value-integer                    [2m(10.47ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-unreferenced-column-mismatch-select  [2m(12.17ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-with-aliases                  [2m(8.91ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-delete-parent-composite-scan          [2m(9.24ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_arithmetic_multiply               [2m(9.33ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-is-text-text-2                   [2m(2.24ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-fail-on-max-rowid                [2m(10.59ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_column_level_basic      [2m(15.44ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] filter-with-expression-condition         [2m(14.95ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-cascade-delete-self-referential-long-chain [2m(12.35ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-trigger-fires-after-rename [2m(10.03ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-is-not-int-int-1                 [2m(2.63ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_ipk_update_valid        [2m(13.04ms)[0m

[1mtests/default_value.sqltest[0m
  [[32mPASS[0m] default-value-real                       [2m(6.50ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-unreferenced-column-mismatch-multi-cte [2m(6.26ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-is-not-int-int-2                 [2m(2.66ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-update-child-to-existing-ok           [2m(6.82ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-is-not-float-float-1             [2m(1.89ms)[0m
  [[32mPASS[0m] compare-is-not-float-float-2             [2m(1.97ms)[0m

[1mtests/correlated-subquery-hash-join.sqltest[0m
  [[32mPASS[0m] correlated-subquery-reversed-join        [2m(17.51ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_column_age_violation    [2m(9.91ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-is-not-text-text-1               [2m(1.69ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-column-view-attached-db-qualified [2m(41.92ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_group_by_explicit_override_binary_index [2m(20.28ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-keyword-in-pk-clause             [2m(11.19ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-where-filter                  [2m(12.68ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_arithmetic_divide                 [2m(11.87ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-agg-unary-positive                [2m(42.88ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-is-not-text-text-2               [2m(2.60ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-fail-on-non-integer-pk           [2m(2.74ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-trigger-body-update   [2m(13.71ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] filter-with-distinct-agg                 [2m(14.72ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-cascade-delete-self-referential-composite-chain [2m(13.89ms)[0m
  [[32mPASS[0m] fk-composite-pk-delete-ok                [2m(10.15ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-insert-values-subquery               [2m(11.19ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_ipk_referenced_in_table_check [2m(12.71ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-int-float-lte-negative-zero      [2m(4.20ms)[0m

[1mtests/default_value.sqltest[0m
  [[32mPASS[0m] default-value-null                       [2m(13.54ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_column_salary_violation [2m(7.61ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-where-false                   [2m(8.48ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_arithmetic_modulo                 [2m(11.64ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] filter-mixed-agg-and-nonagg              [2m(7.68ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_group_by_mixed_columns           [2m(12.19ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-insert-values-multi-subquery         [2m(7.56ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-int-float-lt-negative-zero       [2m(5.71ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-min                               [2m(38.50ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_column_salary_zero_violation [2m(5.96ms)[0m
  [[32mPASS[0m] check_constraint_ipk_not_updated_but_referenced [2m(8.49ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-avg-text                          [2m(44.58ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-empty-insert-select-is-safe      [2m(10.98ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-composite-pk-delete-violate           [2m(10.39ms)[0m

[1mtests/correlated-subquery-hash-join.sqltest[0m
  [[32mPASS[0m] correlated-subquery-with-join-multiple-cols [2m(17.12ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-cascade-delete-self-referential-two-fks [2m(12.56ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-trigger-body-delete   [2m(12.72ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-real-vs-int-literal-int-precision [2m(4.99ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] filter-with-case-condition               [2m(5.16ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_table_level_basic       [2m(4.78ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_comparison_equal                  [2m(5.50ms)[0m

[1mtests/default_value.sqltest[0m
  [[32mPASS[0m] default-value-boolean                    [2m(11.19ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_drop_column_qualified_ref [2m(5.91ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-column-trigger-attached-db-same-db [2m(19.57ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-where-null-predicate          [2m(11.16ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-insert-values-multiple-rows          [2m(7.86ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_subquery_preserves_column_collation [2m(8.00ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] filter-with-subquery-in-condition        [2m(5.62ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-cascade-delete-self-referential-deferred [2m(6.22ms)[0m

[1mtests/correlated-subquery-hash-join.sqltest[0m
  [[32mPASS[0m] correlated-subquery-single-outer-row     [2m(8.97ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-default-parent-pk-composite-ok        [2m(11.40ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-with-xfer-optimization           [2m(12.64ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_comparison_greater                [2m(9.05ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_table_level_violation   [2m(9.32ms)[0m

[1mtests/default_value.sqltest[0m
  [[32mPASS[0m] default-value-function                   [2m(8.97ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-order-by-desc-limit           [2m(7.05ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-insert-returning-subquery            [2m(6.92ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-trigger-body-insert-other-table [2m(9.95ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_subquery_preserves_explicit_collation [2m(8.44ms)[0m

[1mtests/composite-index-sort-elim.sqltest[0m
  [[32mPASS[0m] eq-prefix-order-suffix                   [2m(13.10ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_error_message_unnamed   [2m(11.31ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-column-view-attached-db-same-db [2m(12.03ms)[0m

[1mtests/default_value.sqltest[0m
  [[32mPASS[0m] default-value-identifier                 [2m(4.99ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_comparison_affinity_text_integer  [2m(5.38ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-insert-returning-multi-subquery      [2m(5.72ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-default-parent-pk-composite-missing   [2m(6.42ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_in_operator             [2m(6.54ms)[0m

[1mtests/correlated-subquery-in-clause.sqltest[0m
  [[32mPASS[0m] correlated-in-outer-alias                [2m(9.84ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_subquery_preserves_collation_in_order_by [2m(5.58ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-agg-unary-positive                [2m(54.88ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-limit-offset                  [2m(7.62ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-cascade-delete-self-referential-multi-root [2m(11.35ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-with-upsert                      [2m(8.46ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-trigger-schema-tbl-name [2m(8.93ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] filter-group-by-multiple-groups          [2m(13.59ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_comparison_affinity_equals        [2m(4.89ms)[0m

[1mtests/default_value.sqltest[0m
  [[32mPASS[0m] default-value-ident-to-str               [2m(6.00ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-default-parent-rowid-no-parent-pk     [2m(5.50ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-delete-returning-subquery            [2m(5.69ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-sum-text                          [2m(149.72ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_error_message_named     [2m(8.97ms)[0m

[1mtests/correlated-subquery-in-clause.sqltest[0m
  [[32mPASS[0m] correlated-in-outer-alias-with-where     [2m(6.45ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-column-case-insensitive [2m(8.90ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_orderby_binary_vs_index_rtrim    [2m(7.05ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-distinct-left-column          [2m(7.11ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-cascade-delete-self-referential-adjacent-rows [2m(7.18ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_in_operator_violation   [2m(8.54ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_comparison_affinity_null          [2m(4.95ms)[0m

[1mtests/composite-index-sort-elim.sqltest[0m
  [[32mPASS[0m] two-eq-order-last                        [2m(11.66ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] filter-with-order-by-on-agg              [2m(6.37ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-update-returning-subquery            [2m(5.62ms)[0m

[1mtests/default_value.sqltest[0m
  [[32mPASS[0m] default-value-column-name                [2m(6.72ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-parent-omit-cols-parent-has-pk        [2m(6.84ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-conflict-on-nothing              [2m(10.33ms)[0m

[1mtests/correlated-subquery-in-clause.sqltest[0m
  [[32mPASS[0m] correlated-in-outer-alias-no-match       [2m(5.99ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-cascade-delete-self-referential-explain-terminates [2m(5.49ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_comparison_affinity_negative      [2m(5.46ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-trigger-self-insert-sql-rewritten [2m(11.55ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_orderby_rtrim_matches_index_rtrim [2m(6.64ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_rename_table_qualified_ref [2m(8.49ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-distinct-pairs                [2m(6.79ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] filter-boolean-condition                 [2m(5.75ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-quoted-column         [2m(8.83ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-insert-values-and-returning-subquery [2m(5.18ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-self-ipk-single-ok                    [2m(4.71ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-with-group-by-and-agg-2           [2m(161.56ms)[0m

[1mtests/delete-correlated-subquery-rowid.sqltest[0m
  [[32mPASS[0m] delete-correlated-subquery-rowid-basic   [2m(7.45ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-agg-json-array                    [2m(854.93µs)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_comparison_affinity_real          [2m(5.30ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_null_passes             [2m(11.05ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-agg-json-array-object             [2m(968.17µs)[0m

[1mtests/composite-index-sort-elim.sqltest[0m
  [[32mPASS[0m] eq-col-in-order-by-desc                  [2m(11.27ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-skips-manually-updated-pk        [2m(8.33ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] json-group-array-empty                   [2m(662.61µs)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-self-ipk-single-mismatch              [2m(4.12ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_orderby_nocase_vs_index_binary   [2m(6.24ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] json-group-object-empty                  [2m(640.18µs)[0m
  [[32mPASS[0m] jsonb-group-array-empty                  [2m(654.09µs)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-group-by-left                 [2m(6.97ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_rename_table_qualified_ref_still_enforced [2m(7.68ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] jsonb-group-object-empty                 [2m(674.90µs)[0m
  [[32mPASS[0m] select-sum                               [2m(42.84ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-cascade-delete-two-table-cycle        [2m(9.03ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-fail-on-non-integer-pk-mixed-case-string-name [2m(2.79ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-trigger-self-insert-blob-text-binding [2m(9.29ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-max-text                          [2m(46.13ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-self-ipk-text-child-coerces-rowid-ok  [2m(4.52ms)[0m

[1mtests/correlated-subquery-nested-exists.sqltest[0m
  [[32mPASS[0m] correlated-subquery-nested-exists-count-sum [2m(12.30ms)[0m

[1mtests/delete-correlated-subquery-rowid.sqltest[0m
  [[32mPASS[0m] delete-correlated-subquery-rowid-partial [2m(7.42ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-distinct-agg-functions            [2m(149.78ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_update_success          [2m(7.05ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-add-column                   [2m(10.84ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-dual-ref-indexed-subquery-empty      [2m(11.04ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_orderby_binary_vs_index_nocase   [2m(6.58ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-json-group-object                 [2m(1.52ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] filter-negative-values                   [2m(12.68ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-group-by-right                [2m(6.82ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_self_table_affinity_update_not_null [2m(9.78ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-expicit-negative-rowid-initializes-seq-table [2m(6.15ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-self-ipk-invalid-text-child-is-deferred [2m(4.48ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_rename_table_unqualified [2m(7.81ms)[0m

[1mtests/composite-index-sort-elim.sqltest[0m
  [[32mPASS[0m] eq-prefix-order-suffix-limit             [2m(11.62ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-agg-binary-unary-negative         [2m(65.15ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_update_age_violation    [2m(5.55ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-add-column-typed             [2m(6.63ms)[0m

[1mtests/delete-correlated-subquery-rowid.sqltest[0m
  [[32mPASS[0m] delete-correlated-subquery-underscore-rowid [2m(7.50ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-cascade-delete-three-table-cycle      [2m(10.73ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_orderby_multi_column_mixed       [2m(6.59ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-trigger-self-insert-wide-row [2m(10.67ms)[0m

[1mtests/correlated-subquery-nested-exists.sqltest[0m
  [[32mPASS[0m] update-between-subquery-equivalence      [2m(8.87ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] filter-with-limit                        [2m(7.18ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-avg-text                          [2m(97.83ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-having                        [2m(7.45ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_self_table_affinity_create_index_population [2m(9.13ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-dual-ref-indexed-subquery            [2m(12.44ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-alter-drop-column-preserves-schema [2m(11.07ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-add-column-invalid-multi-column-fk [2m(7.25ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_ddl_column_level_preserved [2m(10.75ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-self-ipk-invalid-blob-child-is-deferred [2m(12.99ms)[0m

[1mtests/composite-index-sort-elim.sqltest[0m
  [[32mPASS[0m] eq-prefix-order-suffix-limit-offset      [2m(11.34ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_orderby_implicit_vs_explicit_index [2m(9.42ms)[0m

[1mtests/correlated-subquery-nested-exists.sqltest[0m
  [[32mPASS[0m] correlated-exists-limit-offset-empty-after-offset [2m(9.37ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-aggregate-sum                 [2m(7.75ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] filter-with-join                         [2m(9.32ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_self_table_affinity_delete_index_maintenance [2m(7.34ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_update_salary_violation [2m(13.49ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-cascade-mixed-actions-two-table-cycle [2m(12.30ms)[0m

[1mtests/delete-correlated-subquery-rowid.sqltest[0m
  [[32mPASS[0m] delete-correlated-subquery-oid           [2m(12.47ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-seq-text-prefix-coercion         [2m(7.93ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-duplicate-name                       [2m(9.15ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-agg-binary-unary-negative         [2m(48.31ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-column-trigger-fires  [2m(16.21ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-self-composite-single-ok              [2m(7.68ms)[0m

[1mtests/correlated-subquery-window.sqltest[0m
  [[32mPASS[0m] exists-correlated-window-count           [2m(6.86ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] filter-all-agg-types-same-table          [2m(5.66ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-count-column-vs-count-star    [2m(6.74ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_self_table_affinity_hash_join_payload [2m(7.60ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-add-column-default           [2m(12.90ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-restrict-two-table-cycle              [2m(7.72ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_orderby_explicit_override_column [2m(11.40ms)[0m

[1mtests/composite-index-sort-elim.sqltest[0m
  [[32mPASS[0m] eq-prefix-wrong-direction                [2m(11.87ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] filter-group-concat-with-filter          [2m(5.37ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-seq-text-with-exponent-stops-at-e [2m(7.85ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-self-join                     [2m(5.61ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-rowid-mustbeint-coercion-ok           [2m(6.26ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_rename_column_to_keyword_ddl [2m(16.09ms)[0m

[1mtests/delete-correlated-subquery-rowid.sqltest[0m
  [[32mPASS[0m] delete-correlated-subquery-empty-table   [2m(11.27ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_multiple                [2m(11.90ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-total                             [2m(54.33ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-add-quoted-column            [2m(6.03ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-total-text                        [2m(870.51µs)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-unreferenced-qualified-column-orderby-exists [2m(11.83ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_self_table_affinity_integrity_check_not_null [2m(7.51ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_multiple_first_violation [2m(4.84ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-self-join-diagonal            [2m(6.00ms)[0m

[1mtests/correlated-subquery-window.sqltest[0m
  [[32mPASS[0m] exists-correlated-window-total           [2m(13.95ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-add-column-hyphen-digit-name [2m(5.73ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_rename_column_to_keyword_still_works [2m(9.98ms)[0m

[1mtests/delete-correlated-subquery-rowid.sqltest[0m
  [[32mPASS[0m] delete-correlated-subquery-with-nulls    [2m(9.95ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-column-trigger-update-of [2m(17.19ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-sum                               [2m(31.99ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-rowid-mustbeint-coercion-fail         [2m(10.99ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] filter-with-coalesce                     [2m(12.79ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_orderby_with_rowid_tiebreaker    [2m(14.31ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-seq-text-with-negative-exponent  [2m(13.05ms)[0m

[1mtests/agg-functions/agg-extreme-exponent.sqltest[0m
  [[32mPASS[0m] avg_hex_large                            [2m(7.09ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-cascade-trigger-cascade               [2m(15.97ms)[0m

[1mtests/agg-functions/agg-extreme-exponent.sqltest[0m
  [[32mPASS[0m] avg_extreme_exponent_text                [2m(4.85ms)[0m
  [[32mPASS[0m] sum_extreme_exponent_text                [2m(4.96ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-sum-text                          [2m(49.25ms)[0m

[1mtests/composite-index-sort-elim.sqltest[0m
  [[32mPASS[0m] all-eq-order-non-index                   [2m(17.51ms)[0m

[1mtests/agg-functions/agg-extreme-exponent.sqltest[0m
  [[32mPASS[0m] sum_hex_large                            [2m(8.18ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_insert_or_ignore_notnull_violation [2m(5.27ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-min-text                          [2m(48.70ms)[0m

[1mtests/delete-correlated-subquery.sqltest[0m
  [[32mPASS[0m] delete-correlated-min-per-group          [2m(6.54ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-group-concat                      [2m(749.76µs)[0m
  [[32mPASS[0m] select-group-concat-with-delimiter       [2m(710.30µs)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-parent-unique-index-ok                [2m(7.57ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-add-column-hyphen-digit-schema-sql [2m(5.43ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-group-concat-with-delimiter-expression [2m(768.75µs)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-seq-blob-prefix-coercion         [2m(7.24ms)[0m

[1mtests/correlated-subquery-window.sqltest[0m
  [[32mPASS[0m] exists-correlated-no-window              [2m(5.42ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-three-tables                  [2m(8.39ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-cascade-update-basic                  [2m(8.42ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-group-concat-with-column-delimiter [2m(3.76ms)[0m

[1mtests/delete-correlated-subquery.sqltest[0m
  [[32mPASS[0m] delete-correlated-min-with-condition     [2m(5.87ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-group-concat-with-column-delimiter-group-by [2m(1.03ms)[0m
  [[32mPASS[0m] select-string-agg-with-delimiter         [2m(714.92µs)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_insert_select_basic               [2m(8.59ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-add-column-keyword-name      [2m(5.92ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_orderby_desc_collation_mismatch  [2m(12.56ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] filter-single-row                        [2m(13.03ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-parent-unique-index-missing           [2m(6.70ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-string-agg-with-delimiter-expression [2m(773.49µs)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_rename_column_to_keyword_violation [2m(10.68ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-string-agg-with-column-delimiter  [2m(708.81µs)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-seq-blob-leading-vtab-whitespace [2m(7.20ms)[0m

[1mtests/composite-index-sort-elim.sqltest[0m
  [[32mPASS[0m] no-eq-full-scan-order                    [2m(12.93ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-cascade-update-multi-children         [2m(8.78ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-column-trigger-body-update-refs [2m(19.93ms)[0m
  [[32mPASS[0m] alter-table-add-column-keyword-schema-sql [2m(6.49ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_alter_add_column_default_violates_check [2m(5.54ms)[0m

[1mtests/create_index.sqltest[0m
  [[32mPASS[0m] create-index-quoted-identifiers          [2m(10.52ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] filter-with-between                      [2m(6.82ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-three-tables-count            [2m(10.74ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_orderby_desc_collation_match     [2m(7.73ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-seq-text-at-i64-max-returns-full [2m(6.30ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-child-null-shortcircuit               [2m(9.76ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_error_drop_column_in_case         [2m(8.82ms)[0m
  [[32mPASS[0m] gencol_insert_select_with_transform      [2m(10.43ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-json-group-object-no-sorting-required [2m(65.93ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] filter-with-in-list                      [2m(6.30ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-add-column-digit-start-name  [2m(6.90ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-cascade-update-self-referential-chain [2m(8.14ms)[0m

[1mtests/delete-correlated-subquery.sqltest[0m
  [[32mPASS[0m] delete-correlated-single-row-groups      [2m(15.41ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-subquery-right                [2m(7.33ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-max-star                          [2m(3.65ms)[0m
  [[32mPASS[0m] select-agg-binary-unary-positive         [2m(65.27ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_orderby_rtrim_vs_binary_spaces   [2m(8.66ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-explicit-rowid-below-text-prefix-preserves-row [2m(8.61ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-agg-binary-unary-positive         [2m(50.34ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-column-trigger-when-clause [2m(13.24ms)[0m

[1mtests/create_index.sqltest[0m
  [[32mPASS[0m] create-unique-index-with-duplicates-1    [2m(12.69ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_rename_then_drop_keeps_renamed_ref [2m(12.86ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-add-column-bracket-quoted-hyphen-digit [2m(6.40ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-max-star-in-expression            [2m(4.25ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-subquery-both-sides           [2m(5.66ms)[0m

[1mtests/composite-index-sort-elim.sqltest[0m
  [[32mPASS[0m] range-prefix-order-suffix                [2m(16.69ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-self-unique-ok                        [2m(11.28ms)[0m

[1mtests/delete-correlated-subquery.sqltest[0m
  [[32mPASS[0m] delete-correlated-exists                 [2m(8.85ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] filter-with-like                         [2m(9.31ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_insert_select_from_gencol_table   [2m(12.36ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-cascade-update-two-table-cycle        [2m(9.64ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_drop_column_with_virtual_present  [2m(13.11ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-scalar-func-star                  [2m(3.95ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-explicit-rowid-above-text-prefix-advances [2m(7.23ms)[0m

[1mtests/create_index.sqltest[0m
  [[32mPASS[0m] create-unique-index-with-duplicates-2    [2m(4.67ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_orderby_prefers_matching_index   [2m(7.95ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-limit                             [2m(711.80µs)[0m
  [[32mPASS[0m] select-count-constant-false              [2m(592.62µs)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-derived-left-constant         [2m(5.00ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-max                               [2m(688.05µs)[0m
  [[32mPASS[0m] select-min                               [2m(703.04µs)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-self-unique-parent-affinity-does-not-coerce-same-row-child [2m(4.85ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] filter-with-null-agg-arg                 [2m(5.17ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-count-2                           [2m(3.57ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[33mSKIP[0m] check_constraint_strict_type_violation   [2m(8.63µs)[0m [2m(requires strict: uses STRICT tables)[0m

[1mtests/delete-correlated-subquery.sqltest[0m
  [[32mPASS[0m] delete-correlated-in-subquery            [2m(6.99ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_rename_then_add_keeps_renamed_ref [2m(11.04ms)[0m
  [[32mPASS[0m] check_constraint_multiple_second_violation [2m(4.56ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-column-trigger-insert-other-table [2m(12.40ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_drop_column_with_multiple_virtual [2m(8.57ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-add-column-default-affinity-text [2m(12.28ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-cascade-update-composite              [2m(9.02ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_strict_numeric_string_coercion [2m(4.47ms)[0m

[1mtests/composite-index-sort-elim.sqltest[0m
  [[32mPASS[0m] group-by-eq-prefix                       [2m(11.91ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_strict_table_basic      [2m(5.37ms)[0m
  [[32mPASS[0m] check_constraint_strict_check_violation_after_coercion [2m(4.83ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-total                             [2m(45.05ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_insert_select_virtual_middle      [2m(12.96ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_complex_expression_violation [2m(4.91ms)[0m
  [[32mPASS[0m] check_constraint_regular_table_numeric_string [2m(6.00ms)[0m
  [[32mPASS[0m] check_constraint_and_logic_violation     [2m(4.75ms)[0m
  [[32mPASS[0m] check_constraint_complex_expression      [2m(5.74ms)[0m
  [[32mPASS[0m] check_constraint_between_lower_violation [2m(4.54ms)[0m
  [[32mPASS[0m] check_constraint_or_logic                [2m(5.67ms)[0m
  [[32mPASS[0m] check_constraint_between                 [2m(6.16ms)[0m
  [[32mPASS[0m] check_constraint_between_upper_violation [2m(4.63ms)[0m

[1mtests/after-insert-trigger-cache-spill.sqltest[0m
  [[32mPASS[0m] after-insert-trigger-cache-spill-no-index [2m(61.22ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_like_pattern            [2m(5.70ms)[0m
  [[32mPASS[0m] check_constraint_not_operator_violation  [2m(4.56ms)[0m
  [[32mPASS[0m] check_constraint_length_too_short        [2m(4.84ms)[0m
  [[32mPASS[0m] check_constraint_not_operator            [2m(5.52ms)[0m
  [[32mPASS[0m] check_constraint_length_too_long         [2m(4.83ms)[0m
  [[32mPASS[0m] check_constraint_like_pattern_violation  [2m(7.46ms)[0m
  [[32mPASS[0m] check_constraint_multi_row_insert_all_pass [2m(4.73ms)[0m
  [[32mPASS[0m] check_constraint_and_logic               [2m(11.09ms)[0m
  [[32mPASS[0m] check_constraint_multi_row_insert_one_fails [2m(4.81ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-count-constant-true               [2m(19.36ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_real_precision          [2m(5.82ms)[0m

[1mtests/after-insert-trigger-cache-spill.sqltest[0m
  [[32mPASS[0m] after-insert-trigger-cache-spill-with-index-small [2m(66.46ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_length_function         [2m(9.70ms)[0m
  [[32mPASS[0m] check_constraint_real_precision_violation [2m(5.18ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] delete_with_unique_gencol                [2m(4.86ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_transaction_rollback    [2m(7.33ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] fk-child-references-parent-virtual-col-using-rowid-missing [2m(8.67ms)[0m
  [[32mPASS[0m] fk-child-references-parent-virtual-col-using-rowid [2m(9.45ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_boolean_style_violation [2m(7.07ms)[0m
  [[32mPASS[0m] check_constraint_boolean_style           [2m(7.73ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] fk-transitive-virtual-cascade            [2m(8.72ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_verify_rollback         [2m(9.46ms)[0m
  [[32mPASS[0m] check_constraint_case_expression         [2m(8.10ms)[0m
  [[32mPASS[0m] check_constraint_rowid_reference_violation [2m(4.32ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] delete-transitive-virtual-dep            [2m(11.69ms)[0m
  [[32mPASS[0m] update-virtual-not-null                  [2m(11.39ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_rowid_reference         [2m(5.94ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] update-virtual-not-null-deferred-trigger-ignore [2m(9.82ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_default_value           [2m(5.63ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] update-virtual-not-null-transitive       [2m(11.50ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_regular_table_text_stored [2m(26.38ms)[0m
  [[32mPASS[0m] check_constraint_default_value_violation [2m(4.65ms)[0m
  [[32mPASS[0m] check_constraint_case_expression_violation [2m(11.81ms)[0m
  [[32mPASS[0m] check_constraint_cast_expression_violation [2m(4.65ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] update-virtual-not-null-transitive-deferred-trigger-ignore [2m(13.32ms)[0m
  [[32mPASS[0m] gencol-virtual-join-inner-inner          [2m(9.21ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_on_conflict_replace     [2m(4.87ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol-virtual-join-two-table            [2m(6.19ms)[0m
  [[32mPASS[0m] gencol-virtual-join-left-inner           [2m(9.87ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_cast_expression         [2m(8.82ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] update-virtual-not-null-deferred-trigger-abort [2m(13.71ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_on_conflict_fail        [2m(7.88ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol-virtual-join-t1-second            [2m(11.02ms)[0m
  [[32mPASS[0m] gencol-notnull-autogenerated-rowid-alias [2m(4.91ms)[0m
  [[32mPASS[0m] update-virtual-not-null-transitive-deferred-trigger-abort [2m(15.08ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-distinct-agg-functions            [2m(125.41ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol-virtual-join-cross                [2m(10.86ms)[0m
  [[32mPASS[0m] gencol-virtual-join-all-left             [2m(8.84ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_on_conflict_ignore      [2m(9.59ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-non-agg-cols-should-be-not-null   [2m(53.10ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_on_conflict_abort       [2m(8.81ms)[0m
  [[32mPASS[0m] check_constraint_before_foreign_key      [2m(8.04ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] replace-conflict-on-virtual-column       [2m(8.67ms)[0m
  [[32mPASS[0m] delete-indexed-virtual-generated-column-old-image [2m(6.11ms)[0m
  [[32mPASS[0m] update-with-virtual-column-before-pk     [2m(5.69ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_collate_nocase          [2m(6.52ms)[0m
  [[32mPASS[0m] check_constraint_on_conflict_rollback    [2m(11.42ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol-virtual-join-comma                [2m(15.33ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_foreign_key_fails       [2m(7.05ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] delete-using-indexed-virtual-column      [2m(9.07ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_collate_nocase_violation [2m(6.66ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol-notnull-autogenerated-rowid-alias-default-values [2m(11.41ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_with_foreign_key        [2m(12.77ms)[0m
  [[32mPASS[0m] check_constraint_arithmetic_overflow     [2m(7.46ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] update-ipk-with-indexed-virtual-col      [2m(7.96ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_arithmetic_overflow_violation [2m(7.88ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-count                             [2m(52.63ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] integrity-check-unique-virtual-column    [2m(4.92ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_alter_table_add_column_violation [2m(6.94ms)[0m
  [[32mPASS[0m] check_constraint_alter_add_column_with_data [2m(7.21ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] upsert-unique-virtual-column-ipk-autoindex [2m(10.69ms)[0m
  [[32mPASS[0m] upsert-unique-virtual-column             [2m(10.82ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_pragma_ignore_then_enforce [2m(5.18ms)[0m
  [[32mPASS[0m] check_constraint_pragma_ignore           [2m(5.47ms)[0m
  [[32mPASS[0m] check_constraint_alter_add_column_with_data_violation [2m(8.28ms)[0m
  [[32mPASS[0m] alter_table_add_column_check_constraint_named [2m(5.34ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol-virtual-union-tag-select-resolves-self-table-type [2m(6.41ms)[0m
  [[32mPASS[0m] upsert-unique-virtual-column-ipk         [2m(11.76ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_alter_add_column_with_data_enforced [2m(11.98ms)[0m
  [[32mPASS[0m] check_constraint_rowid_alias_underscore  [2m(4.94ms)[0m
  [[32mPASS[0m] check_constraint_upsert_do_update_success [2m(6.15ms)[0m
  [[32mPASS[0m] check_constraint_rowid_alias_oid         [2m(4.40ms)[0m
  [[32mPASS[0m] check_constraint_update_or_ignore_single_row [2m(6.14ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-max-text                          [2m(59.74ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_upsert_do_update_violation [2m(8.54ms)[0m
  [[32mPASS[0m] check_constraint_alter_add_column_with_data_update_violation [2m(13.47ms)[0m
  [[32mPASS[0m] check_constraint_update_rowid_violation  [2m(5.50ms)[0m
  [[32mPASS[0m] alter_table_add_column_check_constraint_success [2m(10.21ms)[0m
  [[32mPASS[0m] check_constraint_qualified_column_ref    [2m(4.71ms)[0m
  [[32mPASS[0m] check_constraint_upsert_rowid_check      [2m(4.94ms)[0m
  [[32mPASS[0m] check_constraint_qualified_column_ref_violation [2m(4.42ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] update-virtual-column-index-via-single-column-unique-index-scan [2m(15.73ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_update_or_ignore_all_rows [2m(9.19ms)[0m
  [[32mPASS[0m] check_constraint_qualified_rowid_ref_violation [2m(4.55ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-min-text                          [2m(63.53ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_qualified_rowid_ref     [2m(5.63ms)[0m
  [[32mPASS[0m] check_constraint_qualified_rowid_alias_oid [2m(4.73ms)[0m
  [[32mPASS[0m] check_constraint_qualified_rowid_alias_underscore [2m(5.19ms)[0m
  [[32mPASS[0m] check_constraint_qualified_column_update [2m(5.32ms)[0m
  [[32mPASS[0m] check_constraint_rowid_alias_column_by_name [2m(4.89ms)[0m
  [[32mPASS[0m] check_constraint_trigger_violation       [2m(18.24ms)[0m
  [[32mPASS[0m] check_constraint_rowid_alias_column_by_name_violation [2m(4.99ms)[0m
  [[32mPASS[0m] check_constraint_update_no_rowid_change  [2m(7.29ms)[0m
  [[32mPASS[0m] check_constraint_rowid_alias_column_qualified_violation [2m(4.97ms)[0m
  [[32mPASS[0m] check_constraint_nonexistent_qualified_column [2m(2.63ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] insert-transitive-unique-index           [2m(20.11ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_nonexistent_column_table_level [2m(2.68ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-non-agg-cols-should-be-not-null   [2m(75.58ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_nonexistent_column      [2m(3.18ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] update-covering-index-chained-virtual-gencol [2m(8.25ms)[0m
  [[32mPASS[0m] update-deferred-fk-on-child-virtual-column [2m(22.45ms)[0m

[1mtests/after-insert-trigger-cache-spill.sqltest[0m
  [[32mPASS[0m] before-insert-trigger-cache-spill-with-index [2m(116.70ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_qualified_column_update_violation [2m(9.54ms)[0m
  [[32mPASS[0m] check_constraint_rename_column_inline    [2m(8.99ms)[0m
  [[32mPASS[0m] check_constraint_rename_table_violation  [2m(7.08ms)[0m
  [[32mPASS[0m] check_constraint_rename_column_table_level [2m(9.18ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-scalar-func-star-in-expression    [2m(6.72ms)[0m
  [[32mPASS[0m] select-nested-agg-func                   [2m(6.75ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_rename_column_qualified_ref_violation [2m(9.04ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-nested-agg-func-in-expression     [2m(6.77ms)[0m

[1mtests/agg-functions/group-concat-types.sqltest[0m
  [[32mPASS[0m] group-concat-integer-returns-text-type   [2m(5.40ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_rename_table            [2m(9.36ms)[0m

[1mtests/agg-functions/group-concat-types.sqltest[0m
  [[32mPASS[0m] group-concat-single-integer-returns-text-type [2m(5.50ms)[0m
  [[32mPASS[0m] group-concat-single-integer-value        [2m(5.47ms)[0m
  [[32mPASS[0m] group-concat-null-separator              [2m(3.95ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_rename_column_table_level_violation [2m(10.80ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] printf-19029102                          [2m(7.88ms)[0m

[1mtests/agg-functions/group-concat-types.sqltest[0m
  [[32mPASS[0m] group-concat-integer-separator           [2m(5.08ms)[0m
  [[32mPASS[0m] group-concat-integer-separator-type      [2m(5.16ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] format-19029102                          [2m(8.19ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_rename_column_inline_violation [2m(12.85ms)[0m

[1mtests/agg-functions/group-concat-types.sqltest[0m
  [[32mPASS[0m] group-concat-with-null-values            [2m(4.88ms)[0m
  [[32mPASS[0m] group-concat-only-null-values            [2m(5.24ms)[0m
  [[32mPASS[0m] group-concat-first-value-null            [2m(5.39ms)[0m
  [[32mPASS[0m] group-concat-first-value-null-type       [2m(5.06ms)[0m
  [[32mPASS[0m] group-concat-boolean-expression-returns-text [2m(5.53ms)[0m
  [[32mPASS[0m] group-concat-boolean-expression-value    [2m(5.84ms)[0m
  [[32mPASS[0m] string-agg-integer-returns-text-type     [2m(5.46ms)[0m
  [[32mPASS[0m] group-concat-integer-and-text-type       [2m(5.46ms)[0m
  [[32mPASS[0m] group-concat-integer-and-text            [2m(5.83ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] json_group_array_arrow_operator_repro    [2m(13.39ms)[0m

[1mtests/agg-functions/group-concat-types.sqltest[0m
  [[32mPASS[0m] group-concat-blob-and-text               [2m(5.24ms)[0m
  [[32mPASS[0m] group-concat-null-separator-type         [2m(11.02ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] json_group_array_extract_repro           [2m(14.20ms)[0m

[1mtests/agg-functions/group-concat-types.sqltest[0m
  [[32mPASS[0m] group-concat-blob-and-integer            [2m(4.66ms)[0m
  [[32mPASS[0m] group-concat-real-and-integer-type       [2m(7.89ms)[0m
  [[32mPASS[0m] group-concat-text-and-real               [2m(7.88ms)[0m
  [[32mPASS[0m] group-concat-float-returns-text-type     [2m(9.47ms)[0m
  [[32mPASS[0m] group-concat-real-and-integer            [2m(8.30ms)[0m
  [[32mPASS[0m] group-concat-blob-single-type            [2m(7.99ms)[0m
  [[32mPASS[0m] string-agg-integer-separator             [2m(9.28ms)[0m
  [[32mPASS[0m] group-concat-null-and-text               [2m(4.76ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_rename_column_qualified_ref [2m(19.95ms)[0m

[1mtests/agg-functions/group-concat-types.sqltest[0m
  [[32mPASS[0m] group-concat-null-and-blob               [2m(5.10ms)[0m
  [[32mPASS[0m] group-concat-blob-and-real               [2m(7.00ms)[0m
  [[32mPASS[0m] group-concat-real-blob-text-integer      [2m(4.66ms)[0m
  [[32mPASS[0m] group-concat-empty-string-and-integer    [2m(4.78ms)[0m
  [[32mPASS[0m] group-concat-blob-text-integer-separator [2m(5.07ms)[0m
  [[32mPASS[0m] group-concat-invalid-utf                 [2m(4.32ms)[0m
  [[32mPASS[0m] group-concat-zero-real-and-text          [2m(4.64ms)[0m
  [[32mPASS[0m] group-concat-blob-single                 [2m(14.24ms)[0m
  [[32mPASS[0m] group-concat-blob-multiple               [2m(13.64ms)[0m
  [[32mPASS[0m] string-agg-single-integer-returns-text-type [2m(12.05ms)[0m
  [[32mPASS[0m] group-concat-empty-string-and-blob       [2m(7.26ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] min-null-regression-test                 [2m(6.19ms)[0m

[1mtests/agg-functions/group-concat-types.sqltest[0m
  [[32mPASS[0m] group-concat-null-and-integer            [2m(12.06ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] max-null-regression-test                 [2m(6.56ms)[0m

[1mtests/generate_series.sqltest[0m
  [[32mPASS[0m] generate-series-single-arg-default-stop  [2m(2.06ms)[0m
  [[32mPASS[0m] generate-series-single-arg-via-where     [2m(2.66ms)[0m
  [[32mPASS[0m] generate-series-two-args                 [2m(2.43ms)[0m

[1mtests/agg-functions/group-concat-types.sqltest[0m
  [[32mPASS[0m] group-concat-null-and-real               [2m(16.01ms)[0m
  [[32mPASS[0m] group-concat-zero-integer-and-text       [2m(13.20ms)[0m
  [[32mPASS[0m] group-concat-integer-text-real-blob      [2m(14.52ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] update-expression-index-references-virtual-gencol [2m(12.27ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] group-concat-null-values-test            [2m(10.20ms)[0m

[1mtests/generate_series.sqltest[0m
  [[32mPASS[0m] generate-series-three-args               [2m(2.75ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] ungrouped-agg-empty-table-column-is-null [2m(7.76ms)[0m

[1mtests/generate_series.sqltest[0m
  [[32mPASS[0m] generate-series-explicit-negative-stop   [2m(3.03ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] ungrouped-agg-filtered-rows-literal      [2m(7.60ms)[0m

[1mtests/glob/default.sqltest[0m
  [[32mPASS[0m] glob-fn                                  [2m(3.32ms)[0m

[1mtests/agg-functions/group-concat-types.sqltest[0m
  [[32mPASS[0m] group-concat-all-types-mixed             [2m(19.38ms)[0m

[1mtests/agg-functions/is-true.sqltest[0m
  [[32mPASS[0m] sum-is-true-ungrouped-and-join           [2m(16.36ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol-virtual-affinity-applied-to-index [2m(15.16ms)[0m
  [[32mPASS[0m] update-partial-index-where-references-virtual-gencol [2m(16.33ms)[0m

[1mtests/agg-functions/group-concat-types.sqltest[0m
  [[32mPASS[0m] group-concat-mixed-types-custom-separator [2m(18.17ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] ungrouped-agg-empty-table-literal        [2m(10.77ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol-update-old-image-real-affinity-applied-to-index [2m(12.96ms)[0m

[1mtests/glob/default.sqltest[0m
  [[32mPASS[0m] where-glob-fn                            [2m(822.69µs)[0m
  [[32mPASS[0m] where-not-glob-and                       [2m(692.80µs)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] count-filter-clause                      [2m(3.63ms)[0m

[1mtests/glob/default.sqltest[0m
  [[32mPASS[0m] where-glob-impossible                    [2m(507.70µs)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol-virtual-real-affinity-applied-to-index [2m(15.50ms)[0m

[1mtests/agg-functions/group-concat-types.sqltest[0m
  [[32mPASS[0m] group-concat-all-types-mixed-type        [2m(22.63ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] select-distinct-aggregate-ungrouped      [2m(15.76ms)[0m

[1mtests/glob/default.sqltest[0m
  [[32mPASS[0m] where-glob                               [2m(5.97ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] ungrouped-agg-empty-table-column-is-not-null [2m(13.31ms)[0m
  [[32mPASS[0m] ungrouped-agg-empty-table-column-with-default [2m(7.35ms)[0m
  [[32mPASS[0m] ungrouped-agg-multiple-literals          [2m(7.64ms)[0m

[1mtests/glob/default.sqltest[0m
  [[32mPASS[0m] where-glob-or                            [2m(3.93ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] update-or-virtual-deferred-seek-no-panic-implicit-index [2m(16.53ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] filter-avg                               [2m(5.37ms)[0m

[1mtests/glob/default.sqltest[0m
  [[32mPASS[0m] where-glob-question-mark                 [2m(6.07ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] filter-total                             [2m(6.71ms)[0m
  [[32mPASS[0m] case-when-aggregate-returns-column-value [2m(9.50ms)[0m
  [[32mPASS[0m] filter-min-max                           [2m(7.31ms)[0m
  [[32mPASS[0m] filter-sum                               [2m(8.16ms)[0m

[1mtests/create_index.sqltest[0m
  [[32mPASS[0m] create-unique-index-with-duplicates-3    [2m(5.60ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] ungrouped-agg-filtered-rows-column       [2m(15.26ms)[0m

[1mtests/create_index.sqltest[0m
  [[32mPASS[0m] create-unique-index-with-duplicates-4    [2m(6.29ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] filter-with-group-by                     [2m(9.26ms)[0m

[1mtests/create_index.sqltest[0m
  [[32mPASS[0m] create-index-on-shadowed-rowid           [2m(6.49ms)[0m
  [[32mPASS[0m] create-index-numeric-literal-column      [2m(5.87ms)[0m
  [[32mPASS[0m] create-index-strftime-now-remains-rejected [2m(3.88ms)[0m
  [[32mPASS[0m] create-index-on-rowid                    [2m(8.39ms)[0m
  [[32mPASS[0m] create-index-mixed-string-literal-column [2m(4.41ms)[0m
  [[32mPASS[0m] create-index-on-shadowed-rowid-alias-1   [2m(8.52ms)[0m
  [[32mPASS[0m] create-index-literal-expression          [2m(5.69ms)[0m

[1mtests/agg-functions/memory.sqltest[0m
  [[32mPASS[0m] filter-group-concat                      [2m(12.57ms)[0m

[1mtests/create_index.sqltest[0m
  [[32mPASS[0m] create-index-expression-with-column      [2m(6.00ms)[0m
  [[32mPASS[0m] create-index-sqlite-version              [2m(3.00ms)[0m
  [[32mPASS[0m] create-index-strftime-with-column        [2m(5.96ms)[0m
  [[32mPASS[0m] create-index-string-literal-no-such-column [2m(9.27ms)[0m
  [[32mPASS[0m] create-index-parenthesized-string-matches-column [2m(5.86ms)[0m
  [[32mPASS[0m] create-index-parenthesized-string-no-such-column [2m(4.38ms)[0m
  [[32mPASS[0m] create-index-string-in-expression        [2m(6.14ms)[0m
  [[32mPASS[0m] create-index-on-rowid-alias-2            [2m(11.02ms)[0m

[1mtests/create_table.sqltest[0m
  [[32mPASS[0m] create_table_one_unique_set              [2m(2.84ms)[0m

[1mtests/create_index.sqltest[0m
  [[32mPASS[0m] create-index-on-shadowed-rowid-alias-2   [2m(10.91ms)[0m
  [[32mPASS[0m] create-index-sqlite-source-id            [2m(4.01ms)[0m

[1mtests/create_table.sqltest[0m
  [[32mPASS[0m] create_table_multiple_column_primary_keys [2m(2.58ms)[0m
  [[32mPASS[0m] create_table_column_and_table_primary_keys [2m(2.73ms)[0m
  [[32mPASS[0m] create_table_multiple_table_primary_keys [2m(2.69ms)[0m
  [[32mPASS[0m] create_table_duplicate_primary_key_clause [2m(2.58ms)[0m

[1mtests/create_index.sqltest[0m
  [[32mPASS[0m] create-index-string-literal-matches-column [2m(11.15ms)[0m
  [[32mPASS[0m] create-index-turso-version               [2m(4.96ms)[0m

[1mtests/create_table.sqltest[0m
  [[32mPASS[0m] create_table_same_uniques_and_primary_keys [2m(4.30ms)[0m

[1mtests/create_index.sqltest[0m
  [[32mPASS[0m] create-unique-index-with-duplicates-5    [2m(13.70ms)[0m

[1mtests/create_table.sqltest[0m
  [[32mPASS[0m] create_table_duplicate_column_names_case_insensitive [2m(2.61ms)[0m
  [[32mPASS[0m] create_table_duplicate_column_names_quoted [2m(2.72ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] update-or-virtual-deferred-seek-no-panic-explicit-index [2m(28.02ms)[0m

[1mtests/create_table.sqltest[0m
  [[32mPASS[0m] create_table_duplicate_column_names      [2m(3.52ms)[0m
  [[32mPASS[0m] create_table_unique_constraint_and_autoinc_backticks [2m(5.29ms)[0m

[1mtests/glob/default.sqltest[0m
  [[32mPASS[0m] glob-fn                                  [2m(3.17ms)[0m
  [[32mPASS[0m] where-glob-fn                            [2m(2.45ms)[0m
  [[32mPASS[0m] where-not-glob-and                       [2m(2.24ms)[0m
  [[32mPASS[0m] where-glob-or                            [2m(1.51ms)[0m

[1mtests/create_table.sqltest[0m
  [[32mPASS[0m] create_table_unique_contained_in_primary_keys [2m(6.91ms)[0m

[1mtests/glob/default.sqltest[0m
  [[32mPASS[0m] where-glob-impossible                    [2m(621.12µs)[0m

[1mtests/create_table.sqltest[0m
  [[32mPASS[0m] create_table_rowid_unique_regression_test [2m(6.36ms)[0m

[1mtests/create_index.sqltest[0m
  [[32mPASS[0m] create-index-on-rowid-alias-1            [2m(16.12ms)[0m

[1mtests/create_table.sqltest[0m
  [[32mPASS[0m] col-named-rowid                          [2m(5.45ms)[0m
  [[32mPASS[0m] create-table-only-generated-column-error [2m(2.55ms)[0m

[1mtests/glob/default.sqltest[0m
  [[32mPASS[0m] where-glob                               [2m(5.81ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-1                                   [2m(2.61ms)[0m

[1mtests/create_table.sqltest[0m
  [[32mPASS[0m] col-default-true                         [2m(4.64ms)[0m
  [[32mPASS[0m] create_view_table_collision-1            [2m(7.34ms)[0m
  [[32mPASS[0m] create_table_index_collision-1           [2m(5.63ms)[0m
  [[32mPASS[0m] create_index_table_collision-1           [2m(6.39ms)[0m
  [[32mPASS[0m] col-default-false                        [2m(5.20ms)[0m
  [[32mPASS[0m] create_table_view_collision-1            [2m(8.58ms)[0m

[1mtests/glob/default.sqltest[0m
  [[32mPASS[0m] where-glob-question-mark                 [2m(7.51ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-3                                   [2m(2.52ms)[0m
  [[32mPASS[0m] glob-2                                   [2m(4.32ms)[0m
  [[32mPASS[0m] glob-6                                   [2m(1.87ms)[0m
  [[32mPASS[0m] glob-4                                   [2m(2.72ms)[0m

[1mtests/create_table.sqltest[0m
  [[32mPASS[0m] create_index_view_collision-1            [2m(9.15ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-5                                   [2m(2.85ms)[0m

[1mtests/create_table.sqltest[0m
  [[32mPASS[0m] create_table_trailing_named_column_constraint_primary_key_enforced [2m(6.28ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-8                                   [2m(1.75ms)[0m
  [[32mPASS[0m] glob-7                                   [2m(2.78ms)[0m
  [[32mPASS[0m] glob-9                                   [2m(2.79ms)[0m

[1mtests/create_table.sqltest[0m
  [[32mPASS[0m] create_view_index_collision-1            [2m(11.87ms)[0m
  [[32mPASS[0m] create_table_trailing_named_column_constraint [2m(9.30ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-10                                  [2m(2.90ms)[0m
  [[32mPASS[0m] glob-null-other-types                    [2m(9.81ms)[0m
  [[32mPASS[0m] glob-12                                  [2m(2.70ms)[0m

[1mtests/create_table_as_select.sqltest[0m
  [[32mPASS[0m] ctas_select_columns                      [2m(6.19ms)[0m
  [[32mPASS[0m] ctas_basic                               [2m(7.17ms)[0m
  [[32mPASS[0m] ctas_with_expressions                    [2m(6.59ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-16                                  [2m(2.41ms)[0m

[1mtests/create_table_as_select.sqltest[0m
  [[32mPASS[0m] ctas_aggregate_count                     [2m(4.94ms)[0m
  [[32mPASS[0m] ctas_with_where                          [2m(6.77ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-11                                  [2m(5.56ms)[0m
  [[32mPASS[0m] glob-17                                  [2m(3.08ms)[0m

[1mtests/create_table.sqltest[0m
  [[32mPASS[0m] create_table_primary_key_collate_desc_accepts_distinct_values [2m(9.81ms)[0m

[1mtests/create_table_as_select.sqltest[0m
  [[32mPASS[0m] ctas_with_order_by_limit                 [2m(7.34ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-18                                  [2m(2.82ms)[0m
  [[32mPASS[0m] glob-13                                  [2m(4.96ms)[0m

[1mtests/create_table_as_select.sqltest[0m
  [[32mPASS[0m] ctas_aggregate                           [2m(7.83ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-19                                  [2m(2.76ms)[0m
  [[32mPASS[0m] glob-20                                  [2m(2.72ms)[0m

[1mtests/create_table.sqltest[0m
  [[32mPASS[0m] create_table_quoted_name_does_not_inject_parse_schema_predicate [2m(14.62ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-21                                  [2m(2.79ms)[0m
  [[32mPASS[0m] glob-22                                  [2m(2.77ms)[0m

[1mtests/create_table_as_select.sqltest[0m
  [[32mPASS[0m] ctas_empty_result                        [2m(5.83ms)[0m
  [[32mPASS[0m] ctas_nulls                               [2m(7.22ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-23                                  [2m(2.91ms)[0m
  [[32mPASS[0m] glob-24                                  [2m(2.70ms)[0m

[1mtests/create_table_as_select.sqltest[0m
  [[32mPASS[0m] ctas_type_preservation                   [2m(5.63ms)[0m
  [[32mPASS[0m] ctas_empty_with_where_false              [2m(6.33ms)[0m

[1mtests/create_table.sqltest[0m
  [[32mPASS[0m] create_table_primary_key_collate_nocase_uniqueness [2m(17.11ms)[0m

[1mtests/create_table_as_select.sqltest[0m
  [[32mPASS[0m] ctas_join                                [2m(9.88ms)[0m
  [[32mPASS[0m] ctas_left_join                           [2m(9.25ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-25                                  [2m(2.73ms)[0m

[1mtests/create_table_as_select.sqltest[0m
  [[32mPASS[0m] ctas_literals                            [2m(7.10ms)[0m
  [[32mPASS[0m] ctas_if_not_exists                       [2m(4.92ms)[0m

[1mtests/create_table.sqltest[0m
  [[32mPASS[0m] create_table_unique_collate_nocase_uniqueness [2m(17.56ms)[0m

[1mtests/create_table_as_select.sqltest[0m
  [[32mPASS[0m] ctas_column_names                        [2m(6.26ms)[0m
  [[32mPASS[0m] ctas_union                               [2m(12.33ms)[0m
  [[32mPASS[0m] ctas_aliased_columns                     [2m(11.66ms)[0m
  [[32mPASS[0m] ctas_schema_sql_quoted_source_col        [2m(10.09ms)[0m
  [[32mPASS[0m] ctas_subquery                            [2m(16.74ms)[0m
  [[32mPASS[0m] ctas_expression_column_names             [2m(14.09ms)[0m
  [[32mPASS[0m] ctas_group_by_having                     [2m(14.03ms)[0m
  [[32mPASS[0m] ctas_coalesce                            [2m(13.61ms)[0m
  [[32mPASS[0m] ctas_duplicate_table                     [2m(16.14ms)[0m
  [[32mPASS[0m] ctas_case_expression                     [2m(13.89ms)[0m

[1mtests/after-insert-trigger-cache-spill.sqltest[0m
  [[32mPASS[0m] after-insert-trigger-cache-spill-multi-index [2m(194.27ms)[0m

[1mtests/create_table_as_select.sqltest[0m
  [[32mPASS[0m] ctas_union_all                           [2m(17.75ms)[0m
  [[32mPASS[0m] ctas_schema_sql_mixed_alias_expr         [2m(12.39ms)[0m
  [[32mPASS[0m] ctas_schema_sql                          [2m(13.18ms)[0m
  [[32mPASS[0m] ctas_schema_sql_literals                 [2m(12.20ms)[0m
  [[32mPASS[0m] ctas_cross_join                          [2m(13.83ms)[0m
  [[32mPASS[0m] ctas_schema_sql_expression_columns       [2m(13.23ms)[0m
  [[32mPASS[0m] ctas_mixed_types                         [2m(7.38ms)[0m
  [[32mPASS[0m] ctas_with_functions                      [2m(5.54ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-26                                  [2m(2.82ms)[0m

[1mtests/create_table_as_select.sqltest[0m
  [[32mPASS[0m] ctas_insert_after_create                 [2m(15.86ms)[0m
  [[32mPASS[0m] ctas_distinct                            [2m(17.40ms)[0m
  [[32mPASS[0m] ctas_rowid                               [2m(4.99ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-both-empty                    [2m(4.94ms)[0m

[1mtests/create_table_as_select.sqltest[0m
  [[32mPASS[0m] ctas_duplicate_source_different_alias    [2m(5.58ms)[0m
  [[32mPASS[0m] ctas_many_columns                        [2m(6.79ms)[0m
  [[32mPASS[0m] ctas_compound_select_column_names        [2m(6.97ms)[0m
  [[32mPASS[0m] ctas_except                              [2m(8.19ms)[0m
  [[32mPASS[0m] ctas_schema_expr_reparseable             [2m(17.85ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-shadows-view                         [2m(4.27ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-single-right-row              [2m(6.99ms)[0m

[1mtests/create_table_as_select.sqltest[0m
  [[32mPASS[0m] ctas_offset                              [2m(9.78ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-null-values                   [2m(6.69ms)[0m
  [[32mPASS[0m] cross-join-basic-2x2                     [2m(9.45ms)[0m

[1mtests/create_table_as_select.sqltest[0m
  [[32mPASS[0m] ctas_duplicate_column_names              [2m(10.65ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-single-left-row               [2m(9.48ms)[0m
  [[32mPASS[0m] cross-join-count-star                    [2m(8.99ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-shadows-table                        [2m(6.27ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-left-empty                    [2m(11.71ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-duplicate-name-view                  [2m(9.11ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-right-empty                   [2m(11.88ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-values-multi-ref                     [2m(3.76ms)[0m
  [[32mPASS[0m] cte-does-not-leak-into-view              [2m(5.44ms)[0m

[1mtests/create_table_as_select.sqltest[0m
  [[32mPASS[0m] ctas_triple_duplicate_columns            [2m(13.37ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-shadow-inner-value                   [2m(3.09ms)[0m
  [[32mPASS[0m] cte-shadow-in-exists                     [2m(3.20ms)[0m
  [[32mPASS[0m] cte-multi-ref-except                     [2m(4.41ms)[0m
  [[32mPASS[0m] cte-multi-ref-intersect                  [2m(4.56ms)[0m
  [[32mPASS[0m] cte-multi-ref-join-where                 [2m(4.97ms)[0m
  [[32mPASS[0m] cte-shadow-outer-unused                  [2m(1.97ms)[0m
  [[32mPASS[0m] cte-shadow-outer-still-accessible        [2m(3.22ms)[0m
  [[32mPASS[0m] cte-shadow-three-levels                  [2m(3.37ms)[0m
  [[32mPASS[0m] cte-shadow-in-from-subquery              [2m(3.20ms)[0m
  [[32mPASS[0m] cte-shadow-different-columns             [2m(3.62ms)[0m

[1mtests/create_table_as_select.sqltest[0m
  [[32mPASS[0m] ctas_intersect                           [2m(17.69ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-alias-scalar-subquery-original-name  [2m(2.75ms)[0m
  [[32mPASS[0m] cte-alias-scalar-subquery-alias-name     [2m(3.10ms)[0m
  [[32mPASS[0m] cte-no-alias-scalar-subquery             [2m(3.09ms)[0m
  [[32mPASS[0m] cte-alias-scalar-subquery-multiple-columns [2m(3.20ms)[0m
  [[32mPASS[0m] cte-multi-ref-union-dedupe               [2m(9.47ms)[0m
  [[32mPASS[0m] cte-alias-scalar-subquery-multiple-ctes  [2m(3.63ms)[0m
  [[32mPASS[0m] cte-cross-ref-scalar-subquery            [2m(4.32ms)[0m
  [[32mPASS[0m] cte-circular-ref-table                   [2m(12.86ms)[0m
  [[32mPASS[0m] subquery-aggregate-empty-result-non-agg-columns [2m(6.44ms)[0m
  [[32mPASS[0m] cte-comma-join-alias                     [2m(4.57ms)[0m
  [[32mPASS[0m] cte-shadow-union-all                     [2m(8.41ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-27                                  [2m(3.06ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-shadows-table-refs-other             [2m(13.46ms)[0m
  [[32mPASS[0m] cte-aggregate-empty-result-non-agg-columns [2m(7.39ms)[0m
  [[32mPASS[0m] cte-where-in-subquery                    [2m(5.92ms)[0m
  [[32mPASS[0m] cte-self-join                            [2m(5.55ms)[0m
  [[32mPASS[0m] cte-exists-self-ref                      [2m(3.55ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-28                                  [2m(3.70ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-multi-ref-union-all                  [2m(12.96ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-29                                  [2m(2.90ms)[0m
  [[32mPASS[0m] glob-30                                  [2m(2.80ms)[0m
  [[32mPASS[0m] glob-33                                  [2m(2.31ms)[0m

[1mtests/cte_expressions.sqltest[0m
  [[32mPASS[0m] cte-distinct-multi-column                [2m(2.53ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-32                                  [2m(2.58ms)[0m
  [[32mPASS[0m] glob-31                                  [2m(2.81ms)[0m

[1mtests/cte_expressions.sqltest[0m
  [[32mPASS[0m] cte-distinct-basic                       [2m(3.28ms)[0m
  [[32mPASS[0m] cte-distinct-with-expression             [2m(2.85ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-34                                  [2m(2.72ms)[0m

[1mtests/cte_expressions.sqltest[0m
  [[32mPASS[0m] cte-group-by-count                       [2m(3.33ms)[0m
  [[32mPASS[0m] cte-group-by-sum                         [2m(2.80ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-35                                  [2m(2.76ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-where-distinct-in                    [2m(8.64ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-36                                  [2m(2.44ms)[0m

[1mtests/cte_expressions.sqltest[0m
  [[32mPASS[0m] cte-having-filter                        [2m(3.35ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-37                                  [2m(2.77ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-triple-ref                           [2m(9.35ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-38                                  [2m(2.75ms)[0m

[1mtests/cte_expressions.sqltest[0m
  [[32mPASS[0m] cte-group-by-expression                  [2m(2.95ms)[0m
  [[32mPASS[0m] cte-having-with-sum                      [2m(3.20ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-39                                  [2m(2.85ms)[0m
  [[32mPASS[0m] glob-40                                  [2m(2.76ms)[0m

[1mtests/cte_expressions.sqltest[0m
  [[32mPASS[0m] cte-offset-inside                        [2m(3.48ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-41                                  [2m(2.67ms)[0m

[1mtests/cte_expressions.sqltest[0m
  [[32mPASS[0m] cte-limit-zero-inside                    [2m(2.92ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-42                                  [2m(2.47ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-scalar-and-from-ref                  [2m(12.09ms)[0m
  [[32mPASS[0m] cte-circular-ref-view                    [2m(22.48ms)[0m

[1mtests/cte_expressions.sqltest[0m
  [[32mPASS[0m] cte-null-in-values                       [2m(3.01ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-left-join-self                       [2m(11.45ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-43                                  [2m(2.71ms)[0m

[1mtests/cte_expressions.sqltest[0m
  [[32mPASS[0m] cte-null-is-null                         [2m(2.54ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-44                                  [2m(2.74ms)[0m
  [[32mPASS[0m] glob-45                                  [2m(2.69ms)[0m

[1mtests/cte_expressions.sqltest[0m
  [[32mPASS[0m] cte-null-is-not-null                     [2m(3.02ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-46                                  [2m(2.74ms)[0m
  [[32mPASS[0m] glob-48                                  [2m(2.46ms)[0m

[1mtests/cte_expressions.sqltest[0m
  [[32mPASS[0m] cte-case-when                            [2m(2.62ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-47                                  [2m(2.79ms)[0m

[1mtests/cte_expressions.sqltest[0m
  [[32mPASS[0m] cte-null-in-aggregate                    [2m(3.40ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-49                                  [2m(2.73ms)[0m

[1mtests/cte_expressions.sqltest[0m
  [[32mPASS[0m] cte-case-simple                          [2m(3.09ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-51                                  [2m(2.40ms)[0m

[1mtests/cte_expressions.sqltest[0m
  [[32mPASS[0m] cte-null-ifnull                          [2m(5.24ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-not-exists-self-ref                  [2m(13.24ms)[0m

[1mtests/cte_expressions.sqltest[0m
  [[32mPASS[0m] cte-case-with-null                       [2m(3.42ms)[0m
  [[32mPASS[0m] cte-arithmetic-expressions               [2m(3.02ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-52                                  [2m(2.86ms)[0m

[1mtests/cte_expressions.sqltest[0m
  [[32mPASS[0m] cte-string-functions                     [2m(3.00ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-53                                  [2m(2.73ms)[0m

[1mtests/cte_expressions.sqltest[0m
  [[32mPASS[0m] cte-concat-expression                    [2m(2.93ms)[0m
  [[32mPASS[0m] cte-null-coalesce                        [2m(7.00ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-54                                  [2m(2.36ms)[0m

[1mtests/cte_expressions.sqltest[0m
  [[32mPASS[0m] cte-abs-and-math                         [2m(2.61ms)[0m
  [[32mPASS[0m] cte-limit-inside                         [2m(9.63ms)[0m
  [[32mPASS[0m] cte-typeof-function                      [2m(2.98ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-55                                  [2m(2.69ms)[0m
  [[32mPASS[0m] glob-50                                  [2m(4.89ms)[0m
  [[32mPASS[0m] glob-56                                  [2m(2.93ms)[0m
  [[32mPASS[0m] glob-57                                  [2m(2.78ms)[0m

[1mtests/cte_expressions.sqltest[0m
  [[32mPASS[0m] cte-window-sum-over                      [2m(3.79ms)[0m
  [[32mPASS[0m] cte-window-count-over                    [2m(3.53ms)[0m
  [[32mPASS[0m] cte-multi-cte-simple-chain               [2m(3.31ms)[0m
  [[32mPASS[0m] cte-multi-cte-join                       [2m(3.29ms)[0m
  [[32mPASS[0m] cte-multi-cte-aggregate-chain            [2m(3.36ms)[0m
  [[32mPASS[0m] cte-in-order-by                          [2m(3.01ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-embedded-nul-1                      [2m(3.48ms)[0m

[1mtests/cte_expressions.sqltest[0m
  [[32mPASS[0m] cte-in-select-subquery                   [2m(3.32ms)[0m
  [[32mPASS[0m] cte-multi-cte-filter-chain               [2m(3.53ms)[0m
  [[32mPASS[0m] cte-in-having-subquery                   [2m(3.18ms)[0m
  [[32mPASS[0m] cte-union-all-with-duplicates            [2m(3.26ms)[0m
  [[32mPASS[0m] cte-single-value                         [2m(2.82ms)[0m
  [[32mPASS[0m] cte-self-join-basic                      [2m(4.57ms)[0m
  [[32mPASS[0m] cte-self-join-inner                      [2m(4.44ms)[0m
  [[32mPASS[0m] cte-self-join-left                       [2m(4.40ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-embedded-nul-2                      [2m(2.92ms)[0m

[1mtests/cte_expressions.sqltest[0m
  [[32mPASS[0m] cte-empty-result                         [2m(3.35ms)[0m
  [[32mPASS[0m] cte-empty-with-aggregate                 [2m(3.06ms)[0m
  [[32mPASS[0m] cte-aliased-expressions                  [2m(3.01ms)[0m
  [[32mPASS[0m] cte-multi-union                          [2m(4.24ms)[0m
  [[32mPASS[0m] cte-union-intersect-combined             [2m(4.86ms)[0m
  [[32mPASS[0m] cte-nested-cte-in-from                   [2m(3.15ms)[0m
  [[32mPASS[0m] cte-window-avg-over                      [2m(7.10ms)[0m
  [[32mPASS[0m] cte-in-subquery-select                   [2m(3.38ms)[0m

[1mtests/glob/default.sqltest[0m
  [[32mPASS[0m] where-glob-another-column                [2m(71.79ms)[0m

[1mtests/cte_expressions.sqltest[0m
  [[32mPASS[0m] cte-between-filter                       [2m(3.15ms)[0m
  [[32mPASS[0m] cte-not-between-filter                   [2m(3.16ms)[0m
  [[32mPASS[0m] cte-comparison-chain                     [2m(3.65ms)[0m
  [[32mPASS[0m] cte-glob-filter                          [2m(3.02ms)[0m
  [[32mPASS[0m] cte-like-filter                          [2m(3.12ms)[0m
  [[32mPASS[0m] cte-like-with-escape                     [2m(3.19ms)[0m

[1mtests/agg-functions/sum-inf-cancel.sqltest[0m
  [[32mPASS[0m] sum_inf_cancel_with_finite_values        [2m(2.51ms)[0m

[1mtests/agg-functions/order-by-aggregate-misuse.sqltest[0m
  [[32mPASS[0m] order-by-agg-with-select-agg-ok          [2m(7.12ms)[0m

[1mtests/cte_expressions.sqltest[0m
  [[32mPASS[0m] cte-join-values-duplicate-keys           [2m(7.74ms)[0m

[1mtests/agg-functions/sum-large-float.sqltest[0m
  [[32mPASS[0m] sum-large-float-infinity                 [2m(2.47ms)[0m

[1mtests/agg-functions/sum-blob-types.sqltest[0m
  [[32mPASS[0m] total-blob-ascii-digit-prefix            [2m(7.47ms)[0m

[1mtests/agg-functions/order-by-aggregate-misuse.sqltest[0m
  [[32mPASS[0m] order-by-agg-with-group-by-ok            [2m(7.80ms)[0m
  [[32mPASS[0m] order-by-scalar-min-ok                   [2m(7.98ms)[0m

[1mtests/agg-functions/sum-blob-types.sqltest[0m
  [[32mPASS[0m] total-blob-non-utf8-with-leading-digit   [2m(7.75ms)[0m
  [[32mPASS[0m] total-blob-long-non-utf8-with-leading-digit [2m(8.15ms)[0m

[1mtests/cte_expressions.sqltest[0m
  [[32mPASS[0m] cte-in-where-subquery                    [2m(15.22ms)[0m

[1mtests/agg-functions/sum-blob-types.sqltest[0m
  [[32mPASS[0m] sum-cast-text-to-blob-returns-real       [2m(8.71ms)[0m

[1mtests/agg-functions/sum-inf-cancel.sqltest[0m
  [[32mPASS[0m] sum_inf_minus_inf                        [2m(8.62ms)[0m

[1mtests/agg-functions/sum-blob-types.sqltest[0m
  [[32mPASS[0m] sum-blob-non-utf8-with-leading-digit     [2m(9.29ms)[0m

[1mtests/agg-functions/sum-large-float.sqltest[0m
  [[32mPASS[0m] avg-large-float-infinity                 [2m(3.02ms)[0m
  [[32mPASS[0m] total-large-float-infinity               [2m(3.22ms)[0m

[1mtests/agg-functions/sum-blob-types.sqltest[0m
  [[32mPASS[0m] total-blob-no-leading-digit              [2m(10.31ms)[0m

[1mtests/agg-functions/sum-inf-cancel.sqltest[0m
  [[32mPASS[0m] avg_inf_minus_inf                        [2m(8.92ms)[0m
  [[32mPASS[0m] total_inf_minus_inf                      [2m(8.86ms)[0m

[1mtests/agg-functions/order-by-aggregate-misuse.sqltest[0m
  [[32mPASS[0m] order-by-sum-misuse                      [2m(11.37ms)[0m
  [[32mPASS[0m] order-by-max-misuse                      [2m(11.77ms)[0m
  [[32mPASS[0m] order-by-min-misuse                      [2m(12.19ms)[0m
  [[32mPASS[0m] order-by-count-misuse                    [2m(11.87ms)[0m

[1mtests/agg-functions/sum-text-types.sqltest[0m
  [[32mPASS[0m] sum-non-numeric-text-returns-real        [2m(5.42ms)[0m
  [[32mPASS[0m] sum-mixed-pure-and-partial-text-returns-real [2m(5.31ms)[0m
  [[32mPASS[0m] sum-partial-integer-text-returns-real    [2m(5.72ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-unenclosed-1                        [2m(2.80ms)[0m
  [[32mPASS[0m] glob-unenclosed-2                        [2m(2.86ms)[0m
  [[32mPASS[0m] glob-unenclosed-4                        [2m(2.35ms)[0m

[1mtests/agg-functions/sum-text-types.sqltest[0m
  [[32mPASS[0m] sum-text-default-partial-numeric         [2m(6.04ms)[0m

[1mtests/glob/memory.sqltest[0m
  [[32mPASS[0m] glob-unenclosed-3                        [2m(2.79ms)[0m

[1mtests/aggregate-bare-column-zero-rows.sqltest[0m
  [[32mPASS[0m] sum-zero-rows-bare-col                   [2m(5.59ms)[0m
  [[32mPASS[0m] count-zero-rows-bare-col                 [2m(5.15ms)[0m
  [[32mPASS[0m] sum-zero-rows-bare-col-with-order-by     [2m(5.36ms)[0m
  [[32mPASS[0m] avg-zero-rows-bare-col                   [2m(5.28ms)[0m
  [[32mPASS[0m] max-zero-rows-bare-col                   [2m(5.39ms)[0m

[1mtests/agg-functions/sum-text-types.sqltest[0m
  [[32mPASS[0m] sum-pure-integer-text-returns-integer    [2m(9.45ms)[0m

[1mtests/aggregate-bare-column-zero-rows.sqltest[0m
  [[32mPASS[0m] sum-matching-rows-bare-col               [2m(8.84ms)[0m

[1mtests/group-by-expression-index.sqltest[0m
  [[32mPASS[0m] group-by-compound-expr-no-index          [2m(5.90ms)[0m

[1mtests/groupby/constant-expr.sqltest[0m
  [[32mPASS[0m] group-by-constant-string-in-select       [2m(5.28ms)[0m

[1mtests/alter-rename-column-temp-trigger.sqltest[0m
  [[31mFAIL[0m] rename-column-with-temp-trigger          [2m(10.44ms)[0m

[1mtests/aggregate-bare-column-zero-rows.sqltest[0m
  [[32mPASS[0m] count-star-zero-rows-multiple-bare-cols  [2m(11.65ms)[0m

[1mtests/alter-table-quoted-trigger-target.sqltest[0m
  [[32mPASS[0m] quoted-target-trigger-still-fires        [2m(9.01ms)[0m

[1mtests/cte_cardinality.sqltest[0m
  [[32mPASS[0m] cte-graph-traversal-correctness          [2m(41.75ms)[0m

[1mtests/alter_rename_column_partial_idx.sqltest[0m
  [[32mPASS[0m] rename-column-updates-partial-index-where-clause-sql [2m(9.36ms)[0m

[1mtests/alter-table-quoted-trigger-target.sqltest[0m
  [[32mPASS[0m] rename-column-with-unrelated-quoted-trigger-target [2m(13.64ms)[0m

[1mtests/alter_rename_column_partial_idx.sqltest[0m
  [[32mPASS[0m] rename-column-partial-index-complex-where [2m(9.73ms)[0m
  [[32mPASS[0m] rename-column-partial-index-update-integrity [2m(11.57ms)[0m
  [[32mPASS[0m] rename-column-partial-index-delete-with-data [2m(12.50ms)[0m

[1mtests/groupby/constant-expr.sqltest[0m
  [[32mPASS[0m] group-by-order-by-null                   [2m(3.83ms)[0m
  [[32mPASS[0m] group-by-constant-expr-in-select         [2m(7.40ms)[0m

[1mtests/alter-table-quoted-trigger-target.sqltest[0m
  [[32mPASS[0m] rename-table-with-unrelated-quoted-trigger-target [2m(14.99ms)[0m

[1mtests/group-by-expression-index.sqltest[0m
  [[32mPASS[0m] group-by-expr-index-basic                [2m(14.19ms)[0m

[1mtests/alter_rename_column_partial_idx.sqltest[0m
  [[32mPASS[0m] rename-column-partial-index-delete-no-panic [2m(14.19ms)[0m

[1mtests/groupby/constant-expr.sqltest[0m
  [[32mPASS[0m] group-by-cte-constant-order-by           [2m(3.02ms)[0m

[1mtests/glob/default.sqltest[0m
  [[32mPASS[0m] where-glob-another-column-prefix         [2m(99.94ms)[0m

[1mtests/groupby/constant-expr.sqltest[0m
  [[32mPASS[0m] group-by-constant-integer-in-select      [2m(13.27ms)[0m

[1mtests/alter-table-quoted-trigger-target.sqltest[0m
  [[32mPASS[0m] drop-column-with-unrelated-quoted-trigger-target [2m(20.88ms)[0m

[1mtests/alter_rename_column_partial_idx.sqltest[0m
  [[32mPASS[0m] rename-column-partial-index-rename-where-col-only [2m(13.15ms)[0m
  [[32mPASS[0m] rename-column-partial-index-insert-after-rename [2m(12.06ms)[0m

[1mtests/groupby/constant-expr.sqltest[0m
  [[32mPASS[0m] group-by-order-by-constant-float         [2m(11.82ms)[0m
  [[32mPASS[0m] group-by-view-constant-order-by          [2m(6.37ms)[0m
  [[32mPASS[0m] group-by-order-by-constant-string        [2m(10.94ms)[0m
  [[32mPASS[0m] group-by-window-constant-order-by        [2m(6.09ms)[0m

[1mtests/alter_rename_column_partial_idx.sqltest[0m
  [[32mPASS[0m] rename-column-updates-expression-index-key [2m(13.10ms)[0m

[1mtests/group-by-expression-index.sqltest[0m
  [[32mPASS[0m] group-by-func-expr-index                 [2m(21.64ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-autoindex-updated     [2m(8.08ms)[0m

[1mtests/alter_rename_column_partial_idx.sqltest[0m
  [[32mPASS[0m] rename-column-expression-index-quoted-ident [2m(10.13ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-then-recreate-with-autoindex [2m(9.29ms)[0m

[1mtests/glob/default.sqltest[0m
  [[32mPASS[0m] where-glob-another-column                [2m(127.57ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-table                 [2m(12.17ms)[0m
  [[32mPASS[0m] alter-table-rename-column-view           [2m(9.43ms)[0m

[1mtests/alter_rename_column_partial_idx.sqltest[0m
  [[32mPASS[0m] rename-column-updates-expression-index-multi-column [2m(19.77ms)[0m
  [[32mPASS[0m] rename-column-expression-index-other-col-unchanged [2m(17.68ms)[0m
  [[32mPASS[0m] rename-column-expression-index-then-rename-again [2m(15.89ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-column-view-qualified [2m(10.34ms)[0m
  [[32mPASS[0m] alter-table-rename-column-view-star      [2m(9.89ms)[0m
  [[32mPASS[0m] alter-table-rename-column-view-subquery  [2m(10.61ms)[0m
  [[32mPASS[0m] alter-table-rename-column-view-on-view   [2m(12.16ms)[0m
  [[32mPASS[0m] alter-table-rename-column-view-correlated-subquery [2m(11.45ms)[0m
  [[32mPASS[0m] alter-table-rename-column                [2m(18.57ms)[0m
  [[32mPASS[0m] alter-table-rename-column-view-correlated-outer-shadow [2m(10.24ms)[0m

[1mtests/delete-correlated-subquery.sqltest[0m
  [[32mPASS[0m] delete-correlated-max-per-group          [2m(6.22ms)[0m

[1mtests/graph_traversal_text_pk.sqltest[0m
  [[32mPASS[0m] cte-neighbor-traversal-correctness       [2m(40.39ms)[0m

[1mtests/delete-correlated-subquery.sqltest[0m
  [[32mPASS[0m] delete-correlated-empty-table            [2m(5.19ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-column-view-select-branch [2m(11.23ms)[0m
  [[32mPASS[0m] alter-table-rename-column-view-correlated-outer-shadow-nested [2m(12.72ms)[0m

[1mtests/graph_traversal_text_pk.sqltest[0m
  [[32mPASS[0m] cte-neighbor-traversal-5-seeds           [2m(42.73ms)[0m

[1mtests/delete-limit-offset.sqltest[0m
  [[32mPASS[0m] delete-limit-offset-basic                [2m(6.24ms)[0m
  [[32mPASS[0m] delete-limit-offset-zero                 [2m(4.82ms)[0m
  [[32mPASS[0m] delete-limit-offset-beyond-rows          [2m(5.51ms)[0m
  [[32mPASS[0m] delete-limit-offset-limit-exceeds        [2m(5.96ms)[0m
  [[32mPASS[0m] delete-limit-offset-with-where           [2m(5.78ms)[0m

[1mtests/delete.sqltest[0m
  [[32mPASS[0m] delete-single-1                          [2m(6.35ms)[0m
  [[32mPASS[0m] delete-insert-alternate-1                [2m(5.71ms)[0m

[1mtests/delete-limit-offset.sqltest[0m
  [[32mPASS[0m] delete-limit-no-offset                   [2m(8.79ms)[0m

[1mtests/delete.sqltest[0m
  [[32mPASS[0m] delete-ends-1                            [2m(7.98ms)[0m

[1mtests/delete-limit-offset.sqltest[0m
  [[32mPASS[0m] delete-limit-offset-multiple             [2m(15.27ms)[0m

[1mtests/delete.sqltest[0m
  [[32mPASS[0m] doubly-qualified-delete                  [2m(5.82ms)[0m
  [[32mPASS[0m] delete-all-with-indexes-1                [2m(9.21ms)[0m
  [[32mPASS[0m] delete-in-subquery-empty-1               [2m(7.16ms)[0m
  [[32mPASS[0m] delete-not-in-subquery-1                 [2m(7.91ms)[0m
  [[32mPASS[0m] delete-in-subquery-1                     [2m(8.16ms)[0m
  [[32mPASS[0m] delete-not-in-subquery-empty-1           [2m(8.26ms)[0m
  [[32mPASS[0m] delete-exists-subquery-1                 [2m(8.31ms)[0m
  [[32mPASS[0m] delete-not-exists-subquery-1             [2m(8.05ms)[0m
  [[32mPASS[0m] delete-scalar-eq-subquery-1              [2m(7.56ms)[0m
  [[32mPASS[0m] delete-scalar-gt-subquery-1              [2m(7.28ms)[0m
  [[32mPASS[0m] delete-reuse-1                           [2m(20.45ms)[0m
  [[32mPASS[0m] delete-scalar-lt-subquery-1              [2m(7.85ms)[0m
  [[32mPASS[0m] delete-scalar-agg-subquery-1             [2m(6.67ms)[0m
  [[32mPASS[0m] delete-in-subquery-multicol-1            [2m(14.35ms)[0m
  [[32mPASS[0m] delete-exists-empty-1                    [2m(7.48ms)[0m
  [[32mPASS[0m] delete_where_falsy                       [2m(23.11ms)[0m

[1mtests/distinct.sqltest[0m
  [[32mPASS[0m] distinct-select-null                     [2m(5.56ms)[0m

[1mtests/detach/transaction.sqltest[0m
  [[32mPASS[0m] detach-succeeds-after-commit             [2m(6.75ms)[0m

[1mtests/glob/default.sqltest[0m
  [[32mPASS[0m] where-glob-another-column-prefix         [2m(173.08ms)[0m

[1mtests/distinct.sqltest[0m
  [[32mPASS[0m] distinct-count-null                      [2m(5.80ms)[0m

[1mtests/delete.sqltest[0m
  [[32mPASS[0m] mvcc-delete-via-index-removes-checkpointed-rows [2m(11.95ms)[0m

[1mtests/distinct.sqltest[0m
  [[32mPASS[0m] distinct-order-by                        [2m(5.40ms)[0m

[1mtests/detach/transaction.sqltest[0m
  [[32mPASS[0m] detach-fails-with-active-read-transaction [2m(11.86ms)[0m
  [[32mPASS[0m] detach-fails-with-active-write-transaction [2m(11.72ms)[0m

[1mtests/distinct.sqltest[0m
  [[32mPASS[0m] distinct-collate-count                   [2m(5.12ms)[0m
  [[32mPASS[0m] distinct-multi-column                    [2m(16.65ms)[0m
  [[32mPASS[0m] distinct-text-nocase                     [2m(7.83ms)[0m
  [[32mPASS[0m] distinct-text-binary                     [2m(5.25ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_two_joined_columns              [2m(82.91ms)[0m

[1mtests/distinct.sqltest[0m
  [[32mPASS[0m] distinct-expression                      [2m(25.86ms)[0m
  [[32mPASS[0m] distinct-order-by-nonselect              [2m(5.26ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_order_by                        [2m(87.38ms)[0m

[1mtests/distinct.sqltest[0m
  [[32mPASS[0m] distinct-agg-group-by                    [2m(30.37ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_no_sorting_required             [2m(2.59ms)[0m

[1mtests/distinct.sqltest[0m
  [[32mPASS[0m] distinct-limit-offset                    [2m(5.36ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_order_by_aggregate              [2m(91.59ms)[0m

[1mtests/distinct.sqltest[0m
  [[32mPASS[0m] distinct-where                           [2m(5.27ms)[0m
  [[32mPASS[0m] distinct-offset-applies-after-dedup      [2m(9.48ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] complex_result_expression_containing_aggregate [2m(1.41ms)[0m
  [[32mPASS[0m] group_by_no_sorting_required_and_const_agg_arg [2m(11.22ms)[0m
  [[32mPASS[0m] complex_result_expression_containing_aggregate_and_rowid [2m(1.28ms)[0m

[1mtests/distinct.sqltest[0m
  [[32mPASS[0m] distinct-agg-with-having                 [2m(5.78ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] complex_having_expression_containing_aggregate [2m(1.32ms)[0m

[1mtests/distinct.sqltest[0m
  [[32mPASS[0m] distinct-empty                           [2m(6.90ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] complex_order_by_expression_containing_aggregate [2m(1.68ms)[0m

[1mtests/distinct.sqltest[0m
  [[32mPASS[0m] distinct-order-by-expression             [2m(10.44ms)[0m
  [[32mPASS[0m] distinct-multi-null-keys                 [2m(12.42ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] more_args_than_aggregates                [2m(1.37ms)[0m

[1mtests/distinct.sqltest[0m
  [[32mPASS[0m] distinct-agg-simple-count                [2m(11.84ms)[0m
  [[32mPASS[0m] distinct-subquery                        [2m(18.19ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_count_star                      [2m(122.38ms)[0m

[1mtests/distinct.sqltest[0m
  [[32mPASS[0m] distinct-exists-with-offset              [2m(10.23ms)[0m

[1mtests/drop_index.sqltest[0m
  [[32mPASS[0m] drop-index-basic-1                       [2m(5.68ms)[0m
  [[32mPASS[0m] drop-index-if-exists-1                   [2m(5.90ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] column_alias_in_group_by_order_by_having [2m(83.07ms)[0m

[1mtests/drop_index.sqltest[0m
  [[32mPASS[0m] drop-index-if-exists-2                   [2m(3.49ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by                                 [2m(165.29ms)[0m
  [[32mPASS[0m] group_by_and_binary_expression_that_depends_on_two_aggregates [2m(155.33ms)[0m
  [[32mPASS[0m] group_by_without_aggs                    [2m(172.54ms)[0m

[1mtests/drop_index.sqltest[0m
  [[32mPASS[0m] drop-index-no-index                      [2m(11.18ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_multiple_aggregates_2           [2m(172.47ms)[0m

[1mtests/drop_index.sqltest[0m
  [[32mPASS[0m] drop-index-primary-key-index             [2m(9.38ms)[0m
  [[32mPASS[0m] drop-index-after-ops-1                   [2m(15.80ms)[0m
  [[32mPASS[0m] drop-index-unique-index                  [2m(6.28ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_function_expression             [2m(191.74ms)[0m
  [[32mPASS[0m] group_by_count_no_args_in_expression     [2m(152.70ms)[0m
  [[32mPASS[0m] groupby_orderby_removal_regression_test  [2m(123.88ms)[0m

[1mtests/drop_index.sqltest[0m
  [[32mPASS[0m] drop-explicit-unique-index               [2m(6.95ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_count_star_in_expression        [2m(160.67ms)[0m

[1mtests/drop_index.sqltest[0m
  [[32mPASS[0m] drop-index-user-unique-if-exists         [2m(6.35ms)[0m

[1mtests/after-insert-trigger-cache-spill.sqltest[0m
  [[32mPASS[0m] after-insert-trigger-cache-spill-with-index [2m(646.91ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_column_number                   [2m(146.31ms)[0m

[1mtests/drop_index.sqltest[0m
  [[32mPASS[0m] drop-index-user-unique-data-intact       [2m(7.76ms)[0m
  [[32mPASS[0m] drop-index-user-unique-composite         [2m(7.58ms)[0m

[1mtests/drop_table.sqltest[0m
  [[32mPASS[0m] drop-table-basic-1                       [2m(6.38ms)[0m
  [[32mPASS[0m] drop-table-case-insensitive              [2m(5.84ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] proper-sort-order                        [2m(121.65ms)[0m
  [[32mPASS[0m] having                                   [2m(200.70ms)[0m

[1mtests/drop_table.sqltest[0m
  [[32mPASS[0m] drop-table-if-exists-1                   [2m(13.71ms)[0m
  [[32mPASS[0m] drop-table-if-exists-2                   [2m(3.20ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] having_with_scalar_fn_over_aggregate     [2m(202.58ms)[0m

[1mtests/drop_table.sqltest[0m
  [[32mPASS[0m] drop-table-after-ops-1                   [2m(12.35ms)[0m
  [[32mPASS[0m] drop-table-schema-cleanup-1              [2m(16.10ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_without_aggs                    [2m(134.86ms)[0m
  [[32mPASS[0m] distinct_agg_functions                   [2m(182.07ms)[0m

[1mtests/drop_table.sqltest[0m
  [[32mPASS[0m] drop-table-with-index-1                  [2m(20.84ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_function_expression_ridiculous  [2m(268.16ms)[0m

[1mtests/drop_table.sqltest[0m
  [[32mPASS[0m] drop-table-fk-disabled-ok                [2m(8.81ms)[0m

[1mtests/duplicate-trigger-names.sqltest[0m
  [[32mPASS[0m] duplicate-trigger-name-different-tables  [2m(15.40ms)[0m

[1mtests/drop_table.sqltest[0m
  [[32mPASS[0m] drop-table-fk-constraint-failed          [2m(16.52ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] having_with_binary_cond                  [2m(236.07ms)[0m

[1mtests/duplicate-trigger-names.sqltest[0m
  [[32mPASS[0m] correct-trigger-fires-after-failed-duplicate [2m(9.80ms)[0m
  [[32mPASS[0m] duplicate-trigger-if-not-exists          [2m(18.28ms)[0m
  [[32mPASS[0m] duplicate-trigger-name-same-table        [2m(19.86ms)[0m

[1mtests/expr-index-correlated-subquery.sqltest[0m
  [[32mPASS[0m] expr-index-correlated-exists-mixed-select [2m(7.88ms)[0m
  [[32mPASS[0m] expr-index-correlated-not-exists         [2m(8.32ms)[0m
  [[32mPASS[0m] expr-index-correlated-exists-empty       [2m(4.14ms)[0m
  [[32mPASS[0m] expr-index-correlated-in-subquery        [2m(9.23ms)[0m
  [[32mPASS[0m] expr-index-correlated-exists-select-non-indexed-col [2m(14.36ms)[0m

[1mtests/duplicate-trigger-names.sqltest[0m
  [[32mPASS[0m] trigger-name-case-insensitive            [2m(24.42ms)[0m

[1mtests/expr-index-correlated-subquery.sqltest[0m
  [[32mPASS[0m] expr-index-correlated-exists             [2m(18.41ms)[0m
  [[32mPASS[0m] complex-expr-index-correlated-exists     [2m(7.48ms)[0m
  [[32mPASS[0m] expr-index-multiple-correlated-cols      [2m(5.47ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-column-constraint-multi-parent        [2m(2.97ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_multiple_aggregates             [2m(310.57ms)[0m

[1mtests/expr-index-correlated-subquery.sqltest[0m
  [[32mPASS[0m] expr-index-scalar-correlated-subquery    [2m(8.66ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_complex_order_by                [2m(308.64ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-basic-ok                              [2m(6.66ms)[0m
  [[32mPASS[0m] fk-insert-child-missing-parent           [2m(4.00ms)[0m

[1mtests/expr-index-correlated-subquery.sqltest[0m
  [[32mPASS[0m] expr-index-non-unique                    [2m(11.91ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-update-child-to-missing-parent        [2m(6.33ms)[0m
  [[32mPASS[0m] fk-delete-parent-blocked                 [2m(8.12ms)[0m

[1mtests/expr-index-correlated-subquery.sqltest[0m
  [[32mPASS[0m] expr-index-multiple-correlated-subqueries [2m(18.18ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_complex_order_by_2              [2m(315.11ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-update-child-to-null-ok               [2m(9.64ms)[0m
  [[32mPASS[0m] fk-composite-pk-ok                       [2m(7.48ms)[0m
  [[32mPASS[0m] fk-composite-pk-missing                  [2m(7.11ms)[0m
  [[32mPASS[0m] fk-delete-parent-ok-when-no-child        [2m(9.27ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_two_joined_columns              [2m(179.38ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-composite-update-child-missing        [2m(7.51ms)[0m
  [[32mPASS[0m] fk-rowid-alias-parent-missing            [2m(10.34ms)[0m

[1mtests/expr-index-correlated-subquery.sqltest[0m
  [[32mPASS[0m] expr-index-correlated-in-subquery-duplicates-and-filter [2m(26.28ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] having_with_multiple_conditions          [2m(276.13ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-rowid-alias-parent                    [2m(13.11ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_alter_add_column_default_zero_violates_check [2m(3.91ms)[0m
  [[32mPASS[0m] check_constraint_alter_add_column_default_violates_check_empty_table [2m(13.64ms)[0m
  [[32mPASS[0m] check_constraint_alter_add_column_null_default_passes_check [2m(12.72ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-composite-unique-ok                   [2m(16.67ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by                                 [2m(208.80ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_alter_add_column_default_satisfies_check [2m(13.14ms)[0m
  [[32mPASS[0m] check_constraint_alter_add_column_default_string_satisfies_check [2m(6.28ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-composite-unique-missing              [2m(19.33ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_update_table_level_multi_column [2m(4.91ms)[0m
  [[32mPASS[0m] check_constraint_alter_add_column_default_string_violates_check [2m(10.49ms)[0m
  [[32mPASS[0m] check_constraint_integrity_check_detects_real_violation [2m(5.12ms)[0m
  [[32mPASS[0m] check_constraint_integrity_check_null_not_violation [2m(6.47ms)[0m
  [[32mPASS[0m] check_constraint_keyword_constraint_name_quoting_after_alter [2m(7.45ms)[0m
  [[32mPASS[0m] check_constraint_update_only_checks_changed_columns [2m(11.96ms)[0m
  [[32mPASS[0m] check_constraint_update_table_level_triggers_on_referenced_col [2m(9.42ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_order_by                        [2m(196.29ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_update_enforces_check_on_changed_column [2m(11.35ms)[0m

[1mtests/coalesce/default.sqltest[0m
  [[32mPASS[0m] coalesce-from-table                      [2m(710.40µs)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_text_affinity_coercion_pass [2m(7.83ms)[0m

[1mtests/coalesce/default.sqltest[0m
  [[32mPASS[0m] coalesce-from-table-column               [2m(861.10µs)[0m
  [[32mPASS[0m] coalesce-from-table-column               [2m(3.74ms)[0m
  [[32mPASS[0m] coalesce-from-table                      [2m(3.92ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_inline_rename_then_drop_keeps_renamed_ref [2m(26.60ms)[0m

[1mtests/coalesce/default.sqltest[0m
  [[32mPASS[0m] coalesce-from-table-multiple-columns     [2m(2.79ms)[0m

[1mtests/coalesce/memory.sqltest[0m
  [[32mPASS[0m] coalesce                                 [2m(2.62ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_insert_select_validates_destination_rowid [2m(11.17ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_multiple_aggregates_2           [2m(166.47ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_text_affinity_coercion_update_violation [2m(9.31ms)[0m

[1mtests/coalesce/memory.sqltest[0m
  [[32mPASS[0m] coalesce-nested                          [2m(2.81ms)[0m

[1mtests/coalesce/default.sqltest[0m
  [[32mPASS[0m] coalesce-from-table-multiple-columns     [2m(3.90ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_integer_affinity_coercion_violation [2m(7.78ms)[0m

[1mtests/coalesce/memory.sqltest[0m
  [[32mPASS[0m] coalesce-nested-2                        [2m(3.04ms)[0m

[1mtests/check_constraint.sqltest[0m
  [[32mPASS[0m] check_constraint_integer_affinity_coercion [2m(8.46ms)[0m

[1mtests/coalesce/memory.sqltest[0m
  [[32mPASS[0m] coalesce-null                            [2m(3.76ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_nocase                           [2m(2.95ms)[0m
  [[32mPASS[0m] collate_binary_1                         [2m(2.70ms)[0m

[1mtests/coalesce/memory.sqltest[0m
  [[32mPASS[0m] coalesce-2                               [2m(5.80ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_binary_2                         [2m(2.79ms)[0m

[1mtests/coalesce/memory.sqltest[0m
  [[32mPASS[0m] coalesce-first                           [2m(5.97ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_rtrim_1                          [2m(5.98ms)[0m

[1mtests/coalesce/memory.sqltest[0m
  [[32mPASS[0m] coalesce-constant-in-orderby-regression  [2m(11.46ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_order_by_aggregate              [2m(197.92ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-column-subquery       [2m(12.46ms)[0m
  [[32mPASS[0m] alter-table-rename-then-rename-column-trigger [2m(12.79ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_function_expression             [2m(155.61ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-column-scoping-other-table [2m(15.20ms)[0m
  [[32mPASS[0m] alter-table-rename-column-old-ref-trigger [2m(14.90ms)[0m
  [[32mPASS[0m] alter-table-rename-column-multiple-triggers [2m(15.49ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_count_star                      [2m(156.11ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-trigger-exists-subquery [2m(26.72ms)[0m
  [[32mPASS[0m] alter-table-rename-column-cross-table-insert [2m(14.49ms)[0m
  [[32mPASS[0m] alter-table-rename-column-union-trigger  [2m(24.39ms)[0m
  [[32mPASS[0m] alter-table-rename-column-trigger-order-by-alias [2m(15.30ms)[0m
  [[32mPASS[0m] alter-table-rename-column-trigger-order-by-alias-compound [2m(17.22ms)[0m
  [[32mPASS[0m] alter-table-rename-column-case-expr-trigger [2m(32.36ms)[0m
  [[32mPASS[0m] alter-table-rename-column-trigger-order-by-implicit-alias [2m(15.15ms)[0m
  [[32mPASS[0m] alter-table-rename-column-trigger-order-by-alias-stored-sql [2m(25.32ms)[0m
  [[32mPASS[0m] alter-table-rename-column-trigger-order-by-not-alias [2m(18.67ms)[0m
  [[32mPASS[0m] alter-table-rename-column-trigger-order-by-implicit-alias-stored-sql [2m(19.74ms)[0m
  [[32mPASS[0m] alter-table-rename-column-trigger-order-by-qualified-result-col [2m(16.38ms)[0m
  [[32mPASS[0m] alter-table-rename-column-trigger-round-trip-when-clause [2m(21.27ms)[0m
  [[32mPASS[0m] alter-table-rename-column-temp-trigger-main-cross-table [2m(15.06ms)[0m
  [[32mPASS[0m] alter-table-rename-column-trigger-round-trip-stored-sql [2m(26.66ms)[0m
  [[32mPASS[0m] alter-table-rename-column-trigger-round-trip-update-of [2m(25.23ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_count_star_in_expression        [2m(177.20ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-column-trigger-round-trip-fires [2m(22.11ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_rtrim_2                          [2m(2.90ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-table-rewrites-temp-trigger-on-temp-table [2m(16.68ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] column_alias_in_group_by_order_by_having [2m(63.38ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_rtrim_3                          [2m(2.14ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-temp-table-invalid-temp-trigger-missing-main-table [2m(21.05ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_and_binary_expression_that_depends_on_two_aggregates [2m(211.98ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-column-temp-table-trigger-main-delete-target [2m(28.96ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_rtrim_4                          [2m(2.64ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-column-temp-trigger-exists-result-expr-error [2m(23.39ms)[0m
  [[32mPASS[0m] alter-table-rename-column-temp-table-rejects-invalid-trigger-rewrite [2m(20.56ms)[0m
  [[32mPASS[0m] temp-trigger-attached-read-cross-table   [2m(21.27ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_left_precedence                  [2m(2.89ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-drop-column-temp-trigger-invalid [2m(17.18ms)[0m
  [[32mPASS[0m] alter-rename-col-cross-table-aggregate   [2m(20.66ms)[0m
  [[32mPASS[0m] alter-table-rename-column-temp-trigger-result-scalar-subquery [2m(28.27ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] complex_result_expression_containing_aggregate [2m(1.32ms)[0m
  [[32mPASS[0m] group_by_count_no_args_in_expression     [2m(141.87ms)[0m
  [[32mPASS[0m] complex_result_expression_containing_aggregate_and_rowid [2m(1.14ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-drop-column-trigger-bare-owning-column-in-when-invalid [2m(11.61ms)[0m
  [[32mPASS[0m] alter-table-drop-column-invalid-temp-trigger-missing-main-table [2m(18.04ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_nocase_embedded_nul              [2m(2.55ms)[0m
  [[32mPASS[0m] collate_left_precedence_2                [2m(4.70ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] complex_having_expression_containing_aggregate [2m(1.20ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-column-trigger-multi-hop-rename [2m(54.63ms)[0m
  [[32mPASS[0m] alter-table-drop-column-trigger-bare-owning-column-in-update-invalid [2m(13.97ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] complex_order_by_expression_containing_aggregate [2m(1.26ms)[0m
  [[32mPASS[0m] more_args_than_aggregates                [2m(1.44ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-column-temp-trigger-attached-cross-table [2m(37.05ms)[0m
  [[32mPASS[0m] alter-table-rename-column-temp-trigger-main-alias-cross-table [2m(52.32ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_unique_constraint-2              [2m(4.91ms)[0m
  [[32mPASS[0m] collate_aggregation_default_binary       [2m(5.31ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-drop-column-cross-table-trigger-select-surviving-column [2m(15.82ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_unique_constraint                [2m(8.22ms)[0m
  [[32mPASS[0m] collate_aggregation_default_nocase       [2m(5.04ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-drop-column-temp-trigger-select-surviving-column [2m(16.70ms)[0m
  [[32mPASS[0m] alter-rename-col-cross-table-qualified-ref [2m(15.69ms)[0m

[1mtests/groupby/duplicate-order-by.sqltest[0m
  [[32mPASS[0m] duplicate_order_by_desc_with_group_by    [2m(5.18ms)[0m
  [[32mPASS[0m] duplicate_order_by_with_group_by         [2m(5.58ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_aggregation_explicit_binary      [2m(4.82ms)[0m

[1mtests/groupby/duplicate-order-by.sqltest[0m
  [[32mPASS[0m] triple_duplicate_order_by_with_group_by  [2m(5.10ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-rename-col-cross-table-multi-from  [2m(17.35ms)[0m
  [[32mPASS[0m] alter-rename-col-cross-table-join-on     [2m(17.61ms)[0m

[1mtests/groupby/memory.sqltest[0m
  [[32mPASS[0m] having_or                                [2m(5.96ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_grouped_aggregation_default_binary [2m(4.90ms)[0m

[1mtests/groupby/memory.sqltest[0m
  [[32mPASS[0m] group_by_expr_alias_shadows_column       [2m(4.98ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_aggregation_explicit_nocase      [2m(8.44ms)[0m

[1mtests/groupby/memory.sqltest[0m
  [[32mPASS[0m] group_by_column_over_alias               [2m(7.80ms)[0m
  [[32mPASS[0m] group_by_no_sorting_required_reordered_columns [2m(11.92ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_multiple_aggregates             [2m(289.93ms)[0m

[1mtests/groupby/memory.sqltest[0m
  [[32mPASS[0m] group_by_swapped_aliases                 [2m(4.96ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_grouped_aggregation_default_nocase [2m(5.24ms)[0m

[1mtests/groupby/memory.sqltest[0m
  [[32mPASS[0m] group_by_column_having_alias             [2m(5.18ms)[0m
  [[32mPASS[0m] group_by_nonconflicting_alias            [2m(10.56ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_grouped_aggregation_explicit_binary [2m(5.35ms)[0m

[1mtests/groupby/memory.sqltest[0m
  [[32mPASS[0m] group_by_numeric_index_vs_alias          [2m(5.02ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-rename-col-cross-table-nested-subquery [2m(15.85ms)[0m
  [[32mPASS[0m] alter-rename-col-no-rename-other-table-col [2m(15.41ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_groupped_aggregation_explicit_nocase [2m(5.34ms)[0m

[1mtests/groupby/memory.sqltest[0m
  [[32mPASS[0m] group_by_alias_precedence                [2m(6.00ms)[0m
  [[32mPASS[0m] having_without_group_by_basic            [2m(4.99ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-rename-col-cross-table-update-cmd  [2m(13.41ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] groupby_orderby_removal_regression_test  [2m(118.53ms)[0m

[1mtests/groupby/memory.sqltest[0m
  [[32mPASS[0m] having_without_group_by_multiple_aggregates [2m(6.53ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_join_nocase                      [2m(7.11ms)[0m

[1mtests/groupby/memory.sqltest[0m
  [[32mPASS[0m] having_without_group_by_expression       [2m(5.10ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-rename-col-cross-table-insert-col-list [2m(11.55ms)[0m

[1mtests/groupby/memory.sqltest[0m
  [[32mPASS[0m] having_without_group_by_no_match         [2m(9.65ms)[0m
  [[32mPASS[0m] having_without_group_by_count            [2m(5.19ms)[0m
  [[32mPASS[0m] having_without_group_by_complex_condition [2m(6.69ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_hash_join_nocase_column_with_order_by [2m(7.99ms)[0m

[1mtests/groupby/memory.sqltest[0m
  [[32mPASS[0m] having_without_group_by_non_aggregate_error [2m(4.69ms)[0m
  [[32mPASS[0m] having_without_group_by_max_min          [2m(5.32ms)[0m
  [[32mPASS[0m] group_by_where_false                     [2m(4.72ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_column_number                   [2m(128.13ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_hash_join_nocase_column_without_order_by [2m(7.70ms)[0m

[1mtests/groupby/memory.sqltest[0m
  [[32mPASS[0m] groupby_orderby_agg_not_in_select        [2m(6.56ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-rename-col-verify-schema-rewritten [2m(17.00ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_cached_aggregate_result_does_not_inherit_argument_collation [2m(5.01ms)[0m

[1mtests/groupby/memory.sqltest[0m
  [[32mPASS[0m] groupby_orderby_multiple_keys            [2m(6.02ms)[0m
  [[32mPASS[0m] groupby_orderby_different_key_counts     [2m(5.51ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_cached_window_result_does_not_inherit_argument_collation [2m(5.69ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-rename-col-cross-table-delete-where [2m(26.88ms)[0m

[1mtests/groupby/memory.sqltest[0m
  [[32mPASS[0m] groupby_orderby_multiple_keys_limit      [2m(6.33ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-rename-col-schema-aggregate        [2m(16.98ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_select_comparison_survives_unrelated_hash_join_keys [2m(14.58ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-rename-col-schema-qualified-ref    [2m(15.86ms)[0m

[1mtests/groupby/memory.sqltest[0m
  [[32mPASS[0m] groupby_orderby_same_column_in_both      [2m(5.95ms)[0m
  [[32mPASS[0m] groupby_orderby_more_orderby_keys        [2m(8.29ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-rename-col-cross-table-group-by    [2m(23.36ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_aggregate_step_does_not_leak_to_following_scalar_result [2m(6.27ms)[0m

[1mtests/groupby/memory.sqltest[0m
  [[32mPASS[0m] groupby_desc_tiebreak_extra_columns      [2m(5.40ms)[0m
  [[32mPASS[0m] groupby_desc_tiebreak_extra_columns_limit_offset [2m(5.45ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-rename-col-schema-multi-from       [2m(18.56ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_function_expression_ridiculous  [2m(281.90ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_hash_join_nocase_build_side_select_eq [2m(10.59ms)[0m

[1mtests/groupby/memory.sqltest[0m
  [[32mPASS[0m] group-by-out-of-range                    [2m(9.65ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-rename-col-schema-join-on          [2m(20.18ms)[0m

[1mtests/groupby/memory.sqltest[0m
  [[32mPASS[0m] group-by-negative                        [2m(8.89ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-rename-col-schema-no-rename-other-col [2m(18.38ms)[0m
  [[32mPASS[0m] alter-rename-col-schema-update-cmd       [2m(15.73ms)[0m

[1mtests/hash-join-three-table-nested.sqltest[0m
  [[32mPASS[0m] three-table-hash-join-in-union-all       [2m(9.78ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-rename-col-schema-delete-where     [2m(10.72ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_three_way_join_different_collations [2m(8.87ms)[0m

[1mtests/groupby/memory.sqltest[0m
  [[32mPASS[0m] group-by-zero                            [2m(14.59ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-rename-col-schema-update-cmd-from  [2m(16.71ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_hash_join_nocase_probe_side      [2m(15.84ms)[0m
  [[32mPASS[0m] collate_multiple_nocase_comparisons_in_select [2m(5.36ms)[0m

[1mtests/hash-join-three-table-nested.sqltest[0m
  [[32mPASS[0m] three-table-hash-join-in-union           [2m(10.15ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-rename-col-schema-insert-col-list  [2m(14.79ms)[0m

[1mtests/hash-join-three-table-nested.sqltest[0m
  [[32mPASS[0m] three-table-hash-join-in-intersect       [2m(10.36ms)[0m
  [[32mPASS[0m] three-table-hash-join-in-except          [2m(10.10ms)[0m
  [[32mPASS[0m] three-table-hash-join-in-cte             [2m(10.34ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_nocase_col_vs_literal_after_hash_join [2m(6.98ms)[0m
  [[32mPASS[0m] collate_aggregate_and_scalar_nocase_in_same_select [2m(5.67ms)[0m

[1mtests/hash-join-three-table-nested.sqltest[0m
  [[32mPASS[0m] three-table-hash-join-in-view            [2m(10.64ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_multiple_aggs_no_cross_leak      [2m(4.92ms)[0m

[1mtests/hash-join-three-table-nested.sqltest[0m
  [[32mPASS[0m] three-table-hash-join-top-level          [2m(6.97ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_self_join_nocase                 [2m(13.88ms)[0m
  [[32mPASS[0m] collate_window_then_scalar_nocase        [2m(6.00ms)[0m
  [[32mPASS[0m] collate_having_aggregate_nocase_select_scalar [2m(5.52ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-rename-col-schema-group-by         [2m(18.21ms)[0m
  [[32mPASS[0m] fail-drop-column-cross-table-trigger-update-where-expr [2m(12.57ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_explicit_override_in_hash_join_select [2m(7.06ms)[0m
  [[32mPASS[0m] collate_left_join_nocase_nullable_side   [2m(7.21ms)[0m

[1mtests/hash-join-three-table-nested.sqltest[0m
  [[32mPASS[0m] three-table-hash-join-wide-cte           [2m(10.44ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] fail-drop-column-cross-table-trigger-update-where-only [2m(12.28ms)[0m

[1mtests/hex-real-compare.sqltest[0m
  [[32mPASS[0m] compare-hex-real-le-delete-regression    [2m(6.26ms)[0m

[1mtests/hash-join-three-table-nested.sqltest[0m
  [[32mPASS[0m] three-table-hash-join-wide-union-all     [2m(10.19ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_union_all_nocase_branch          [2m(6.80ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] fail-drop-column-cross-table-trigger-set-value-expr [2m(12.45ms)[0m
  [[32mPASS[0m] drop-column-cross-table-trigger-insert-col-list-deferred [2m(9.51ms)[0m
  [[32mPASS[0m] alter-rename-col-schema-same-table-trigger [2m(23.66ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_in_operator_nocase               [2m(4.71ms)[0m
  [[32mPASS[0m] collate_subquery_from_nocase_outer_comparison [2m(10.01ms)[0m
  [[32mPASS[0m] collate_case_when_nocase_after_join      [2m(7.19ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-rename-col-schema-nested-subquery  [2m(28.97ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_complex_order_by                [2m(337.94ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_between_nocase                   [2m(4.97ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] rowid-in-basic                           [2m(9.44ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_group_by_nocase_with_scalar_comparison [2m(5.40ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] drop-column-cross-table-trigger-set-target-deferred [2m(11.45ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] index-in-basic                           [2m(9.18ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_coalesce_nocase                  [2m(5.57ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-no-match                              [2m(9.08ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_complex_order_by_2              [2m(329.26ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] fail-drop-column-cross-table-trigger-delete-where [2m(12.47ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_no_sorting_required             [2m(123.35ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-single-value                          [2m(10.09ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] drop-column-cross-table-trigger-unreferenced [2m(10.58ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-single-value-index                    [2m(8.51ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_count_star_with_nocase_comparison [2m(9.48ms)[0m
  [[32mPASS[0m] collate_numeric_aggregate_no_collation_leak [2m(6.72ms)[0m
  [[32mPASS[0m] collate_nested_expr_nocase_hash_join     [2m(8.52ms)[0m
  [[32mPASS[0m] collate_hash_join_order_by_nocase_inequality [2m(12.79ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-with-nulls                            [2m(11.99ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] fail-drop-column-trigger-invalid-col-ref-other-table [2m(17.29ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_columns_where_implicit           [2m(8.39ms)[0m
  [[32mPASS[0m] collate_group_concat_no_inherit_nocase   [2m(9.57ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] drop-column-other-table-trigger-insert-col-deferred [2m(17.09ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_window_partition_then_scalar_nocase [2m(14.36ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-duplicates                            [2m(10.46ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_nocase_join_predicate_and_select_comparison [2m(11.88ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] drop-column-owning-table-trigger-insert-col-deferred [2m(16.67ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_group_by_repeated_col_ref_after_agg [2m(13.60ms)[0m
  [[32mPASS[0m] collate_interleaved_agg_and_nocase_scalars [2m(12.64ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-with-nulls-index                      [2m(16.10ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] drop-column-owning-table-trigger-update-set-deferred [2m(15.67ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_mixed_nocase_rtrim_hash_join     [2m(14.15ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] group_by_no_sorting_required_and_const_agg_arg [2m(130.23ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_order_by_binary                  [2m(5.13ms)[0m
  [[32mPASS[0m] collate_multiple_windows_nocase_interleave [2m(17.62ms)[0m
  [[32mPASS[0m] collate_columns_where_explicit_override  [2m(8.66ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] fail-drop-column-when-clause-new-ref     [2m(11.82ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_order_by_explicit_override       [2m(4.95ms)[0m
  [[32mPASS[0m] collate_unique_nocase_conflict           [2m(5.44ms)[0m
  [[32mPASS[0m] collate_unique_rtrim_conflict            [2m(4.70ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] fail-drop-column-new-qualified-ref-in-set [2m(12.29ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] having                                   [2m(232.68ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_order_by_nocase                  [2m(8.77ms)[0m
  [[32mPASS[0m] collate_pk_rtrim_conflict                [2m(5.21ms)[0m
  [[32mPASS[0m] collate_unique_binary_allows_case        [2m(6.10ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-duplicates-index                      [2m(13.25ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_distinct_rtrim                   [2m(9.08ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] fail-drop-column-insert-values-new-ref   [2m(12.81ms)[0m
  [[32mPASS[0m] fail-drop-column-raise-when-new-ref      [2m(9.68ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_join_implicit_nocase_columns     [2m(6.67ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] fail-drop-column-table-qualified-ref     [2m(10.95ms)[0m
  [[32mPASS[0m] fail-drop-column-subquery-in-delete-where [2m(12.63ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_join_mixed_implicit_binary_left  [2m(7.27ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] fail-drop-column-exists-subquery-ref     [2m(12.22ms)[0m
  [[32mPASS[0m] fail-drop-column-multiple-triggers-one-bad [2m(13.09ms)[0m
  [[32mPASS[0m] fail-drop-column-insert-select-ref       [2m(15.99ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_join_mixed_explicit_nocase_left  [2m(9.96ms)[0m
  [[32mPASS[0m] collate_where_with_and_without_explicit  [2m(8.97ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] not-in-fallback                          [2m(13.88ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-add-column-nonconstant-default-empty-table [2m(9.13ms)[0m
  [[32mPASS[0m] drop-column-unreferenced-with-complex-trigger [2m(13.61ms)[0m
  [[32mPASS[0m] fail-drop-column-coalesce-ref            [2m(12.61ms)[0m
  [[32mPASS[0m] fail-drop-column-in-expr-ref             [2m(12.07ms)[0m
  [[32mPASS[0m] fail-drop-column-between-ref             [2m(12.03ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-sequence-table-is-initially-empty [2m(8.36ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] fail-drop-column-case-expr-ref           [2m(12.05ms)[0m
  [[32mPASS[0m] alter-table-rename-then-rename-column-with-trigger [2m(11.81ms)[0m
  [[32mPASS[0m] fail-drop-column-aggregate-subquery-cross-table [2m(16.37ms)[0m
  [[32mPASS[0m] fail-drop-column-nested-function-ref     [2m(16.88ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-fail-index-sequence-table        [2m(9.48ms)[0m
  [[32mPASS[0m] autoinc-larger-insert-updates-sequence   [2m(7.53ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] fail-drop-column-cross-table-subquery-in-insert [2m(22.84ms)[0m
  [[32mPASS[0m] fail-drop-column-compound-expr-ref       [2m(16.61ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-null-insert-increments-sequence  [2m(10.57ms)[0m
  [[32mPASS[0m] autoinc-first-insert-populates-sequence  [2m(10.72ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-add-column-nonconstant-default-nonempty-table-errors [2m(16.68ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-sequence-not-decremented-by-delete [2m(9.34ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_drop_column_rebinds_generated_after_preceding_drop [2m(7.41ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-create-sequence-table            [2m(17.00ms)[0m

[1mtests/gencol.sqltest[0m
  [[33mSKIP[0m] gencol_alter_add_column_generated_default_rejected [2m(3.01µs)[0m [2m(TODO: parser accepts DEFAULT + GENERATED without error)[0m
  [[32mPASS[0m] gencol_drop_virtual_column_itself        [2m(6.92ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-order-by-indexed-col                  [2m(11.83ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-fail-drop-sequence-table         [2m(17.77ms)[0m
  [[32mPASS[0m] autoinc-manual-delete-from-sqlite-sequence [2m(9.93ms)[0m
  [[32mPASS[0m] autoinc-manual-update-to-sequence-table  [2m(11.57ms)[0m
  [[32mPASS[0m] autoinc-drop-table-removes-sequence-entry [2m(10.93ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_alter_add_virtual                 [2m(8.43ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] distinct_agg_functions                   [2m(153.81ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_alter_add_virtual_complex_expr    [2m(5.78ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-delete-all-does-not-reset-sequence [2m(13.55ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_drop_last_non_generated_error     [2m(11.04ms)[0m
  [[32mPASS[0m] gencol_drop_only_regular_column_error    [2m(11.88ms)[0m
  [[32mPASS[0m] gencol_alter_add_multiple_virtual        [2m(8.98ms)[0m
  [[32mPASS[0m] gencol_alter_add_virtual_check_with_existing_virtual_middle [2m(5.18ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] having_with_binary_cond                  [2m(234.05ms)[0m
  [[32mPASS[0m] having_with_scalar_fn_over_aggregate     [2m(234.58ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_alter_add_virtual_insert_after    [2m(5.33ms)[0m
  [[32mPASS[0m] gencol_alter_add_virtual_not_null_ok     [2m(7.51ms)[0m
  [[32mPASS[0m] gencol_alter_add_virtual_not_null_violation [2m(8.56ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-insert-after-delete-uses-high-water-mark [2m(18.46ms)[0m
  [[32mPASS[0m] autoinc-multiple-tables                  [2m(17.43ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_alter_add_virtual_check_violation [2m(8.55ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] proper-sort-order                        [2m(143.68ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_alter_add_virtual_check_with_existing_virtual_middle_violation [2m(8.76ms)[0m
  [[33mSKIP[0m] gencol_vacuum_virtual                    [2m(5.62µs)[0m [2m(TODO: VACUUM is not supported yet)[0m
  [[33mSKIP[0m] gencol_vacuum_virtual_with_index         [2m(1.87µs)[0m [2m(TODO: VACUUM is not supported yet)[0m
  [[32mPASS[0m] gencol_alter_add_stored_rejected         [2m(9.42ms)[0m
  [[32mPASS[0m] gencol_alter_add_virtual_explicit        [2m(17.03ms)[0m
  [[32mPASS[0m] gencol_alter_add_virtual_random_error    [2m(13.72ms)[0m
  [[32mPASS[0m] gencol_rename_column_middle              [2m(9.68ms)[0m
  [[32mPASS[0m] gencol_rename_column_case_insensitive    [2m(8.56ms)[0m
  [[32mPASS[0m] gencol_rename_column_updates_expression  [2m(11.23ms)[0m
  [[32mPASS[0m] gencol_pragma_table_info_hides_virtual   [2m(4.41ms)[0m
  [[32mPASS[0m] gencol_rename_column_chain               [2m(12.55ms)[0m
  [[32mPASS[0m] gencol_alter_add_virtual_function        [2m(19.16ms)[0m
  [[32mPASS[0m] gencol_alter_add_virtual_after_add_default_notnull [2m(13.99ms)[0m
  [[32mPASS[0m] gencol_null_concat                       [2m(4.69ms)[0m
  [[32mPASS[0m] gencol_rename_column_complex_expr        [2m(12.55ms)[0m
  [[32mPASS[0m] gencol_rename_column_multiple_refs       [2m(12.02ms)[0m
  [[32mPASS[0m] gencol_rowid_alias_autogenerated         [2m(9.61ms)[0m
  [[32mPASS[0m] gencol_rename_column_other_table_unaffected [2m(11.74ms)[0m
  [[32mPASS[0m] gencol_rowid_alias_chain                 [2m(8.96ms)[0m
  [[32mPASS[0m] gencol_alter_add_column_generated_pk_rejected [2m(19.53ms)[0m
  [[32mPASS[0m] gencol_rowid_alias_with_other_cols       [2m(10.72ms)[0m
  [[32mPASS[0m] gencol_pk_plus_only_virtual              [2m(9.96ms)[0m
  [[32mPASS[0m] gencol_rename_column_with_index_on_generated [2m(14.57ms)[0m
  [[32mPASS[0m] gencol_rename_column_other_table_matching_name_unaffected [2m(14.39ms)[0m
  [[32mPASS[0m] gencol_strict_multiple_virtual           [2m(5.45ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-with-expressions                      [2m(8.70ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_virtual_collate_where_eq          [2m(5.11ms)[0m
  [[32mPASS[0m] gencol_drop_column_shifts_index_expr     [2m(15.29ms)[0m
  [[32mPASS[0m] gencol_virtual_collate_where_in          [2m(5.25ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-empty-table-rowid                     [2m(6.65ms)[0m
  [[32mPASS[0m] in-with-extra-filter                     [2m(10.33ms)[0m
  [[32mPASS[0m] in-empty-table-index                     [2m(6.16ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_virtual_collate_where_and         [2m(5.23ms)[0m
  [[32mPASS[0m] rename_then_drop_keeps_renamed_column    [2m(17.27ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-composite-index-prefix                [2m(8.96ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_strict_insert_type_violation      [2m(8.49ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-many-values                           [2m(9.77ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_virtual_collate_group_by          [2m(5.18ms)[0m
  [[32mPASS[0m] gencol_virtual_collate_distinct          [2m(5.24ms)[0m
  [[32mPASS[0m] gencol_virtual_collate_index_in          [2m(6.99ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-nocase-index                          [2m(8.85ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_rowid_alias_virtual               [2m(22.10ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-nocase-index-mixed-case-list          [2m(7.80ms)[0m
  [[32mPASS[0m] rowid-in-many-values                     [2m(11.60ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_deep_dependency_chain             [2m(18.99ms)[0m
  [[32mPASS[0m] gencol_default_value_with_virtual        [2m(18.86ms)[0m
  [[32mPASS[0m] gencol_strict_type_enforcement           [2m(15.84ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-index-collation-mismatch-falls-back   [2m(7.28ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_virtual_collate_index_eq          [2m(11.72ms)[0m
  [[32mPASS[0m] gencol_error_update_virtual              [2m(5.30ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-text-index                            [2m(16.82ms)[0m
  [[32mPASS[0m] in-subquery-collation-mismatch-falls-back [2m(9.30ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_strict_basic                      [2m(21.67ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-index-collation-match-explicit-rtrim  [2m(6.54ms)[0m
  [[32mPASS[0m] in-composite-index-collation-mismatch-falls-back [2m(6.56ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_error_primary_key_virtual         [2m(6.60ms)[0m
  [[32mPASS[0m] gencol_error_composite_pk_virtual        [2m(6.25ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-collate-binary-override               [2m(13.23ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_error_circular_reference          [2m(4.78ms)[0m
  [[32mPASS[0m] gencol_error_self_reference_case_insensitive [2m(6.54ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-nocase-subquery                       [2m(15.74ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_error_self_reference              [2m(7.50ms)[0m
  [[32mPASS[0m] gencol_error_insert_virtual              [2m(10.81ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-desc-index-select                     [2m(8.71ms)[0m

[1mtests/groupby/default.sqltest[0m
  [[32mPASS[0m] having_with_multiple_conditions          [2m(267.85ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-index-collation-match-explicit-nocase [2m(16.45ms)[0m
  [[32mPASS[0m] in-desc-index-delete                     [2m(9.64ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_error_transitive_circular_reference [2m(5.64ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-index-collation-mismatch-rtrim-falls-back [2m(14.78ms)[0m
  [[32mPASS[0m] in-desc-index-update                     [2m(12.13ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_error_all_generated               [2m(6.90ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-partial-index-predicate-implied       [2m(8.44ms)[0m
  [[32mPASS[0m] in-partial-index-predicate-not-implied   [2m(7.73ms)[0m
  [[32mPASS[0m] not-in-with-null-in-list                 [2m(5.67ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_error_default_virtual             [2m(13.53ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-with-null-in-list                     [2m(5.54ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_large_row_update                  [2m(21.77ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-index-update-or-replace-multirow      [2m(9.72ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_error_rowid                       [2m(10.35ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-index-delete-returning                [2m(9.18ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_error_oid                         [2m(9.80ms)[0m
  [[32mPASS[0m] gencol_error_in_select                   [2m(8.43ms)[0m
  [[32mPASS[0m] gencol_error_subquery                    [2m(9.31ms)[0m
  [[32mPASS[0m] gencol_error_exists                      [2m(8.08ms)[0m
  [[32mPASS[0m] gencol_error_rowid2                      [2m(10.90ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] not-in-subquery-with-null-notin          [2m(8.21ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_error_nondeterministic_virtual    [2m(6.64ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-index-update-or-replace               [2m(17.31ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_large_row_count                   [2m(27.19ms)[0m
  [[32mPASS[0m] gencol_error_doubly_qualified            [2m(5.99ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] rowid-in-with-filter                     [2m(38.24ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_error_aggregate_star              [2m(9.12ms)[0m
  [[32mPASS[0m] gencol_stored_rejected                   [2m(4.56ms)[0m
  [[32mPASS[0m] gencol_error_nondeterministic_datetime   [2m(9.18ms)[0m
  [[32mPASS[0m] gencol_error_window_function             [2m(7.65ms)[0m
  [[32mPASS[0m] gencol_error_nondeterministic_strftime   [2m(9.48ms)[0m
  [[32mPASS[0m] gencol_error_nondeterministic_unixepoch  [2m(9.52ms)[0m
  [[32mPASS[0m] gencol_error_bind_parameter              [2m(7.57ms)[0m
  [[32mPASS[0m] gencol_stored_generated_always_rejected  [2m(6.77ms)[0m
  [[32mPASS[0m] gencol_error_aggregate                   [2m(12.61ms)[0m
  [[32mPASS[0m] gencol_error_nested_nondeterministic     [2m(6.29ms)[0m
  [[32mPASS[0m] gencol_error_bind_parameter_unnamed      [2m(7.69ms)[0m
  [[32mPASS[0m] gencol_error_dot_operator                [2m(11.94ms)[0m
  [[32mPASS[0m] gencol_error_nested_aggregate            [2m(7.47ms)[0m
  [[32mPASS[0m] gencol_validate_nested_cast              [2m(3.89ms)[0m
  [[32mPASS[0m] gencol_error_nested_subquery_in_func     [2m(8.48ms)[0m
  [[32mPASS[0m] gencol_error_nested_subquery_in_case     [2m(8.01ms)[0m
  [[32mPASS[0m] gencol_error_nested_doubly_qualified     [2m(6.56ms)[0m
  [[32mPASS[0m] gencol_validate_nested_isnull            [2m(5.64ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] not-in-subquery-with-null                [2m(18.31ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_error_nested_exists_in_arith      [2m(8.32ms)[0m
  [[32mPASS[0m] gencol_error_nested_in_select            [2m(8.56ms)[0m
  [[32mPASS[0m] gencol_error_nested_dot_in_case          [2m(9.96ms)[0m
  [[32mPASS[0m] gencol_error_nested_window_function      [2m(13.70ms)[0m
  [[32mPASS[0m] gencol_validate_nested_inlist            [2m(7.33ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-integer-col-string-values             [2m(11.65ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_error_nested_aggregate_star       [2m(9.90ms)[0m
  [[32mPASS[0m] gencol_validate_nested_like              [2m(8.60ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-index-update-returning                [2m(26.28ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_validate_nested_between           [2m(10.14ms)[0m
  [[32mPASS[0m] gencol_three_table_inner_join_select_gencol [2m(7.31ms)[0m
  [[32mPASS[0m] gencol_validate_nested_collate           [2m(12.02ms)[0m
  [[32mPASS[0m] gencol_delete_trigger_ref_virtual_col    [2m(7.66ms)[0m
  [[32mPASS[0m] gencol_delete_after_trigger_virtual      [2m(8.29ms)[0m
  [[32mPASS[0m] gencol_upsert_trigger_virtual_col        [2m(8.32ms)[0m
  [[32mPASS[0m] gencol_large_row_delete                  [2m(44.22ms)[0m
  [[32mPASS[0m] gencol_four_table_inner_join             [2m(10.11ms)[0m
  [[32mPASS[0m] gencol_validate_nested_like_escape       [2m(14.71ms)[0m
  [[32mPASS[0m] gencol_validate_nested_isnull_arith      [2m(14.59ms)[0m
  [[32mPASS[0m] fk-gencol-unique-idx-delete-cascade      [2m(9.55ms)[0m
  [[32mPASS[0m] fk-gencol-unique-idx-drop-table          [2m(8.24ms)[0m
  [[32mPASS[0m] fk-gencol-unique-idx-delete-set-null     [2m(9.91ms)[0m
  [[32mPASS[0m] fk-gencol-unique-idx-mid-schema-update-no-action [2m(6.92ms)[0m
  [[32mPASS[0m] fk-gencol-unique-idx-update-cascade      [2m(9.79ms)[0m
  [[32mPASS[0m] gencol_self_table_affinity_update_returning_not_null [2m(3.97ms)[0m
  [[32mPASS[0m] gencol_validate_nested_case_base         [2m(17.32ms)[0m
  [[32mPASS[0m] gencol_validate_nested_notnull_arith     [2m(18.34ms)[0m
  [[32mPASS[0m] gencol_delete_before_trigger_virtual     [2m(15.59ms)[0m
  [[32mPASS[0m] gencol_unary_positive                    [2m(4.85ms)[0m
  [[32mPASS[0m] gencol_unary_negative                    [2m(5.04ms)[0m
  [[32mPASS[0m] gencol_unary_not                         [2m(5.88ms)[0m
  [[32mPASS[0m] gencol_self_table_affinity_integrity_check_index [2m(9.81ms)[0m
  [[32mPASS[0m] gencol_shift_left                        [2m(5.10ms)[0m
  [[32mPASS[0m] fk-child-references-parent-virtual-col   [2m(10.82ms)[0m
  [[32mPASS[0m] gencol_three_table_inner_join_where_gencol [2m(19.39ms)[0m
  [[32mPASS[0m] gencol_self_table_affinity_insert_returning_index_integrity [2m(8.19ms)[0m
  [[32mPASS[0m] gencol_bitwise_not                       [2m(4.94ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-integer-col-float-values              [2m(22.00ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_self_table_affinity_indexed_update_returning [2m(10.75ms)[0m
  [[32mPASS[0m] gencol_shift_right                       [2m(8.72ms)[0m
  [[32mPASS[0m] gencol_delete_trigger_multiple_virtual   [2m(22.28ms)[0m
  [[32mPASS[0m] gencol_case_multi_column                 [2m(6.25ms)[0m
  [[32mPASS[0m] fk-gencol-unique-idx-update-no-action    [2m(21.84ms)[0m
  [[32mPASS[0m] gencol_case_base_column                  [2m(6.48ms)[0m
  [[32mPASS[0m] gencol_bitwise_or                        [2m(10.55ms)[0m
  [[32mPASS[0m] fk-child-references-parent-virtual-col-missing [2m(15.08ms)[0m
  [[32mPASS[0m] fk-gencol-unique-idx-child-insert-reject [2m(19.19ms)[0m
  [[32mPASS[0m] gencol_self_join_virtual                 [2m(26.38ms)[0m
  [[32mPASS[0m] gencol_bitwise_and                       [2m(11.76ms)[0m
  [[32mPASS[0m] gencol_case_expression                   [2m(9.12ms)[0m
  [[32mPASS[0m] gencol_three_table_inner_join            [2m(27.42ms)[0m
  [[32mPASS[0m] gencol_case_nested                       [2m(7.06ms)[0m
  [[32mPASS[0m] gencol_validate_collate                  [2m(5.32ms)[0m
  [[32mPASS[0m] gencol_validate_cast                     [2m(6.72ms)[0m
  [[32mPASS[0m] gencol_case_no_else                      [2m(7.04ms)[0m
  [[32mPASS[0m] gencol_self_table_affinity_indexed_delete_returning [2m(15.78ms)[0m
  [[32mPASS[0m] gencol_case_with_not                     [2m(8.50ms)[0m
  [[32mPASS[0m] gencol_parentheses                       [2m(8.01ms)[0m
  [[32mPASS[0m] gencol_between_update                    [2m(6.83ms)[0m
  [[32mPASS[0m] fk-gencol-unique-idx-mid-schema-delete-cascade [2m(22.39ms)[0m
  [[32mPASS[0m] gencol_func_abs                          [2m(5.24ms)[0m
  [[32mPASS[0m] gencol_like_update                       [2m(5.94ms)[0m
  [[32mPASS[0m] gencol_validate_parenthesized            [2m(11.50ms)[0m
  [[32mPASS[0m] gencol_like                              [2m(7.48ms)[0m
  [[32mPASS[0m] gencol_trigger_when_generated_text_affinity [2m(19.46ms)[0m
  [[32mPASS[0m] gencol_func_lower                        [2m(5.84ms)[0m
  [[32mPASS[0m] gencol_func_nested_abs                   [2m(5.41ms)[0m
  [[32mPASS[0m] gencol_in_list                           [2m(9.18ms)[0m
  [[32mPASS[0m] gencol_func_substr                       [2m(6.87ms)[0m
  [[32mPASS[0m] gencol_func_length                       [2m(8.31ms)[0m
  [[32mPASS[0m] gencol_func_ifnull                       [2m(6.83ms)[0m
  [[32mPASS[0m] gencol_validate_like_escape              [2m(9.56ms)[0m
  [[32mPASS[0m] gencol_between                           [2m(12.70ms)[0m
  [[32mPASS[0m] gencol_validate_isnull                   [2m(9.54ms)[0m
  [[32mPASS[0m] gencol_validate_notnull                  [2m(9.68ms)[0m
  [[32mPASS[0m] gencol_func_nested_upper_substr          [2m(6.40ms)[0m
  [[32mPASS[0m] gencol_func_upper                        [2m(10.43ms)[0m
  [[32mPASS[0m] gencol_func_coalesce_multi_col           [2m(7.13ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] rowid-in-string-values                   [2m(9.15ms)[0m
  [[32mPASS[0m] in-mixed-types-with-null                 [2m(7.53ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_func_ifnull_two_cols              [2m(5.86ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-subquery-type-coercion                [2m(10.07ms)[0m
  [[32mPASS[0m] in-mixed-types                           [2m(8.10ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_in_list_update                    [2m(12.72ms)[0m
  [[32mPASS[0m] gencol_func_multi_abs_arithmetic         [2m(5.53ms)[0m
  [[32mPASS[0m] gencol_func_abs_arithmetic               [2m(5.86ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] rowid-in-float-values                    [2m(12.19ms)[0m
  [[32mPASS[0m] in-non-indexed-column                    [2m(9.05ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_func_min_cols                     [2m(5.78ms)[0m
  [[32mPASS[0m] gencol_func_dependency_chain             [2m(6.36ms)[0m
  [[32mPASS[0m] gencol_func_max_cols                     [2m(5.56ms)[0m
  [[32mPASS[0m] gencol_glob_pattern_column               [2m(6.17ms)[0m
  [[32mPASS[0m] gencol_dependency_chain_virtual          [2m(5.60ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-list-with-join                        [2m(8.15ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_dependency_reverse_order_virtual  [2m(5.85ms)[0m
  [[32mPASS[0m] gencol_dependency_out_of_order_virtual   [2m(6.18ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-correlated-subquery                   [2m(9.37ms)[0m
  [[32mPASS[0m] in-subquery-empty                        [2m(10.41ms)[0m
  [[32mPASS[0m] in-subquery-with-join                    [2m(8.82ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_affinity_integer                  [2m(5.05ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-self-referencing-rowid                [2m(5.79ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_forward_reference                 [2m(7.04ms)[0m
  [[32mPASS[0m] gencol_affinity_text                     [2m(5.06ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-subquery-with-duplicates              [2m(12.42ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_affinity_real                     [2m(5.21ms)[0m
  [[32mPASS[0m] gencol_dependency_diamond_virtual        [2m(8.51ms)[0m

[1mtests/in-null-or.sqltest[0m
  [[32mPASS[0m] in-null-or-truthy                        [2m(5.66ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_affinity_float_to_int_exact       [2m(5.03ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-list-left-join-on-no-match            [2m(9.66ms)[0m
  [[32mPASS[0m] in-list-left-join-on-match               [2m(10.01ms)[0m

[1mtests/in-null-or.sqltest[0m
  [[32mPASS[0m] in-null-or-falsy                         [2m(5.62ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_affinity_float_fractional         [2m(5.11ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-nested-subqueries                     [2m(10.22ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_affinity_large_int_exact          [2m(4.98ms)[0m
  [[32mPASS[0m] gencol_affinity_numeric_float_to_int     [2m(5.46ms)[0m
  [[32mPASS[0m] gencol_affinity_numeric_fractional       [2m(4.98ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-subquery-left-join-on-match           [2m(11.83ms)[0m

[1mtests/in-null-or.sqltest[0m
  [[32mPASS[0m] in-null-or-column-multiple-rows          [2m(6.10ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-cross-table-column-ref                [2m(9.27ms)[0m

[1mtests/in-null-or.sqltest[0m
  [[32mPASS[0m] in-null-or-subquery                      [2m(6.26ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_float_literal_in_expr             [2m(5.22ms)[0m
  [[32mPASS[0m] gencol_division_exact_int                [2m(5.23ms)[0m

[1mtests/in-null-or.sqltest[0m
  [[32mPASS[0m] in-null-or-with-offset                   [2m(6.05ms)[0m
  [[32mPASS[0m] in-match-or-null                         [2m(5.39ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_division_truncated                [2m(5.01ms)[0m

[1mtests/in-null-or.sqltest[0m
  [[32mPASS[0m] in-null-column-or                        [2m(5.58ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_numeric_affinity                  [2m(5.30ms)[0m

[1mtests/in-null-or.sqltest[0m
  [[32mPASS[0m] not-in-null-or-truthy                    [2m(8.82ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] in-self-referencing-indexed              [2m(12.39ms)[0m

[1mtests/in-null-or.sqltest[0m
  [[32mPASS[0m] not-in-self-or-true-counts-all-rows      [2m(5.33ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_intreal_typeof                    [2m(5.42ms)[0m

[1mtests/in-null-or.sqltest[0m
  [[32mPASS[0m] not-in-self-or-true-is-true-for-all-rows [2m(5.36ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_virtual_affinity_chain            [2m(6.60ms)[0m
  [[32mPASS[0m] gencol_collation_not_inherited           [2m(5.39ms)[0m
  [[32mPASS[0m] gencol_type_mismatch_text_in_integer     [2m(4.89ms)[0m
  [[32mPASS[0m] gencol_type_mismatch_int_in_text         [2m(4.92ms)[0m
  [[32mPASS[0m] gencol_type_mismatch_blob_in_text        [2m(4.97ms)[0m
  [[32mPASS[0m] gencol_first_position                    [2m(4.81ms)[0m
  [[32mPASS[0m] gencol_middle_position                   [2m(4.88ms)[0m
  [[32mPASS[0m] gencol_type_mismatch_real_in_integer     [2m(5.50ms)[0m
  [[32mPASS[0m] gencol_last_position                     [2m(4.85ms)[0m
  [[32mPASS[0m] gencol_null_in_expression                [2m(5.05ms)[0m
  [[32mPASS[0m] gencol_null_input                        [2m(5.21ms)[0m

[1mtests/in-null-or.sqltest[0m
  [[32mPASS[0m] not-in-self-or-false-filters-all-rows    [2m(5.25ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_in_list_null_handling             [2m(5.48ms)[0m
  [[32mPASS[0m] gencol_null_or_semantics                 [2m(5.66ms)[0m
  [[32mPASS[0m] gencol_returning_update_star             [2m(5.50ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] multi-left-join-two-independent-in-subqueries [2m(24.68ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_modulo_by_zero                    [2m(6.84ms)[0m
  [[32mPASS[0m] gencol_null_and_semantics                [2m(8.72ms)[0m
  [[32mPASS[0m] gencol_division_by_zero_null             [2m(7.39ms)[0m
  [[32mPASS[0m] gencol_returning_insert_specific_cols    [2m(6.71ms)[0m
  [[32mPASS[0m] gencol_returning_insert_star_middle      [2m(6.74ms)[0m
  [[32mPASS[0m] gencol_insert_default_values_virtual     [2m(7.98ms)[0m
  [[33mSKIP[0m] gencol_insert_or_replace_with_index      [2m(3.55µs)[0m [2m(TODO: INSERT OR REPLACE with virtual column index triggers SELF_TABLE parse error)[0m
  [[32mPASS[0m] gencol_division_by_zero_case             [2m(7.15ms)[0m
  [[32mPASS[0m] gencol_coalesce_null                     [2m(9.82ms)[0m
  [[32mPASS[0m] gencol_update_virtual_unchanged          [2m(8.96ms)[0m
  [[32mPASS[0m] gencol_division_by_zero_coalesce         [2m(7.77ms)[0m

[1mtests/in-null-or.sqltest[0m
  [[32mPASS[0m] in-subquery-or-in-subquery-indexed       [2m(14.96ms)[0m
  [[32mPASS[0m] in-subquery-or-in-list-left-join         [2m(14.80ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_returning_virtual_first           [2m(7.50ms)[0m

[1mtests/in-null-or.sqltest[0m
  [[32mPASS[0m] not-in-self-or-false-preserves-null-result [2m(7.21ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_virtual_simple_update_2           [2m(7.53ms)[0m
  [[32mPASS[0m] gencol_returning_multiple_virtual        [2m(8.99ms)[0m
  [[32mPASS[0m] gencol_virtual_simple_update             [2m(8.51ms)[0m

[1mtests/in-index-seek.sqltest[0m
  [[32mPASS[0m] multi-join-two-independent-in-subqueries [2m(29.05ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_rowid_alias_update_virtual        [2m(5.43ms)[0m
  [[32mPASS[0m] gencol_virtual_chain_update              [2m(11.11ms)[0m
  [[32mPASS[0m] gencol_insert_or_replace                 [2m(5.48ms)[0m
  [[32mPASS[0m] gencol_insert_or_ignore                  [2m(5.83ms)[0m
  [[32mPASS[0m] gencol_delete_multiple_verify_remaining  [2m(6.24ms)[0m
  [[32mPASS[0m] gencol_update_virtual_first_col          [2m(9.51ms)[0m
  [[32mPASS[0m] gencol_virtual_three_deep_update         [2m(9.80ms)[0m
  [[32mPASS[0m] gencol_virtual_three_deep                [2m(11.45ms)[0m
  [[32mPASS[0m] gencol_delete_where_virtual              [2m(7.28ms)[0m
  [[32mPASS[0m] gencol_replace_virtual_first_col         [2m(6.09ms)[0m
  [[32mPASS[0m] gencol_delete_basic                      [2m(7.26ms)[0m
  [[32mPASS[0m] gencol_delete_virtual_middle             [2m(7.18ms)[0m
  [[32mPASS[0m] gencol_insert_or_replace_not_null_no_default [2m(7.84ms)[0m

[1mtests/composite-index-sort-elim.sqltest[0m
  [[32mPASS[0m] eq-prefix-collation-mismatch-is-not-constant [2m(7.24ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_insert_or_replace_multiple        [2m(8.17ms)[0m
  [[32mPASS[0m] gencol_delete_all                        [2m(10.43ms)[0m
  [[32mPASS[0m] gencol_delete_chained_virtual            [2m(13.35ms)[0m

[1mtests/composite-index-sort-elim.sqltest[0m
  [[32mPASS[0m] join-left-on-constant-is-not-global-order [2m(11.34ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_replace_into                      [2m(15.30ms)[0m

[1mtests/composite-index-sort-elim.sqltest[0m
  [[32mPASS[0m] group-by-two-eq-prefix                   [2m(14.54ms)[0m
  [[32mPASS[0m] expr-index-two-eq-order-last             [2m(10.12ms)[0m
  [[32mPASS[0m] join-left-order-outer-only               [2m(11.69ms)[0m
  [[32mPASS[0m] join-left-expr-index-is-not-global-order [2m(10.49ms)[0m
  [[32mPASS[0m] expr-index-eq-prefix-order-suffix        [2m(11.80ms)[0m
  [[32mPASS[0m] join-eq-skip-order-outer-only            [2m(15.47ms)[0m
  [[32mPASS[0m] join-left-order-right                    [2m(15.05ms)[0m
  [[32mPASS[0m] join-order-inner-only                    [2m(15.77ms)[0m
  [[32mPASS[0m] expr-index-group-by-eq-prefix            [2m(12.55ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] union-all-order-by-col-name              [2m(12.09ms)[0m

[1mtests/in-subquery-ungrouped-aggregate.sqltest[0m
  [[32mPASS[0m] in-subquery-empty-aggregate-null         [2m(5.96ms)[0m

[1mtests/composite-index-sort-elim.sqltest[0m
  [[32mPASS[0m] join-eq-both-tables                      [2m(16.84ms)[0m
  [[32mPASS[0m] join-three-way                           [2m(13.03ms)[0m

[1mtests/in-null-or.sqltest[0m
  [[32mPASS[0m] not-in-self-or-nullable-rhs-matches-null-rhs-row [2m(11.91ms)[0m
  [[32mPASS[0m] left-join-is-not-predicate-keeps-unmatched-rows [2m(13.34ms)[0m

[1mtests/composite-index-sort-elim.sqltest[0m
  [[32mPASS[0m] join-duplicate-outer-key                 [2m(15.57ms)[0m
  [[32mPASS[0m] join-eq-skip-wrong-direction-outer       [2m(16.31ms)[0m

[1mtests/in-subquery-ungrouped-aggregate.sqltest[0m
  [[32mPASS[0m] in-subquery-empty-aggregate-decision     [2m(7.70ms)[0m

[1mtests/composite-index-sort-elim.sqltest[0m
  [[32mPASS[0m] join-left-order-outer-rowid-suffix       [2m(18.55ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] union-all-order-by-desc                  [2m(14.11ms)[0m

[1mtests/composite-index-sort-elim.sqltest[0m
  [[32mPASS[0m] join-cross-table-wrong-direction-inner   [2m(18.52ms)[0m
  [[32mPASS[0m] join-cross-table-both-indexes            [2m(18.90ms)[0m

[1mtests/in-subquery-ungrouped-aggregate.sqltest[0m
  [[32mPASS[0m] in-subquery-empty-aggregate-stored       [2m(11.30ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] union-all-order-by-col-number            [2m(19.91ms)[0m
  [[32mPASS[0m] union-all-order-by-multiple-cols         [2m(12.12ms)[0m
  [[32mPASS[0m] union-all-order-by-second-col            [2m(13.27ms)[0m

[1mtests/index_seek_comparison_affinity.sqltest[0m
  [[32mPASS[0m] issue-7373-derived-join-on-text-le-int   [2m(8.20ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] union-order-by-col-number                [2m(13.38ms)[0m

[1mtests/index_seek_comparison_affinity.sqltest[0m
  [[32mPASS[0m] issue-7373-literal-affinity-preserved    [2m(6.35ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] union-order-by-col-name                  [2m(13.32ms)[0m

[1mtests/index_seek_comparison_affinity.sqltest[0m
  [[32mPASS[0m] issue-7373-direct-index-text-eq-int      [2m(8.20ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] union-order-by-desc                      [2m(13.83ms)[0m
  [[32mPASS[0m] union-order-by-multiple-cols             [2m(13.66ms)[0m

[1mtests/index_seek_comparison_affinity.sqltest[0m
  [[32mPASS[0m] issue-7374-right-join-mixed-affinity     [2m(7.48ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] intersect-order-by                       [2m(12.71ms)[0m
  [[32mPASS[0m] intersect-order-by-desc                  [2m(12.89ms)[0m

[1mtests/index_seek_comparison_affinity.sqltest[0m
  [[32mPASS[0m] in-subquery-text-idx-numeric-source      [2m(8.37ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] except-order-by                          [2m(12.27ms)[0m
  [[32mPASS[0m] intersect-order-by-col-name              [2m(13.21ms)[0m
  [[32mPASS[0m] except-order-by-desc                     [2m(12.83ms)[0m

[1mtests/indexed_by.sqltest[0m
  [[32mPASS[0m] indexed-by-no-where                      [2m(7.11ms)[0m
  [[32mPASS[0m] indexed-by-select-basic                  [2m(8.96ms)[0m
  [[32mPASS[0m] not-indexed-select                       [2m(6.95ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] except-order-by-col-name                 [2m(12.26ms)[0m
  [[32mPASS[0m] union-all-order-by-limit                 [2m(12.55ms)[0m

[1mtests/indexed_by.sqltest[0m
  [[32mPASS[0m] indexed-by-nonexistent                   [2m(8.46ms)[0m
  [[32mPASS[0m] indexed-by-delete                        [2m(7.27ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] union-order-by-limit                     [2m(13.06ms)[0m
  [[32mPASS[0m] except-order-by-limit                    [2m(13.25ms)[0m

[1mtests/index-cursor-record-after-overflow-update.sqltest[0m
  [[32mPASS[0m] select-via-index-after-overflow-update   [2m(24.02ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] intersect-order-by-limit                 [2m(12.59ms)[0m

[1mtests/indexed_by.sqltest[0m
  [[32mPASS[0m] indexed-by-update                        [2m(7.47ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] union-all-order-by-limit-offset          [2m(12.48ms)[0m

[1mtests/index-cursor-record-after-overflow-update.sqltest[0m
  [[32mPASS[0m] select-via-index-empty-result-after-update [2m(24.70ms)[0m

[1mtests/indexed_by.sqltest[0m
  [[32mPASS[0m] not-indexed-delete                       [2m(7.46ms)[0m
  [[32mPASS[0m] not-indexed-update                       [2m(7.15ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] union-order-by-limit-offset              [2m(13.54ms)[0m

[1mtests/index_seek_comparison_affinity.sqltest[0m
  [[32mPASS[0m] in-literal-list-text-idx-numeric-source  [2m(16.65ms)[0m

[1mtests/indexed_by.sqltest[0m
  [[32mPASS[0m] indexed-by-wrong-table                   [2m(14.55ms)[0m
  [[32mPASS[0m] indexed-by-join                          [2m(10.34ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] three-way-union-order-by                 [2m(13.51ms)[0m

[1mtests/indexed_by.sqltest[0m
  [[32mPASS[0m] indexed-by-multiple-rows                 [2m(8.05ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] three-way-union-order-by-desc            [2m(13.78ms)[0m

[1mtests/indexed_by.sqltest[0m
  [[32mPASS[0m] indexed-by-with-alias                    [2m(9.90ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] union-all-then-except-order-by           [2m(13.35ms)[0m
  [[32mPASS[0m] union-then-union-all-order-by            [2m(14.36ms)[0m
  [[32mPASS[0m] three-way-union-all-order-by             [2m(19.21ms)[0m
  [[32mPASS[0m] union-then-intersect-order-by            [2m(13.17ms)[0m

[1mtests/indexed_by.sqltest[0m
  [[32mPASS[0m] indexed-by-attached-qualified-table      [2m(9.61ms)[0m

[1mtests/insert-cte-compound.sqltest[0m
  [[32mPASS[0m] insert-cte-union-all-basic               [2m(7.71ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] simple-union-order-by-1                  [2m(12.34ms)[0m
  [[32mPASS[0m] simple-union-all-order-by-1              [2m(12.13ms)[0m

[1mtests/indexed_by.sqltest[0m
  [[32mPASS[0m] indexed-by-temp-qualified-table          [2m(11.25ms)[0m

[1mtests/insert-cte-compound.sqltest[0m
  [[32mPASS[0m] select-cte-union-all-no-insert           [2m(3.06ms)[0m

[1mtests/indexed_by.sqltest[0m
  [[32mPASS[0m] indexed-by-attached-update-delete        [2m(10.80ms)[0m

[1mtests/index-cursor-record-after-overflow-update.sqltest[0m
  [[32mPASS[0m] update-indexed-by-overflow-keys          [2m(37.80ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] simple-union-order-by-1-desc             [2m(13.59ms)[0m

[1mtests/index_seek_comparison_affinity.sqltest[0m
  [[32mPASS[0m] issue-7373-direct-index-text-le-int      [2m(35.88ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] union-all-expressions-order-by           [2m(11.46ms)[0m
  [[32mPASS[0m] union-all-order-by-alias-desc            [2m(12.87ms)[0m
  [[32mPASS[0m] union-all-nulls-order-by                 [2m(12.57ms)[0m

[1mtests/concat.sqltest[0m
  [[32mPASS[0m] concat-3                                 [2m(1.75ms)[0m
  [[32mPASS[0m] concat                                   [2m(2.80ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] union-nulls-order-by-desc                [2m(12.52ms)[0m

[1mtests/concat.sqltest[0m
  [[32mPASS[0m] concat-4                                 [2m(1.79ms)[0m
  [[32mPASS[0m] concat-4-2                               [2m(725.16µs)[0m
  [[32mPASS[0m] concat-blob                              [2m(490.26µs)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] union-order-by-alias                     [2m(17.72ms)[0m
  [[32mPASS[0m] union-all-where-order-by                 [2m(15.16ms)[0m
  [[32mPASS[0m] union-all-order-by-mixed-asc-desc        [2m(11.58ms)[0m

[1mtests/concat.sqltest[0m
  [[32mPASS[0m] concat-2                                 [2m(1.72ms)[0m
  [[32mPASS[0m] concat-3                                 [2m(404.34µs)[0m
  [[32mPASS[0m] concat                                   [2m(3.18ms)[0m
  [[32mPASS[0m] concat-blob                              [2m(443.14µs)[0m
  [[32mPASS[0m] concat-4                                 [2m(756.12µs)[0m
  [[32mPASS[0m] concat-4-2                               [2m(702.67µs)[0m
  [[32mPASS[0m] concat-no-args-error                     [2m(4.69ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] intersect-union-order-by                 [2m(13.59ms)[0m

[1mtests/concat.sqltest[0m
  [[32mPASS[0m] concat-ws-no-args-error                  [2m(4.84ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] union-all-literals-order-by-desc         [2m(12.27ms)[0m
  [[32mPASS[0m] union-literals-order-by                  [2m(12.38ms)[0m
  [[32mPASS[0m] subquery-union-order-by                  [2m(13.17ms)[0m

[1mtests/concat.sqltest[0m
  [[32mPASS[0m] concat-ws-one-arg-error                  [2m(5.19ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] union-all-many-order-by-desc             [2m(12.46ms)[0m

[1mtests/insert-cte-compound.sqltest[0m
  [[32mPASS[0m] insert-cte-union-distinct                [2m(5.95ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] union-order-by-non-leftmost-col-name     [2m(12.81ms)[0m
  [[32mPASS[0m] union-all-order-by-non-leftmost-col-name [2m(12.34ms)[0m
  [[32mPASS[0m] except-order-by-limit-offset             [2m(13.59ms)[0m

[1mtests/concat.sqltest[0m
  [[32mPASS[0m] concat-ws-one-arg-error                  [2m(4.93ms)[0m
  [[32mPASS[0m] concat-ws-no-args-error                  [2m(5.00ms)[0m
  [[32mPASS[0m] concat-no-args-error                     [2m(5.88ms)[0m
  [[32mPASS[0m] concat-2                                 [2m(14.27ms)[0m

[1mtests/correlated-subquery-aggregate-groupby.sqltest[0m
  [[32mPASS[0m] correlated-subquery-in-sum-group-by      [2m(7.34ms)[0m
  [[32mPASS[0m] correlated-subquery-in-count-group-by    [2m(6.98ms)[0m
  [[32mPASS[0m] correlated-subquery-in-avg-group-by      [2m(7.29ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-setnull-delete-basic                  [2m(7.89ms)[0m
  [[32mPASS[0m] fk-setnull-delete-multi-children         [2m(7.77ms)[0m
  [[32mPASS[0m] fk-setnull-delete-composite              [2m(8.32ms)[0m
  [[32mPASS[0m] fk-setnull-update-basic                  [2m(8.56ms)[0m
  [[32mPASS[0m] fk-setnull-delete-two-table-cycle        [2m(8.85ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] three-way-union-all-order-by-limit-offset [2m(24.46ms)[0m
  [[32mPASS[0m] except-reversed-order-by                 [2m(24.72ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-setdefault-delete-two-table-cycle-null-default [2m(8.53ms)[0m
  [[32mPASS[0m] fk-setdefault-delete-two-table-cycle-missing-default-fails [2m(6.57ms)[0m
  [[32mPASS[0m] fk-cascade-delete-noaction-update        [2m(7.17ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] union-order-by-limit-zero                [2m(21.82ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-noaction-delete-cascade-update        [2m(7.48ms)[0m
  [[32mPASS[0m] fk-setnull-update-two-table-cycle        [2m(10.47ms)[0m
  [[32mPASS[0m] fk-replace-delete-cascade                [2m(8.41ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] three-way-union-all-order-by-limit       [2m(30.85ms)[0m
  [[32mPASS[0m] union-all-many-order-by                  [2m(26.38ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-replace-delete-setdefault             [2m(8.53ms)[0m
  [[32mPASS[0m] fk-replace-delete-cascade-composite      [2m(9.05ms)[0m
  [[32mPASS[0m] fk-upsert-update-cascade                 [2m(7.98ms)[0m
  [[32mPASS[0m] fk-setdefault-update-two-table-cycle-missing-default-fails [2m(12.51ms)[0m
  [[32mPASS[0m] fk-delete-restrict-fails                 [2m(6.12ms)[0m

[1mtests/compound-select-orderby.sqltest[0m
  [[32mPASS[0m] union-order-by-expr-no-match             [2m(25.61ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-replace-delete-cascade-recursive      [2m(9.92ms)[0m
  [[32mPASS[0m] fk-replace-delete-cascade-recursive-check-c [2m(9.71ms)[0m
  [[32mPASS[0m] fk-upsert-update-setdefault              [2m(8.71ms)[0m
  [[32mPASS[0m] fk-upsert-update-setnull                 [2m(9.17ms)[0m
  [[32mPASS[0m] fk-replace-delete-setnull                [2m(11.59ms)[0m
  [[32mPASS[0m] fk-upsert-update-cascade-noop            [2m(8.43ms)[0m
  [[32mPASS[0m] fk-upsert-update-cascade-multi-children  [2m(9.07ms)[0m
  [[32mPASS[0m] fk-replace-restrict-fails                [2m(6.66ms)[0m
  [[32mPASS[0m] fk-upsert-update-cascade-composite       [2m(8.98ms)[0m
  [[32mPASS[0m] fk-delete-restrict-ok-no-child           [2m(7.46ms)[0m
  [[32mPASS[0m] fk-update-restrict-ok-no-child           [2m(7.20ms)[0m
  [[32mPASS[0m] fk-update-restrict-fails                 [2m(7.52ms)[0m
  [[32mPASS[0m] fk-setdefault-update-two-table-cycle-null-default [2m(16.58ms)[0m
  [[32mPASS[0m] fk-upsert-update-cascade-recursive       [2m(11.36ms)[0m
  [[32mPASS[0m] fk-restrict-null-child-ok                [2m(7.27ms)[0m
  [[32mPASS[0m] fk-upsert-restrict-pk-fails              [2m(9.56ms)[0m
  [[32mPASS[0m] fk-upsert-restrict-unique-fails          [2m(8.28ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-add-column-default-affinity-integer [2m(6.37ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-upsert-restrict-ok-no-change          [2m(8.77ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-add-column-default-affinity-real [2m(6.46ms)[0m

[1mtests/insert-cte-compound.sqltest[0m
  [[32mPASS[0m] insert-cte-intersect                     [2m(6.77ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-add-column-default-affinity-numeric [2m(6.57ms)[0m
  [[32mPASS[0m] fail-alter-table-drop-unique-column-constraint [2m(5.48ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-update-restrict-null-parent-key-noop-scan [2m(8.69ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] fail-alter-table-drop-unique-column      [2m(6.11ms)[0m

[1mtests/insert-cte-compound.sqltest[0m
  [[32mPASS[0m] insert-cte-multiple-union-all            [2m(6.80ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-add-column-trailing-named-constraint [2m(7.49ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-update-restrict-null-parent-key-noop  [2m(9.60ms)[0m

[1mtests/insert-cte-compound.sqltest[0m
  [[32mPASS[0m] insert-cte-except                        [2m(9.94ms)[0m
  [[32mPASS[0m] insert-cte-union-all-nulls               [2m(8.55ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-delete-restrict-null-parent-key-noop-scan [2m(10.10ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-drop-column                  [2m(10.11ms)[0m

[1mtests/index-cursor-record-after-overflow-update.sqltest[0m
  [[32mPASS[0m] rowid-after-overflow-update              [2m(76.74ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] fail-alter-table-add-duplicate-column    [2m(6.85ms)[0m
  [[32mPASS[0m] fail-alter-table-add-duplicate-column-case-insensitive [2m(6.94ms)[0m
  [[32mPASS[0m] fail-alter-table-drop-primary-key-column-constrait [2m(6.09ms)[0m

[1mtests/insert-cte-compound.sqltest[0m
  [[32mPASS[0m] insert-cte-empty-union-all               [2m(7.94ms)[0m
  [[32mPASS[0m] insert-cte-expressions-union-all         [2m(5.78ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-update-restrict-partial-null-composite-parent-key-noop [2m(8.32ms)[0m
  [[32mPASS[0m] fk-update-noaction-null-parent-key-noop  [2m(13.58ms)[0m

[1mtests/insert-cte-compound.sqltest[0m
  [[32mPASS[0m] insert-multiple-ctes-union-all           [2m(7.45ms)[0m
  [[32mPASS[0m] insert-cte-single-row-union-all          [2m(11.36ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] fail-alter-table-drop-primary-key-column [2m(7.93ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-delete-restrict-null-parent-key-noop-index [2m(13.14ms)[0m
  [[32mPASS[0m] fk-drop-parent-with-child-references     [2m(7.61ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-drop-column-special-name     [2m(13.71ms)[0m
  [[32mPASS[0m] alter-table-add-notnull-col              [2m(5.40ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-delete-restrict-partial-null-composite-parent-key [2m(10.31ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] fail-alter-table-rename-to-existing-index [2m(9.01ms)[0m
  [[32mPASS[0m] fail-alter-table-rename-to-existing-table [2m(10.11ms)[0m

[1mtests/insert-cte-compound.sqltest[0m
  [[32mPASS[0m] insert-cte-union-all-type-mixing         [2m(5.92ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-drop-column-unique-special-name-schema-sql [2m(14.17ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] basic-insert                             [2m(5.10ms)[0m

[1mtests/insert-cte-compound.sqltest[0m
  [[32mPASS[0m] insert-cte-from-table-union-all          [2m(9.66ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-drop-parent-ok-when-child-empty       [2m(7.76ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] must-be-int-insert                       [2m(5.91ms)[0m
  [[32mPASS[0m] strict-basic-creation                    [2m(5.34ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-drop-parent-ok-after-child-fk-set-null [2m(8.12ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] drop-column-regression                   [2m(10.04ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-drop-parent-ok-after-child-deleted    [2m(7.85ms)[0m
  [[32mPASS[0m] fk-drop-parent-mixed-null-references     [2m(7.32ms)[0m
  [[32mPASS[0m] fk-drop-parent-ok-when-child-fk-null     [2m(13.94ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] self-insert-sum-pk                       [2m(5.98ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-tbl-regression                     [2m(10.39ms)[0m
  [[32mPASS[0m] drop-column-removes-check-constraint     [2m(7.95ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] strict-require-datatype                  [2m(2.69ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-drop-parent-ok-with-orphaned-fk       [2m(8.11ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-column-updates-sibling-fk-table-level [2m(11.13ms)[0m
  [[32mPASS[0m] strict-alter-add-column-invalid-varchar  [2m(6.80ms)[0m
  [[32mPASS[0m] fail-drop-column-table-level-check-constraint [2m(8.76ms)[0m
  [[32mPASS[0m] alter-table-rename-column-sibling-fk-still-enforced [2m(11.88ms)[0m
  [[32mPASS[0m] alter-table-rename-column-sibling-fk-violation-after-rename [2m(11.66ms)[0m
  [[32mPASS[0m] strict-alter-add-column-no-type          [2m(8.17ms)[0m
  [[32mPASS[0m] strict-alter-add-column-invalid-datetime [2m(8.80ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-drop-parent-ok-with-multiple-orphaned-fks [2m(8.84ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] strict-alter-add-column-integer          [2m(7.16ms)[0m
  [[32mPASS[0m] strict-alter-add-column-text             [2m(7.52ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] strict-valid-datatypes                   [2m(5.84ms)[0m
  [[32mPASS[0m] strict-type-enforcement                  [2m(4.57ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-drop-parent-ok-with-orphaned-and-null [2m(8.87ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] strict-alter-add-column-default-real-from-text [2m(5.31ms)[0m
  [[32mPASS[0m] alter-table-rename-parent-col-then-drop-on-child-keeps-renamed-fk [2m(14.02ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-drop-parent-fail-with-orphaned-and-valid [2m(7.63ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] strict-type-case-insensitivity           [2m(11.03ms)[0m
  [[32mPASS[0m] strict-int-vs-integer-pk                 [2m(2.42ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-to-quoted-identifier  [2m(22.10ms)[0m
  [[32mPASS[0m] strict-alter-add-column-default-text-from-integer [2m(7.59ms)[0m
  [[32mPASS[0m] strict-alter-add-column-any              [2m(9.71ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] strict-any-flexibility                   [2m(5.26ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-drop-parent-ok-composite-orphaned     [2m(9.69ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-rename-column-updates-sibling-fk-inline [2m(19.52ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] strict-update-basic                      [2m(5.52ms)[0m
  [[32mPASS[0m] strict-update-type-enforcement           [2m(5.13ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-drop-parent-cascade-multi-children    [2m(7.79ms)[0m
  [[32mPASS[0m] fk-drop-parent-setnull-sets-children     [2m(7.24ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] strict-alter-add-column-default-type-mismatch [2m(13.67ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-drop-parent-cascade-null-preserved    [2m(8.26ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] strict-any-preservation                  [2m(9.64ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-drop-parent-fail-composite-valid      [2m(12.03ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] strict-alter-add-column-default-integer-from-text [2m(14.47ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] strict-type-coercion                     [2m(11.22ms)[0m
  [[32mPASS[0m] strict-update-any-flexibility            [2m(6.31ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-drop-parent-setnull-multi-children    [2m(8.49ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] strict-alter-add-column-default-real-nonnumeric [2m(12.28ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-drop-parent-setnull-null-unchanged    [2m(8.23ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] strict-mixed-inserts                     [2m(12.33ms)[0m
  [[32mPASS[0m] strict-update-pk-constraint              [2m(6.05ms)[0m
  [[32mPASS[0m] strict-text-float-to-integer-zero        [2m(3.46ms)[0m
  [[32mPASS[0m] strict-integer-pk-behavior               [2m(13.19ms)[0m
  [[32mPASS[0m] strict-update-where-clause               [2m(6.43ms)[0m
  [[32mPASS[0m] strict-update-type-coercion              [2m(11.60ms)[0m
  [[32mPASS[0m] strict-update-not-null-constraint        [2m(9.01ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] strict-alter-add-column-default-mismatch-empty-table [2m(9.83ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] strict-update-expression                 [2m(5.42ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-drop-parent-setdefault-null-default   [2m(8.42ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] strict-text-float-to-integer-negative    [2m(4.64ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] strict-alter-add-column-default-text-blob [2m(13.89ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] strict-text-float-to-integer-basic       [2m(5.74ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-drop-parent-cascade-composite-fk      [2m(7.81ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-add-column-invalid-collation       [2m(8.89ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] strict-text-float-to-integer-scientific  [2m(5.42ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-drop-parent-setnull-composite-fk      [2m(9.02ms)[0m
  [[32mPASS[0m] fk-drop-column-child-fk-fails            [2m(5.10ms)[0m
  [[32mPASS[0m] fk-drop-parent-setdefault-nonexistent-default [2m(11.21ms)[0m
  [[32mPASS[0m] fk-drop-column-orphaned-fk-fails         [2m(4.70ms)[0m
  [[32mPASS[0m] fk-drop-parent-mixed-with-restrict       [2m(8.92ms)[0m
  [[32mPASS[0m] fk-drop-parent-mixed-cascade-setnull     [2m(9.70ms)[0m
  [[32mPASS[0m] fk-drop-parent-cascade-deletes-children  [2m(21.53ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] strict-text-float-to-integer-fractional-error [2m(4.26ms)[0m
  [[32mPASS[0m] strict-update-multiple-columns           [2m(5.79ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-drop-parent-cascade-recursive         [2m(9.59ms)[0m
  [[32mPASS[0m] fk-deferred-update-null-to-null          [2m(6.87ms)[0m
  [[32mPASS[0m] fk-deferred-noop-update-null-multi-row   [2m(7.06ms)[0m
  [[32mPASS[0m] fk-immediate-noop-update-null            [2m(7.19ms)[0m
  [[32mPASS[0m] fk-deferred-noop-update-valid-parent     [2m(8.03ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] fail-alter-table-drop-column-expr-index-substr [2m(7.43ms)[0m
  [[32mPASS[0m] fail-alter-table-drop-column-expr-index-arithmetic-first-col [2m(3.79ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-update-null-row-does-not-clear-violation [2m(6.86ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] strict-update-any-preservation           [2m(18.82ms)[0m
  [[32mPASS[0m] strict-text-integer-to-real-basic        [2m(5.01ms)[0m
  [[32mPASS[0m] strict-text-integer-to-real-negative     [2m(5.05ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-noop-update-null             [2m(9.79ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] strict-text-integer-to-real-zero         [2m(5.15ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-noop-update-on-violating-row [2m(6.57ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] strict-alter-add-column-default-integer-fractional [2m(28.46ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-drop-parent-cascade-with-orphaned     [2m(12.19ms)[0m
  [[32mPASS[0m] fk-deferred-update-null-ipk-parent-does-not-clear [2m(8.16ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] strict-text-string-to-real-error         [2m(4.47ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] strict-alter-add-column-default-blob-text [2m(25.55ms)[0m
  [[32mPASS[0m] fail-alter-table-drop-column-expr-index-arithmetic [2m(10.15ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-update-multiple-null-rows    [2m(10.16ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] named-insert-1                           [2m(6.37ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-update-or-replace-cascade-on-unique-conflict [2m(8.93ms)[0m
  [[32mPASS[0m] fk-update-or-replace-set-default-on-unique-conflict [2m(8.74ms)[0m
  [[32mPASS[0m] fk-drop-column-child-fk-enabled-fails    [2m(19.76ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] strict-text-float-to-real                [2m(13.39ms)[0m
  [[32mPASS[0m] unique_insert_no_pkey                    [2m(4.03ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] fail-alter-table-drop-column-expr-index-multi-col [2m(11.24ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-update-or-replace-set-null-on-rowid-conflict [2m(8.30ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-add-column-default-negative-float  [2m(9.53ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-update-or-replace-set-null-on-unique-conflict [2m(12.89ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-add-column-default-negative-integer [2m(11.13ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-update-or-replace-cascade-on-rowid-conflict [2m(10.80ms)[0m
  [[32mPASS[0m] fk-deferred-update-null-to-valid         [2m(15.05ms)[0m
  [[32mPASS[0m] fk-drop-column-composite-fk-fails        [2m(21.93ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-drop-column-expr-index-unreferenced [2m(16.22ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] multi-rows                               [2m(9.81ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-update-or-replace-cascade-multiple-children [2m(13.63ms)[0m
  [[32mPASS[0m] fk-update-or-replace-set-default-on-rowid-conflict [2m(9.21ms)[0m
  [[32mPASS[0m] fk-update-or-replace-no-conflict         [2m(19.03ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] duplicate-set                            [2m(20.50ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-add-column-default-positive-unary  [2m(15.12ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] unique_insert_with_pkey                  [2m(12.22ms)[0m
  [[32mPASS[0m] not_null_primary_key                     [2m(10.54ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-add-column-default-negative-text-affinity [2m(16.30ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] named-insert-2                           [2m(22.57ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-table-drop-column-preserves-expression-index-sentinel [2m(26.59ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] not_null_insert                          [2m(13.24ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-update-or-replace-set-null-update-on-rowid-conflict [2m(15.53ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-add-column-default-boolean         [2m(5.85ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert_from_select_where                 [2m(5.88ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-alter-table-add-column-enforced       [2m(6.69ms)[0m
  [[32mPASS[0m] fk-alter-table-add-column-rejects-missing [2m(4.83ms)[0m
  [[32mPASS[0m] fk-update-or-replace-set-default-update-on-rowid-conflict [2m(17.05ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-add-column-default-parenthesized-negative [2m(5.14ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-update-or-replace-rowid-conflict-with-update-cascade-only [2m(15.77ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-add-column-negative-default-new-rows [2m(16.38ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-update-or-replace-restrict-update-on-rowid-conflict-blocks [2m(17.30ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-add-column-negative-default-index-scan [2m(15.92ms)[0m
  [[32mPASS[0m] alter-table-rename-column-updates-unique-sets [2m(30.31ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-update-or-replace-restrict-on-rowid-conflict [2m(22.78ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-add-column-default-negative-real-affinity [2m(19.17ms)[0m
  [[32mPASS[0m] alter-add-column-default-negative-zero-float [2m(4.38ms)[0m
  [[32mPASS[0m] alter-add-column-default-large-negative  [2m(5.67ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-update-or-replace-no-action-update-on-rowid-conflict-blocks [2m(18.89ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert_from_select_1                     [2m(18.34ms)[0m

[1mtests/gencol-virtual-constant.sqltest[0m
  [[32mPASS[0m] gencol-virtual-mixed-notnull-select      [2m(8.17ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-add-column-multiple-negative-defaults [2m(22.44ms)[0m

[1mtests/gencol-virtual-constant.sqltest[0m
  [[32mPASS[0m] gencol-virtual-constant-notnull-int-select [2m(9.81ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_virtual_select_star               [2m(4.99ms)[0m
  [[32mPASS[0m] gencol_update_fanout_multiple_rows       [2m(4.74ms)[0m

[1mtests/gencol-virtual-constant.sqltest[0m
  [[32mPASS[0m] gencol-virtual-constant-notnull-text-select [2m(11.55ms)[0m
  [[32mPASS[0m] gencol-virtual-constant-index-lookup     [2m(8.62ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-add-column-default-string-spaces   [2m(5.60ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] not_null_insert_select                   [2m(22.78ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-add-column-default-blob            [2m(8.96ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert_from_select_union_all_where       [2m(12.20ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_virtual_basic_1                   [2m(9.39ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] values-union-all                         [2m(8.96ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_virtual_basic_2                   [2m(9.10ms)[0m

[1mtests/alter_table.sqltest[0m
  [[32mPASS[0m] alter-add-column-default-null            [2m(9.48ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert_from_select_union_all_values      [2m(12.06ms)[0m
  [[32mPASS[0m] insert_from_select_same_table            [2m(10.68ms)[0m
  [[32mPASS[0m] insert_from_select_union_all             [2m(16.13ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_tvf_left_join_virtual_column      [2m(4.77ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-update-or-replace-no-action-blocks    [2m(27.65ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_cross_join                        [2m(7.10ms)[0m
  [[32mPASS[0m] gencol_tvf_comma_join_json_each_virtual_column [2m(6.74ms)[0m
  [[32mPASS[0m] gencol_update_fanout                     [2m(12.52ms)[0m
  [[32mPASS[0m] gencol_tvf_first_comma_join_virtual_column [2m(6.05ms)[0m
  [[32mPASS[0m] gencol_tvf_left_join_virtual_column_table_first [2m(5.53ms)[0m
  [[32mPASS[0m] gencol_tvf_comma_join_base_column        [2m(6.28ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] not_null_insert_multiple_values          [2m(30.35ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_tvf_comma_join_generate_series_virtual_column [2m(8.59ms)[0m
  [[32mPASS[0m] gencol_cte_virtual                       [2m(5.80ms)[0m
  [[32mPASS[0m] gencol_virtual_join                      [2m(10.15ms)[0m
  [[32mPASS[0m] gencol_group_by_virtual                  [2m(11.44ms)[0m
  [[32mPASS[0m] gencol_hash_join_virtual_affinity        [2m(10.88ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert_from_select_same_table_2          [2m(17.56ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_limit_offset_virtual              [2m(6.01ms)[0m
  [[32mPASS[0m] gencol_where_virtual                     [2m(16.90ms)[0m
  [[32mPASS[0m] gencol_correlated_subquery_virtual       [2m(5.57ms)[0m
  [[32mPASS[0m] gencol_exists_subquery                   [2m(14.40ms)[0m
  [[32mPASS[0m] gencol_window_functions_multiple         [2m(6.72ms)[0m
  [[32mPASS[0m] gencol_window_sum_partition              [2m(11.90ms)[0m
  [[32mPASS[0m] gencol_intersect_virtual                 [2m(9.99ms)[0m
  [[32mPASS[0m] gencol_having_count                      [2m(16.51ms)[0m
  [[32mPASS[0m] gencol_distinct_virtual                  [2m(10.24ms)[0m
  [[32mPASS[0m] gencol_union_virtual                     [2m(11.70ms)[0m
  [[32mPASS[0m] gencol_having_multiple_aggregates        [2m(8.74ms)[0m
  [[32mPASS[0m] gencol_constraint_notnull_virtual_ifnull_success [2m(4.50ms)[0m
  [[32mPASS[0m] gencol_scalar_subquery_in_select         [2m(7.33ms)[0m
  [[32mPASS[0m] gencol_left_join_virtual                 [2m(10.69ms)[0m
  [[32mPASS[0m] gencol_order_by                          [2m(20.04ms)[0m
  [[32mPASS[0m] gencol_view_virtual                      [2m(13.81ms)[0m
  [[32mPASS[0m] gencol_constraint_notnull_virtual_violation [2m(7.63ms)[0m
  [[32mPASS[0m] gencol_union_all_virtual                 [2m(9.87ms)[0m
  [[32mPASS[0m] gencol_except_virtual                    [2m(13.66ms)[0m
  [[32mPASS[0m] gencol_check_table_level_referencing_virtual [2m(4.70ms)[0m
  [[32mPASS[0m] gencol_distinct_virtual_with_base        [2m(13.55ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert_from_select_union-2               [2m(6.83ms)[0m
  [[32mPASS[0m] insert_from_select_union                 [2m(7.35ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_check_on_virtual_violation        [2m(7.02ms)[0m
  [[32mPASS[0m] gencol_offset_past_end                   [2m(13.60ms)[0m
  [[32mPASS[0m] gencol_tvf_subquery_virtual_column       [2m(20.72ms)[0m
  [[32mPASS[0m] gencol_check_table_level_referencing_virtual_violation [2m(6.04ms)[0m
  [[32mPASS[0m] gencol_check_update_virtual_success      [2m(5.47ms)[0m
  [[32mPASS[0m] gencol_constraint_notnull_virtual_case_success [2m(10.94ms)[0m
  [[32mPASS[0m] gencol_constraint_notnull_virtual_success [2m(13.45ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert_from_select_except                [2m(6.85ms)[0m
  [[32mPASS[0m] insert_from_select_intersect             [2m(8.33ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_constraint_notnull_virtual_multiple [2m(11.23ms)[0m
  [[32mPASS[0m] gencol_constraint_notnull_virtual_coalesce_success [2m(12.34ms)[0m
  [[32mPASS[0m] gencol_check_update_virtual_violation    [2m(9.13ms)[0m
  [[32mPASS[0m] gencol_check_update_virtual_chain        [2m(8.77ms)[0m
  [[32mPASS[0m] gencol_strict_update_type_check          [2m(13.74ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] rowid-overflow-random-generation         [2m(10.30ms)[0m
  [[32mPASS[0m] negative-primary-integer-key             [2m(11.64ms)[0m
  [[32mPASS[0m] not-null-rowid-alias                     [2m(11.11ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_check_multiple_virtual            [2m(9.72ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert-tricky-column-order-values        [2m(9.68ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_check_virtual_null_passes         [2m(8.73ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert-explicit-rowid                    [2m(9.62ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_check_update_virtual_chain_success [2m(12.42ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert-tricky-column-order-table         [2m(10.97ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_check_virtual_coalesce            [2m(10.57ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert-explicit-rowid-with-rowidalias    [2m(10.18ms)[0m
  [[32mPASS[0m] returning-multiple-columns               [2m(4.70ms)[0m
  [[32mPASS[0m] returning-basic-column                   [2m(6.78ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_glob_in_expression                [2m(4.89ms)[0m
  [[32mPASS[0m] gencol_check_update_or_ignore            [2m(5.95ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] returning-literal                        [2m(4.62ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_check_on_virtual                  [2m(21.98ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] uniq_constraint                          [2m(12.54ms)[0m
  [[32mPASS[0m] returning-all-columns                    [2m(5.22ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_is_null_operator                  [2m(5.42ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert_from_select_intersect-2           [2m(21.39ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_virtual_no_index_where            [2m(4.12ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] returning-mixed-expressions              [2m(3.67ms)[0m
  [[32mPASS[0m] returning-function-call                  [2m(4.30ms)[0m
  [[32mPASS[0m] null-value-insert-null-type-column       [2m(15.39ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_virtual_non_covering_index        [2m(5.34ms)[0m
  [[32mPASS[0m] gencol_check_virtual_function            [2m(15.57ms)[0m
  [[32mPASS[0m] gencol_check_insert_or_ignore            [2m(12.83ms)[0m
  [[32mPASS[0m] gencol_check_multiple_virtual_violation  [2m(18.20ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] large-text-index-seek                    [2m(19.21ms)[0m
  [[32mPASS[0m] returning-arithmetic                     [2m(8.14ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_index_virtual_where_filter        [2m(7.20ms)[0m
  [[32mPASS[0m] gencol_is_not_null_operator              [2m(10.12ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] returning-true-literal                   [2m(3.41ms)[0m
  [[32mPASS[0m] returning-null-values                    [2m(5.35ms)[0m
  [[32mPASS[0m] returning-with-autoincrement             [2m(6.44ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_index_virtual_update_integrity    [2m(8.03ms)[0m
  [[32mPASS[0m] gencol_check_virtual_coalesce_violation  [2m(21.12ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] null-insert-in-nulltype-column-notnull-constraint [2m(6.58ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_index_virtual_multiple_updates    [2m(8.13ms)[0m
  [[32mPASS[0m] gencol_cte_insert_into_gencol            [2m(36.74ms)[0m
  [[32mPASS[0m] gencol_check_virtual_function_violation  [2m(21.35ms)[0m
  [[32mPASS[0m] gencol_index_virtual_multirow_update     [2m(7.17ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] returning-multiple-rows                  [2m(10.50ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_integrity_virtual_middle_verify   [2m(4.81ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert-explicit-rowid-conflict           [2m(20.68ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_index_on_virtual                  [2m(13.89ms)[0m
  [[32mPASS[0m] gencol_update_deep_virtual_chain_in_index [2m(5.61ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] boolean-literal-edgecase-false           [2m(4.82ms)[0m
  [[32mPASS[0m] returning-rowid                          [2m(13.32ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_integrity_multiple_virtual_before [2m(8.96ms)[0m
  [[32mPASS[0m] gencol_expression_index_on_virtual_basic_integrity [2m(6.40ms)[0m
  [[32mPASS[0m] gencol_multiple_expr_indexes_virtual_integrity [2m(6.85ms)[0m
  [[32mPASS[0m] gencol_expr_index_multi_virtual_integrity [2m(7.18ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] boolean-literal-edgecase                 [2m(11.61ms)[0m

[1mtests/gencol.sqltest[0m
  [[33mSKIP[0m] delete-text-pk-indexed-virtual-column    [2m(3.31µs)[0m [2m(Same col_used_mask bug exists in DELETE path - panics (see #6206))[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] returning-complex-expression             [2m(18.83ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_expr_index_virtual_multirow_update [2m(5.94ms)[0m
  [[32mPASS[0m] gencol_expr_index_virtual_basic          [2m(7.85ms)[0m
  [[32mPASS[0m] gencol_expr_index_virtual_delete         [2m(6.42ms)[0m
  [[32mPASS[0m] gencol_update_virtual_chain_in_index     [2m(12.08ms)[0m
  [[32mPASS[0m] gencol_integrity_virtual_index           [2m(15.32ms)[0m
  [[32mPASS[0m] gencol_expr_index_virtual_update_returning_halloween [2m(9.86ms)[0m
  [[32mPASS[0m] gencol_covering_index_virtual_column_dependency [2m(13.60ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] default-values-population                [2m(3.87ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_integrity_virtual_middle          [2m(17.38ms)[0m
  [[32mPASS[0m] gencol_index_virtual_update_basic        [2m(20.32ms)[0m
  [[32mPASS[0m] gencol_multiple_indexes_delete           [2m(6.92ms)[0m
  [[32mPASS[0m] gencol_update_index_on_virtual_col       [2m(9.74ms)[0m
  [[32mPASS[0m] gencol_expression_index_on_virtual       [2m(13.85ms)[0m
  [[32mPASS[0m] gencol_multiple_indexes_insert           [2m(8.32ms)[0m
  [[32mPASS[0m] gencol_index_virtual_chain               [2m(21.42ms)[0m
  [[32mPASS[0m] gencol_expr_index_virtual_update_transitive_halloween [2m(12.44ms)[0m
  [[32mPASS[0m] gencol_update_virtual_chain_multiple_rows [2m(17.24ms)[0m
  [[32mPASS[0m] gencol_multiple_indexes_update           [2m(10.32ms)[0m
  [[32mPASS[0m] gencol_two_virtual_indexes               [2m(9.42ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] returning-false-literal                  [2m(21.90ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_trigger_virtual_expression        [2m(6.82ms)[0m
  [[33mSKIP[0m] gencol_upsert_virtual_before_regular_with_index [2m(2.73µs)[0m [2m(TODO requires https://github.com/tursodatabase/turso/issues/6087)[0m
  [[32mPASS[0m] update-text-pk-virtual-multi-dep         [2m(10.22ms)[0m
  [[32mPASS[0m] update-text-pk-mixed-indexes             [2m(9.62ms)[0m
  [[32mPASS[0m] update-text-pk-expression-index          [2m(11.15ms)[0m
  [[32mPASS[0m] gencol_trigger_when_virtual              [2m(7.11ms)[0m
  [[32mPASS[0m] gencol_trigger_virtual_chain             [2m(6.46ms)[0m
  [[32mPASS[0m] gencol_trigger_after_insert_virtual      [2m(10.75ms)[0m
  [[32mPASS[0m] gencol_expr_index_virtual_update         [2m(17.99ms)[0m
  [[32mPASS[0m] gencol_trigger_before_insert_virtual     [2m(9.89ms)[0m
  [[32mPASS[0m] gencol_trigger_after_update_virtual_old_new [2m(8.28ms)[0m
  [[32mPASS[0m] gencol_upsert_set_current_row            [2m(5.93ms)[0m
  [[32mPASS[0m] gencol_upsert_virtual_error              [2m(6.86ms)[0m
  [[32mPASS[0m] gencol_returning_upsert                  [2m(6.23ms)[0m
  [[32mPASS[0m] update-text-pk-with-indexed-virtual-column [2m(13.91ms)[0m
  [[32mPASS[0m] gencol_upsert_where_clause               [2m(6.21ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] set-explicit-null-default-value          [2m(5.35ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_upsert_set_references_virtual     [2m(5.47ms)[0m
  [[32mPASS[0m] gencol_upsert_virtual                    [2m(9.31ms)[0m
  [[32mPASS[0m] gencol_upsert_index_integrity            [2m(7.07ms)[0m
  [[32mPASS[0m] gencol_update_trigger_new_values         [2m(9.76ms)[0m
  [[32mPASS[0m] gencol_update_indexed_virtual_col        [2m(19.05ms)[0m
  [[32mPASS[0m] gencol_trigger_for_each_row_virtual      [2m(12.52ms)[0m
  [[32mPASS[0m] gencol_error_drop_column_referenced_virtual [2m(7.92ms)[0m
  [[32mPASS[0m] gencol_trigger_before_update_virtual_new [2m(12.81ms)[0m
  [[32mPASS[0m] gencol_arithmetic_add                    [2m(5.39ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert-select-nested-subquery            [2m(5.49ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-limit                             [2m(4.86ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_error_drop_column_in_function     [2m(8.77ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-total-text                        [2m(5.63ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_drop_virtual_column               [2m(8.05ms)[0m
  [[32mPASS[0m] gencol_generated_always_virtual          [2m(7.71ms)[0m
  [[32mPASS[0m] gencol_pragma_table_xinfo_integer_as_virtual [2m(7.25ms)[0m
  [[32mPASS[0m] gencol_virtual_expression_concat         [2m(8.03ms)[0m
  [[32mPASS[0m] gencol_error_drop_column_case_insensitive [2m(8.42ms)[0m
  [[32mPASS[0m] gencol_trigger_multiple_virtual          [2m(14.63ms)[0m
  [[32mPASS[0m] gencol_multiple_virtual                  [2m(7.78ms)[0m
  [[32mPASS[0m] gencol_arithmetic_subtract               [2m(7.04ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_groupby_ordinal_with_explicit_collate [2m(4.62ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_upsert_fk_enforcement             [2m(11.70ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_orderby_ordinal_with_explicit_collate [2m(4.99ms)[0m

[1mtests/gencol.sqltest[0m
  [[32mPASS[0m] gencol_error_drop_generated_column_with_dependent [2m(10.65ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_orderby_parenthesized_ordinal_with_explicit_collate [2m(5.03ms)[0m
  [[32mPASS[0m] collate_concat_nocase_comparison         [2m(2.87ms)[0m
  [[32mPASS[0m] collate_concat_binary_comparison         [2m(2.89ms)[0m
  [[32mPASS[0m] collate_concat_check_constraint_nocase   [2m(3.65ms)[0m
  [[32mPASS[0m] collate_orderby_ordinal_overrides_column_collate [2m(4.47ms)[0m
  [[32mPASS[0m] collate_orderby_nested_parens_ordinal_with_explicit_collate [2m(4.68ms)[0m
  [[32mPASS[0m] collate_orderby_ordinal_inherits_column_collate [2m(5.33ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-count-2                           [2m(9.30ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_orderby_expression_collation     [2m(9.33ms)[0m
  [[32mPASS[0m] agg_collate_no_leak_min_max              [2m(5.11ms)[0m

[1mtests/column_name_case.sqltest[0m
  [[32mPASS[0m] multiple_mixed_case_columns              [2m(4.65ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] agg_collate_no_leak_three_aggs           [2m(5.11ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-eq-int-int-1                     [2m(2.78ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] agg_collate_no_leak_max_min              [2m(5.69ms)[0m
  [[32mPASS[0m] collate_orderby_ordinal_with_collate_inside_parens [2m(7.67ms)[0m

[1mtests/column_name_case.sqltest[0m
  [[32mPASS[0m] select_star_headers                      [2m(5.81ms)[0m
  [[32mPASS[0m] update_with_mixed_case_column_names      [2m(5.55ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-eq-int-null                      [2m(2.00ms)[0m
  [[32mPASS[0m] compare-eq-int-int-2                     [2m(2.85ms)[0m

[1mtests/column_name_case.sqltest[0m
  [[32mPASS[0m] primary_key_with_mixed_case              [2m(5.09ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-eq-float-float-1                 [2m(2.32ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] collate_concat_check_constraint_allows_valid [2m(8.08ms)[0m

[1mtests/column_name_case.sqltest[0m
  [[32mPASS[0m] table_level_primary_key                  [2m(5.32ms)[0m
  [[32mPASS[0m] unique_constraint_with_mixed_case        [2m(5.42ms)[0m
  [[32mPASS[0m] quoted_identifiers                       [2m(5.15ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-eq-float-float-2                 [2m(2.81ms)[0m

[1mtests/column_name_case.sqltest[0m
  [[32mPASS[0m] case_insensitive_column_reference        [2m(7.79ms)[0m
  [[32mPASS[0m] basic_case_preservation                  [2m(8.02ms)[0m
  [[32mPASS[0m] create_index_on_mixed_case_column        [2m(5.89ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-eq-float-null                    [2m(2.93ms)[0m
  [[32mPASS[0m] compare-eq-text-text-1                   [2m(2.78ms)[0m
  [[32mPASS[0m] compare-eq-text-text-2                   [2m(2.70ms)[0m

[1mtests/column_name_case.sqltest[0m
  [[32mPASS[0m] foreign_key_with_mixed_case              [2m(6.69ms)[0m

[1mtests/collate.sqltest[0m
  [[32mPASS[0m] agg_collate_explicit_nocase_both         [2m(9.02ms)[0m
  [[32mPASS[0m] agg_collate_no_leak_with_count           [2m(9.21ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-eq-text-null                     [2m(2.79ms)[0m

[1mtests/column_name_case.sqltest[0m
  [[32mPASS[0m] insert_with_mixed_case_column_names      [2m(9.43ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-eq-null-int                      [2m(2.74ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert-rowid-select-rowid                [2m(5.22ms)[0m
  [[32mPASS[0m] insert-rowidalias-select-rowid           [2m(5.18ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-eq-null-float                    [2m(2.75ms)[0m
  [[32mPASS[0m] compare-eq-null-text                     [2m(2.76ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert-rowid-backwards-compability-2     [2m(4.70ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-eq-null-null                     [2m(2.66ms)[0m
  [[32mPASS[0m] compare-neq-int-int-1                    [2m(2.65ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert-rowid-backwards-compability       [2m(5.53ms)[0m
  [[32mPASS[0m] insert-rowid-select-rowid-success        [2m(5.83ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-neq-int-int-2                    [2m(2.68ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] ignore-pk-conflict                       [2m(5.25ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-neq-float-float-1                [2m(2.74ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] ignore-unique-conflict                   [2m(5.56ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-neq-float-float-2                [2m(2.77ms)[0m
  [[32mPASS[0m] compare-neq-float-null                   [2m(2.74ms)[0m
  [[32mPASS[0m] compare-neq-text-text-1                  [2m(2.73ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] wrong-column-count-values-too-few        [2m(4.22ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-neq-text-text-2                  [2m(2.74ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] ignore-some-conflicts-multirow           [2m(6.07ms)[0m
  [[32mPASS[0m] ignore-multi-unique-conflict             [2m(6.84ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-neq-text-null                    [2m(2.81ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] ignore-null-in-unique                    [2m(5.88ms)[0m
  [[32mPASS[0m] ignore-intra-statement-dups              [2m(5.22ms)[0m
  [[32mPASS[0m] wrong-column-count-values-specified-columns-too-few [2m(4.49ms)[0m
  [[32mPASS[0m] wrong-column-count-values-specified-columns-too-many [2m(4.38ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-neq-null-float                   [2m(2.75ms)[0m
  [[32mPASS[0m] compare-neq-null-text                    [2m(2.69ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] ignore-from-select                       [2m(7.50ms)[0m
  [[32mPASS[0m] ignore-preserves-rowid                   [2m(7.13ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-neq-null-null                    [2m(2.82ms)[0m
  [[32mPASS[0m] compare-gt-int-int-1                     [2m(3.07ms)[0m
  [[32mPASS[0m] compare-gt-int-int-3                     [2m(2.83ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] wrong-column-count-select-named-too-few  [2m(6.19ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-gt-int-null                      [2m(2.61ms)[0m
  [[32mPASS[0m] compare-neq-int-null                     [2m(8.24ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] wrong-column-count-partial-columns-values-too-few [2m(4.37ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-gt-int-int-2                     [2m(3.15ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] wrong-column-count-select-star-too-few   [2m(5.92ms)[0m
  [[32mPASS[0m] wrong-column-count-select-named-too-many [2m(6.18ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-gt-float-float-1                 [2m(2.72ms)[0m
  [[32mPASS[0m] compare-gt-float-float-2                 [2m(2.40ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] wrong-column-count-specified-columns-select-mismatch [2m(5.66ms)[0m
  [[32mPASS[0m] wrong-column-count-partial-columns-values-too-many [2m(4.69ms)[0m
  [[32mPASS[0m] wrong-column-count-select-star-too-many  [2m(6.81ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-neq-null-int                     [2m(7.25ms)[0m
  [[32mPASS[0m] compare-gt-float-float-3                 [2m(3.21ms)[0m
  [[32mPASS[0m] compare-gt-float-null                    [2m(2.74ms)[0m
  [[32mPASS[0m] compare-gt-text-text-3                   [2m(2.69ms)[0m
  [[32mPASS[0m] compare-gt-text-text-1                   [2m(2.82ms)[0m
  [[32mPASS[0m] compare-gt-text-null                     [2m(2.77ms)[0m
  [[32mPASS[0m] compare-gt-null-int                      [2m(2.81ms)[0m
  [[32mPASS[0m] compare-gt-null-float                    [2m(2.87ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] wrong-column-count-values-too-many       [2m(10.60ms)[0m
  [[32mPASS[0m] insert-on-conflict-replace-single-row    [2m(6.18ms)[0m
  [[32mPASS[0m] insert-on-conflict-do-nothing-multiple-rows [2m(5.75ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-gt-null-text                     [2m(2.75ms)[0m
  [[32mPASS[0m] compare-gt-null-null                     [2m(2.83ms)[0m
  [[32mPASS[0m] compare-gte-int-int-1                    [2m(2.70ms)[0m
  [[32mPASS[0m] compare-gte-int-int-2                    [2m(2.66ms)[0m
  [[32mPASS[0m] compare-gt-text-text-2                   [2m(3.93ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] onconflict-ignore-selfdup-values         [2m(5.57ms)[0m
  [[32mPASS[0m] onconflict-ignore-existing               [2m(6.41ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-gte-int-int-3                    [2m(3.16ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] onconflict-ignore-selfdup-values-targeted [2m(5.46ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-gte-int-null                     [2m(2.75ms)[0m
  [[32mPASS[0m] compare-gte-float-float-1                [2m(2.69ms)[0m
  [[32mPASS[0m] compare-gte-float-null                   [2m(2.46ms)[0m
  [[32mPASS[0m] compare-gte-float-float-2                [2m(2.58ms)[0m
  [[32mPASS[0m] compare-gte-float-float-3                [2m(2.69ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] onconflict-ignore-selfdup-multirow-targeted [2m(5.95ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-gte-text-text-1                  [2m(2.84ms)[0m
  [[32mPASS[0m] compare-gte-text-text-2                  [2m(2.79ms)[0m
  [[32mPASS[0m] compare-gte-null-int                     [2m(2.66ms)[0m
  [[32mPASS[0m] compare-gte-null-float                   [2m(2.74ms)[0m
  [[32mPASS[0m] compare-gte-null-text                    [2m(2.67ms)[0m
  [[32mPASS[0m] compare-gte-null-null                    [2m(2.69ms)[0m
  [[32mPASS[0m] compare-lt-int-int-1                     [2m(2.74ms)[0m
  [[32mPASS[0m] compare-lt-int-int-2                     [2m(2.74ms)[0m
  [[32mPASS[0m] compare-gte-text-text-3                  [2m(3.90ms)[0m
  [[32mPASS[0m] compare-lt-int-null                      [2m(2.71ms)[0m
  [[32mPASS[0m] compare-lt-float-float-1                 [2m(2.72ms)[0m
  [[32mPASS[0m] compare-lt-float-float-2                 [2m(2.70ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] onconflict-replace-existing              [2m(6.19ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-lt-text-text-1                   [2m(2.55ms)[0m
  [[32mPASS[0m] compare-lt-float-null                    [2m(2.58ms)[0m
  [[32mPASS[0m] compare-lt-float-float-3                 [2m(2.72ms)[0m
  [[32mPASS[0m] compare-lt-text-text-2                   [2m(2.67ms)[0m
  [[32mPASS[0m] compare-lt-int-int-3                     [2m(3.95ms)[0m
  [[32mPASS[0m] compare-lt-text-null                     [2m(2.78ms)[0m
  [[32mPASS[0m] compare-lt-null-int                      [2m(2.74ms)[0m
  [[32mPASS[0m] compare-lt-text-text-3                   [2m(2.84ms)[0m
  [[32mPASS[0m] compare-lt-null-float                    [2m(2.70ms)[0m
  [[32mPASS[0m] compare-lt-null-text                     [2m(2.70ms)[0m
  [[32mPASS[0m] compare-lt-null-null                     [2m(2.62ms)[0m
  [[32mPASS[0m] compare-lte-int-int-1                    [2m(2.75ms)[0m
  [[32mPASS[0m] compare-lte-int-int-2                    [2m(2.71ms)[0m
  [[32mPASS[0m] compare-lte-int-int-3                    [2m(2.67ms)[0m
  [[32mPASS[0m] compare-gte-text-null                    [2m(6.60ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] onconflict-replace-selfdup-values-lastwins [2m(5.65ms)[0m
  [[32mPASS[0m] insert-on-conflict-do-nothing-single-row [2m(15.61ms)[0m

[1mtests/compare.sqltest[0m
  [[32mPASS[0m] compare-lte-int-null                     [2m(5.54ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-self-unique-parent-affinity-does-not-coerce-same-row-child [2m(5.16ms)[0m
  [[32mPASS[0m] fk-self-unique-mismatch                  [2m(5.00ms)[0m
  [[32mPASS[0m] fk-self-unique-parent-collation-does-not-satisfy-same-row-child [2m(6.06ms)[0m
  [[32mPASS[0m] fk-self-unique-parent-text-does-not-coerce-same-row-child [2m(7.54ms)[0m
  [[32mPASS[0m] fk-cross-table-parent-affinity-still-coerces-child [2m(7.09ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert-999999999                         [2m(18.82ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-self-unique-same-row-stored-values-match [2m(8.06ms)[0m
  [[32mPASS[0m] fk-deferred-self-ref-late-parent         [2m(5.51ms)[0m
  [[32mPASS[0m] fk-deferred-update-fixes-parent-before-commit [2m(7.93ms)[0m
  [[32mPASS[0m] fk-self-unique-multirow-no-fastpath      [2m(9.34ms)[0m
  [[32mPASS[0m] fk-deferred-commit-doesnt-fail-early     [2m(10.96ms)[0m
  [[32mPASS[0m] fk-deferred-commit-fails                 [2m(10.99ms)[0m
  [[32mPASS[0m] fk-self-unique-reference-existing-ok     [2m(11.70ms)[0m
  [[32mPASS[0m] fk-deferred-upsert-late-parent           [2m(7.38ms)[0m
  [[32mPASS[0m] fk-deferred-upsert-late-child            [2m(7.21ms)[0m
  [[32mPASS[0m] fk-cross-table-parent-collation-still-matches-child [2m(12.84ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-count                             [2m(43.51ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-self-multirow-one-bad                 [2m(13.91ms)[0m
  [[32mPASS[0m] fk-deferred-insert-parent-fixes-before-commit [2m(13.86ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] onconflict-replace-existing-then-selfdup-lastwins [2m(5.63ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-parent-update-ignore-noop-keeps-old-key-valid [2m(9.37ms)[0m
  [[32mPASS[0m] fk-deferred-parent-update-ignore-notnull-noop-keeps-old-key-valid [2m(8.80ms)[0m
  [[32mPASS[0m] fk-deferred-delete-fixes-child-before-commit [2m(15.34ms)[0m
  [[32mPASS[0m] fk-deferred-fix-before-commit-succeeds   [2m(16.19ms)[0m
  [[32mPASS[0m] fk-deferred-child-update-ignore-noop-keeps-old-key-valid [2m(9.60ms)[0m
  [[32mPASS[0m] fk-deferred-self-ref-succeeds            [2m(15.78ms)[0m
  [[32mPASS[0m] fk-deferred-parent-ipk-update-ignore-noop-keeps-violation [2m(10.66ms)[0m
  [[32mPASS[0m] fk-deferred-parent-update-ignore-noop-keeps-violation [2m(12.54ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] onconflict-replace-notnull-default-existing-omit [2m(5.04ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-child-update-ignore-noop-keeps-violation [2m(10.55ms)[0m
  [[32mPASS[0m] fk-deferred-rollback-clears              [2m(18.99ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] onconflict-replace-composite-unique-existing [2m(4.88ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-neutralize_1                 [2m(17.05ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] test-1_2                                 [2m(5.39ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-insert-self-ref-succeeds     [2m(5.69ms)[0m
  [[32mPASS[0m] fk-deferred-insert-then-delete-child-fix [2m(6.99ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] onconflict-replace-notnull-default-existing-null [2m(11.22ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-insert-commit-fails          [2m(11.54ms)[0m
  [[32mPASS[0m] fk-deferred-update-fixes-child-before-commit [2m(22.94ms)[0m
  [[32mPASS[0m] fk-deferred-insert-multi-children-one-parent-fix [2m(10.38ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] onconflict-replace-pk-rowid-alias        [2m(13.51ms)[0m
  [[32mPASS[0m] partial-expr-index-upsert-conflict       [2m(5.58ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-update-child-breaks-commit-fails [2m(8.71ms)[0m
  [[32mPASS[0m] fk-deferred-child-ipk-update-ignore-noop-keeps-old-key-valid [2m(14.53ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] test-1_1                                 [2m(11.71ms)[0m
  [[32mPASS[0m] onconflict-replace-unique-index-existing [2m(9.91ms)[0m
  [[32mPASS[0m] onconflict-replace-composite-unique-selfdup-lastwins [2m(10.25ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-parent-update-or-ignore-noop-keeps-violation [2m(21.77ms)[0m
  [[32mPASS[0m] fk-deferred-child-update-or-ignore-noop-keeps-old-key-valid [2m(19.18ms)[0m
  [[32mPASS[0m] fk-deferred-update-parent-fix-by-reverting-parent [2m(6.95ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] onconflict-replace-unique-index-selfdup-lastwins [2m(10.79ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-update-parent-fix-by-updating-child [2m(7.97ms)[0m
  [[32mPASS[0m] fk-deferred-insert-parent-fix-before-commit [2m(16.89ms)[0m
  [[32mPASS[0m] fk-deferred-update-self-ref-id-and-pid-one-stmt [2m(6.22ms)[0m
  [[32mPASS[0m] fk-deferred-update-child-fix-by-inserting-parent [2m(12.67ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] partial-expr-index-delete-clears-entry   [2m(8.33ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-update-self-ref-indexed-child-key-one-stmt [2m(5.86ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] onconflict-replace-from-select-selfdup   [2m(19.71ms)[0m
  [[32mPASS[0m] collate-unique-index-upsert              [2m(8.31ms)[0m
  [[32mPASS[0m] upsert-fires-after-update-trigger        [2m(7.98ms)[0m
  [[32mPASS[0m] upsert-fires-after-update-trigger-2      [2m(6.71ms)[0m
  [[32mPASS[0m] ignore-notnull-single-row                [2m(4.19ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-update-self-ref-unique-key-one-stmt [2m(6.59ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] upsert-fires-before-update-trigger       [2m(7.80ms)[0m
  [[32mPASS[0m] ignore-notnull-multi-row                 [2m(4.73ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-update-self-ref-one-stmt-still-fails-if-missing [2m(6.28ms)[0m
  [[32mPASS[0m] fk-deferred-update-self-ref-composite-key-one-stmt [2m(7.10ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] partial-expr-index-update-removes-entry  [2m(14.00ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-update-self-ref-rowid-affinity-one-stmt [2m(10.91ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] ignore-notnull-all-null                  [2m(6.24ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-delete-self-ref-row-ok       [2m(6.00ms)[0m
  [[32mPASS[0m] fk-deferred-delete-parent-then-delete-child-fix [2m(7.26ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] ignore-notnull-from-select               [2m(5.37ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-delete-parent-then-reinsert-parent-fix [2m(7.66ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] ignore-notnull-with-unique               [2m(5.97ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-autocommit-insert-missing-parent [2m(5.73ms)[0m
  [[32mPASS[0m] fk-deferred-autocommit-selfref-ok        [2m(5.13ms)[0m
  [[32mPASS[0m] fk-deferred-update-self-ref-id-change-and-fix [2m(15.41ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert-select-union-all-all-branches     [2m(5.69ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-update-child-fix-before-commit [2m(21.97ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] upsert-fires-before-and-after-triggers   [2m(13.07ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-delete-parent-commit-fails   [2m(11.03ms)[0m
  [[32mPASS[0m] fk-deferred-autocommit-delete-parent-fails [2m(6.82ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] ignore-notnull-multiple-notnull-cols     [2m(9.53ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-delete-parent-then-update-child-to-null-fix [2m(8.39ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert-select-union-all-branches         [2m(7.71ms)[0m
  [[32mPASS[0m] insert-integer-pk-i64-min-float-error    [2m(4.40ms)[0m
  [[32mPASS[0m] insert-integer-pk-i64-max-float-error    [2m(4.30ms)[0m
  [[32mPASS[0m] insert-select-union-with-table-reference [2m(6.71ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-tx-delete-parent-then-reinsert-ok [2m(7.55ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] upsert-trigger-causes-fk-violation       [2m(14.21ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-delete-child-resolves        [2m(5.86ms)[0m
  [[32mPASS[0m] fk-deferred-tx-multi-children-one-left-fails [2m(8.02ms)[0m
  [[32mPASS[0m] fk-deferred-null-fk-never-violates       [2m(6.34ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert-select-union-three-branches       [2m(10.75ms)[0m
  [[32mPASS[0m] insert-integer-pk-large-float-ok         [2m(4.81ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-deferred-tx-multi-children-fixed-ok   [2m(9.21ms)[0m
  [[32mPASS[0m] fk-deferred-update-child-null-resolves   [2m(7.20ms)[0m
  [[32mPASS[0m] fk-deferred-child-update-rowid-alias-composite-old-probe-second-component [2m(8.13ms)[0m
  [[32mPASS[0m] fk-deferred-update-parent-pk-resolves    [2m(8.02ms)[0m
  [[32mPASS[0m] fk-deferred-composite-parent-update-fix  [2m(9.66ms)[0m
  [[32mPASS[0m] fk-deferred-update-or-replace-parent-pk-resolves-without-conflict [2m(7.85ms)[0m
  [[32mPASS[0m] fk-deferred-selfref-delete-ok            [2m(5.76ms)[0m
  [[32mPASS[0m] fk-deferred-update-or-replace-parent-pk-conflict-does-not-clear-unrelated [2m(7.89ms)[0m
  [[32mPASS[0m] fk-deferred-update-parent-breaks-commit-fails [2m(29.13ms)[0m
  [[32mPASS[0m] fk-deferred-update-or-replace-parent-pk-noop-does-not-clear [2m(7.96ms)[0m
  [[32mPASS[0m] fk-deferred-update-or-replace-parent-unique-key-noop-does-not-clear [2m(8.63ms)[0m
  [[32mPASS[0m] fk-deferred-update-or-replace-parent-pk-noop-does-not-clear-unrelated [2m(8.25ms)[0m
  [[32mPASS[0m] fk-deferred-update-or-replace-parent-unique-key-conflict-does-not-clear-unrelated [2m(9.12ms)[0m
  [[32mPASS[0m] fk-deferred-update-or-replace-parent-unique-key-noop-does-not-clear-unrelated [2m(8.88ms)[0m
  [[32mPASS[0m] fk-parentcomp-donothing-conflict-noop    [2m(7.15ms)[0m
  [[32mPASS[0m] fk-deferred-child-update-rowid-alias-composite-old-probe [2m(12.96ms)[0m
  [[32mPASS[0m] fk-parentcomp-deferred-fix-inside-tx-ok  [2m(7.40ms)[0m
  [[32mPASS[0m] fk-parentcomp-donothing-conflict-stays-quiet [2m(7.75ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert-integer-pk-large-negative-float-ok [2m(6.07ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-parentcomp-donothing-noconflict-ok    [2m(10.96ms)[0m
  [[32mPASS[0m] fk-parentcomp-autocommit-unrelated-children-ok [2m(8.61ms)[0m
  [[32mPASS[0m] fk-delete-composite-bounds               [2m(8.09ms)[0m
  [[32mPASS[0m] fk-parentcomp-donothing-unrelated-immediate-ok [2m(9.37ms)[0m
  [[32mPASS[0m] fk-deferred-two-stmt-fix-inside-tx-ok    [2m(8.33ms)[0m
  [[32mPASS[0m] fk-deferred-update-or-replace-parent-unique-key-resolves [2m(14.20ms)[0m
  [[32mPASS[0m] fk-update-parent-unique-single-col       [2m(8.35ms)[0m
  [[32mPASS[0m] fk-update-parent-unique-multi-col-2      [2m(7.67ms)[0m
  [[32mPASS[0m] fk-no-val-on-parent                      [2m(6.07ms)[0m
  [[32mPASS[0m] fk-val-on-parent                         [2m(6.09ms)[0m
  [[32mPASS[0m] fk-rollback-clears-then-donothing-ok     [2m(11.13ms)[0m
  [[32mPASS[0m] fk-update-parent-int-pk                  [2m(7.49ms)[0m
  [[32mPASS[0m] fk-deferred-cycle-two-tables-ok          [2m(15.74ms)[0m
  [[32mPASS[0m] fk-update-parent-unique-multi-col        [2m(10.60ms)[0m
  [[32mPASS[0m] fk-update-parent-explicit-unique-multi-col [2m(9.58ms)[0m
  [[32mPASS[0m] fk-update-parent-explicit-unique-single-col [2m(11.52ms)[0m
  [[32mPASS[0m] fk-update-parent-multi-col-pk-2          [2m(8.83ms)[0m
  [[32mPASS[0m] fk-update-parent-text-pk                 [2m(10.04ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert-integer-pk-near-min-float-ok      [2m(5.43ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-cascade-delete-no-children            [2m(7.84ms)[0m

[1mtests/boolean.sqltest[0m
  [[32mPASS[0m] boolean-not-int-1                        [2m(2.45ms)[0m
  [[32mPASS[0m] boolean-not-int-3                        [2m(2.05ms)[0m
  [[32mPASS[0m] boolean-not-int-2                        [2m(3.05ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert-integer-pk-computed-overflow-ok   [2m(7.54ms)[0m

[1mtests/boolean.sqltest[0m
  [[32mPASS[0m] boolean-not-float-1                      [2m(2.53ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-cascade-delete-recursive-check-c      [2m(8.45ms)[0m
  [[32mPASS[0m] fk-cascade-delete-composite              [2m(8.83ms)[0m

[1mtests/boolean.sqltest[0m
  [[32mPASS[0m] boolean-not-float-2                      [2m(2.77ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-update-parent-explicit-unique-multi-col-2 [2m(13.70ms)[0m
  [[32mPASS[0m] fk-cascade-delete-self-referential-chain [2m(8.40ms)[0m

[1mtests/boolean.sqltest[0m
  [[32mPASS[0m] boolean-not-text-int-1                   [2m(1.80ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-cascade-delete-null-child             [2m(9.25ms)[0m

[1mtests/boolean.sqltest[0m
  [[32mPASS[0m] boolean-not-float-3                      [2m(2.72ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-update-parent-multi-col-pk            [2m(13.62ms)[0m

[1mtests/boolean.sqltest[0m
  [[32mPASS[0m] boolean-not-text                         [2m(2.69ms)[0m

[1mtests/before-update-trigger-panic-on-update.sqltest[0m
  [[32mPASS[0m] before-update-trigger-panic-on-update    [2m(6.17ms)[0m

[1mtests/boolean.sqltest[0m
  [[32mPASS[0m] boolean-not-text-float-edge              [2m(2.15ms)[0m
  [[32mPASS[0m] boolean-not-blob-2                       [2m(1.72ms)[0m
  [[32mPASS[0m] boolean-not-text-float-2                 [2m(2.42ms)[0m
  [[32mPASS[0m] boolean-not-text-float-1                 [2m(2.75ms)[0m
  [[32mPASS[0m] boolean-not-null                         [2m(2.78ms)[0m
  [[32mPASS[0m] boolean-not-empty-blob                   [2m(2.73ms)[0m
  [[32mPASS[0m] boolean-not-blob                         [2m(2.76ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-cascade-delete-basic                  [2m(14.70ms)[0m

[1mtests/boolean.sqltest[0m
  [[32mPASS[0m] boolean-and-blob-blob                    [2m(2.73ms)[0m
  [[32mPASS[0m] boolean-and-1-blob                       [2m(2.67ms)[0m
  [[32mPASS[0m] boolean-not-text-int-2                   [2m(4.24ms)[0m
  [[32mPASS[0m] boolean-and-0-blob                       [2m(2.76ms)[0m
  [[32mPASS[0m] boolean-and-0-1                          [2m(2.66ms)[0m

[1mtests/before-update-trigger-correlated-subquery.sqltest[0m
  [[32mPASS[0m] before-update-trigger-correlated-subquery-single-row [2m(9.93ms)[0m
  [[32mPASS[0m] before-update-trigger-uncorrelated-subquery [2m(10.08ms)[0m

[1mtests/boolean.sqltest[0m
  [[32mPASS[0m] boolean-and-int-float                    [2m(2.21ms)[0m
  [[32mPASS[0m] boolean-and-1-1                          [2m(2.70ms)[0m
  [[32mPASS[0m] boolean-and-int-int                      [2m(2.67ms)[0m
  [[32mPASS[0m] boolean-and-int-0_0                      [2m(2.42ms)[0m
  [[32mPASS[0m] boolean-and-0_0-0_0                      [2m(2.74ms)[0m

[1mtests/before-update-trigger-correlated-subquery.sqltest[0m
  [[32mPASS[0m] before-and-after-update-trigger-correlated-subquery [2m(10.40ms)[0m

[1mtests/boolean.sqltest[0m
  [[32mPASS[0m] boolean-and-text                         [2m(2.74ms)[0m

[1mtests/autoincr.sqltest[0m
  [[32mPASS[0m] autoinc-explicit-rowid-below-blob-prefix-preserves-row [2m(13.20ms)[0m

[1mtests/boolean.sqltest[0m
  [[32mPASS[0m] boolean-and-text-int-1                   [2m(2.74ms)[0m
  [[32mPASS[0m] boolean-not-cast-blob                    [2m(5.79ms)[0m
  [[32mPASS[0m] boolean-and-text-float-1                 [2m(2.71ms)[0m
  [[32mPASS[0m] boolean-and-text-float-2                 [2m(2.70ms)[0m
  [[32mPASS[0m] boolean-and-text-float-edge              [2m(2.41ms)[0m
  [[32mPASS[0m] boolean-and-text-float-3                 [2m(2.73ms)[0m
  [[32mPASS[0m] boolean-and-text-int-2                   [2m(3.69ms)[0m

[1mtests/before-update-trigger-correlated-subquery.sqltest[0m
  [[32mPASS[0m] before-update-trigger-correlated-subquery-basic [2m(14.40ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert-select-nonexistent-column         [2m(4.84ms)[0m

[1mtests/boolean.sqltest[0m
  [[32mPASS[0m] boolean-and-null-null                    [2m(2.53ms)[0m
  [[32mPASS[0m] boolean-and-1_0-null                     [2m(2.71ms)[0m
  [[32mPASS[0m] boolean-and-1-null                       [2m(2.75ms)[0m
  [[32mPASS[0m] boolean-and-blob-null                    [2m(2.69ms)[0m
  [[32mPASS[0m] boolean-and-blob2-null                   [2m(2.77ms)[0m

[1mtests/foreign_keys.sqltest[0m
  [[32mPASS[0m] fk-cascade-delete-recursive              [2m(18.96ms)[0m

[1mtests/boolean.sqltest[0m
  [[32mPASS[0m] boolean-and-0_0-null                     [2m(2.47ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] serial-pk-omit-column                    [2m(5.50ms)[0m
  [[32mPASS[0m] insert-ipk-index-no-autoincrement        [2m(6.79ms)[0m

[1mtests/boolean.sqltest[0m
  [[32mPASS[0m] boolean-and-str0_0-null                  [2m(2.75ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] serial-pk-explicit-null                  [2m(5.60ms)[0m
  [[32mPASS[0m] integer-pk-omit-column                   [2m(4.57ms)[0m
  [[32mPASS[0m] text-pk-omit-column                      [2m(5.45ms)[0m
  [[32mPASS[0m] insert-ipk-index-explicit-value          [2m(6.79ms)[0m
  [[32mPASS[0m] insert-autoincrement-ipk-index-sync      [2m(8.04ms)[0m
  [[32mPASS[0m] insert-select-correlated-subquery-not-in [2m(9.38ms)[0m

[1mtests/boolean.sqltest[0m
  [[32mPASS[0m] where-string-numeric-prefix-1            [2m(2.75ms)[0m

[1mtests/before-update-trigger-correlated-subquery.sqltest[0m
  [[32mPASS[0m] before-update-trigger-subquery-in-where-and-set [2m(18.59ms)[0m

[1mtests/boolean.sqltest[0m
  [[32mPASS[0m] where-string-numeric-prefix-2            [2m(2.37ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert-select-multiple-correlated-subqueries [2m(9.95ms)[0m

[1mtests/boolean.sqltest[0m
  [[32mPASS[0m] boolean-and-0-null                       [2m(5.84ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert-select-two-correlated-subqueries  [2m(10.20ms)[0m

[1mtests/boolean.sqltest[0m
  [[32mPASS[0m] where-string-numeric-prefix-3            [2m(2.81ms)[0m
  [[32mPASS[0m] where-string-non-numeric                 [2m(2.85ms)[0m
  [[32mPASS[0m] where-string-empty                       [2m(2.73ms)[0m

[1mtests/before-update-trigger-correlated-subquery.sqltest[0m
  [[32mPASS[0m] before-update-trigger-correlated-subquery-all-rows [2m(21.01ms)[0m

[1mtests/boolean.sqltest[0m
  [[32mPASS[0m] where-string-numeric                     [2m(2.77ms)[0m
  [[32mPASS[0m] where-string-zero                        [2m(2.58ms)[0m
  [[32mPASS[0m] where-string-float-prefix                [2m(2.63ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert-1-3b                              [2m(8.26ms)[0m
  [[32mPASS[0m] insert-column-count-mismatch-select      [2m(8.25ms)[0m
  [[32mPASS[0m] insert-1-3                               [2m(11.05ms)[0m

[1mtests/bracket-quoting.sqltest[0m
  [[32mPASS[0m] bracket-cast-type-name                   [2m(2.91ms)[0m

[1mtests/insert_autorowid_index.sqltest[0m
  [[32mPASS[0m] auto-rowid-index-values-correct          [2m(7.13ms)[0m

[1mtests/insert_not_null_default_index.sqltest[0m
  [[32mPASS[0m] insert-or-replace-null-expr-not-null-default [2m(6.89ms)[0m

[1mtests/insert_autorowid_index.sqltest[0m
  [[32mPASS[0m] auto-rowid-upsert-index                  [2m(7.87ms)[0m
  [[32mPASS[0m] auto-rowid-plain-insert-index            [2m(7.97ms)[0m

[1mtests/insert_not_null_default_index.sqltest[0m
  [[32mPASS[0m] insert-or-replace-null-expr-select-back  [2m(6.77ms)[0m

[1mtests/bracket-quoting.sqltest[0m
  [[32mPASS[0m] bracket-qualified-column-reference       [2m(5.47ms)[0m
  [[32mPASS[0m] bracket-update-set-column                [2m(6.06ms)[0m
  [[32mPASS[0m] bracket-default-identifier               [2m(5.60ms)[0m
  [[32mPASS[0m] bracket-type-name                        [2m(5.52ms)[0m
  [[32mPASS[0m] bracket-collate-name                     [2m(2.86ms)[0m

[1mtests/insert_or_ignore_autoincrement.sqltest[0m
  [[32mPASS[0m] insert-or-ignore-check-failure-updates-sequence [2m(7.21ms)[0m

[1mtests/insert_not_null_default_index.sqltest[0m
  [[32mPASS[0m] insert-not-null-default-middle-column-index [2m(8.02ms)[0m

[1mtests/insert_or_ignore_autoincrement.sqltest[0m
  [[32mPASS[0m] insert-or-ignore-check-failure-does-not-insert-row [2m(6.84ms)[0m

[1mtests/insert_not_null_default_index.sqltest[0m
  [[32mPASS[0m] insert-not-null-default-middle-column-select [2m(8.50ms)[0m

[1mtests/insert_or_ignore_autoincrement.sqltest[0m
  [[32mPASS[0m] insert-or-ignore-check-pass-updates-sequence [2m(6.75ms)[0m

[1mtests/bracket-quoting.sqltest[0m
  [[32mPASS[0m] bracket-pragma-argument                  [2m(4.35ms)[0m

[1mtests/insert_not_null_default_index.sqltest[0m
  [[32mPASS[0m] insert-or-replace-null-expr-not-null-default-text-select [2m(9.87ms)[0m

[1mtests/int64-overflow-seek.sqltest[0m
  [[32mPASS[0m] int64-max-overflow-ge                    [2m(5.22ms)[0m

[1mtests/bracket-quoting.sqltest[0m
  [[32mPASS[0m] bracket-transaction-name                 [2m(4.70ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert-column-count-mismatch-values      [2m(16.13ms)[0m

[1mtests/int64-overflow-seek.sqltest[0m
  [[32mPASS[0m] int64-max-overflow-ge-expr               [2m(5.60ms)[0m

[1mtests/insert_not_null_default_index.sqltest[0m
  [[32mPASS[0m] insert-or-replace-null-expr-not-null-default-text [2m(12.77ms)[0m

[1mtests/bracket-quoting.sqltest[0m
  [[32mPASS[0m] bracket-trigger-body-update              [2m(6.80ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert-or-replace-unique-index-no-corruption [2m(34.24ms)[0m

[1mtests/changes.sqltest[0m
  [[32mPASS[0m] changes-shows-most-recent                [2m(4.82ms)[0m
  [[32mPASS[0m] changes-on-delete-no-match               [2m(5.15ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert-two-partial-unique-indexes-expr-key-no-hang [2m(19.95ms)[0m

[1mtests/changes.sqltest[0m
  [[32mPASS[0m] changes-on-delete-all                    [2m(4.82ms)[0m

[1mtests/bracket-quoting.sqltest[0m
  [[32mPASS[0m] bracket-analyze-table-name               [2m(10.70ms)[0m
  [[32mPASS[0m] bracket-upsert-do-update-set             [2m(11.58ms)[0m

[1mtests/changes.sqltest[0m
  [[32mPASS[0m] changes-resets-after-select              [2m(6.62ms)[0m

[1mtests/int64-overflow-seek.sqltest[0m
  [[32mPASS[0m] int64-max-gt                             [2m(5.40ms)[0m

[1mtests/bracket-quoting.sqltest[0m
  [[32mPASS[0m] bracket-alter-rename-column              [2m(13.44ms)[0m

[1mtests/changes.sqltest[0m
  [[32mPASS[0m] changes-doesnt-track-indexes             [2m(10.02ms)[0m
  [[32mPASS[0m] changes-on-delete                        [2m(9.72ms)[0m

[1mtests/insert_or_ignore_autoincrement.sqltest[0m
  [[32mPASS[0m] insert-or-ignore-check-pass-inserts-row  [2m(14.52ms)[0m

[1mtests/changes.sqltest[0m
  [[32mPASS[0m] changes-on-update-rowid                  [2m(9.33ms)[0m
  [[32mPASS[0m] changes-delete-doesnt-track-indexes      [2m(10.47ms)[0m
  [[32mPASS[0m] changes-1_69                             [2m(10.63ms)[0m
  [[32mPASS[0m] changes-insert-in-transaction-after-commit [2m(4.39ms)[0m
  [[32mPASS[0m] changes-on-basic-insert                  [2m(13.96ms)[0m
  [[32mPASS[0m] changes-multiple-stmts-in-transaction    [2m(4.82ms)[0m
  [[32mPASS[0m] changes-on-update                        [2m(12.60ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert-or-replace-data-correctness       [2m(33.47ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-agg-json-array                    [2m(1.71ms)[0m

[1mtests/insert.sqltest[0m
  [[32mPASS[0m] insert-or-replace-unique-index-no-corruption-2 [2m(34.11ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] json-group-object-empty                  [2m(674.55µs)[0m
  [[32mPASS[0m] jsonb-group-array-empty                  [2m(660.31µs)[0m

[1mtests/changes.sqltest[0m
  [[32mPASS[0m] changes-on-update-no-match               [2m(12.83ms)[0m
  [[32mPASS[0m] changes-with-trigger-body-statements     [2m(6.75ms)[0m
  [[32mPASS[0m] changes-on-multiple-row-insert           [2m(16.65ms)[0m
  [[32mPASS[0m] changes-insert-in-transaction-with-prior-data [2m(8.26ms)[0m
  [[32mPASS[0m] changes-with-returning                   [2m(8.08ms)[0m
  [[32mPASS[0m] changes-update-in-transaction            [2m(8.91ms)[0m
  [[32mPASS[0m] changes-mixed-operations                 [2m(13.00ms)[0m
  [[32mPASS[0m] changes-restores-previous-value-between-trigger-invocations [2m(8.35ms)[0m
  [[32mPASS[0m] changes-insert-in-transaction            [2m(14.18ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] json-group-array-empty                   [2m(5.29ms)[0m

[1mtests/changes.sqltest[0m
  [[32mPASS[0m] changes-trigger-body-constraint-ignore-keeps-outer-count [2m(9.16ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-agg-json-array-object             [2m(5.69ms)[0m

[1mtests/changes.sqltest[0m
  [[32mPASS[0m] changes-delete-trigger-body-reset-count  [2m(8.46ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-derived-right-constant        [2m(5.21ms)[0m

[1mtests/changes.sqltest[0m
  [[32mPASS[0m] changes-update-trigger-body-reset-count  [2m(9.40ms)[0m

[1mtests/int64-overflow-seek.sqltest[0m
  [[32mPASS[0m] int64-max-ge                             [2m(6.08ms)[0m

[1mtests/cte-real-affinity-join.sqltest[0m
  [[32mPASS[0m] cte-real-affinity-left-join              [2m(2.71ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-comma-equivalence             [2m(5.96ms)[0m

[1mtests/changes.sqltest[0m
  [[32mPASS[0m] changes-delete-in-transaction            [2m(13.94ms)[0m
  [[32mPASS[0m] changes-sub-trigger-isolation            [2m(11.12ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-order-by-expression           [2m(6.10ms)[0m
  [[32mPASS[0m] cross-join-duplicate-rows-preserved      [2m(6.33ms)[0m

[1mtests/changes.sqltest[0m
  [[32mPASS[0m] changes-insert-select-from-same-table-no-double-count [2m(11.91ms)[0m

[1mtests/cte-real-affinity-join.sqltest[0m
  [[32mPASS[0m] cte-real-affinity-zero                   [2m(4.37ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-select-constants              [2m(6.70ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-basic                                [2m(2.81ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-null-safe-aggregates          [2m(7.75ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-as-not-materialized                  [2m(2.57ms)[0m

[1mtests/cte-real-affinity-join.sqltest[0m
  [[32mPASS[0m] cte-real-affinity-negative               [2m(5.62ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-multiple                             [2m(3.03ms)[0m

[1mtests/cte-union-all-aggregate-literals.sqltest[0m
  [[32mPASS[0m] union-all-aggregates-no-cte              [2m(4.13ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-as-materialized                      [2m(3.41ms)[0m
  [[32mPASS[0m] cte-mixed-materialization                [2m(3.52ms)[0m

[1mtests/cte-union-all-aggregate-literals.sqltest[0m
  [[32mPASS[0m] cte-union-all-string-literals            [2m(6.70ms)[0m
  [[32mPASS[0m] cte-union-all-mixed-agg-functions        [2m(6.23ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-chain                                [2m(3.36ms)[0m
  [[32mPASS[0m] cte-union-all                            [2m(2.23ms)[0m

[1mtests/cte-real-affinity-join.sqltest[0m
  [[32mPASS[0m] cte-real-affinity-multiple-columns       [2m(8.39ms)[0m
  [[32mPASS[0m] cte-real-affinity-inner-join             [2m(10.51ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-chain-long                           [2m(5.13ms)[0m
  [[32mPASS[0m] cte-union-limit                          [2m(2.60ms)[0m
  [[32mPASS[0m] cte-union-aggregate                      [2m(3.12ms)[0m

[1mtests/int64-overflow-seek.sqltest[0m
  [[32mPASS[0m] int64-min-le                             [2m(3.94ms)[0m

[1mtests/cte-union-all-aggregate-literals.sqltest[0m
  [[32mPASS[0m] cte-union-all-empty-table                [2m(10.36ms)[0m
  [[32mPASS[0m] cte-union-all-numeric-literals-three-branches [2m(11.40ms)[0m

[1mtests/int64-overflow-seek.sqltest[0m
  [[32mPASS[0m] int64-min-overflow-le                    [2m(6.32ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-union-multi-ref                      [2m(7.65ms)[0m

[1mtests/cross_join.sqltest[0m
  [[32mPASS[0m] cross-join-where-like-inner-join         [2m(17.94ms)[0m

[1mtests/cte-real-affinity-join.sqltest[0m
  [[32mPASS[0m] subquery-real-affinity-join              [2m(14.37ms)[0m
  [[32mPASS[0m] cte-real-affinity-join-original-issue    [2m(16.61ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-union-ref                            [2m(8.81ms)[0m
  [[32mPASS[0m] cte-materialized-random-same-value       [2m(10.74ms)[0m
  [[32mPASS[0m] cte-multiple-unions                      [2m(9.18ms)[0m
  [[32mPASS[0m] cte-except                               [2m(9.47ms)[0m
  [[32mPASS[0m] cte-intersect                            [2m(9.54ms)[0m
  [[32mPASS[0m] cte-union                                [2m(10.64ms)[0m

[1mtests/int64-overflow-seek.sqltest[0m
  [[32mPASS[0m] int64-min-lt                             [2m(7.74ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-except-limit                         [2m(3.75ms)[0m

[1mtests/integrity_check/memory.sqltest[0m
  [[32mPASS[0m] integrity-check-empty                    [2m(2.80ms)[0m
  [[32mPASS[0m] quick-check-empty                        [2m(2.85ms)[0m
  [[32mPASS[0m] integrity-check-simple-table             [2m(3.91ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-union-limit-offset                   [2m(8.47ms)[0m

[1mtests/integrity_check/expression_index.sqltest[0m
  [[32mPASS[0m] integrity-check-expression-index         [2m(10.29ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-union-limit-one                      [2m(10.20ms)[0m
  [[32mPASS[0m] cte-intersect-limit                      [2m(7.92ms)[0m

[1mtests/integrity_check/expression_index.sqltest[0m
  [[32mPASS[0m] integrity-check-multiple-expression-indexes [2m(8.62ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-insert-union                         [2m(4.50ms)[0m

[1mtests/integrity_check/expression_index.sqltest[0m
  [[32mPASS[0m] quick-check-expression-index             [2m(5.93ms)[0m
  [[32mPASS[0m] integrity-check-mixed-column-and-expression-index [2m(7.65ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-insert-basic                         [2m(5.73ms)[0m
  [[32mPASS[0m] cte-insert-chain                         [2m(5.10ms)[0m
  [[32mPASS[0m] cte-delete-basic                         [2m(5.83ms)[0m

[1mtests/integrity_check/expression_index.sqltest[0m
  [[32mPASS[0m] integrity-check-expression-index-empty-table [2m(12.12ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-delete-chain                         [2m(7.92ms)[0m

[1mtests/integrity_check/memory.sqltest[0m
  [[32mPASS[0m] integrity-check-not-null-valid           [2m(8.28ms)[0m
  [[32mPASS[0m] quick-check-with-index                   [2m(8.14ms)[0m
  [[32mPASS[0m] integrity-check-with-index               [2m(10.01ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-delete-union                         [2m(10.06ms)[0m
  [[32mPASS[0m] cte-update-where                         [2m(8.38ms)[0m
  [[32mPASS[0m] cte-update-chain                         [2m(7.28ms)[0m

[1mtests/btree-large-page-overflow.sqltest[0m
  [[32mPASS[0m] large-page-index-insert                  [2m(47.52ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-json-group-object                 [2m(5.06ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-insert-returning                     [2m(9.45ms)[0m
  [[32mPASS[0m] cte-multi-ref-where                      [2m(9.68ms)[0m

[1mtests/integrity_check/memory.sqltest[0m
  [[32mPASS[0m] quick-check-skips-index                  [2m(10.04ms)[0m
  [[32mPASS[0m] integrity-check-multi-index              [2m(10.97ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-delete-multiple-ctes                 [2m(11.05ms)[0m

[1mtests/integrity_check/memory.sqltest[0m
  [[32mPASS[0m] integrity-check-ipk-not-null-multi       [2m(10.94ms)[0m
  [[32mPASS[0m] integrity-check-alter-table-default-index [2m(11.17ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-update-returning                     [2m(11.24ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-scalar-func-star                  [2m(5.99ms)[0m
  [[32mPASS[0m] select-max-star-in-expression            [2m(6.05ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-update-direct-ref-keyed              [2m(9.89ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-nested-agg-func                   [2m(5.50ms)[0m

[1mtests/integrity_check/memory.sqltest[0m
  [[32mPASS[0m] integrity-check-ipk-not-null             [2m(13.47ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-scalar-func-star-in-expression    [2m(7.51ms)[0m

[1mtests/integrity_check/parity_check_constraint.sqltest[0m
  [[32mPASS[0m] integrity-check-check-constraint-violation [2m(3.46ms)[0m

[1mtests/integrity_check/memory.sqltest[0m
  [[32mPASS[0m] integrity-check-check-constraint-null-passes [2m(5.13ms)[0m
  [[32mPASS[0m] integrity-check-table-check-constraint-null-passes [2m(5.72ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-update-direct-ref-nonkeyed           [2m(6.67ms)[0m
  [[32mPASS[0m] cte-update-keyed-where                   [2m(6.63ms)[0m
  [[32mPASS[0m] cte-delete-returning                     [2m(16.08ms)[0m

[1mtests/integrity_check/memory.sqltest[0m
  [[32mPASS[0m] integrity-check-max-errors               [2m(16.74ms)[0m
  [[32mPASS[0m] integrity-check-alter-table-default-desc-index [2m(15.24ms)[0m
  [[32mPASS[0m] integrity-check-alter-table-mixed-rows   [2m(9.94ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-update-keyed-set-pk-subquery         [2m(5.94ms)[0m
  [[32mPASS[0m] cte-update-keyed-set-subquery            [2m(6.87ms)[0m

[1mtests/integrity_check/parity_corrupt_index.sqltest[0m
  [[32mPASS[0m] integrity-check-corrupt-missing-index-entry [2m(6.80ms)[0m

[1mtests/integrity_check/parity_corrupt_partial_index.sqltest[0m
  [[32mPASS[0m] quick-check-corrupt-missing-partial-index-entry [2m(3.57ms)[0m

[1mtests/integrity_check/parity_corrupt_expression_index.sqltest[0m
  [[32mPASS[0m] quick-check-corrupt-missing-expression-index-entry [2m(7.95ms)[0m

[1mtests/integrity_check/parity_corrupt_partial_index.sqltest[0m
  [[32mPASS[0m] integrity-check-corrupt-missing-partial-index-entry [2m(6.01ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-update-keyed-set-and-where-subquery  [2m(7.92ms)[0m

[1mtests/integrity_check/parity_corrupt_expression_index.sqltest[0m
  [[32mPASS[0m] integrity-check-corrupt-missing-expression-index-entry [2m(9.26ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-update-keyed-multiple-ctes-set-where [2m(6.81ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-max-star                          [2m(17.19ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-update-keyed-returning               [2m(9.79ms)[0m
  [[32mPASS[0m] cte-update-keyed-multirow-where          [2m(13.34ms)[0m
  [[32mPASS[0m] cte-update-keyed-chained-where           [2m(14.46ms)[0m
  [[32mPASS[0m] cte-update-keyed-with-index              [2m(8.24ms)[0m
  [[32mPASS[0m] cte-update-keyed-self-ref-where          [2m(6.81ms)[0m
  [[32mPASS[0m] cte-update-keyed-pk-returning            [2m(9.12ms)[0m
  [[32mPASS[0m] cte-update-keyed-text-returning          [2m(8.22ms)[0m

[1mtests/integrity_check/parity_gencol_not_null_violation.sqltest[0m
  [[32mPASS[0m] integrity-check-gencol-not-null-violation [2m(8.41ms)[0m

[1mtests/integrity_check/parity_missing_unique_index.sqltest[0m
  [[32mPASS[0m] integrity-check-missing-unique-index-entry [2m(6.92ms)[0m

[1mtests/integrity_check/parity_corrupt_index.sqltest[0m
  [[32mPASS[0m] quick-check-corrupt-missing-index-entry  [2m(14.45ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-update-keyed-aggregate-set           [2m(7.92ms)[0m
  [[32mPASS[0m] cte-update-keyed-no-match                [2m(6.93ms)[0m
  [[32mPASS[0m] cte-update-keyed-limit-where             [2m(7.95ms)[0m

[1mtests/integrity_check/parity_freelist_trunk_corrupt.sqltest[0m
  [[32mPASS[0m] quick-check-freelist-trunk-corrupt       [2m(9.55ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-update-keyed-self-ref-set            [2m(9.73ms)[0m

[1mtests/integrity_check/parity_gencol_not_null_violation.sqltest[0m
  [[32mPASS[0m] quick-check-gencol-not-null-violation    [2m(8.78ms)[0m

[1mtests/integrity_check/parity_missing_unique_index.sqltest[0m
  [[32mPASS[0m] quick-check-missing-unique-index-entry   [2m(6.94ms)[0m

[1mtests/integrity_check/parity_not_null_violation.sqltest[0m
  [[32mPASS[0m] quick-check-not-null-violation           [2m(1.81ms)[0m
  [[32mPASS[0m] integrity-check-not-null-violation       [2m(4.75ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-update-keyed-multi-col-set           [2m(6.52ms)[0m
  [[32mPASS[0m] cte-delete-keyed-where                   [2m(6.37ms)[0m

[1mtests/integrity_check/parity_freelist_count_mismatch.sqltest[0m
  [[32mPASS[0m] integrity-check-freelist-count-mismatch  [2m(15.05ms)[0m

[1mtests/integrity_check/parity_quick_check_constraint.sqltest[0m
  [[32mPASS[0m] quick-check-check-constraint-violation   [2m(3.10ms)[0m

[1mtests/issue_5116.sqltest[0m
  [[32mPASS[0m] setup_db_for_cli_test_workaround_parser  [2m(2.67ms)[0m

[1mtests/integrity_check/parity_freelist_trunk_corrupt.sqltest[0m
  [[32mPASS[0m] integrity-check-freelist-trunk-corrupt   [2m(14.37ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-update-unique-where                  [2m(6.77ms)[0m
  [[32mPASS[0m] cte-compound-colname-union-all           [2m(3.32ms)[0m

[1mtests/integrity_check/parity_non_unique_index.sqltest[0m
  [[32mPASS[0m] integrity-check-non-unique-index-entry   [2m(7.64ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-update-keyed-empty-cte               [2m(7.90ms)[0m
  [[32mPASS[0m] cte-compound-colname-union               [2m(4.41ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] cross-join                               [2m(993.81µs)[0m
  [[32mPASS[0m] cross-join-specific-columns              [2m(851.21µs)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-compound-colname-intersect           [2m(3.80ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] cross-join-where-right-tbl               [2m(939.15µs)[0m

[1mtests/integrity_check/parity_freelist_count_mismatch.sqltest[0m
  [[32mPASS[0m] quick-check-freelist-count-mismatch      [2m(17.08ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-update-keyed-all-rows                [2m(5.70ms)[0m
  [[32mPASS[0m] cte-delete-keyed-returning               [2m(5.74ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] cross-join-where-left-tbl                [2m(1.04ms)[0m
  [[32mPASS[0m] inner-join-non-pk-unqualified            [2m(996.18µs)[0m
  [[32mPASS[0m] inner-join-self                          [2m(815.60µs)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-compound-from-union                  [2m(3.50ms)[0m
  [[32mPASS[0m] cte-compound-colname-except              [2m(5.48ms)[0m
  [[32mPASS[0m] cte-compound-colname-multi               [2m(4.18ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] inner-join-self-with-where               [2m(897.99µs)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-compound-from-aggregate              [2m(2.77ms)[0m

[1mtests/integrity_check/parity_overflow_list_length_mismatch.sqltest[0m
  [[32mPASS[0m] integrity-check-overflow-list-length-mismatch [2m(8.08ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] inner-join-constant-condition-false      [2m(535.31µs)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-compound-from-alias                  [2m(3.28ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] inner-join-constant-condition-true       [2m(1.01ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-scalar-subquery-aggregate            [2m(2.77ms)[0m
  [[32mPASS[0m] cte-scalar-subquery-no-from              [2m(3.16ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] left-join-with-where                     [2m(841.93µs)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-compound-from-intersect              [2m(5.10ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] left-join-pk                             [2m(1.28ms)[0m

[1mtests/issue_5212.sqltest[0m
  [[32mPASS[0m] multiple_aliases                         [2m(6.24ms)[0m
  [[32mPASS[0m] regression_no_explicit_columns           [2m(6.15ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] left-join-with-where-2                   [2m(953.21µs)[0m
  [[32mPASS[0m] left-join-row-id                         [2m(764.06µs)[0m
  [[32mPASS[0m] left-join-constant-condition-true        [2m(883.15µs)[0m
  [[32mPASS[0m] left-join-constant-condition-where-false [2m(757.25µs)[0m
  [[32mPASS[0m] left-join-constant-condition-false       [2m(871.87µs)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-scalar-subquery-multi-cte            [2m(3.27ms)[0m
  [[32mPASS[0m] cte-scalar-subquery-chain                [2m(3.28ms)[0m
  [[32mPASS[0m] cte-explicit-columns-basic               [2m(2.81ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] left-join-self-2                         [2m(963.30µs)[0m
  [[32mPASS[0m] left-join-multiple-cond-and              [2m(1.03ms)[0m

[1mtests/issue_5212.sqltest[0m
  [[32mPASS[0m] basic_alias                              [2m(9.92ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] left-join-multiple-cond-or               [2m(1.17ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-scalar-subquery-with-from            [2m(5.47ms)[0m
  [[32mPASS[0m] cte-compound-from-except                 [2m(7.88ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] left-join-constant-condition-true-2      [2m(899.85µs)[0m
  [[32mPASS[0m] left-join-no-join-conditions-but-multiple-where [2m(1.47ms)[0m
  [[32mPASS[0m] left-join-constant-condition-false-2     [2m(1.06ms)[0m

[1mtests/integrity_check/parity_non_unique_index.sqltest[0m
  [[32mPASS[0m] quick-check-non-unique-index-entry       [2m(15.97ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] three-way-inner-join-with-two-seeks      [2m(1.74ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-compound-from-union-all              [2m(9.83ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] left-join-constant-condition-false-inner-join-constant-condition-true [2m(1.22ms)[0m
  [[32mPASS[0m] four-way-inner-join                      [2m(2.51ms)[0m
  [[32mPASS[0m] left-join-constant-condition-true-inner-join-constant-condition-false [2m(602.47µs)[0m
  [[32mPASS[0m] join-using                               [2m(1.09ms)[0m
  [[32mPASS[0m] join-using-multiple                      [2m(1.28ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-explicit-columns-ref                 [2m(7.07ms)[0m

[1mtests/integrity_check/parity_overflow_list_length_mismatch.sqltest[0m
  [[32mPASS[0m] quick-check-overflow-list-length-mismatch [2m(15.46ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] join-using-multiple-with-quoting         [2m(1.43ms)[0m

[1mtests/cte.sqltest[0m
  [[32mPASS[0m] cte-scalar-subquery-compound             [2m(8.10ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] left-join-backwards-iteration            [2m(1.25ms)[0m
  [[32mPASS[0m] cross-join                               [2m(959.66µs)[0m
  [[32mPASS[0m] natural-join-multiple                    [2m(2.07ms)[0m
  [[32mPASS[0m] cross-join-specific-columns              [2m(904.04µs)[0m
  [[32mPASS[0m] cross-join-where-right-tbl               [2m(1.08ms)[0m
  [[32mPASS[0m] join-using-2                             [2m(3.14ms)[0m
  [[32mPASS[0m] cross-join-where-left-tbl                [2m(1.04ms)[0m
  [[32mPASS[0m] inner-join-non-pk-unqualified            [2m(955.66µs)[0m
  [[32mPASS[0m] inner-join-self                          [2m(1.01ms)[0m
  [[32mPASS[0m] inner-join-constant-condition-true       [2m(1.10ms)[0m
  [[32mPASS[0m] inner-join-constant-condition-false      [2m(381.76µs)[0m
  [[32mPASS[0m] inner-join-self-with-where               [2m(2.24ms)[0m
  [[32mPASS[0m] left-join-pk                             [2m(1.05ms)[0m
  [[32mPASS[0m] left-join-with-where-2                   [2m(918.32µs)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-json-group-object-no-sorting-required [2m(47.53ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] left-join-with-where                     [2m(3.48ms)[0m
  [[32mPASS[0m] left-join-row-id                         [2m(1.16ms)[0m
  [[32mPASS[0m] left-join-constant-condition-true        [2m(855.94µs)[0m
  [[32mPASS[0m] left-join-constant-condition-false       [2m(877.78µs)[0m
  [[32mPASS[0m] left-join-constant-condition-where-false [2m(767.50µs)[0m
  [[32mPASS[0m] join-utilizing-both-seekrowid-and-secondary-index [2m(29.59ms)[0m
  [[32mPASS[0m] left-join-self-2                         [2m(1.24ms)[0m

[1mtests/btree-backward-scan.sqltest[0m
  [[32mPASS[0m] backward-scan-deep-btree                 [2m(123.55ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] left-join-self                           [2m(46.34ms)[0m
  [[32mPASS[0m] left-join-multiple-cond-and              [2m(1.30ms)[0m
  [[32mPASS[0m] left-join-multiple-cond-or               [2m(1.50ms)[0m
  [[32mPASS[0m] left-join-no-join-conditions-but-multiple-where [2m(1.56ms)[0m
  [[32mPASS[0m] left-join-self-with-where                [2m(47.90ms)[0m
  [[32mPASS[0m] left-join-constant-condition-true-2      [2m(1.10ms)[0m
  [[32mPASS[0m] left-join-constant-condition-false-2     [2m(1.04ms)[0m
  [[32mPASS[0m] leftjoin-leftjoin-where                  [2m(49.25ms)[0m
  [[32mPASS[0m] leftjoin-innerjoin-where                 [2m(50.89ms)[0m
  [[32mPASS[0m] four-way-inner-join                      [2m(3.40ms)[0m
  [[32mPASS[0m] three-way-inner-join-with-two-seeks      [2m(2.52ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-with-group-by-and-agg-2           [2m(121.86ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] left-join-order-by-qualified             [2m(72.67ms)[0m
  [[32mPASS[0m] left-join-constant-condition-false-inner-join-constant-condition-true [2m(1.33ms)[0m
  [[32mPASS[0m] left-join-constant-condition-true-inner-join-constant-condition-false [2m(606.24µs)[0m
  [[32mPASS[0m] left-join-self                           [2m(65.51ms)[0m
  [[32mPASS[0m] join-using                               [2m(1.40ms)[0m
  [[32mPASS[0m] join-using-multiple                      [2m(1.86ms)[0m
  [[32mPASS[0m] join-using-multiple-with-quoting         [2m(1.85ms)[0m
  [[32mPASS[0m] join-using-2                             [2m(1.29ms)[0m
  [[32mPASS[0m] left-join-self-with-where                [2m(55.85ms)[0m
  [[32mPASS[0m] natural-join-multiple                    [2m(2.61ms)[0m
  [[32mPASS[0m] left-join-backwards-iteration            [2m(1.46ms)[0m
  [[32mPASS[0m] leftjoin-leftjoin-where                  [2m(48.48ms)[0m
  [[32mPASS[0m] leftjoin-innerjoin-where                 [2m(52.96ms)[0m
  [[32mPASS[0m] inner-join-pk                            [2m(119.77ms)[0m
  [[32mPASS[0m] left-join-order-by-qualified             [2m(89.92ms)[0m
  [[32mPASS[0m] left-join-with-where-right-table         [2m(145.06ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-with-group-by-and-agg-1           [2m(216.46ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] join-utilizing-both-seekrowid-and-secondary-index [2m(97.50ms)[0m
  [[32mPASS[0m] inner-join-pk                            [2m(172.79ms)[0m
  [[32mPASS[0m] inner-join-non-pk-qualified              [2m(195.03ms)[0m

[1mtests/agg-functions/default.sqltest[0m
  [[32mPASS[0m] select-with-group-by-and-agg-1           [2m(200.75ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] left-join-with-where-right-table         [2m(190.83ms)[0m
  [[32mPASS[0m] hash-join-expr-equality-pred             [2m(209.47ms)[0m
  [[32mPASS[0m] innerjoin-leftjoin-with-or-terms         [2m(218.34ms)[0m
  [[32mPASS[0m] inner-join-non-pk-qualified              [2m(221.63ms)[0m
  [[32mPASS[0m] innerjoin-leftjoin-where                 [2m(255.38ms)[0m
  [[32mPASS[0m] left-join-order-by-qualified-nullable-sorting-col [2m(267.80ms)[0m
  [[32mPASS[0m] hash-join-expr-equality-pred             [2m(192.41ms)[0m

[1mtests/join/hash.sqltest[0m
  [[32mPASS[0m] grace-mixed-int-real-join                [2m(127.95ms)[0m
  [[32mPASS[0m] grace-mixed-int-real-spot-check          [2m(149.94ms)[0m
  [[32mPASS[0m] grace-inner-join-basic                   [2m(270.12ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] left-join-order-by-qualified-nullable-sorting-col [2m(334.06ms)[0m

[1mtests/join/hash.sqltest[0m
  [[32mPASS[0m] grace-inner-many-to-many                 [2m(287.82ms)[0m
  [[32mPASS[0m] grace-full-outer-no-overlap              [2m(160.83ms)[0m
  [[32mPASS[0m] grace-left-outer-all-unmatched-nulls     [2m(210.18ms)[0m
  [[32mPASS[0m] grace-null-keys-inner                    [2m(288.01ms)[0m
  [[32mPASS[0m] grace-left-outer-all-unmatched           [2m(229.84ms)[0m
  [[32mPASS[0m] grace-null-keys-left-join                [2m(299.08ms)[0m
  [[32mPASS[0m] grace-null-keys-left-join-count-matched  [2m(294.09ms)[0m
  [[32mPASS[0m] grace-left-outer-with-duplicates         [2m(245.88ms)[0m
  [[32mPASS[0m] grace-left-outer-partial-match           [2m(263.22ms)[0m
  [[32mPASS[0m] grace-multi-column-key                   [2m(293.78ms)[0m
  [[32mPASS[0m] grace-multi-column-key-partial-overlap   [2m(298.26ms)[0m
  [[32mPASS[0m] grace-full-outer-complete-overlap        [2m(234.29ms)[0m
  [[32mPASS[0m] grace-full-outer-null-counting           [2m(207.10ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] left-join-non-pk                         [2m(517.32ms)[0m

[1mtests/join/hash.sqltest[0m
  [[32mPASS[0m] grace-agg-sum                            [2m(230.19ms)[0m
  [[32mPASS[0m] grace-full-outer-partial-overlap         [2m(282.77ms)[0m
  [[32mPASS[0m] grace-nocase-text-keys                   [2m(154.53ms)[0m
  [[32mPASS[0m] grace-order-by                           [2m(208.93ms)[0m
  [[32mPASS[0m] grace-empty-probe                        [2m(103.39ms)[0m
  [[32mPASS[0m] grace-agg-count-distinct                 [2m(235.42ms)[0m
  [[32mPASS[0m] grace-left-join-empty-right-large        [2m(114.89ms)[0m
  [[32mPASS[0m] grace-asymmetric-large-build-small-probe [2m(144.40ms)[0m
  [[32mPASS[0m] grace-full-outer-empty-right             [2m(112.85ms)[0m
  [[32mPASS[0m] grace-order-by-desc                      [2m(211.47ms)[0m
  [[32mPASS[0m] grace-asymmetric-small-build-large-probe [2m(150.81ms)[0m
  [[32mPASS[0m] grace-text-keys                          [2m(205.66ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] left-join-non-pk                         [2m(629.79ms)[0m

[1mtests/join/hash.sqltest[0m
  [[32mPASS[0m] grace-empty-build                        [2m(161.65ms)[0m
  [[32mPASS[0m] grace-full-outer-empty-left              [2m(105.17ms)[0m
  [[32mPASS[0m] grace-inner-many-to-many-agg             [2m(555.45ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] innerjoin-leftjoin-with-or-terms         [2m(616.84ms)[0m

[1mtests/join/hash.sqltest[0m
  [[32mPASS[0m] grace-large-payload                      [2m(71.31ms)[0m

[1mtests/join/memory.sqltest[0m
  [[32mPASS[0m] min-null-regression-test                 [2m(6.99ms)[0m
  [[32mPASS[0m] left-join-seek-key-regression-test       [2m(5.82ms)[0m

[1mtests/join/hash.sqltest[0m
  [[32mPASS[0m] grace-text-keys-spot-check               [2m(276.83ms)[0m
  [[32mPASS[0m] grace-agg-group-by                       [2m(350.03ms)[0m

[1mtests/join/memory.sqltest[0m
  [[32mPASS[0m] next-crash                               [2m(8.05ms)[0m
  [[32mPASS[0m] left-join-column-crash                   [2m(8.15ms)[0m
  [[32mPASS[0m] left-join-subquery-on                    [2m(7.13ms)[0m
  [[32mPASS[0m] using-deduplicates-columns               [2m(6.69ms)[0m
  [[32mPASS[0m] left-join-using-star-vs-explicit         [2m(7.75ms)[0m
  [[32mPASS[0m] left-join-using-null                     [2m(6.43ms)[0m
  [[32mPASS[0m] redundant-join-condition                 [2m(5.16ms)[0m
  [[32mPASS[0m] left-join-where-clause-regression        [2m(15.64ms)[0m
  [[32mPASS[0m] left-join-duplicate-column-constraints   [2m(9.16ms)[0m
  [[32mPASS[0m] left-join-subquery-order-by-duplicate-prefix [2m(9.99ms)[0m
  [[32mPASS[0m] inner-join-using-null                    [2m(6.86ms)[0m
  [[32mPASS[0m] left-join-null-probe-key                 [2m(6.98ms)[0m
  [[32mPASS[0m] left-join-indexed-equality-no-match-nulls-right-side [2m(8.26ms)[0m

[1mtests/join/hash.sqltest[0m
  [[32mPASS[0m] grace-limit-small                        [2m(350.59ms)[0m
  [[32mPASS[0m] grace-post-join-filter                   [2m(303.97ms)[0m

[1mtests/join/memory.sqltest[0m
  [[32mPASS[0m] three-way-left-chain-null                [2m(9.03ms)[0m
  [[32mPASS[0m] four-way-left-joins-null-keys            [2m(11.03ms)[0m
  [[32mPASS[0m] left-join-integer-real                   [2m(7.23ms)[0m
  [[32mPASS[0m] left-join-real-integer                   [2m(7.09ms)[0m
  [[32mPASS[0m] three-way-inner-using-null               [2m(22.38ms)[0m
  [[32mPASS[0m] inner-join-integer-real                  [2m(7.25ms)[0m
  [[32mPASS[0m] left-join-integer-real-zero              [2m(6.89ms)[0m
  [[32mPASS[0m] left-join-integer-real-no-match          [2m(7.02ms)[0m
  [[32mPASS[0m] left-join-integer-real-negative          [2m(12.59ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_chained                       [2m(500.21µs)[0m

[1mtests/join/memory.sqltest[0m
  [[32mPASS[0m] left-join-integer-real-multi-key         [2m(7.13ms)[0m

[1mtests/joins/derived_subquery_on_predicate.sqltest[0m
  [[32mPASS[0m] issue-7366-right-join-coalesce           [2m(8.57ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_insert_new_key_simple               [2m(466.93µs)[0m
  [[32mPASS[0m] json_insert_existing_key_not_replaced    [2m(3.46ms)[0m
  [[32mPASS[0m] json_extract_multiple_null_paths         [2m(439.72µs)[0m
  [[32mPASS[0m] json_insert_deeply_nested_new_key        [2m(457.54µs)[0m
  [[32mPASS[0m] json_extract_array                       [2m(448.13µs)[0m
  [[32mPASS[0m] json_insert_array_append                 [2m(441.27µs)[0m
  [[32mPASS[0m] json_arrow_array                         [2m(427.58µs)[0m
  [[32mPASS[0m] json_insert_array_existing_not_replaced  [2m(437.48µs)[0m

[1mtests/join/hash.sqltest[0m
  [[32mPASS[0m] grace-blob-keys                          [2m(190.44ms)[0m

[1mtests/join/memory.sqltest[0m
  [[32mPASS[0m] affinity-hash-join                       [2m(6.94ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_insert_multiple_paths               [2m(572.77µs)[0m
  [[32mPASS[0m] json_quote_string_literal                [2m(445.55µs)[0m
  [[32mPASS[0m] json-tree-fullkey-remains-absolute-under-subpath [2m(804.80µs)[0m
  [[32mPASS[0m] json_arrow_shift_array                   [2m(3.45ms)[0m
  [[32mPASS[0m] json-tree-count-includes-containers-and-leaves [2m(627.46µs)[0m
  [[32mPASS[0m] json-tree-path-points-to-container       [2m(838.90µs)[0m
  [[32mPASS[0m] json_extract_quote                       [2m(462.91µs)[0m
  [[32mPASS[0m] json_quote_float                         [2m(467.84µs)[0m
  [[32mPASS[0m] json-tree-escapes-in-fullkey             [2m(674.16µs)[0m
  [[32mPASS[0m] json-tree-deeply-nested-mixed-types      [2m(825.55µs)[0m
  [[32mPASS[0m] json_extract_overflow_int32_1            [2m(438.49µs)[0m
  [[32mPASS[0m] json_quote_integer                       [2m(438.51µs)[0m

[1mtests/join/hash.sqltest[0m
  [[32mPASS[0m] grace-left-outer-unmatched-build-rows    [2m(164.41ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-type-spectrum                  [2m(908.38µs)[0m
  [[32mPASS[0m] json_quote_null                          [2m(446.82µs)[0m
  [[32mPASS[0m] json_extract_true_type                   [2m(476.72µs)[0m
  [[32mPASS[0m] json-tree-key-null-at-root               [2m(865.59µs)[0m
  [[32mPASS[0m] json_quote_null_caps                     [2m(442.13µs)[0m

[1mtests/joins/derived_subquery_on_predicate.sqltest[0m
  [[32mPASS[0m] issue-7367-boolean-test                  [2m(12.54ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_true                          [2m(430.37µs)[0m
  [[32mPASS[0m] json-tree-key-integer-for-array-elements [2m(1.02ms)[0m
  [[32mPASS[0m] json_arrow_true_type                     [2m(478.93µs)[0m
  [[32mPASS[0m] json_quote_json_value                    [2m(473.20µs)[0m
  [[32mPASS[0m] json-tree-ordering-by-fullkey-stable-hierarchy [2m(3.75ms)[0m
  [[32mPASS[0m] json_arrow_shift_true                    [2m(429.88µs)[0m
  [[32mPASS[0m] json_extract_overflow_int32_2            [2m(447.69µs)[0m
  [[32mPASS[0m] json-tree-key-text-for-object-entries    [2m(810.79µs)[0m
  [[32mPASS[0m] json_basics                              [2m(476.43µs)[0m
  [[32mPASS[0m] json_arrow_shift_true_type               [2m(450.70µs)[0m
  [[32mPASS[0m] json_extract_false                       [2m(443.71µs)[0m
  [[32mPASS[0m] json_extract_overflow_int32_3            [2m(442.99µs)[0m
  [[32mPASS[0m] json-tree-id-uniqueness                  [2m(1.16ms)[0m
  [[32mPASS[0m] json_complex_nested                      [2m(484.70µs)[0m
  [[32mPASS[0m] json_extract_false_type                  [2m(455.41µs)[0m
  [[32mPASS[0m] json_arrow_false                         [2m(427.15µs)[0m
  [[32mPASS[0m] json_extract_overflow_int32_3-2          [2m(447.81µs)[0m
  [[32mPASS[0m] json-tree-no-arguments                   [2m(698.41µs)[0m
  [[32mPASS[0m] json_array_of_objects                    [2m(483.86µs)[0m
  [[32mPASS[0m] json_arrow_shift_false                   [2m(432.49µs)[0m

[1mtests/join/memory.sqltest[0m
  [[32mPASS[0m] correlated-subquery-hash-join-reuse      [2m(11.00ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_special_chars                       [2m(472.81µs)[0m
  [[32mPASS[0m] json_arrow_shift_false_type              [2m(444.40µs)[0m
  [[32mPASS[0m] json_unicode_emoji                       [2m(460.57µs)[0m
  [[32mPASS[0m] json_extract_string                      [2m(478.12µs)[0m
  [[32mPASS[0m] json_value_types                         [2m(464.12µs)[0m
  [[32mPASS[0m] json_arrow_false_type                    [2m(3.46ms)[0m
  [[32mPASS[0m] json_extract_string_type                 [2m(469.68µs)[0m
  [[32mPASS[0m] json_extract_overflow_int64              [2m(454.32µs)[0m
  [[32mPASS[0m] json_deeply_nested                       [2m(475.29µs)[0m
  [[32mPASS[0m] json_arrow_string                        [2m(420.09µs)[0m
  [[32mPASS[0m] json_arrow_string_type                   [2m(447.12µs)[0m
  [[32mPASS[0m] json_extract_blob                        [2m(464.33µs)[0m
  [[32mPASS[0m] json_mixed_array                         [2m(473.22µs)[0m
  [[32mPASS[0m] json_arrow_shift_string                  [2m(426.92µs)[0m
  [[32mPASS[0m] json_tree_3_arguments                    [2m(5.50ms)[0m
  [[32mPASS[0m] json_arrow_shift_string_type             [2m(456.01µs)[0m
  [[32mPASS[0m] json_extract_bracket_quoted_key_double_quote [2m(444.30µs)[0m
  [[32mPASS[0m] json_single_line_comments                [2m(475.03µs)[0m

[1mtests/join/hash.sqltest[0m
  [[32mPASS[0m] grace-full-outer-heavy-duplicates        [2m(196.55ms)[0m

[1mtests/joins/derived_subquery_on_predicate.sqltest[0m
  [[32mPASS[0m] issue-7368-right-join-correlated-subquery [2m(9.23ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_shift_implicit_root_path      [2m(445.84µs)[0m
  [[32mPASS[0m] json_arrow_implicit_root_path_undefined_key [2m(462.08µs)[0m
  [[32mPASS[0m] json-tree-nested-object                  [2m(1.29ms)[0m
  [[32mPASS[0m] json_set_add_value_to_array_out_of_bounds [2m(483.44µs)[0m

[1mtests/join/memory.sqltest[0m
  [[32mPASS[0m] full-outer-join                          [2m(6.11ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_multi_line_comments                 [2m(463.30µs)[0m
  [[32mPASS[0m] json_arrow_shift_implicit_root_path_undefined_key [2m(451.14µs)[0m
  [[32mPASS[0m] json-nan-to-null-upper                   [2m(659.80µs)[0m
  [[32mPASS[0m] json_set_replace_value_in_array          [2m(486.13µs)[0m
  [[32mPASS[0m] json_arrow_implicit_root_path_array      [2m(442.22µs)[0m
  [[32mPASS[0m] json_trailing_commas                     [2m(465.25µs)[0m
  [[32mPASS[0m] json-nan-to-null-lower                   [2m(550.61µs)[0m
  [[32mPASS[0m] json_arrow_shift_implicit_root_path_array [2m(537.73µs)[0m
  [[32mPASS[0m] json_set_null_path                       [2m(464.89µs)[0m
  [[32mPASS[0m] json_arrow_implicit_root_path_array_negative_idx [2m(443.67µs)[0m
  [[32mPASS[0m] json-nan-to-null-mixed                   [2m(525.01µs)[0m
  [[32mPASS[0m] json_unquoted_keys                       [2m(461.34µs)[0m
  [[32mPASS[0m] json_arrow_implicit_root_path            [2m(3.45ms)[0m
  [[32mPASS[0m] json_arrow_shift_implicit_root_path_array_negative_idx [2m(444.41µs)[0m
  [[32mPASS[0m] json_set_multiple_keys                   [2m(644.34µs)[0m
  [[32mPASS[0m] json_set_add_array_in_nested_object      [2m(495.36µs)[0m
  [[32mPASS[0m] json_extract_bracket_quoted_key_single_quote [2m(448.54µs)[0m
  [[32mPASS[0m] json_arrow_implicit_real_cast            [2m(445.68µs)[0m
  [[32mPASS[0m] json_newlines                            [2m(461.32µs)[0m
  [[32mPASS[0m] json_arrow_shift_implicit_real_cast      [2m(440.26µs)[0m
  [[32mPASS[0m] json_set_add_array_in_array_in_nested_object [2m(661.59µs)[0m
  [[32mPASS[0m] json_set_add_array_in_array_in_nested_object_out_of_bounds [2m(507.28µs)[0m
  [[32mPASS[0m] json_arrow_implicit_true_cast            [2m(435.46µs)[0m
  [[32mPASS[0m] json_valid_blob_utf8_non_json_word       [2m(529.64µs)[0m
  [[32mPASS[0m] json_arrow_shift_implicit_true_cast      [2m(415.26µs)[0m
  [[32mPASS[0m] json_hex_values                          [2m(462.59µs)[0m
  [[32mPASS[0m] json_arrow_implicit_false_cast           [2m(440.35µs)[0m
  [[32mPASS[0m] json_insert_type                         [2m(502.80µs)[0m
  [[32mPASS[0m] json_insert_postive-expo                 [2m(562.93µs)[0m
  [[32mPASS[0m] json_insert_negative-expo                [2m(449.82µs)[0m
  [[32mPASS[0m] json_unicode_escape                      [2m(469.69µs)[0m
  [[32mPASS[0m] json_valid_blob_utf8_non_json_word_with_whitespace [2m(709.27µs)[0m
  [[32mPASS[0m] json_arrow_shift_implicit_false_cast     [2m(431.71µs)[0m
  [[32mPASS[0m] json_insert_notation                     [2m(493.20µs)[0m
  [[32mPASS[0m] json_insert_new_key_in_nested_object     [2m(485.45µs)[0m
  [[32mPASS[0m] json_insert_existing_key_not_replaced    [2m(564.31µs)[0m
  [[32mPASS[0m] json_insert_new_key_simple               [2m(458.73µs)[0m
  [[32mPASS[0m] json_extract_bracket_quoted_key_with_dots [2m(471.27µs)[0m
  [[32mPASS[0m] json_tabs_whitespace                     [2m(480.41µs)[0m
  [[32mPASS[0m] json_arrow_chained                       [2m(487.02µs)[0m
  [[32mPASS[0m] json_valid_blob_header_overlap_scalar_bytes [2m(580.70µs)[0m
  [[32mPASS[0m] json_insert_array_append                 [2m(469.28µs)[0m
  [[32mPASS[0m] json_insert_deeply_nested_new_key        [2m(503.28µs)[0m
  [[32mPASS[0m] json_insert_array_existing_not_replaced  [2m(458.89µs)[0m

[1mtests/join/hash.sqltest[0m
  [[32mPASS[0m] grace-full-outer-partial-with-duplicates [2m(198.17ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_mixed_escaping                      [2m(483.98µs)[0m
  [[32mPASS[0m] json_extract_bracket_quoted_key_missing  [2m(471.62µs)[0m
  [[32mPASS[0m] json_insert_multiple_paths               [2m(837.18µs)[0m
  [[32mPASS[0m] json_extract_multiple_null_paths         [2m(449.35µs)[0m
  [[32mPASS[0m] json_quote_float                         [2m(453.61µs)[0m
  [[32mPASS[0m] json_quote_string_literal                [2m(461.55µs)[0m
  [[32mPASS[0m] json_quote_integer                       [2m(445.35µs)[0m
  [[32mPASS[0m] json_quote_null                          [2m(422.83µs)[0m
  [[32mPASS[0m] json101-9-2                              [2m(600.71µs)[0m
  [[32mPASS[0m] json_quote_null_caps                     [2m(425.09µs)[0m
  [[32mPASS[0m] json_extract_bracket_quoted_key_nested   [2m(462.60µs)[0m
  [[32mPASS[0m] json_control_chars                       [2m(469.00µs)[0m
  [[32mPASS[0m] json_extract_array                       [2m(454.06µs)[0m
  [[32mPASS[0m] json-tree-ordering-by-fullkey-stable-hierarchy [2m(766.17µs)[0m
  [[32mPASS[0m] json-tree-key-null-at-root               [2m(778.05µs)[0m
  [[32mPASS[0m] json-tree-type-spectrum                  [2m(901.50µs)[0m
  [[32mPASS[0m] json-tree-key-integer-for-array-elements [2m(777.01µs)[0m
  [[32mPASS[0m] json_quote_json_value                    [2m(475.18µs)[0m
  [[32mPASS[0m] json_replace_basic_1                     [2m(476.07µs)[0m
  [[32mPASS[0m] json_arrow_array                         [2m(447.12µs)[0m
  [[32mPASS[0m] json101-9-3                              [2m(691.19µs)[0m
  [[32mPASS[0m] json-tree-no-arguments                   [2m(509.46µs)[0m
  [[32mPASS[0m] json-tree-key-text-for-object-entries    [2m(758.71µs)[0m
  [[32mPASS[0m] json-tree-id-uniqueness                  [2m(800.95µs)[0m
  [[32mPASS[0m] json_arrow_shift_array                   [2m(443.27µs)[0m
  [[32mPASS[0m] json_basics                              [2m(471.93µs)[0m
  [[32mPASS[0m] json_replace_basic_2                     [2m(461.93µs)[0m
  [[32mPASS[0m] json-nan-to-null-upper                   [2m(429.88µs)[0m
  [[32mPASS[0m] json-nan-to-null-lower                   [2m(456.65µs)[0m

[1mtests/join/hash.sqltest[0m
  [[32mPASS[0m] grace-self-join                          [2m(422.69ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-nested-object                  [2m(914.31µs)[0m
  [[32mPASS[0m] json_complex_nested                      [2m(465.30µs)[0m
  [[32mPASS[0m] json-nan-to-null-mixed                   [2m(452.30µs)[0m
  [[32mPASS[0m] json_replace_multiple_paths              [2m(484.00µs)[0m
  [[32mPASS[0m] json_valid_blob_utf8_non_json_word       [2m(456.64µs)[0m
  [[32mPASS[0m] json_extract_quote                       [2m(443.10µs)[0m
  [[32mPASS[0m] json_valid_blob_utf8_non_json_word_with_whitespace [2m(424.93µs)[0m
  [[32mPASS[0m] json_valid_blob_header_overlap_scalar_bytes [2m(451.61µs)[0m
  [[32mPASS[0m] json101-9-2                              [2m(442.75µs)[0m
  [[32mPASS[0m] json_array_of_objects                    [2m(455.76µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-fn-esc-13                           [2m(458.62µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_string                      [2m(474.73µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-fn-esc-14                           [2m(439.24µs)[0m
  [[32mPASS[0m] unicode-like-case-sensitivity-1          [2m(417.23µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_overflow_int32_1            [2m(451.81µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] unicode-like-case-sensitivity-2          [2m(445.93µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json101-9-3                              [2m(422.64µs)[0m
  [[32mPASS[0m] json_special_chars                       [2m(472.19µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] unicode-like-with-escape-1               [2m(453.02µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_number_with_string          [2m(479.17µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] unicode-like-with-escape-2               [2m(448.80µs)[0m
  [[32mPASS[0m] unicode-like-prefix-multibyte-text-1     [2m(428.91µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_overflow_int32_2            [2m(456.20µs)[0m

[1mtests/joins/derived_subquery_on_predicate.sqltest[0m
  [[32mPASS[0m] issue-7369-left-join-correlated-exists   [2m(14.03ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] unicode-like-prefix-multibyte-text-2     [2m(435.76µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json101-24-4-insert                      [2m(469.91µs)[0m
  [[32mPASS[0m] json_unicode_emoji                       [2m(475.21µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] unicode-like-prefix-multibyte-text-3     [2m(440.38µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_bracket_quoted_key_with_array_index [2m(2.95ms)[0m
  [[32mPASS[0m] json_replace_with_null                   [2m(469.42µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] unicode-like-suffix-multibyte-text-2     [2m(426.57µs)[0m
  [[32mPASS[0m] unicode-like-suffix-multibyte-text-3     [2m(416.36µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_overflow_int32_3            [2m(454.98µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] unicode-like-contains-multibyte-1        [2m(428.73µs)[0m
  [[32mPASS[0m] unicode-like-contains-multibyte-2        [2m(431.57µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json104-200                              [2m(451.04µs)[0m
  [[32mPASS[0m] json_value_types                         [2m(468.47µs)[0m
  [[32mPASS[0m] json_array_length                        [2m(451.81µs)[0m
  [[32mPASS[0m] json_replace_with_json_object            [2m(470.38µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-complexity-2                        [2m(498.88µs)[0m
  [[32mPASS[0m] like-complexity-3                        [2m(579.98µs)[0m
  [[32mPASS[0m] like-escape-int-0                        [2m(417.26µs)[0m
  [[32mPASS[0m] like-escape-trailing-char                [2m(433.97µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json104-210                              [2m(470.76µs)[0m
  [[32mPASS[0m] json_array_length_empty                  [2m(453.99µs)[0m
  [[32mPASS[0m] json_deeply_nested                       [2m(487.62µs)[0m
  [[32mPASS[0m] json_replace_array_element               [2m(469.78µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-escape-int-5                        [2m(423.88µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_tree_3_arguments                    [2m(6.94ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-escape-null                         [2m(424.02µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json104-311                              [2m(443.98µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] unicode-like-suffix-multibyte-text-1     [2m(3.45ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json101-24-4-insert                      [2m(810.58µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-perf                                [2m(415.21µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json104-310                              [2m(416.59µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-fn                                  [2m(984.35µs)[0m
  [[32mPASS[0m] where-like                               [2m(719.84µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json104-200                              [2m(518.98µs)[0m
  [[32mPASS[0m] json104-310a                             [2m(452.59µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-complexity-1                        [2m(5.04ms)[0m
  [[32mPASS[0m] where-like-underscore                    [2m(700.98µs)[0m
  [[32mPASS[0m] where-like-case-insensitive              [2m(894.07µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_length_root                   [2m(3.50ms)[0m
  [[32mPASS[0m] json_extract_overflow_int32_3-2          [2m(509.33µs)[0m
  [[32mPASS[0m] json104-313                              [2m(457.51µs)[0m
  [[32mPASS[0m] json104-210                              [2m(629.85µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-escape-int-10                       [2m(4.67ms)[0m
  [[32mPASS[0m] where-like-underscore-case-insensitive   [2m(841.29µs)[0m
  [[32mPASS[0m] where-not-like-and                       [2m(787.17µs)[0m
  [[32mPASS[0m] where-like-fn                            [2m(872.37µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_mixed_array                         [2m(505.01µs)[0m
  [[32mPASS[0m] json_array_length_not_array              [2m(441.95µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-escape-float                        [2m(5.05ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json107-1-8                              [2m(448.23µs)[0m
  [[32mPASS[0m] json_extract_overflow_int64              [2m(487.01µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] where-like-or                            [2m(1.08ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_array_negative_index        [2m(522.60µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] where-like-impossible                    [2m(629.82µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json104-311                              [2m(464.75µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-with-backslash                      [2m(450.96µs)[0m
  [[32mPASS[0m] like-with-dollar                         [2m(432.22µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_blob                        [2m(501.81µs)[0m
  [[32mPASS[0m] json_replace_array_out_of_bounds         [2m(469.66µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-with-dot                            [2m(474.82µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json104-310                              [2m(693.58µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-embedded-nul                        [2m(690.27µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_bracket_quoted_key_double_quote [2m(477.94µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-fn-esc-1                            [2m(603.65µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_entire_array                [2m(485.20µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-fn-esc-2                            [2m(488.19µs)[0m
  [[32mPASS[0m] like-fn-esc-3                            [2m(443.21µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json104-310a                             [2m(464.00µs)[0m
  [[32mPASS[0m] json_extract_bracket_quoted_key_single_quote [2m(480.16µs)[0m
  [[32mPASS[0m] json_replace_nested_object               [2m(481.32µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-fn-esc-4                            [2m(442.82µs)[0m
  [[32mPASS[0m] like-fn-esc-5                            [2m(469.54µs)[0m
  [[32mPASS[0m] like-fn-esc-6                            [2m(441.06µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_bracket_quoted_key_with_dots [2m(485.17µs)[0m
  [[32mPASS[0m] json_replace_nested_array                [2m(477.67µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-fn-esc-7                            [2m(470.46µs)[0m
  [[32mPASS[0m] like-fn-esc-8                            [2m(437.74µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_bracket_quoted_key_missing  [2m(467.04µs)[0m
  [[32mPASS[0m] json_replace_deep_nesting                [2m(477.25µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-fn-esc-9                            [2m(468.84µs)[0m
  [[32mPASS[0m] like-fn-esc-10                           [2m(443.47µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_bracket_quoted_key_nested   [2m(474.82µs)[0m
  [[32mPASS[0m] json_replace_empty_object                [2m(455.91µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-fn-esc-11                           [2m(473.99µs)[0m
  [[32mPASS[0m] like-fn-esc-12                           [2m(445.38µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_bracket_quoted_key_with_array_index [2m(469.41µs)[0m
  [[32mPASS[0m] json_replace_empty_array                 [2m(461.99µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-fn-esc-13                           [2m(463.04µs)[0m
  [[32mPASS[0m] like-fn-esc-14                           [2m(440.12µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_length                        [2m(443.78µs)[0m
  [[32mPASS[0m] json_replace_quoted_key                  [2m(501.45µs)[0m
  [[32mPASS[0m] json104-313                              [2m(4.91ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] unicode-like-case-sensitivity-1          [2m(449.89µs)[0m
  [[32mPASS[0m] unicode-like-case-sensitivity-2          [2m(457.74µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_length_empty                  [2m(432.72µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] unicode-like-with-escape-1               [2m(566.17µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_root                        [2m(491.58µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] unicode-like-with-escape-2               [2m(449.93µs)[0m
  [[32mPASS[0m] unicode-like-prefix-multibyte-text-1     [2m(452.93µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json107-1-8                              [2m(651.23µs)[0m
  [[32mPASS[0m] json_array_length_root                   [2m(462.81µs)[0m
  [[32mPASS[0m] json_replace_types_boolean               [2m(563.90µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] unicode-like-prefix-multibyte-text-2     [2m(432.52µs)[0m
  [[32mPASS[0m] unicode-like-prefix-multibyte-text-3     [2m(535.60µs)[0m
  [[32mPASS[0m] unicode-like-suffix-multibyte-text-1     [2m(438.70µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_length_not_array              [2m(442.40µs)[0m
  [[32mPASS[0m] json_extract-unescape-double-quote       [2m(590.37µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] unicode-like-suffix-multibyte-text-2     [2m(425.13µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_types_integer               [2m(537.47µs)[0m
  [[32mPASS[0m] json_array_length_via_prop               [2m(456.74µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] unicode-like-suffix-multibyte-text-3     [2m(542.72µs)[0m
  [[32mPASS[0m] unicode-like-contains-multibyte-1        [2m(475.65µs)[0m
  [[32mPASS[0m] unicode-like-contains-multibyte-2        [2m(437.57µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_types_real                  [2m(544.20µs)[0m
  [[32mPASS[0m] json_extract-unescape-backslash          [2m(717.53µs)[0m
  [[32mPASS[0m] json_array_length_via_index              [2m(455.21µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-complexity-2                        [2m(558.32µs)[0m
  [[32mPASS[0m] like-complexity-3                        [2m(610.96µs)[0m

[1mtests/joins/derived_subquery_on_predicate.sqltest[0m
  [[32mPASS[0m] issue-7370-cross-join-count              [2m(18.13ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_length_via_index_not_array    [2m(458.51µs)[0m
  [[32mPASS[0m] json_extract-unescape-forward-slash      [2m(590.86µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-escape-int-0                        [2m(495.72µs)[0m
  [[32mPASS[0m] like-escape-trailing-char                [2m(439.63µs)[0m
  [[32mPASS[0m] like-escape-int-5                        [2m(656.43µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_length_via_bad_prop           [2m(451.14µs)[0m
  [[32mPASS[0m] json_extract-unescape-newline            [2m(531.08µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-escape-null                         [2m(426.93µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_length_nested                 [2m(452.90µs)[0m

[1mtests/join/hash.sqltest[0m
  [[32mPASS[0m] grace-subquery-join                      [2m(272.55ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-perf                                [2m(480.31µs)[0m

[1mtests/join/memory.sqltest[0m
  [[32mPASS[0m] left-join-materialized-subquery-nullrow-bug [2m(39.17ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-complexity-1                        [2m(5.12ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_types_text                  [2m(535.12µs)[0m

[1mtests/limit.sqltest[0m
  [[32mPASS[0m] limit-i64-min-no-panic                   [2m(2.72ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-escape-float                        [2m(4.72ms)[0m
  [[32mPASS[0m] like-escape-int-10                       [2m(5.34ms)[0m

[1mtests/math/degrees-radians-precision.sqltest[0m
  [[32mPASS[0m] degrees-2pi                              [2m(2.68ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_length_NULL                   [2m(439.62µs)[0m
  [[32mPASS[0m] json_extract-unescape-tab                [2m(728.76µs)[0m
  [[32mPASS[0m] json_remove_basic_1                      [2m(492.99µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] add-int-1                                [2m(2.67ms)[0m

[1mtests/join/hash.sqltest[0m
  [[32mPASS[0m] grace-coalesce-select                    [2m(211.35ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] add-int-2                                [2m(2.72ms)[0m
  [[32mPASS[0m] add-float                                [2m(2.62ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_type_no_path                        [2m(449.84µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] add-int-3                                [2m(2.80ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_basic_2                      [2m(482.96µs)[0m
  [[32mPASS[0m] json_extract-unescape-carriage-return-length [2m(481.95µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] add-int-float-1                          [2m(2.84ms)[0m
  [[32mPASS[0m] ln-negative                              [2m(2.64ms)[0m
  [[32mPASS[0m] ln-null                                  [2m(2.77ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_type_root_path                      [2m(475.84µs)[0m

[1mtests/limit.sqltest[0m
  [[32mPASS[0m] limit-negative-no-limit                  [2m(4.72ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] log10-int                                [2m(2.76ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_multiple_paths               [2m(501.08µs)[0m

[1mtests/join/memory.sqltest[0m
  [[32mPASS[0m] join-text-index-integer-probe            [2m(10.70ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract-unescape-backspace-length   [2m(652.03µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] log10-float                              [2m(2.62ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_type_array                          [2m(475.00µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] log10-negative                           [2m(2.68ms)[0m
  [[32mPASS[0m] log10-null                               [2m(2.56ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_array_element                [2m(471.69µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] add-int-float-2                          [2m(5.69ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract-unescape-formfeed-length    [2m(730.44µs)[0m

[1mtests/join/hash.sqltest[0m
  [[32mPASS[0m] grace-exact-values-checksum              [2m(284.49ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] log2-int                                 [2m(2.60ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_type_integer                        [2m(469.88µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] log2-float                               [2m(2.65ms)[0m
  [[32mPASS[0m] add-agg-int-agg-int                      [2m(2.77ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_array_negative_index         [2m(483.84µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] log2-str                                 [2m(2.73ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract-unescape-unicode            [2m(458.77µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] log2-negative                            [2m(2.75ms)[0m

[1mtests/join/hash.sqltest[0m
  [[32mPASS[0m] grace-negative-keys-spot-check           [2m(264.82ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] log2-null                                [2m(2.61ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_type_real                           [2m(461.33µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] radians-int                              [2m(2.63ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_array_multiple_elements      [2m(490.09µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] radians-str                              [2m(2.70ms)[0m
  [[32mPASS[0m] radians-float                            [2m(3.00ms)[0m

[1mtests/join/memory.sqltest[0m
  [[32mPASS[0m] join-text-index-integer-probe-reversed   [2m(10.33ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract-unescape-unicode-emoji      [2m(549.04µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] multiply-text-7                          [2m(2.69ms)[0m
  [[32mPASS[0m] remainder-multiplication-overflow        [2m(2.80ms)[0m
  [[32mPASS[0m] log10-str                                [2m(12.25ms)[0m
  [[32mPASS[0m] add-agg-int-agg-float                    [2m(5.82ms)[0m
  [[32mPASS[0m] remainder-multiplication-overflow-typeof [2m(2.63ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_type_true                           [2m(471.93µs)[0m
  [[32mPASS[0m] json_remove_array_out_of_bounds          [2m(474.97µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] multiply-text-8                          [2m(2.54ms)[0m

[1mtests/limit.sqltest[0m
  [[32mPASS[0m] limit-zero                               [2m(3.08ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] remainder-multiplication-overflow-insert [2m(5.96ms)[0m
  [[32mPASS[0m] multiply-text-9                          [2m(2.54ms)[0m
  [[32mPASS[0m] is-nonnull-divide-by-zero                [2m(5.68ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract-unescape-newline-length     [2m(532.67µs)[0m

[1mtests/join/memory.sqltest[0m
  [[32mPASS[0m] join-text-index-integer-probe-no-index   [2m(7.27ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] add-agg-float-agg-int                    [2m(5.89ms)[0m
  [[32mPASS[0m] multiply-overflow-1                      [2m(2.55ms)[0m
  [[32mPASS[0m] radians-null                             [2m(2.74ms)[0m

[1mtests/joins/derived_subquery_on_predicate.sqltest[0m
  [[32mPASS[0m] issue-7372-text-range-join-after-analyze [2m(32.78ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] multiply-overflow-2                      [2m(2.55ms)[0m

[1mtests/join/hash.sqltest[0m
  [[32mPASS[0m] grace-negative-keys                      [2m(280.14ms)[0m

[1mtests/multi_index_dml.sqltest[0m
  [[32mPASS[0m] update-multi-index-or-stable-write-set   [2m(10.85ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] subtract-int                             [2m(5.71ms)[0m
  [[32mPASS[0m] multiply-overflow-3                      [2m(2.54ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] where-like-another-column-prefix-2       [2m(51.87ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_nested_object                [2m(521.75µs)[0m
  [[32mPASS[0m] json_extract-unescape-double-quote       [2m(496.77µs)[0m

[1mtests/join/memory.sqltest[0m
  [[32mPASS[0m] text-index-range-seek-integer-probes     [2m(6.87ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] multiply-overflow-4                      [2m(2.60ms)[0m

[1mtests/multi_index_dml.sqltest[0m
  [[32mPASS[0m] update-multi-index-or-updating-indexed-column [2m(11.85ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] multiply-overflow-5                      [2m(2.57ms)[0m

[1mtests/multi_index_dml.sqltest[0m
  [[32mPASS[0m] delete-multi-index-or-safe-materialization [2m(11.74ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] subtract-float                           [2m(5.71ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract-arrow-unescape-double-quote [2m(586.10µs)[0m

[1mtests/limit.sqltest[0m
  [[32mPASS[0m] limit-one                                [2m(3.09ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] multiply-overflow-6                      [2m(2.54ms)[0m

[1mtests/multi_index_dml.sqltest[0m
  [[32mPASS[0m] delete-multi-index-or-before-trigger-does-not-grow-write-set [2m(13.31ms)[0m
  [[32mPASS[0m] delete-multi-index-or-with-trigger       [2m(14.11ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] subtract-int-float                       [2m(4.14ms)[0m

[1mtests/multi_index_dml.sqltest[0m
  [[32mPASS[0m] delete-multi-index-or-limit-preserved    [2m(10.87ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] multiply-overflow-7                      [2m(2.55ms)[0m

[1mtests/multi_index_dml.sqltest[0m
  [[32mPASS[0m] delete-multi-index-or-two-row-subset-compatible [2m(9.83ms)[0m
  [[32mPASS[0m] delete-multi-index-or-limit-offset-preserved [2m(10.54ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract-unescape-backslash          [2m(497.55µs)[0m
  [[32mPASS[0m] json_remove_nested_array                 [2m(540.45µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] sinh-int                                 [2m(2.80ms)[0m
  [[32mPASS[0m] multiply-overflow-8                      [2m(2.58ms)[0m

[1mtests/matview-create-index.sqltest[0m
  [[32mPASS[0m] create-index-does-not-corrupt-matview    [2m(25.62ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_type_false                          [2m(527.78µs)[0m

[1mtests/join/memory.sqltest[0m
  [[32mPASS[0m] left-hash-join-materialized-prefix-keeps-build-side-on-const [2m(13.99ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] subtract-agg-int-agg-int                 [2m(5.83ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] text-desc-nulls-first                    [2m(4.78ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] divide-text-1                            [2m(2.86ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract-unescape-forward-slash      [2m(453.98µs)[0m

[1mtests/multi_index_dml.sqltest[0m
  [[32mPASS[0m] update-multi-index-and-stable-write-set  [2m(10.50ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] text-asc-nulls-first                     [2m(5.49ms)[0m
  [[32mPASS[0m] text-desc-nulls-last                     [2m(5.30ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_deep_nesting                 [2m(574.48µs)[0m

[1mtests/multi_index_dml.sqltest[0m
  [[32mPASS[0m] delete-multi-index-or-one-row-subset-compatible [2m(11.95ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract-arrow-unescape-newline      [2m(503.83µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] divide-text-2                            [2m(2.87ms)[0m

[1mtests/joins/left_join_null_index_bug.sqltest[0m
  [[32mPASS[0m] left-join-null-index-bug                 [2m(10.90ms)[0m

[1mtests/limit.sqltest[0m
  [[32mPASS[0m] limit-in-subquery                        [2m(3.46ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] sinh-float                               [2m(2.88ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] real-desc-nulls-first                    [2m(5.07ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] subtract-agg-int-agg-float               [2m(5.87ms)[0m
  [[32mPASS[0m] divide-text-3                            [2m(2.85ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] where-like-another-column                [2m(73.85ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract-unescape-newline            [2m(508.59µs)[0m
  [[32mPASS[0m] json_single_line_comments                [2m(530.80µs)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] real-asc-nulls-last                      [2m(8.16ms)[0m
  [[32mPASS[0m] multi-key-nulls-last-nulls-first         [2m(5.46ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_type_null                           [2m(497.55µs)[0m
  [[32mPASS[0m] json_remove_empty_object                 [2m(559.85µs)[0m

[1mtests/multi_index_dml.sqltest[0m
  [[32mPASS[0m] update-multi-index-and-before-trigger-does-not-grow-write-set [2m(13.39ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] divide-text-4                            [2m(2.87ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] multi-key-both-nulls-last                [2m(5.70ms)[0m
  [[32mPASS[0m] multi-key-nulls-first-nulls-last         [2m(6.67ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract-combined-escapes            [2m(544.27µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] subtract-agg-float-agg-int               [2m(5.33ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] asc-nulls-last-limit                     [2m(6.87ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] divide-text-5                            [2m(2.86ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] desc-nulls-first-limit                   [2m(6.61ms)[0m
  [[32mPASS[0m] asc-nulls-last-limit-offset              [2m(6.36ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract-unescape-tab                [2m(502.89µs)[0m
  [[32mPASS[0m] json_multi_line_comments                 [2m(547.45µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] sinh-str                                 [2m(2.36ms)[0m
  [[32mPASS[0m] subtract-blob                            [2m(2.68ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] all-nulls-desc-nulls-first               [2m(5.97ms)[0m
  [[32mPASS[0m] all-nulls-asc-nulls-last                 [2m(6.23ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] divide-text-7                            [2m(2.88ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_empty_array                  [2m(502.59µs)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] multi-key-both-nulls-first               [2m(10.62ms)[0m
  [[32mPASS[0m] all-nulls-asc-nulls-first                [2m(5.79ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] subtract-blob-empty                      [2m(2.64ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-set-field-null                      [2m(630.48µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] divide-text-8                            [2m(2.90ms)[0m

[1mtests/join/memory.sqltest[0m
  [[32mPASS[0m] left-hash-join-materialized-prefix-keeps-build-side-on-in-list [2m(18.66ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] no-nulls-desc-nulls-first                [2m(4.63ms)[0m
  [[32mPASS[0m] desc-nulls-first-limit-offset            [2m(11.08ms)[0m
  [[32mPASS[0m] no-nulls-asc-nulls-last                  [2m(5.05ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] subtract-blob-charcter                   [2m(2.54ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_trailing_commas                     [2m(577.87µs)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] empty-table-asc-nulls-last               [2m(5.36ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_quoted_key                   [2m(526.75µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] sinh-null                                [2m(2.82ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] all-nulls-desc-nulls-last                [2m(8.18ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] multiply-int                             [2m(2.52ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_type_text                           [2m(3.54ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] single-null-asc-nulls-last               [2m(5.11ms)[0m
  [[32mPASS[0m] empty-table-desc-nulls-first             [2m(5.66ms)[0m
  [[32mPASS[0m] single-null-desc-nulls-first             [2m(5.08ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-insert-null                         [2m(569.86µs)[0m

[1mtests/joins/left_join_null_index_bug.sqltest[0m
  [[32mPASS[0m] left-join-null-index-bug-composite-key   [2m(14.43ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] multiply-float                           [2m(2.59ms)[0m

[1mtests/multi_index_dml.sqltest[0m
  [[32mPASS[0m] delete-multi-index-and-safe-materialization [2m(10.55ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] expression-asc-nulls-last                [2m(5.38ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] divide-text-9                            [2m(2.57ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract-unescape-carriage-return-length [2m(3.61ms)[0m
  [[32mPASS[0m] json_unquoted_keys                       [2m(552.21µs)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] expression-desc-nulls-first              [2m(5.76ms)[0m
  [[32mPASS[0m] group-by-nulls-last                      [2m(5.19ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] multiply-int-float                       [2m(2.63ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_all_properties               [2m(540.94µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] divide-overflow-5                        [2m(2.48ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] subquery-nulls-last                      [2m(5.38ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] sqrt-int                                 [2m(2.79ms)[0m
  [[32mPASS[0m] multiply-float-int                       [2m(2.59ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] subquery-nulls-first-desc                [2m(6.67ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] divide-overflow-6                        [2m(2.61ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] union-all-desc-nulls-first               [2m(6.07ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-remove-null                         [2m(701.20µs)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] distinct-desc-nulls-first                [2m(6.22ms)[0m
  [[32mPASS[0m] group-by-desc-nulls-first                [2m(9.34ms)[0m
  [[32mPASS[0m] distinct-nulls-last                      [2m(6.54ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_type_NULL                           [2m(495.93µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] multiply-agg-int-agg-int                 [2m(2.74ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract-unescape-backspace-length   [2m(543.63µs)[0m
  [[32mPASS[0m] json_newlines                            [2m(1.00ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] divide-overflow-7                        [2m(2.46ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] window-row-number-nulls-last             [2m(6.49ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_all_array_elements           [2m(549.84µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] multiply-agg-int-agg-float               [2m(2.66ms)[0m
  [[32mPASS[0m] divide-overflow-8                        [2m(2.49ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] window-row-number-desc-nulls-first       [2m(6.19ms)[0m
  [[32mPASS[0m] union-all-nulls-last                     [2m(12.20ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] multiply-agg-float-agg-int               [2m(2.65ms)[0m
  [[32mPASS[0m] bitwise-and-text-1                       [2m(2.45ms)[0m

[1mtests/join/memory.sqltest[0m
  [[32mPASS[0m] left-hash-join-unmatched-where-references-outer-table [2m(9.69ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] sqrt-float                               [2m(2.79ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] indexed-asc-nulls-last                   [2m(7.41ms)[0m
  [[32mPASS[0m] indexed-desc-nulls-first                 [2m(6.43ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_type_cast                           [2m(482.92µs)[0m
  [[32mPASS[0m] json_set_array_append_at_root            [2m(576.46µs)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] indexed-asc-nulls-first                  [2m(7.06ms)[0m
  [[32mPASS[0m] indexed-desc-nulls-last                  [2m(6.96ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract-unescape-formfeed-length    [2m(548.50µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] divide-int                               [2m(2.53ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_hex_values                          [2m(558.54µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-and-text-2                       [2m(2.52ms)[0m

[1mtests/limit.sqltest[0m
  [[32mPASS[0m] limit-non-integer-datatype-mismatch      [2m(26.19ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] composite-idx-a-nulls-last-b-asc         [2m(7.31ms)[0m

[1mtests/multi_index_dml.sqltest[0m
  [[32mPASS[0m] delete-multi-index-compound-or-safe-materialization [2m(10.97ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_root                         [2m(583.62µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-and-text-3                       [2m(2.52ms)[0m
  [[32mPASS[0m] bitwise-and-text-4                       [2m(2.49ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] mixed-first-col-nulls-last-second-default [2m(5.43ms)[0m
  [[32mPASS[0m] composite-idx-a-desc-nulls-first-b-desc  [2m(7.35ms)[0m
  [[32mPASS[0m] composite-idx-a-asc-b-nulls-last         [2m(7.60ms)[0m
  [[32mPASS[0m] mixed-first-default-second-nulls-last    [2m(5.70ms)[0m
  [[32mPASS[0m] composite-idx-where-eq-nulls-last        [2m(6.84ms)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] innerjoin-leftjoin-where                 [2m(918.37ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_type_null_arg                       [2m(465.08µs)[0m
  [[32mPASS[0m] json_unicode_escape                      [2m(558.42µs)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] negative-asc-nulls-last                  [2m(5.40ms)[0m
  [[32mPASS[0m] composite-idx-nulls-last-limit           [2m(7.55ms)[0m
  [[32mPASS[0m] negative-desc-nulls-first                [2m(5.68ms)[0m
  [[32mPASS[0m] composite-idx-where-eq-nulls-first       [2m(8.60ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-and-text-5                       [2m(2.52ms)[0m

[1mtests/joins/using_clause_case_insensitive.sqltest[0m
  [[32mPASS[0m] issue-7371-natural-join-quoted-hyphen    [2m(11.85ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract-unescape-unicode            [2m(547.04µs)[0m
  [[32mPASS[0m] jsonb_remove_root                        [2m(506.28µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] sqrt-str                                 [2m(2.93ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_array_append_after_array_index  [2m(1.26ms)[0m

[1mtests/numeric-text-overflow.sqltest[0m
  [[32mPASS[0m] ceil-text-overflow-i64                   [2m(2.92ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_tabs_whitespace                     [2m(503.69µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-and-text-7                       [2m(2.54ms)[0m

[1mtests/offset/default.sqltest[0m
  [[32mPASS[0m] select-offset-0                          [2m(658.00µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_mixed_escaping                      [2m(486.76µs)[0m

[1mtests/numeric-text-overflow.sqltest[0m
  [[32mPASS[0m] trunc-text-overflow-i64                  [2m(3.03ms)[0m
  [[32mPASS[0m] add-text-overflow-i64                    [2m(3.00ms)[0m
  [[32mPASS[0m] mul-text-overflow-i64                    [2m(2.76ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract-unescape-unicode-emoji      [2m(522.19µs)[0m

[1mtests/offset/default.sqltest[0m
  [[32mPASS[0m] select-offset-1                          [2m(662.81µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_complex_1                    [2m(573.52µs)[0m

[1mtests/numeric-text-overflow.sqltest[0m
  [[32mPASS[0m] floor-text-overflow-i64                  [2m(3.84ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_control_chars                       [2m(474.94µs)[0m

[1mtests/numeric-text-overflow.sqltest[0m
  [[32mPASS[0m] ceil-text-i64-max-stays-integer          [2m(3.12ms)[0m

[1mtests/offset/default.sqltest[0m
  [[32mPASS[0m] select-offset-negative                   [2m(647.43µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract-unescape-newline-length     [2m(518.40µs)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] case-expr-nulls-last                     [2m(11.91ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-and-text-8                       [2m(2.77ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] union-desc-nulls-first                   [2m(7.28ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] sqrt-negative                            [2m(2.04ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_array_append_after_object_key   [2m(1.15ms)[0m

[1mtests/join/hash.sqltest[0m
  [[32mPASS[0m] grace-having                             [2m(356.73ms)[0m

[1mtests/offset/default.sqltest[0m
  [[32mPASS[0m] select-offset-subquery                   [2m(1.11ms)[0m
  [[32mPASS[0m] select-offset-0                          [2m(648.42µs)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] union-nulls-last                         [2m(8.87ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract-arrow-unescape-double-quote [2m(464.20µs)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] window-two-different-nulls-orderings     [2m(8.45ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_type_blob_with_trailing_bytes       [2m(467.49µs)[0m

[1mtests/join/hash.sqltest[0m
  [[32mPASS[0m] grace-three-way-join                     [2m(414.47ms)[0m

[1mtests/literal.sqltest[0m
  [[32mPASS[0m] numberic-literal-1                       [2m(2.90ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-and-text-9                       [2m(2.73ms)[0m

[1mtests/join/memory.sqltest[0m
  [[32mPASS[0m] join-3-1                                 [2m(10.85ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-and-text-10                      [2m(2.47ms)[0m

[1mtests/multi_index_dml.sqltest[0m
  [[32mPASS[0m] update-multi-index-compound-or-stable-write-set [2m(12.07ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] divide-int-no-fraction                   [2m(4.67ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] ddl-reject-conflicting-unique-on-conflict-replace-rollback [2m(6.58ms)[0m
  [[32mPASS[0m] ddl-reject-conflicting-unique-on-conflict [2m(7.23ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-and-text-11                      [2m(2.56ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] upsert-partial-doupdate-where-skip       [2m(7.05ms)[0m

[1mtests/pragma/index_info.sqltest[0m
  [[32mPASS[0m] pragma-index-info-single-column          [2m(5.56ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-ecma-script-1                      [2m(766.92µs)[0m

[1mtests/offset/default.sqltest[0m
  [[32mPASS[0m] select-offset-1                          [2m(689.13µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] upsert-partial-omitted-target-matches    [2m(7.29ms)[0m

[1mtests/pragma/index_info.sqltest[0m
  [[32mPASS[0m] pragma-index-info-equals-syntax          [2m(5.52ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract-arrow-unescape-newline      [2m(463.88µs)[0m

[1mtests/joins/using_clause_case_insensitive.sqltest[0m
  [[32mPASS[0m] issue-7371-natural-join-mixed-case       [2m(11.27ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-or-text-1                        [2m(2.68ms)[0m

[1mtests/pragma/index_info.sqltest[0m
  [[32mPASS[0m] pragma-index-info-nonexistent            [2m(2.41ms)[0m

[1mtests/literal.sqltest[0m
  [[32mPASS[0m] numberic-literal-2                       [2m(2.83ms)[0m

[1mtests/offset/default.sqltest[0m
  [[32mPASS[0m] select-offset-negative                   [2m(689.06µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-or-text-2                        [2m(2.44ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] ddl-reject-conflicting-composite-unique  [2m(6.16ms)[0m

[1mtests/pragma/index_info.sqltest[0m
  [[32mPASS[0m] pragma-index-info-autoindex              [2m(5.31ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] update-rollback-terminates-transaction   [2m(14.08ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-ecma-script-2                      [2m(583.00µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] upsert-partial-multicol-leave-predicate  [2m(12.98ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_error_position_valid                [2m(442.10µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-or-text-3                        [2m(2.75ms)[0m
  [[32mPASS[0m] sqrt-null                                [2m(2.65ms)[0m

[1mtests/pragma/index_list.sqltest[0m
  [[32mPASS[0m] pragma-index-list-equals-syntax          [2m(5.87ms)[0m

[1mtests/pragma/index_info.sqltest[0m
  [[32mPASS[0m] pragma-index-info-unique-index           [2m(11.54ms)[0m

[1mtests/join/memory.sqltest[0m
  [[32mPASS[0m] join-3-2                                 [2m(9.15ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-or-text-4                        [2m(2.67ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] ddl-allow-same-on-conflict-unique        [2m(5.38ms)[0m

[1mtests/pragma/index_info.sqltest[0m
  [[32mPASS[0m] pragma-index-info-function-syntax        [2m(9.62ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract-combined-escapes            [2m(532.83µs)[0m

[1mtests/pragma/index_list.sqltest[0m
  [[32mPASS[0m] pragma-index-list-autoindex-unique       [2m(4.97ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] upsert-partial-func-predicate            [2m(16.92ms)[0m

[1mtests/pragma/index_list.sqltest[0m
  [[32mPASS[0m] pragma-index-list-autoindex-pk           [2m(4.20ms)[0m

[1mtests/literal.sqltest[0m
  [[32mPASS[0m] numberic-literal-3                       [2m(2.91ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_error_position_valid_ws             [2m(452.51µs)[0m
  [[32mPASS[0m] json5-ecma-script-3                      [2m(783.48µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-or-text-5                        [2m(2.78ms)[0m

[1mtests/pragma/index_list.sqltest[0m
  [[32mPASS[0m] pragma-index-list-single-index           [2m(11.45ms)[0m
  [[32mPASS[0m] pragma-index-list-multiple-indexes       [2m(9.81ms)[0m
  [[32mPASS[0m] pragma-index-list-nonexistent-table      [2m(2.78ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] divide-float                             [2m(5.64ms)[0m

[1mtests/pragma/index_list.sqltest[0m
  [[32mPASS[0m] pragma-index-list-partial-index          [2m(5.55ms)[0m
  [[32mPASS[0m] pragma-index-list-mixed-origins          [2m(6.67ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] ddl-allow-one-on-conflict-unique         [2m(5.15ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-or-text-7                        [2m(2.63ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] where-like-another-column-prefix         [2m(145.62ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_length_via_prop               [2m(516.46µs)[0m

[1mtests/pragma/index_list.sqltest[0m
  [[32mPASS[0m] pragma-index-list-no-indexes             [2m(6.10ms)[0m

[1mtests/literal.sqltest[0m
  [[32mPASS[0m] numberic-literal-4                       [2m(2.78ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-set-field-null                      [2m(484.50µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] upsert-partial-rowid-predicate           [2m(6.53ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] tanh-int                                 [2m(2.91ms)[0m
  [[32mPASS[0m] bitwise-or-text-8                        [2m(2.76ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] ddl-allow-one-on-conflict-unique-reversed [2m(5.30ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-or-text-9                        [2m(2.65ms)[0m

[1mtests/pragma/index_list.sqltest[0m
  [[32mPASS[0m] pragma-index-list-function-syntax        [2m(6.05ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_length_via_index              [2m(566.67µs)[0m

[1mtests/pragma/index_xinfo.sqltest[0m
  [[32mPASS[0m] pragma-index-xinfo-single-column         [2m(5.21ms)[0m

[1mtests/pragma/index_list.sqltest[0m
  [[32mPASS[0m] pragma-index-list-prefers-temp-shadowed-table [2m(11.78ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-or-text-10                       [2m(2.74ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_error_position_object               [2m(487.93µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] upsert-partial-excluded-rewrite          [2m(6.33ms)[0m

[1mtests/join/memory.sqltest[0m
  [[32mPASS[0m] join-3-4-1                               [2m(10.11ms)[0m

[1mtests/pragma/index_list.sqltest[0m
  [[32mPASS[0m] pragma-index-info-prefers-temp-shadowed-name [2m(13.67ms)[0m
  [[32mPASS[0m] pragma-index-list-attached-database      [2m(8.74ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] ddl-allow-different-cols-different-on-conflict [2m(5.37ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] tanh-float                               [2m(2.68ms)[0m
  [[32mPASS[0m] bitwise-or-text-11                       [2m(2.70ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-with-single-trailing-comma-valid   [2m(748.27µs)[0m

[1mtests/literal.sqltest[0m
  [[32mPASS[0m] numberic-literal-5                       [2m(2.69ms)[0m

[1mtests/joins/using_clause_case_insensitive.sqltest[0m
  [[32mPASS[0m] issue-7371-using-quoted-mixed-case       [2m(16.01ms)[0m

[1mtests/pragma/index_xinfo.sqltest[0m
  [[32mPASS[0m] pragma-index-xinfo-equals-syntax         [2m(5.70ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-int-int                       [2m(2.72ms)[0m

[1mtests/pragma/index_xinfo.sqltest[0m
  [[32mPASS[0m] pragma-index-xinfo-multi-column          [2m(5.47ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_length_via_index_not_array    [2m(527.90µs)[0m
  [[32mPASS[0m] json_error_position_array_valid          [2m(503.89µs)[0m

[1mtests/join/hash.sqltest[0m
  [[32mPASS[0m] grace-distinct                           [2m(418.16ms)[0m

[1mtests/pragma/index_info.sqltest[0m
  [[32mPASS[0m] pragma-index-info-multi-column           [2m(33.11ms)[0m

[1mtests/pragma/index_xinfo.sqltest[0m
  [[32mPASS[0m] pragma-index-xinfo-desc-column           [2m(5.66ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] upsert-partial-stay-change-to-unique     [2m(7.15ms)[0m

[1mtests/pragma/index_xinfo.sqltest[0m
  [[32mPASS[0m] pragma-index-xinfo-nonexistent           [2m(2.72ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] ddl-allow-different-order-composite      [2m(5.51ms)[0m

[1mtests/pragma/index_xinfo.sqltest[0m
  [[32mPASS[0m] pragma-index-xinfo-collation             [2m(5.62ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-int-neg_int                   [2m(2.67ms)[0m

[1mtests/pragma/index_xinfo.sqltest[0m
  [[32mPASS[0m] pragma-index-xinfo-autoindex             [2m(5.52ms)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-cache-size-default                [2m(2.74ms)[0m
  [[32mPASS[0m] pragma-cache-size                        [2m(2.42ms)[0m
  [[32mPASS[0m] pragma-set-cache-size                    [2m(2.59ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-single-quoted                      [2m(614.84µs)[0m
  [[32mPASS[0m] json_array_length_via_bad_prop           [2m(505.84µs)[0m
  [[32mPASS[0m] json_error_position_array_valid_ws       [2m(427.61µs)[0m

[1mtests/literal.sqltest[0m
  [[32mPASS[0m] numberic-literal-6                       [2m(2.77ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-int-float                     [2m(2.71ms)[0m
  [[32mPASS[0m] tanh-str                                 [2m(2.78ms)[0m

[1mtests/pragma/index_xinfo.sqltest[0m
  [[32mPASS[0m] pragma-index-xinfo-function-syntax       [2m(5.75ms)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-function-cache-size               [2m(2.57ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-int-text                      [2m(2.45ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] ddl-reject-multiple-pk-column-and-table  [2m(6.38ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] upsert-partial-toggle-predicate          [2m(7.29ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] upsert-returning-mixed-operations        [2m(6.26ms)[0m

[1mtests/pragma/index_list.sqltest[0m
  [[32mPASS[0m] pragma-index-xinfo-prefers-temp-shadowed-name [2m(24.76ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-int-text_float                [2m(2.68ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-hexadecimal-1                      [2m(681.49µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-select-returning-basic            [2m(7.13ms)[0m
  [[32mPASS[0m] insert-select-returning-where            [2m(7.35ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_length_nested                 [2m(523.92µs)[0m

[1mtests/join/memory.sqltest[0m
  [[32mPASS[0m] join-3-4-2                               [2m(9.68ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-select-returning-columns          [2m(6.77ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_error_position_array                [2m(462.14µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-select-returning-expressions      [2m(6.77ms)[0m

[1mtests/multi_index_dml.sqltest[0m
  [[32mPASS[0m] update-multi-index-and-returning         [2m(10.94ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-select-returning-functions        [2m(6.35ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] divide-int-float                         [2m(2.79ms)[0m
  [[32mPASS[0m] shift-left-int-text_int                  [2m(2.60ms)[0m

[1mtests/literal.sqltest[0m
  [[32mPASS[0m] numberic-literal-7                       [2m(2.81ms)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-function-update-journal-mode      [2m(2.57ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] tanh-null                                [2m(2.79ms)[0m

[1mtests/pragma/index_list.sqltest[0m
  [[32mPASS[0m] pragma-index-info-explicit-main-vs-temp  [2m(26.76ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] ddl-reject-multiple-pk-columns           [2m(6.29ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-insert-null                         [2m(521.09µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-int-null                      [2m(2.49ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-single-column           [2m(5.21ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_error_position_null                 [2m(467.89µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-multiple-columns        [2m(5.99ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-int-int-overflow              [2m(2.72ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-all-columns             [2m(5.40ms)[0m
  [[32mPASS[0m] insert-select-returning-literals         [2m(7.10ms)[0m
  [[32mPASS[0m] insert-select-returning-ordered          [2m(7.10ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_length_NULL                   [2m(484.71µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] upsert-partial-target-pk-only            [2m(9.62ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-select-returning-aggregate-source [2m(8.87ms)[0m
  [[32mPASS[0m] update-returning-constant-expression     [2m(4.74ms)[0m
  [[32mPASS[0m] update-returning-table-qualified         [2m(5.56ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-hexadecimal-2                      [2m(598.74µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-literal                 [2m(5.64ms)[0m
  [[32mPASS[0m] update-returning-column-arithmetic       [2m(5.58ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-int-int-underflow             [2m(2.67ms)[0m
  [[32mPASS[0m] divide-float-int                         [2m(2.78ms)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-update-journal-mode-wal           [2m(2.35ms)[0m

[1mtests/literal.sqltest[0m
  [[32mPASS[0m] numberic-literal-8                       [2m(2.77ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-multiple-column-expression [2m(5.47ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] trunc-int                                [2m(2.73ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_error_position_complex              [2m(483.06µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-int-float-overflow            [2m(2.71ms)[0m

[1mtests/joins/using_clause_case_insensitive.sqltest[0m
  [[32mPASS[0m] issue-7371-using-bare-different-case     [2m(19.05ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] upsert-partial-omitted-no-conflict       [2m(5.66ms)[0m

[1mtests/join/natural_join_no_common.sqltest[0m
  [[32mPASS[0m] natural-join-no-common-columns           [2m(6.94ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-function-call           [2m(6.84ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-int-float-underflow           [2m(2.52ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_type_no_path                        [2m(516.83µs)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] ddl-reject-multiple-pk-table-level       [2m(6.16ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-function-multiple-columns [2m(6.53ms)[0m
  [[32mPASS[0m] update-returning-complex-expression      [2m(11.45ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-hexadecimal-2-2                    [2m(516.20µs)[0m
  [[32mPASS[0m] json-remove-null                         [2m(506.90µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-multiple-rows           [2m(5.44ms)[0m
  [[32mPASS[0m] update-returning-multiple-rows-expressions [2m(5.47ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-float-int                     [2m(2.78ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-mixed-expressions       [2m(8.64ms)[0m

[1mtests/multi_index_dml.sqltest[0m
  [[32mPASS[0m] delete-multi-index-compound-or-returning [2m(11.70ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-null-values             [2m(5.36ms)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-table-info-vtable                 [2m(2.67ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-null-expression         [2m(4.75ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-between-create             [2m(5.79ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-ecma-script-1                      [2m(489.73µs)[0m

[1mtests/join/hash.sqltest[0m
  [[32mPASS[0m] grace-stress-large-result                [2m(382.45ms)[0m

[1mtests/literal.sqltest[0m
  [[32mPASS[0m] numberic-literal-9                       [2m(2.73ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-float-neg_int                 [2m(2.69ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-rowid                   [2m(5.50ms)[0m
  [[32mPASS[0m] update-returning-rowid-expression        [2m(5.21ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_type_root_path                      [2m(510.70µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-old-vs-new-values       [2m(8.89ms)[0m
  [[32mPASS[0m] update-returning-nested-expressions      [2m(5.20ms)[0m
  [[32mPASS[0m] update-returning-with-where              [2m(9.40ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_object_simple                       [2m(473.02µs)[0m
  [[32mPASS[0m] json5-number-1                           [2m(445.33µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-case-expression         [2m(6.79ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-float-float                   [2m(2.79ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-substring               [2m(4.80ms)[0m

[1mtests/join/hash.sqltest[0m
  [[32mPASS[0m] grace-complex-on-and-where               [2m(378.02ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_array_append_at_root            [2m(634.05µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] divide-by-zero                           [2m(6.06ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-string-concat           [2m(6.04ms)[0m
  [[32mPASS[0m] update-returning-row-values              [2m(5.75ms)[0m

[1mtests/join/natural_join_no_common.sqltest[0m
  [[32mPASS[0m] natural-join-no-common-columns-empty     [2m(5.99ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-may-not-reference-target-alias [2m(5.39ms)[0m
  [[32mPASS[0m] update-returning-may-reference-base-table-name-when-aliased [2m(5.27ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-ecma-script-2                      [2m(512.22µs)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] ddl-allow-notnull-and-pk-on-conflict     [2m(5.22ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-float-text                    [2m(2.70ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-from-returning-may-not-reference-target-alias [2m(5.23ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_type_array                          [2m(506.35µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] trunc-float                              [2m(5.81ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-between-insert             [2m(8.30ms)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-page-count-table                  [2m(3.90ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-target-selfread-reruns-per-row [2m(6.07ms)[0m

[1mtests/literal.sqltest[0m
  [[32mPASS[0m] numberic-literal-10                      [2m(2.87ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-number-2                           [2m(487.37µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-float-text_float              [2m(2.86ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-total-changes           [2m(4.85ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-ecma-script-3                      [2m(467.90µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-correlated-in-selfread  [2m(9.39ms)[0m
  [[32mPASS[0m] delete-returning-single-column           [2m(6.10ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_object_f64                          [2m(533.00µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-before-after-trigger-same-row-direct [2m(7.12ms)[0m
  [[32mPASS[0m] insert-returning-iif                     [2m(5.35ms)[0m
  [[32mPASS[0m] insert-returning-aggregate               [2m(4.70ms)[0m
  [[32mPASS[0m] insert-returning-json-array              [2m(5.30ms)[0m
  [[32mPASS[0m] insert-returning-aggregate-count         [2m(4.72ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_type_integer                        [2m(537.73µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-multi-column-subquery   [2m(4.77ms)[0m
  [[32mPASS[0m] insert-returning-likely                  [2m(7.67ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-number-3                           [2m(520.46µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-target-selfread-correlated-updated-group [2m(10.63ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] basic-order-by                           [2m(1.01ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-out-of-range                        [2m(458.17µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] trunc-float-negative                     [2m(2.83ms)[0m

[1mtests/literal.sqltest[0m
  [[32mPASS[0m] numberic-literal-11                      [2m(2.71ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-multi-column-subquery   [2m(4.77ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-date-only                           [2m(466.58µs)[0m
  [[32mPASS[0m] time-with-timezone-utc                   [2m(483.17µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-between-update             [2m(8.58ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-with-single-trailing-comma-valid   [2m(486.89µs)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] basic-order-by-2                         [2m(1.01ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-multi-column-subquery   [2m(4.71ms)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-function-argument-with-space      [2m(4.09ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-after-trigger-same-row-subquery [2m(11.96ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-timezone-positive              [2m(483.60µs)[0m
  [[32mPASS[0m] time-with-timezone-negative              [2m(444.40µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_object_nested                       [2m(558.64µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-timezone-day-change-positive   [2m(453.78µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-single-quoted                      [2m(464.61µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-timezone-day-change-negative   [2m(453.77µs)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] basic-order-by-and-limit                 [2m(858.81µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-timezone-iso8601               [2m(461.52µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_object_quoted_json                  [2m(555.82µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-timezone-and-milliseconds      [2m(438.35µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-hexadecimal-1                      [2m(458.37µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-single-column-subquery  [2m(5.31ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-invalid-timezone               [2m(446.91µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-after-trigger-same-row-direct [2m(14.59ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-modifier-start-of-day          [2m(457.62µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_type_real                           [2m(526.36µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] trunc-str                                [2m(2.82ms)[0m

[1mtests/literal.sqltest[0m
  [[32mPASS[0m] numberic-literal-12                      [2m(2.80ms)[0m

[1mtests/join/natural_join_no_common.sqltest[0m
  [[32mPASS[0m] natural-join-no-common-columns-single    [2m(6.75ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] basic-order-by-and-limit-2               [2m(864.56µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_object_unquoted_json                [2m(491.83µs)[0m
  [[32mPASS[0m] json5-hexadecimal-2                      [2m(456.89µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-modifier-add-hours             [2m(498.87µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] divide-int-null                          [2m(2.62ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-modifier-add-minutes           [2m(499.88µs)[0m

[1mtests/rollback.sqltest[0m
  [[32mPASS[0m] simple-rollback                          [2m(4.88ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-modifier-add-seconds           [2m(486.92µs)[0m
  [[32mPASS[0m] time-with-modifier-subtract-hours        [2m(492.63µs)[0m
  [[32mPASS[0m] time-with-modifier-subtract-minutes      [2m(461.38µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_array_append_after_array_index  [2m(963.12µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-modifier-subtract-seconds      [2m(472.37µs)[0m
  [[32mPASS[0m] time-with-multiple-modifiers             [2m(500.36µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_type_true                           [2m(490.45µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-invalid-modifier               [2m(448.06µs)[0m
  [[32mPASS[0m] time-with-invalid-modifier-2             [2m(444.17µs)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-function-sql-injection            [2m(3.02ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_object_multiple_values              [2m(544.66µs)[0m
  [[32mPASS[0m] json5-hexadecimal-2-2                    [2m(463.58µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-unixepoch                      [2m(487.20µs)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] basic-order-by-and-limit-2-2             [2m(816.58µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-unixepoch-modifiers            [2m(471.00µs)[0m
  [[32mPASS[0m] time-with-julianday                      [2m(460.37µs)[0m
  [[32mPASS[0m] time-with-julianday-2                    [2m(488.88µs)[0m
  [[32mPASS[0m] time-with-auto                           [2m(1.21ms)[0m
  [[32mPASS[0m] time-with-julianday-modifiers            [2m(484.00µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_type_false                          [2m(487.01µs)[0m

[1mtests/multi_index_dml.sqltest[0m
  [[32mPASS[0m] update-multi-index-and-with-trigger      [2m(16.11ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_object_empty                        [2m(407.61µs)[0m
  [[32mPASS[0m] json5-number-1                           [2m(456.87µs)[0m
  [[32mPASS[0m] json_set_array_append_after_object_key   [2m(485.54µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-at-start                       [2m(475.40µs)[0m
  [[32mPASS[0m] unixepoch-at-1-second-before-epochtime   [2m(460.85µs)[0m
  [[32mPASS[0m] unixepoch-at-future                      [2m(432.23µs)[0m
  [[32mPASS[0m] unixepoch-at-start-of-time               [2m(446.69µs)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] basic-order-by-and-limit-2-3             [2m(804.65µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-at-millisecond-precision-input-produces-seconds-precision-output [2m(447.29µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_type_null                           [2m(484.96µs)[0m
  [[32mPASS[0m] json_object_json_array                   [2m(446.45µs)[0m
  [[32mPASS[0m] json5-number-2                           [2m(445.26µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-modifier-start-of-day     [2m(440.00µs)[0m

[1mtests/literal.sqltest[0m
  [[32mPASS[0m] numberic-literal-13                      [2m(2.71ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] trunc-null                               [2m(2.81ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-modifier-start-of-month   [2m(487.04µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] divide-null-int                          [2m(2.56ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-modifier-start-of-year    [2m(454.00µs)[0m
  [[32mPASS[0m] unixepoch-with-modifier-add-months       [2m(463.94µs)[0m
  [[32mPASS[0m] unixepoch-with-modifier-subtract-months  [2m(466.35µs)[0m
  [[32mPASS[0m] unixepoch-with-modifier-add-years        [2m(489.00µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_type_text                           [2m(489.85µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-modifier-subtract-years   [2m(452.50µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_from_json_object                    [2m(444.65µs)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] basic-order-by-and-limit-3               [2m(783.05µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-number-3                           [2m(456.33µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-default-ceiling                [2m(486.25µs)[0m
  [[32mPASS[0m] unixepoch-default-ceiling-fractional     [2m(452.41µs)[0m
  [[32mPASS[0m] unixepoch-floor                          [2m(460.07µs)[0m
  [[32mPASS[0m] unixepoch-floor-keeps-time               [2m(477.94µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_object_infinity                     [2m(469.01µs)[0m
  [[32mPASS[0m] json_type_NULL                           [2m(482.80µs)[0m
  [[32mPASS[0m] json5-number-5                           [2m(459.48µs)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] basic-order-by-and-limit-3-2             [2m(755.13µs)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-page-size-set-initialized-db      [2m(3.76ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-ceiling-floor-2                [2m(478.35µs)[0m
  [[32mPASS[0m] unixepoch-ceiling-keeps-time             [2m(519.03µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-between-delete             [2m(8.24ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-ceiling-floor-5                [2m(499.15µs)[0m
  [[32mPASS[0m] unixepoch-ceiling-floor-6                [2m(465.56µs)[0m

[1mtests/rollback.sqltest[0m
  [[32mPASS[0m] simple-rollback-2                        [2m(4.87ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_object_single_infinity              [2m(455.24µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-ceiling-floor-7                [2m(477.67µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_type_cast                           [2m(476.88µs)[0m
  [[32mPASS[0m] json5-number-6                           [2m(455.14µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-ceiling-floor-8                [2m(455.97µs)[0m
  [[32mPASS[0m] unixepoch-with-modifier-weekday          [2m(444.15µs)[0m
  [[32mPASS[0m] unixepoch-with-subsec                    [2m(489.80µs)[0m

[1mtests/literal.sqltest[0m
  [[32mPASS[0m] numberic-literal-14                      [2m(2.74ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-modifier-add-months-31    [2m(446.18µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] divide-null                              [2m(2.60ms)[0m
  [[32mPASS[0m] atan2-int-int                            [2m(2.75ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_object_duplicated_keys              [2m(461.43µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-target-selfread-reruns-per-row [2m(11.96ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_type_null_arg                       [2m(455.58µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-modifier-subtract-months-31 [2m(455.96µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-multi-comment                      [2m(454.44µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-modifier-add-months-large [2m(466.23µs)[0m
  [[32mPASS[0m] unixepoch-with-modifier-subtract-months-large [2m(460.05µs)[0m
  [[32mPASS[0m] unixepoch-with-modifier-february-leap-year [2m(442.03µs)[0m
  [[32mPASS[0m] unixepoch-with-modifier-february-non-leap-year [2m(505.86µs)[0m
  [[32mPASS[0m] unixepoch-with-modifier-date             [2m(469.84µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_valid_1                             [2m(441.96µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-target-selfread-correlated-group-count [2m(12.45ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_type_blob_with_trailing_bytes       [2m(481.71µs)[0m
  [[32mPASS[0m] json5-ecma-script-1-pretty               [2m(476.23µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-modifier-datetime-pos     [2m(445.60µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] left-hash-join-basic                     [2m(7.91ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-modifier-sub              [2m(497.93µs)[0m
  [[32mPASS[0m] unixepoch-with-modifier-add              [2m(476.12µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_valid_2                             [2m(439.61µs)[0m
  [[32mPASS[0m] json_error_position_valid                [2m(480.48µs)[0m
  [[32mPASS[0m] json5-ecma-script-2-pretty               [2m(475.54µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-multiple-modifiers        [2m(531.90µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-float-text_int                [2m(19.12ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-multiple-modifiers-datetime [2m(473.06µs)[0m
  [[32mPASS[0m] date-with-modifier-start-of-day          [2m(463.72µs)[0m
  [[32mPASS[0m] date-with-modifier-start-of-month        [2m(478.02µs)[0m

[1mtests/literal.sqltest[0m
  [[32mPASS[0m] numberic-literal-exceed-max-i64          [2m(2.75ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_valid_3                             [2m(474.95µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] divide-agg-int-agg-int                   [2m(2.69ms)[0m
  [[32mPASS[0m] atan2-int-float                          [2m(2.71ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-ecma-script-3-pretty               [2m(464.33µs)[0m
  [[32mPASS[0m] json_error_position_valid_ws             [2m(472.54µs)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-max-page-count-clamping-with-data [2m(4.18ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-modifier-start-of-year         [2m(447.87µs)[0m
  [[32mPASS[0m] date-with-modifier-add-months            [2m(475.39µs)[0m
  [[32mPASS[0m] datetime-default-ceiling                 [2m(463.92µs)[0m
  [[32mPASS[0m] datetime-floor-keeps-time                [2m(491.23µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-number-5                           [2m(3.56ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-julianday-negative              [2m(503.08µs)[0m
  [[32mPASS[0m] datetime-unixepoch-negative              [2m(495.21µs)[0m
  [[32mPASS[0m] datetime-unixepoch-4decimal              [2m(493.75µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_valid_4                             [2m(445.77µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-fractional-month                [2m(480.50µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_error_position_object               [2m(476.71µs)[0m
  [[32mPASS[0m] json5-with-single-trailing-comma-valid-pretty [2m(481.25µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-fractional-year                 [2m(446.69µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-number-6                           [2m(469.81µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-ceiling-floor-2                     [2m(491.01µs)[0m
  [[32mPASS[0m] datetime-ceiling-keeps-time              [2m(502.12µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_valid_5                             [2m(438.78µs)[0m

[1mtests/rollback.sqltest[0m
  [[32mPASS[0m] rollback-after-update                    [2m(5.61ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-ceiling-floor-3                     [2m(472.66µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-float-null                    [2m(2.76ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_error_position_array_valid          [2m(475.79µs)[0m
  [[32mPASS[0m] json5-single-quoted-pretty               [2m(487.67µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-ceiling-floor-4                     [2m(469.98µs)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_group_object_value-issue-5754-reproducer [2m(9.34ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-multi-comment                      [2m(460.03µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-ceiling-floor-5                     [2m(514.26µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_valid_6                             [2m(432.08µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-ceiling-floor-7                     [2m(490.91µs)[0m
  [[32mPASS[0m] date-ceiling-floor-6                     [2m(496.58µs)[0m
  [[32mPASS[0m] date-ceiling-floor-8                     [2m(496.31µs)[0m
  [[32mPASS[0m] date-with-modifier-subtract-months       [2m(444.25µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] divide-agg-int-agg-float                 [2m(2.72ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-modifier-add-years             [2m(442.64µs)[0m

[1mtests/literal.sqltest[0m
  [[32mPASS[0m] numberic-literal-exceed-min-i64          [2m(2.75ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-hexadecimal-1-pretty               [2m(466.39µs)[0m
  [[32mPASS[0m] json_error_position_array_valid_ws       [2m(469.68µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] atan2-int-str                            [2m(2.79ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-modifier-subtract-years        [2m(460.87µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_valid_7                             [2m(439.10µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-modifier-weekday               [2m(495.91µs)[0m
  [[32mPASS[0m] date-with-multiple-modifiers-2           [2m(476.32µs)[0m
  [[32mPASS[0m] date-with-subsec                         [2m(490.00µs)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-max-page-count-enforcement-error  [2m(2.94ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-modifier-add-hours-2           [2m(508.78µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-between-select             [2m(8.16ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-modifier-subtract-hours-2      [2m(446.54µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-hexadecimal-2-pretty               [2m(471.01µs)[0m
  [[32mPASS[0m] json_error_position_array                [2m(456.31µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-modifier-add-minutes-2         [2m(444.15µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_valid_8                             [2m(433.46µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-modifier-subtract-seconds-2    [2m(478.27µs)[0m
  [[32mPASS[0m] time-with-subsec                         [2m(473.95µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-float-int-overflow            [2m(2.75ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-modifier-add                   [2m(451.30µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_error_position_null                 [2m(513.41µs)[0m
  [[32mPASS[0m] json5-hexadecimal-2-pretty-2             [2m(518.45µs)[0m
  [[32mPASS[0m] json_valid_9                             [2m(506.00µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-modifier-add-months-2          [2m(515.14µs)[0m
  [[32mPASS[0m] time-with-modifier-sub                   [2m(521.51µs)[0m
  [[32mPASS[0m] date-with-modifier-subtract-months-2     [2m(442.57µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_error_position_complex              [2m(468.15µs)[0m
  [[32mPASS[0m] json5-number-1-pretty                    [2m(466.20µs)[0m

[1mtests/literal.sqltest[0m
  [[32mPASS[0m] invalid-numberic-literal-1               [2m(2.63ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] divide-agg-float-agg-int                 [2m(2.77ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_valid_blob_embedded_null            [2m(464.01µs)[0m
  [[32mPASS[0m] json5-ecma-script-1-pretty               [2m(3.47ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] atan2-float-int                          [2m(2.82ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-modifier-add-months-large      [2m(482.82µs)[0m
  [[32mPASS[0m] date-with-modifier-subtract-months-large [2m(499.83µs)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-page-count-empty                  [2m(2.54ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-modifier-february-leap-year    [2m(474.91µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] left-hash-join-no-matches                [2m(7.69ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-modifier-february-non-leap-year [2m(493.05µs)[0m
  [[32mPASS[0m] date-with-modifier-invalid-date          [2m(446.90µs)[0m
  [[32mPASS[0m] date-with-modifier-date                  [2m(482.26µs)[0m
  [[32mPASS[0m] datetime-with-modifier-datetime-pos      [2m(473.26µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-number-2-pretty                    [2m(475.70µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-with-modifier-datetime-neg      [2m(459.63µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_object_simple                       [2m(495.85µs)[0m
  [[32mPASS[0m] json_valid_no_args                       [2m(371.45µs)[0m

[1mtests/multi_index_dml.sqltest[0m
  [[32mPASS[0m] delete-multi-index-compound-or-with-trigger [2m(14.76ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-with-modifier-datetime-large    [2m(486.86µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-ecma-script-2-pretty               [2m(477.20µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-float-int-underflow           [2m(2.75ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-with-modifier-datetime-sub-large [2m(483.14µs)[0m
  [[32mPASS[0m] datetime-with-timezone-utc               [2m(469.29µs)[0m
  [[32mPASS[0m] datetime-with-modifier-sub               [2m(460.03µs)[0m
  [[32mPASS[0m] datetime-with-modifier-add               [2m(508.01µs)[0m
  [[32mPASS[0m] time-with-multiple-modifiers-2           [2m(483.74µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-number-3-pretty                    [2m(474.03µs)[0m
  [[32mPASS[0m] json_object_f64                          [2m(488.55µs)[0m
  [[32mPASS[0m] json-patch-basic-1                       [2m(468.73µs)[0m
  [[32mPASS[0m] json5-ecma-script-3-pretty               [2m(459.52µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-with-multiple-modifiers         [2m(497.12µs)[0m
  [[32mPASS[0m] datetime-with-modifier-ceiling           [2m(476.26µs)[0m
  [[32mPASS[0m] datetime-with-modifier-ceiling-already-ceiled [2m(451.76µs)[0m
  [[32mPASS[0m] datetime-with-ceiling-modifier-invalid-input [2m(446.45µs)[0m

[1mtests/rollback.sqltest[0m
  [[32mPASS[0m] rollback-after-delete                    [2m(6.41ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-number-5-pretty                    [2m(464.30µs)[0m
  [[32mPASS[0m] json-patch-basic-2                       [2m(472.08µs)[0m
  [[32mPASS[0m] json_object_nested                       [2m(569.02µs)[0m

[1mtests/literal.sqltest[0m
  [[32mPASS[0m] invalid-numberic-literal-2               [2m(2.54ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-with-single-trailing-comma-valid-pretty [2m(447.16µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-with-ceiling-modifier-stacked   [2m(505.28µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-multiple-columns        [2m(11.60ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-ceiling-modifier-basic         [2m(456.99µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] add-float-text-edgecase                  [2m(2.78ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-with-weekday                    [2m(449.05µs)[0m
  [[32mPASS[0m] unixepoch-invalid-date                   [2m(452.12µs)[0m
  [[32mPASS[0m] unixepoch-subsec                         [2m(477.51µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] atan2-float-float                        [2m(2.85ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-negative-timestamp             [2m(455.23µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-number-6-pretty                    [2m(476.47µs)[0m
  [[32mPASS[0m] json-patch-preserve-duplicates-1         [2m(470.21µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-large-date                     [2m(490.20µs)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-page-count-empty-2                [2m(3.01ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-float-float-overflow          [2m(2.68ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_object_quoted_json                  [2m(499.24µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-with-timezone                   [2m(474.69µs)[0m
  [[32mPASS[0m] julianday-fractional                     [2m(474.00µs)[0m
  [[32mPASS[0m] julianday-fractional-2                   [2m(463.53µs)[0m
  [[32mPASS[0m] julianday-rounded-up                     [2m(459.72µs)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_group_object_with_direct_json_object_value [2m(8.03ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] julianday-with-timezone                  [2m(499.37µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-indexed-by-between-predicate-can-prove-index [2m(7.28ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-multi-comment-pretty               [2m(486.78µs)[0m
  [[32mPASS[0m] json-patch-preserve-duplicates-2         [2m(472.32µs)[0m
  [[32mPASS[0m] json_object_unquoted_json                [2m(516.18µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] julianday-time-only                      [2m(464.47µs)[0m
  [[32mPASS[0m] julianday-midnight                       [2m(468.61µs)[0m
  [[32mPASS[0m] julianday-noon                           [2m(479.03µs)[0m
  [[32mPASS[0m] julianday-fractional-seconds             [2m(1.24ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-pretty-ident-1                      [2m(464.10µs)[0m
  [[32mPASS[0m] json-patch-last-update-wins              [2m(469.04µs)[0m

[1mtests/literal.sqltest[0m
  [[32mPASS[0m] invalid-numberic-literal-3               [2m(2.63ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_object_multiple_values              [2m(606.72µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] julianday-fractional-zero                [2m(447.78µs)[0m
  [[32mPASS[0m] julianday-date-only                      [2m(455.96µs)[0m
  [[32mPASS[0m] julianday-with-modifier-day              [2m(459.24µs)[0m
  [[32mPASS[0m] unixepoch-leap-second                    [2m(3.59ms)[0m
  [[32mPASS[0m] julianday-with-modifier-hour             [2m(490.77µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-pretty-ident-2                      [2m(455.45µs)[0m
  [[32mPASS[0m] json-patch-override-1                    [2m(451.01µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] add-str-edgecase                         [2m(2.87ms)[0m
  [[32mPASS[0m] shift-left-float-float-underflow         [2m(2.75ms)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-user-version-default              [2m(2.40ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] atan2-float-str                          [2m(2.75ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-single-quoted-pretty               [2m(475.74µs)[0m
  [[32mPASS[0m] json_object_empty                        [2m(459.72µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-error-null                      [2m(458.75µs)[0m
  [[32mPASS[0m] julianday-max-day                        [2m(469.71µs)[0m
  [[32mPASS[0m] strftime-day                             [2m(498.14µs)[0m
  [[32mPASS[0m] strftime-day-without-leading-zero-1      [2m(468.33µs)[0m
  [[32mPASS[0m] strftime-day-without-leading-zero-2      [2m(485.38µs)[0m
  [[32mPASS[0m] strftime-fractional-seconds              [2m(449.32µs)[0m
  [[32mPASS[0m] strftime-iso-8601-date                   [2m(483.37µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-override-2                    [2m(468.36µs)[0m
  [[32mPASS[0m] json-pretty-ident-null                   [2m(475.37µs)[0m

[1mtests/offset/default.sqltest[0m
  [[32mPASS[0m] select-offset-1-groupby                  [2m(113.08ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-hexadecimal-1-pretty               [2m(462.14µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-iso-8601-year                   [2m(458.05µs)[0m
  [[32mPASS[0m] strftime-iso-8601-year-2_digit           [2m(478.34µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_object_json_array                   [2m(523.92µs)[0m
  [[32mPASS[0m] json_replace_basic_1                     [2m(540.57µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-hour                            [2m(457.82µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] left-hash-join-all-match                 [2m(7.64ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-nested-1                      [2m(487.50µs)[0m
  [[32mPASS[0m] json-pretty-ident-blob-1                 [2m(482.04µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-hour-12-hour-clock              [2m(491.68µs)[0m
  [[32mPASS[0m] strftime-day-of-year                     [2m(485.07µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-hexadecimal-2-pretty               [2m(448.84µs)[0m

[1mtests/literal.sqltest[0m
  [[32mPASS[0m] invalid-numberic-literal-4               [2m(2.54ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-hour-without-leading-zero-1     [2m(453.72µs)[0m
  [[32mPASS[0m] strftime-julianday                       [2m(497.60µs)[0m
  [[32mPASS[0m] strftime-hour-without-leading-zero-2     [2m(466.13µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_from_json_object                    [2m(501.15µs)[0m
  [[32mPASS[0m] json_replace_basic_2                     [2m(501.31µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-text-int                      [2m(2.71ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-hour-12-hour-clock-without-leading-zero-2 [2m(458.79µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-nested-2                      [2m(488.26µs)[0m
  [[32mPASS[0m] json_array_postive-expo                  [2m(479.37µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-month                           [2m(479.19µs)[0m
  [[32mPASS[0m] strftime-minute                          [2m(470.52µs)[0m

[1mtests/rollback.sqltest[0m
  [[32mPASS[0m] rollback-mixed-operations                [2m(6.80ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] subtract-str-float-edgecase              [2m(2.75ms)[0m
  [[32mPASS[0m] atan2-str-str                            [2m(2.74ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-am-pm1                          [2m(467.63µs)[0m
  [[32mPASS[0m] strftime-am-pm-2                         [2m(493.09µs)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-user-version-update               [2m(2.96ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_object_infinity                     [2m(534.86µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] add-agg-int                              [2m(719.10µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_multiple_paths              [2m(535.21µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-am-pm-lower-2                   [2m(477.71µs)[0m
  [[32mPASS[0m] strftime-am-pm-lower-1                   [2m(525.38µs)[0m
  [[32mPASS[0m] strftime-iso8601-time                    [2m(487.46µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-array-1                       [2m(465.55µs)[0m
  [[32mPASS[0m] json_array_negative-expo                 [2m(475.59µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-seconds-since-epoch             [2m(441.54µs)[0m
  [[32mPASS[0m] strftime-with-subsec                     [2m(447.70µs)[0m
  [[32mPASS[0m] strftime-with-subsec-math                [2m(501.30µs)[0m
  [[32mPASS[0m] strftime-seconds                         [2m(451.08µs)[0m
  [[32mPASS[0m] strftime-iso8601-with-seconds            [2m(470.45µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_object_single_infinity              [2m(420.44µs)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_group_object_with_indirect_json_object_value [2m(5.59ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] add-int-agg                              [2m(676.69µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-array-2                       [2m(462.92µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-week-year-start-sunday          [2m(510.98µs)[0m
  [[32mPASS[0m] strftime-day-week-start-monday           [2m(431.96µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_str                           [2m(464.28µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-iso8601-week-year               [2m(481.45µs)[0m
  [[32mPASS[0m] strftime-day-week-start-sunday           [2m(472.38µs)[0m
  [[32mPASS[0m] strftime-day-week-start-sunday-2         [2m(462.21µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-all-columns             [2m(8.43ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-hexadecimal-2-pretty-2             [2m(498.63µs)[0m
  [[32mPASS[0m] json_object_duplicated_keys              [2m(470.94µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-week-year-start-sunday-2        [2m(466.46µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-text-float                    [2m(2.74ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-empty-1                       [2m(460.24µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-year                            [2m(480.10µs)[0m
  [[32mPASS[0m] strftime-invalid-S-3f                    [2m(438.70µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_numbers                       [2m(455.13µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] add-agg-float                            [2m(674.48µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-C                       [2m(468.40µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] multiply-str-floats-edgecase             [2m(2.44ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-y                       [2m(441.28µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_valid_1                             [2m(429.78µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-b                       [2m(465.24µs)[0m
  [[32mPASS[0m] strftime-invalid-B                       [2m(464.24µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] atan2-null-int                           [2m(2.73ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_string                      [2m(508.78µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-h                       [2m(482.51µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-empty-2                       [2m(454.76µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-a                       [2m(445.37µs)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-user-version-negative-value       [2m(2.89ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_numbers_2                     [2m(493.33µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-A                       [2m(479.54µs)[0m
  [[32mPASS[0m] strftime-invalid-D                       [2m(466.63µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] add-float-agg                            [2m(678.30µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-x                       [2m(445.60µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-between-multiple           [2m(9.22ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_valid_2                             [2m(435.60µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-v                       [2m(468.99µs)[0m
  [[32mPASS[0m] strftime-invalid-dotf                    [2m(470.84µs)[0m
  [[32mPASS[0m] strftime-invalid-dot3f                   [2m(457.32µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_number_with_string          [2m(506.82µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-dot6f                   [2m(429.54µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-deep-nested-1                 [2m(483.70µs)[0m
  [[32mPASS[0m] json_array_null                          [2m(438.21µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-text-text                     [2m(2.65ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_valid_3                             [2m(456.37µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-3f                      [2m(450.32µs)[0m
  [[32mPASS[0m] strftime-percent                         [2m(2.83ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] add-agg-int-agg-int-2                    [2m(674.69µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-6f                      [2m(476.09µs)[0m
  [[32mPASS[0m] strftime-invalid-9f                      [2m(437.40µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-mixed-types-1                 [2m(496.21µs)[0m
  [[32mPASS[0m] json5-number-1-pretty                    [2m(488.17µs)[0m
  [[32mPASS[0m] json_array_not_json                      [2m(450.18µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-X                       [2m(449.20µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_valid_4                             [2m(412.02µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-r                       [2m(417.63µs)[0m
  [[32mPASS[0m] strftime-invalid-Z                       [2m(462.09µs)[0m
  [[32mPASS[0m] strftime-invalid-z                       [2m(473.98µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-and-int-null                     [2m(2.70ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-colonz                  [2m(433.92µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] add-agg-float-agg-float                  [2m(701.68µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-add-all-dup-keys-from-patch   [2m(469.11µs)[0m
  [[32mPASS[0m] json5-number-2-pretty                    [2m(467.92µs)[0m
  [[32mPASS[0m] json_array_json                          [2m(476.81µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] atan2-int-null                           [2m(2.70ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-doublecolonz            [2m(467.09µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_valid_5                             [2m(425.24µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-triplecolonz            [2m(461.35µs)[0m
  [[32mPASS[0m] strftime-invalid-hashz                   [2m(436.26µs)[0m
  [[32mPASS[0m] strftime-invalid-c                       [2m(493.93µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] left-hash-join-duplicate-keys            [2m(7.87ms)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-user-version-float-value          [2m(2.91ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-plus                    [2m(496.03µs)[0m
  [[32mPASS[0m] strftime-invalid-t                       [2m(480.81µs)[0m
  [[32mPASS[0m] strftime-invalid-n                       [2m(472.50µs)[0m

[1mtests/multi_index_dml.sqltest[0m
  [[32mPASS[0m] update-multi-index-and-returning-with-trigger-and-fk [2m(15.73ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-first-occurrence-patch        [2m(492.42µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-dot9f                   [2m(2.96ms)[0m
  [[32mPASS[0m] strftime-invalid-underscorequestion      [2m(448.87µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] add-agg-int-agg-float-2                  [2m(753.45µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-dashquestion            [2m(485.06µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-text_int-text_int             [2m(2.69ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_valid_6                             [2m(418.31µs)[0m

[1mtests/offset/default.sqltest[0m
  [[32mPASS[0m] select-offset-0-groupby                  [2m(123.33ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_nested                        [2m(533.39µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-zeroquestion            [2m(503.18µs)[0m
  [[32mPASS[0m] strftime-julianday-2                     [2m(430.03µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_with_null                   [2m(504.66µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] timediff-basic-positive                  [2m(463.37µs)[0m
  [[32mPASS[0m] timediff-basic-negative                  [2m(446.69µs)[0m
  [[32mPASS[0m] timediff-with-milliseconds-positive      [2m(474.64µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-table-qualified         [2m(5.57ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_complex_1                   [2m(656.20µs)[0m
  [[32mPASS[0m] json-patch-complex-nested-dup-keys       [2m(507.19µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] timediff-same-time                       [2m(477.65µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_valid_7                             [2m(424.79µs)[0m
  [[32mPASS[0m] json_array_infinity                      [2m(455.11µs)[0m

[1mtests/rollback.sqltest[0m
  [[32mPASS[0m] insert-after-rollback                    [2m(8.12ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] timediff-across-dates                    [2m(476.40µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] subtract-agg-int                         [2m(665.20µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] timediff-different-formats               [2m(439.13µs)[0m
  [[32mPASS[0m] timediff-across-dates-negative           [2m(467.93µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_with_json_object            [2m(492.99µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] timediff-with-timezone                   [2m(422.84µs)[0m
  [[32mPASS[0m] timediff-large-difference                [2m(466.57µs)[0m
  [[32mPASS[0m] timediff-with-seconds-precision          [2m(493.01µs)[0m
  [[32mPASS[0m] timediff-null-first-arg                  [2m(452.97µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-unicode-dup-keys              [2m(465.02µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-and-int-int                      [2m(2.76ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_after_remove                [2m(1.00ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] timediff-null-second-arg                 [2m(883.77µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_valid_8                             [2m(859.36µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] timediff-invalid-first-arg               [2m(619.05µs)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_group_array_with_direct_json_object_value [2m(7.89ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_negative_infinity             [2m(796.86µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] mod-int-int                              [2m(2.82ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] timediff-invalid-second-arg              [2m(467.41µs)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-application-id-default            [2m(2.41ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] subtract-int-agg                         [2m(892.15µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] timediff-julian-day                      [2m(480.01µs)[0m
  [[32mPASS[0m] timediff-different-time-formats          [2m(439.62µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-text_int-text_float           [2m(2.69ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_array_element               [2m(500.04µs)[0m
  [[32mPASS[0m] json-patch-empty-string-dup-keys         [2m(454.25µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-whitespace-now-rejected         [2m(452.97µs)[0m
  [[32mPASS[0m] datetime-whitespace-now-rejected-leading [2m(454.16µs)[0m
  [[32mPASS[0m] datetime-whitespace-now-rejected-trailing [2m(447.52µs)[0m
  [[32mPASS[0m] datetime-negative-year                   [2m(431.02µs)[0m
  [[32mPASS[0m] datetime-negative-year-with-time         [2m(448.56µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_valid_9                             [2m(408.42µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-date-overflow-feb30             [2m(416.22µs)[0m
  [[32mPASS[0m] date-negative-year                       [2m(472.93µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_bare_infinity                       [2m(462.63µs)[0m
  [[32mPASS[0m] json_remove_after_replace                [2m(548.42µs)[0m
  [[32mPASS[0m] json5-number-3-pretty                    [2m(2.47ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-date-overflow-feb31             [2m(439.85µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] subtract-agg-float                       [2m(687.18µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-multiple-types-dup-keys       [2m(487.52µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-date-overflow-apr31             [2m(455.10µs)[0m
  [[32mPASS[0m] datetime-date-overflow-feb29-non-leap    [2m(462.95µs)[0m
  [[32mPASS[0m] date-date-overflow-feb30                 [2m(470.38µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_valid_blob_embedded_null            [2m(440.15µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-fractional-days                 [2m(425.03µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_bare_negative_infinity              [2m(447.38µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-fractional-hours                [2m(476.49µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_idempotence                 [2m(499.89µs)[0m
  [[32mPASS[0m] json5-number-5-pretty                    [2m(485.70µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-fractional-days-negative        [2m(456.81µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-deep-nested-dup-keys          [2m(513.46µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-fractional-minutes              [2m(483.93µs)[0m
  [[32mPASS[0m] datetime-fractional-seconds              [2m(476.21µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] subtract-float-agg                       [2m(684.84µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-trailing-garbage-rejected       [2m(443.39µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-and-int-float                    [2m(2.74ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-trailing-garbage-date-rejected  [2m(414.45µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_idempotence                  [2m(508.66µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] arithmetic-overflow                      [2m(459.70µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-text_int-text                 [2m(2.67ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json5-number-6-pretty                    [2m(470.03µs)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-application-id-update             [2m(2.73ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] out-of-bounds-unixepoch                  [2m(484.84µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] mod-int-float                            [2m(2.80ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] arithmetic-overflow-julianday            [2m(480.82µs)[0m
  [[32mPASS[0m] overflows-with-large-days-modifier       [2m(486.19µs)[0m
  [[32mPASS[0m] time_date-overflow-issue-5251            [2m(460.51µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] subtract-agg-int-agg-int-2               [2m(655.87µs)[0m

[1mtests/rollback.sqltest[0m
  [[32mPASS[0m] schema-change-rollback                   [2m(4.50ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] invalid-date                             [2m(464.57µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-abomination                   [2m(982.55µs)[0m
  [[32mPASS[0m] json_remove_with_extract                 [2m(525.27µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] invalid-time                             [2m(436.12µs)[0m
  [[32mPASS[0m] datetime-multibyte-utf8-emoji            [2m(426.73µs)[0m
  [[32mPASS[0m] invalid-datetime                         [2m(482.06µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_array_negative_index        [2m(3.54ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-multibyte-utf8-chinese              [2m(452.10µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-remove-1                            [2m(483.45µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] left-hash-join-empty-right               [2m(7.11ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] subtract-agg-float-agg-float             [2m(687.43µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-multibyte-utf8-japanese             [2m(465.40µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_with_extract                [2m(507.83µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-alias-correlated-subquery-returning [2m(6.33ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-multibyte-utf8-greek            [2m(433.31µs)[0m
  [[32mPASS[0m] julianday-multibyte-utf8-cyrillic        [2m(497.85µs)[0m
  [[32mPASS[0m] datetime-multibyte-modifier-arithmetic   [2m(450.56µs)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_group_array_with_indirect_json_object_value [2m(4.99ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_array_out_of_bounds         [2m(496.92µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-leapyear-floor                      [2m(469.47µs)[0m
  [[32mPASS[0m] date-leapyear-floor2                     [2m(462.23µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-remove-2                            [2m(459.13µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-text_float-text_int           [2m(2.74ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-leapyear-ceiling                    [2m(450.22µs)[0m
  [[32mPASS[0m] date-leapyear-ceiling2                   [2m(453.62µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-and-float-float                  [2m(2.73ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_with_arrow                  [2m(515.17µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] subtract-agg-int-agg-float-2             [2m(688.37µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] leapyear-plus-month                      [2m(434.32µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-between-text               [2m(12.39ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_entire_array                [2m(494.98µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] invalid-date-format                      [2m(467.13µs)[0m
  [[32mPASS[0m] invalid-datetime-modifier-format         [2m(470.46µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] mod-int-str                              [2m(2.77ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_malformed_json_1            [2m(4.22ms)[0m
  [[32mPASS[0m] json-remove-3                            [2m(455.34µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-utc-basic                       [2m(566.26µs)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-application-id-float-value        [2m(2.96ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-utc-with-z-suffix               [2m(466.20µs)[0m
  [[32mPASS[0m] datetime-utc-idempotent                  [2m(466.63µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_with_arrow                   [2m(509.33µs)[0m
  [[32mPASS[0m] json5-multi-comment-pretty               [2m(464.72µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-localtime-basic                 [2m(534.23µs)[0m
  [[32mPASS[0m] datetime-julianday-modifier              [2m(468.43µs)[0m
  [[32mPASS[0m] datetime-localtime-utc-roundtrip         [2m(568.19µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] subtract-agg-float-agg-int-2             [2m(718.71µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-julianday-modifier              [2m(511.76µs)[0m
  [[32mPASS[0m] unixepoch-julianday-modifier             [2m(495.50µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-remove-4                            [2m(488.44µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-unixepoch-modifier                  [2m(464.05µs)[0m
  [[32mPASS[0m] julianday-unixepoch-modifier             [2m(431.09µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-pretty-ident-1                      [2m(496.41µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-text_float-text_float         [2m(2.69ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-remove-5                            [2m(428.95µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] multiply-agg-int                         [2m(643.66µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-unixepoch-modifier              [2m(499.93µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_arrays_heterogeneous_primitives [2m(1.10ms)[0m

[1mtests/rollback.sqltest[0m
  [[32mPASS[0m] schema-change-rollback-version           [2m(4.49ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-unixepoch-negative-2            [2m(423.71µs)[0m
  [[32mPASS[0m] datetime-auto-julianday                  [2m(449.80µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-pretty-ident-2                      [2m(466.24µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-auto-unixepoch                  [2m(471.85µs)[0m
  [[32mPASS[0m] julianday-auto                           [2m(482.60µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-and-float-int-rev                [2m(2.76ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-remove-6                            [2m(455.41µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-auto                           [2m(456.15µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_valid_no_args                       [2m(7.42ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] multiply-int-agg                         [2m(646.20µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] mod-float-int                            [2m(2.70ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-auto                            [2m(482.60µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_arrays_parent_is_always_null   [2m(692.55µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-without-subsec                  [2m(423.12µs)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-application-id-large-value        [2m(2.90ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-with-subsec                     [2m(487.80µs)[0m
  [[32mPASS[0m] datetime-floor-localtime                 [2m(484.75µs)[0m
  [[32mPASS[0m] datetime-auto-start-of-month             [2m(510.81µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-remove-7                            [2m(472.80µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-julianday-floor                     [2m(495.24µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_nested_object               [2m(3.56ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-format-integer                  [2m(435.39µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-basic-1                       [2m(476.17µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] multiply-agg-float                       [2m(634.31µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-format-integer-positive         [2m(428.00µs)[0m
  [[32mPASS[0m] strftime-format-integer-negative         [2m(516.96µs)[0m
  [[32mPASS[0m] strftime-format-float                    [2m(484.35µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-literal                 [2m(5.60ms)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_group_object_with_nested_agg_value  [2m(5.30ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_arrays_id_uniqueness           [2m(836.62µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-text_float-text               [2m(2.74ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_malformed_json_2            [2m(4.16ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-format-float-negative           [2m(491.93µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-remove-8                            [2m(493.17µs)[0m
  [[32mPASS[0m] json-patch-basic-2                       [2m(447.30µs)[0m
  [[32mPASS[0m] json_replace_nested_array                [2m(522.07µs)[0m
  [[32mPASS[0m] json-pretty-ident-null                   [2m(2.01ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-format-null                     [2m(468.73µs)[0m
  [[32mPASS[0m] strftime-format-blob                     [2m(480.82µs)[0m
  [[32mPASS[0m] strftime-format-bool-true                [2m(488.82µs)[0m
  [[32mPASS[0m] date-current-date                        [2m(460.13µs)[0m
  [[32mPASS[0m] strftime-format-bool-false               [2m(509.06µs)[0m
  [[32mPASS[0m] date-specific-date                       [2m(424.11µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] multiply-float-agg                       [2m(683.56µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-time                           [2m(476.96µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-remove-9                            [2m(469.43µs)[0m
  [[32mPASS[0m] json_each_arrays_empty_container_yields_zero_rows [2m(635.72µs)[0m
  [[32mPASS[0m] json-patch-preserve-duplicates-1         [2m(467.91µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-iso8601                             [2m(458.45µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-pretty-ident-blob-1                 [2m(463.27µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-or-int-null                      [2m(2.84ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-milliseconds                   [2m(410.87µs)[0m
  [[32mPASS[0m] date-julian-day-integer                  [2m(461.88µs)[0m
  [[32mPASS[0m] date-julian-day-float                    [2m(434.57µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] mod-float-float                          [2m(2.78ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-invalid-input                       [2m(464.68µs)[0m
  [[32mPASS[0m] date-null-input                          [2m(440.34µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] multiply-agg-int-agg-int-2               [2m(640.31µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_type                            [2m(479.17µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] left-hash-join-with-where                [2m(8.09ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-preserve-duplicates-2         [2m(476.52µs)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-application-id-negative-value     [2m(2.95ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-out-of-range                        [2m(458.84µs)[0m

[1mtests/rollback.sqltest[0m
  [[32mPASS[0m] schema-version-after-update              [2m(4.32ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_postive-expo                  [2m(508.86µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-time-only                           [2m(442.47µs)[0m
  [[32mPASS[0m] date-with-timezone-utc                   [2m(429.58µs)[0m
  [[32mPASS[0m] date-with-timezone-positive              [2m(478.45µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_objects_simple_integer_values  [2m(921.31µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-timezone-negative              [2m(429.40µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-text-null                     [2m(2.66ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-timezone-day-change-positive   [2m(448.85µs)[0m
  [[32mPASS[0m] date-with-timezone-day-change-negative   [2m(443.49µs)[0m
  [[32mPASS[0m] date-with-timezone-iso8601               [2m(473.48µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_postive-expo                    [2m(438.52µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-timezone-and-milliseconds      [2m(454.14µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] multiply-agg-float-agg-float             [2m(675.70µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-last-update-wins              [2m(482.22µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-invalid-timezone               [2m(414.62µs)[0m

[1mtests/multi_index_dml.sqltest[0m
  [[32mPASS[0m] delete-multi-index-compound-or-returning-with-trigger-and-fk [2m(16.84ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_negative-expo                 [2m(504.62µs)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_subtype_strip_function_roundtrip    [2m(2.94ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-modifier-add-days              [2m(446.17µs)[0m
  [[32mPASS[0m] date-with-modifier-subtract-days         [2m(437.86µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_negative-expo                   [2m(422.54µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] julianday-null-input                     [2m(433.21µs)[0m
  [[32mPASS[0m] date-with-multiple-modifiers             [2m(514.77µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_objects_nested_containers_value_is_valid_json [2m(870.23µs)[0m
  [[32mPASS[0m] json-patch-override-1                    [2m(458.77µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] julianday-invalid-input                  [2m(402.52µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] multiply-agg-int-agg-float-2             [2m(677.48µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] julianday-empty-string                   [2m(467.89µs)[0m
  [[32mPASS[0m] julianday-negative-timezone              [2m(444.11µs)[0m
  [[32mPASS[0m] julianday-z-suffix                       [2m(440.88µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-or-null-int                      [2m(2.73ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_malformed_json_3            [2m(3.98ms)[0m
  [[32mPASS[0m] json_set_notation                        [2m(446.03µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] julianday-iso8601-t-separator            [2m(458.56µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_deep_nesting                [2m(525.50µs)[0m
  [[32mPASS[0m] json-patch-override-2                    [2m(482.56µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] julianday-out-of-range                   [2m(474.61µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_objects_empty_container_yields_zero_rows [2m(627.65µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] julianday-string-with-modifier           [2m(493.03µs)[0m
  [[32mPASS[0m] julianday-string-with-start-of-month     [2m(435.01µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] mod-float-str                            [2m(2.84ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] julianday-string-with-minutes-modifier   [2m(489.15µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] multiply-agg-float-agg-int-2             [2m(731.95µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-null-int                      [2m(2.77ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-floor-then-add-day                  [2m(481.40µs)[0m
  [[32mPASS[0m] datetime-floor-then-add-hours            [2m(494.77µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_empty_object                [2m(509.24µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-constant-expression     [2m(5.51ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-fractional-month-floor              [2m(487.13µs)[0m

[1mtests/join/hash.sqltest[0m
  [[32mPASS[0m] grace-left-outer-agg-group               [2m(503.69ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-fractional-month-floor          [2m(499.86µs)[0m
  [[32mPASS[0m] unixepoch-not-first-modifier             [2m(467.05µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_objects_keys_require_quoting_in_json_path [2m(844.32µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] divide-agg-int                           [2m(661.07µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] julianday-not-first-modifier             [2m(520.85µs)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_subtype_strip_extract_through_subquery [2m(2.96ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_empty_array                 [2m(500.44µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-after-start-of-day             [2m(473.97µs)[0m
  [[32mPASS[0m] julianday-after-hours-modifier           [2m(462.30µs)[0m
  [[32mPASS[0m] date-negative-date-offset                [2m(495.02µs)[0m
  [[32mPASS[0m] datetime-with-modifier-datetime-sub-large [2m(471.15µs)[0m

[1mtests/rollback.sqltest[0m
  [[32mPASS[0m] schema-change-rollback-2                 [2m(4.58ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-with-timezone-utc               [2m(470.09µs)[0m
  [[32mPASS[0m] datetime-with-modifier-sub               [2m(454.63µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_top_level_integer_single_row_key_null [2m(749.14µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] divide-int-agg                           [2m(643.61µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-invalid-modifier               [2m(4.48ms)[0m
  [[32mPASS[0m] datetime-with-modifier-add               [2m(493.01µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-or-int-int                       [2m(2.80ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-negative-date-offset-2              [2m(471.34µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-null-float                    [2m(2.70ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-multiple-modifiers-2           [2m(519.02µs)[0m
  [[32mPASS[0m] datetime-with-multiple-modifiers         [2m(467.58µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_str                           [2m(474.57µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-with-modifier-ceiling           [2m(461.05µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] mod-str-str                              [2m(2.73ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-with-modifier-ceiling-already-ceiled [2m(468.78µs)[0m
  [[32mPASS[0m] datetime-with-ceiling-modifier-invalid-input [2m(477.31µs)[0m
  [[32mPASS[0m] timediff-one-arg                         [2m(3.81ms)[0m
  [[32mPASS[0m] datetime-with-ceiling-modifier-stacked   [2m(467.82µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_top_level_true_single_row_key_null [2m(706.87µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] divide-agg-float                         [2m(664.58µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_malformed_json_4            [2m(3.88ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-1200-offset                    [2m(443.70µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_numbers                       [2m(500.80µs)[0m
  [[32mPASS[0m] json-patch-nested-1                      [2m(494.03µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-ceiling-modifier-basic         [2m(462.88µs)[0m
  [[32mPASS[0m] unixepoch-subsec                         [2m(462.55µs)[0m
  [[32mPASS[0m] datetime-with-weekday                    [2m(475.71µs)[0m
  [[32mPASS[0m] unixepoch-invalid-date                   [2m(464.48µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-between-combined           [2m(14.63ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-negative-timestamp             [2m(445.04µs)[0m
  [[32mPASS[0m] unixepoch-leap-second                    [2m(486.89µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_null                        [2m(463.74µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-12-offset                      [2m(463.94µs)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_subtype_strip_array_direct_value    [2m(2.83ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] divide-float-agg                         [2m(692.82µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_top_level_null_single_row_key_null [2m(743.05µs)[0m
  [[32mPASS[0m] json-patch-nested-2                      [2m(486.52µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] left-hash-join-count-with-nulls          [2m(7.80ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_numbers_2                     [2m(500.33µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-large-date                     [2m(463.35µs)[0m
  [[32mPASS[0m] datetime-with-timezone                   [2m(454.33µs)[0m
  [[32mPASS[0m] julianday-fractional                     [2m(458.48µs)[0m
  [[32mPASS[0m] julianday-fractional-2                   [2m(460.14µs)[0m
  [[32mPASS[0m] julianday-rounded-up                     [2m(456.96µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_blob_value                      [2m(5.61ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] julianday-with-timezone                  [2m(439.26µs)[0m
  [[32mPASS[0m] julianday-fractional-seconds             [2m(480.03µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_quoted_key                  [2m(3.51ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-null-text                     [2m(2.72ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_json_null_type              [2m(492.75µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-huge-number                     [2m(475.04µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-or-int-float                     [2m(2.74ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] julianday-midnight                       [2m(493.42µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-array-1                       [2m(447.40µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] divide-agg-int-agg-int-2                 [2m(665.17µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] julianday-noon                           [2m(468.90µs)[0m
  [[32mPASS[0m] julianday-fractional-zero                [2m(461.78µs)[0m
  [[32mPASS[0m] julianday-date-only                      [2m(452.99µs)[0m
  [[32mPASS[0m] julianday-with-modifier-day              [2m(474.43µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] mod-null-int                             [2m(2.74ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] julianday-with-modifier-hour             [2m(449.60µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_field_empty_object              [2m(517.52µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] julianday-max-day                        [2m(468.69µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_json_null_type                [2m(477.10µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-column-arithmetic       [2m(5.52ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_root                        [2m(520.16µs)[0m
  [[32mPASS[0m] json_each_atom_equals_value_for_primitives_containers_are_json_text [2m(1.25ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-negative-date-offset            [2m(472.01µs)[0m
  [[32mPASS[0m] julianday-time-only                      [2m(1.68ms)[0m
  [[32mPASS[0m] strftime-error-null                      [2m(492.45µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-array-2                       [2m(455.17µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-day                             [2m(467.26µs)[0m
  [[32mPASS[0m] strftime-day-without-leading-zero-1      [2m(469.48µs)[0m
  [[32mPASS[0m] strftime-day-without-leading-zero-2      [2m(474.79µs)[0m
  [[32mPASS[0m] strftime-iso-8601-date                   [2m(470.37µs)[0m
  [[32mPASS[0m] strftime-iso-8601-year                   [2m(457.77µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] divide-agg-float-agg-float               [2m(687.93µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_null                          [2m(467.87µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-iso-8601-year-2_digit           [2m(435.60µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_replace_field                   [2m(499.94µs)[0m
  [[32mPASS[0m] json_arrow_shift_json_null_type          [2m(464.62µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-negative-date-offset           [2m(459.54µs)[0m
  [[32mPASS[0m] strftime-hour-12-hour-clock              [2m(493.15µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-empty-1                       [2m(460.41µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-day-of-year                     [2m(459.74µs)[0m
  [[32mPASS[0m] strftime-julianday                       [2m(487.05µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_typeof_key_array_indices_integer [2m(980.64µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-hour-without-leading-zero-1     [2m(451.64µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_not_json                      [2m(468.33µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-hour-without-leading-zero-2     [2m(444.37µs)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_subtype_strip_array_subquery_value  [2m(3.00ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-left-null-null                     [2m(2.69ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_replace_field_2                 [2m(547.42µs)[0m
  [[32mPASS[0m] json_extract_empty                       [2m(461.79µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-hour-12-hour-clock-without-leading-zero-2 [2m(505.25µs)[0m
  [[32mPASS[0m] date-with-auto-julianday-top             [2m(487.89µs)[0m
  [[32mPASS[0m] strftime-month                           [2m(476.21µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-empty-2                       [2m(441.46µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-minute                          [2m(471.63µs)[0m
  [[32mPASS[0m] strftime-am-pm1                          [2m(459.15µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-or-float-float                   [2m(2.74ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-am-pm-2                         [2m(488.47µs)[0m
  [[32mPASS[0m] strftime-am-pm-lower-1                   [2m(454.30µs)[0m
  [[32mPASS[0m] strftime-am-pm-lower-2                   [2m(441.50µs)[0m
  [[32mPASS[0m] strftime-fractional-seconds              [2m(2.83ms)[0m
  [[32mPASS[0m] strftime-iso8601-time                    [2m(477.17µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] mod-int-null                             [2m(2.72ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_set_deeply_nested_key           [2m(518.79µs)[0m

[1mtests/rollback.sqltest[0m
  [[32mPASS[0m] schema-change-rollback-2-2               [2m(7.02ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_single_param                [2m(472.36µs)[0m
  [[32mPASS[0m] json_each_typeof_key_object_keys_text    [2m(957.38µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-auto-unixepoch-top             [2m(481.11µs)[0m
  [[32mPASS[0m] strftime-hour                            [2m(2.49ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-deep-nested-1                 [2m(492.27µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-seconds-since-epoch             [2m(470.57µs)[0m
  [[32mPASS[0m] strftime-seconds                         [2m(468.33µs)[0m
  [[32mPASS[0m] strftime-with-subsec                     [2m(478.02µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_types_boolean               [2m(607.47µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-iso8601-with-seconds            [2m(459.64µs)[0m
  [[32mPASS[0m] strftime-week-year-start-sunday          [2m(471.32µs)[0m
  [[32mPASS[0m] strftime-with-subsec-math                [2m(548.51µs)[0m
  [[32mPASS[0m] strftime-day-week-start-monday           [2m(439.89µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] divide-agg-int-agg-float-2               [2m(756.64µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_null_invalid_path           [2m(440.73µs)[0m
  [[32mPASS[0m] json_set_add_value_to_empty_array        [2m(500.61µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-auto-julianday-bottom          [2m(465.41µs)[0m
  [[32mPASS[0m] strftime-iso8601-week-year               [2m(506.28µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-mixed-types-1                 [2m(494.17µs)[0m
  [[32mPASS[0m] json_each_parent_column_always_null      [2m(713.24µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-int-int                      [2m(2.63ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-day-week-start-sunday-2         [2m(486.06µs)[0m
  [[32mPASS[0m] strftime-week-year-start-sunday-2        [2m(466.14µs)[0m
  [[32mPASS[0m] strftime-year                            [2m(454.47µs)[0m
  [[32mPASS[0m] strftime-percent                         [2m(440.49µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_types_integer               [2m(554.23µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-S-3f                    [2m(461.69µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_null_invalid_path_2         [2m(506.79µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-auto-unixepoch-bottom          [2m(474.96µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_add_value_to_nonexistent_array  [2m(500.86µs)[0m
  [[32mPASS[0m] json-patch-add-all-dup-keys-from-patch   [2m(455.10µs)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_subtype_strip_insert_direct_value   [2m(2.84ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-C                       [2m(467.69µs)[0m
  [[32mPASS[0m] strftime-invalid-y                       [2m(486.81µs)[0m
  [[32mPASS[0m] strftime-invalid-B                       [2m(444.36µs)[0m
  [[32mPASS[0m] strftime-invalid-b                       [2m(459.74µs)[0m
  [[32mPASS[0m] strftime-invalid-h                       [2m(422.58µs)[0m
  [[32mPASS[0m] strftime-invalid-a                       [2m(458.42µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_types_real                  [2m(544.37µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-not-null                         [2m(2.73ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-complex-expression      [2m(5.44ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_add_value_to_array              [2m(505.79µs)[0m
  [[32mPASS[0m] json-patch-first-occurrence-patch        [2m(474.58µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-unixepoch                      [2m(529.11µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_multiple_nulls              [2m(533.66µs)[0m
  [[32mPASS[0m] json_array_json                          [2m(3.98ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-D                       [2m(479.55µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] mod-float-zero                           [2m(2.77ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-x                       [2m(458.44µs)[0m
  [[32mPASS[0m] strftime-invalid-dotf                    [2m(467.15µs)[0m
  [[32mPASS[0m] strftime-invalid-v                       [2m(492.55µs)[0m
  [[32mPASS[0m] strftime-invalid-dot3f                   [2m(477.28µs)[0m
  [[32mPASS[0m] strftime-invalid-dot6f                   [2m(507.64µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] right-join-basic                         [2m(7.60ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-dot9f                   [2m(489.45µs)[0m
  [[32mPASS[0m] strftime-invalid-3f                      [2m(447.58µs)[0m
  [[32mPASS[0m] date-with-julianday                      [2m(462.18µs)[0m
  [[32mPASS[0m] strftime-invalid-6f                      [2m(460.61µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-complex-nested-dup-keys       [2m(508.53µs)[0m
  [[32mPASS[0m] json_extract_number                      [2m(489.09µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-char-from-float                   [2m(510.90µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-int-neg_int                  [2m(2.74ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_nested                        [2m(569.96µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-exp-decimal-float                 [2m(454.85µs)[0m
  [[32mPASS[0m] format-exp-negative-large                [2m(485.93µs)[0m
  [[32mPASS[0m] format-exp-large-number                  [2m(488.49µs)[0m
  [[32mPASS[0m] format-exp-small-positive                [2m(485.99µs)[0m
  [[32mPASS[0m] format-exp-zero                          [2m(485.94µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-unicode-dup-keys              [2m(492.48µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-9f                      [2m(489.44µs)[0m
  [[32mPASS[0m] time-no-arg                              [2m(496.62µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_number_type                 [2m(512.20µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-between-unique             [2m(10.17ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-day-week-start-sunday           [2m(4.33ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_infinity                      [2m(481.66µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-exp-string-integer                [2m(455.44µs)[0m
  [[32mPASS[0m] format-exp-string-text                   [2m(452.91µs)[0m

[1mtests/multi_index_dml.sqltest[0m
  [[32mPASS[0m] delete-multi-index-compound-or-fk-cascade-chain [2m(16.51ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-exp-string-float                  [2m(472.78µs)[0m
  [[32mPASS[0m] format-hex-simple                        [2m(464.90µs)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_subtype_strip_insert_subquery_value [2m(3.07ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-hex-large-number                  [2m(458.32µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-not-int                          [2m(2.74ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-empty-string-dup-keys         [2m(470.88µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-X                       [2m(470.00µs)[0m
  [[32mPASS[0m] time-current-time                        [2m(491.74µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-hex-uppercase                     [2m(490.76µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] divide-agg-float-agg-int-2               [2m(779.19µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_number                        [2m(459.04µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-hex-negative                      [2m(499.00µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_array_negative_infinity             [2m(498.67µs)[0m

[1mtests/rollback.sqltest[0m
  [[32mPASS[0m] schema-alter-rollback                    [2m(5.83ms)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-application-id-zero               [2m(18.67ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_malformed_json_raises_error    [2m(4.70ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-octal-simple                      [2m(471.00µs)[0m
  [[32mPASS[0m] format-hex-float                         [2m(507.21µs)[0m
  [[32mPASS[0m] format-octal-large-number                [2m(486.36µs)[0m
  [[32mPASS[0m] format-octal-negative                    [2m(491.79µs)[0m
  [[32mPASS[0m] format-mixed-string-integer              [2m(515.09µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] pow-int-int                              [2m(2.88ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-mixed-all-types                   [2m(499.63µs)[0m
  [[32mPASS[0m] format-mixed-complex-format              [2m(537.32µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-int-float                    [2m(2.77ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-mixed-string-exponential          [2m(494.89µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-multiple-types-dup-keys       [2m(508.90µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-specific-time                       [2m(472.95µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-mixed-integer-octal               [2m(516.21µs)[0m
  [[32mPASS[0m] format-mixed-exponential-float           [2m(473.92µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_number_type                   [2m(505.23µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-A                       [2m(4.53ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-mixed-octal-hex                   [2m(432.83µs)[0m
  [[32mPASS[0m] format-equals-printf-string              [2m(521.52µs)[0m
  [[32mPASS[0m] format-equals-printf-integer             [2m(528.52µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_object_member_order_preserved  [2m(683.15µs)[0m
  [[32mPASS[0m] json_replace_types_text                  [2m(592.51µs)[0m
  [[32mPASS[0m] json-patch-deep-nested-dup-keys          [2m(529.82µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-of-datetime                         [2m(471.57µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_shift_number                  [2m(472.35µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-equals-printf-float               [2m(546.11µs)[0m
  [[32mPASS[0m] format-equals-printf-mixed               [2m(554.39µs)[0m
  [[32mPASS[0m] format-equals-printf-hex                 [2m(476.65µs)[0m
  [[32mPASS[0m] format-basic-string                      [2m(448.80µs)[0m
  [[32mPASS[0m] format-string-replacement                [2m(498.99µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-function-call           [2m(5.40ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_basic_1                      [2m(506.93µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-iso8601                             [2m(476.47µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_shift_number_type             [2m(495.00µs)[0m
  [[32mPASS[0m] json_each_json_extract_on_value          [2m(925.65µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-not-float                        [2m(2.77ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-string-substitution               [2m(478.78µs)[0m
  [[32mPASS[0m] format-multiple-strings                  [2m(446.66µs)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-page-size-default                 [2m(2.45ms)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_subtype_strip_double_nested_subquery [2m(3.25ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-null-string                       [2m(467.96µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-patch-abomination                   [2m(982.48µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-integer-as-string                 [2m(489.02µs)[0m
  [[32mPASS[0m] format-percent-escape                    [2m(469.48µs)[0m
  [[32mPASS[0m] format-integer-replacement               [2m(464.24µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-int-text                     [2m(2.87ms)[0m
  [[32mPASS[0m] pow-int-float                            [2m(2.80ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-integer-negative                  [2m(473.34µs)[0m
  [[32mPASS[0m] format-integer-arithmetic-expression     [2m(473.34µs)[0m
  [[32mPASS[0m] format-text-as-integer                   [2m(440.61µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-milliseconds                   [2m(463.90µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_bare_infinity                       [2m(3.18ms)[0m
  [[32mPASS[0m] json_extract_object_1                    [2m(487.86µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-unsigned-basic                    [2m(462.77µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-remove-1                            [2m(460.44µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-unsigned-arithmetic-expression    [2m(479.16µs)[0m
  [[32mPASS[0m] format-unsigned-negative                 [2m(481.58µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-2arg-array-basic               [2m(933.83µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-text-as-unsigned                  [2m(489.12µs)[0m
  [[32mPASS[0m] format-float-basic                       [2m(454.30µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-julian-day-integer                  [2m(457.00µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_bare_negative_infinity              [2m(490.33µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-float-negative                    [2m(493.80µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_object                        [2m(479.43µs)[0m
  [[32mPASS[0m] json-remove-2                            [2m(465.78µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-integer-as-float                  [2m(465.53µs)[0m
  [[32mPASS[0m] format-float-arithmetic                  [2m(491.38µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] right-join-no-matches                    [2m(7.64ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-text-as-float                     [2m(500.74µs)[0m
  [[32mPASS[0m] format-char-single                       [2m(456.06µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-julian-day-float                    [2m(485.38µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-2arg-object-basic              [2m(1.01ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-char-first-from-string            [2m(493.95µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_blob_object                   [2m(510.99µs)[0m
  [[32mPASS[0m] json-remove-3                            [2m(458.48µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-char-empty-string                 [2m(451.83µs)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-page-size-set                     [2m(2.52ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-not-text-float                   [2m(2.71ms)[0m
  [[32mPASS[0m] shift-right-int-text_float               [2m(2.69ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-char-from-integer                 [2m(472.76µs)[0m
  [[32mPASS[0m] format-exp-large-number                  [2m(445.86µs)[0m
  [[32mPASS[0m] format-char-from-float                   [2m(490.11µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-invalid-input                       [2m(467.88µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-exp-decimal-float                 [2m(471.80µs)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_subtype_strip_valid_through_subquery [2m(3.01ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-exp-negative-large                [2m(486.85µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] pow-int-str                              [2m(2.76ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_blob_array                    [2m(468.07µs)[0m
  [[32mPASS[0m] json-remove-4                            [2m(500.99µs)[0m
  [[32mPASS[0m] json_remove_basic_2                      [2m(3.94ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-exp-small-positive                [2m(488.93µs)[0m
  [[32mPASS[0m] format-exp-zero                          [2m(458.59µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-between-unique-conflict    [2m(8.40ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-2arg-root-dollar-array         [2m(811.64µs)[0m

[1mtests/rollback.sqltest[0m
  [[32mPASS[0m] schema-alter-rollback-and-repeat         [2m(6.64ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-exp-string-integer                [2m(498.93µs)[0m
  [[32mPASS[0m] format-exp-string-text                   [2m(451.54µs)[0m
  [[32mPASS[0m] format-exp-string-float                  [2m(501.70µs)[0m
  [[32mPASS[0m] format-hex-simple                        [2m(486.24µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-null-input                          [2m(436.99µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-hex-large-number                  [2m(474.97µs)[0m
  [[32mPASS[0m] format-hex-uppercase                     [2m(487.48µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_blob_array_2                  [2m(422.20µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-octal-float                       [2m(8.18ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-remove-5                            [2m(501.95µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-hex-negative                      [2m(478.05µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_multiple_paths               [2m(509.14µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-hex-float                         [2m(453.96µs)[0m
  [[32mPASS[0m] format-octal-simple                      [2m(424.50µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-out-of-range                        [2m(466.08µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-mixed-expressions       [2m(5.64ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_blob_number                   [2m(460.68µs)[0m
  [[32mPASS[0m] json-remove-6                            [2m(476.99µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-int-text_int                 [2m(2.75ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-octal-negative                    [2m(476.23µs)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-max-page-count-default            [2m(2.42ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-octal-float                       [2m(477.53µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-not-text-int-1                   [2m(2.70ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-mixed-string-integer              [2m(451.01µs)[0m
  [[32mPASS[0m] format-mixed-all-types                   [2m(447.35µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-date-only                           [2m(451.03µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-mixed-complex-format              [2m(465.29µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_blob_number_2                 [2m(453.06µs)[0m
  [[32mPASS[0m] json-remove-7                            [2m(467.55µs)[0m
  [[32mPASS[0m] json-each-2arg-start-at-primitive        [2m(772.91µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-mixed-integer-octal               [2m(467.95µs)[0m
  [[32mPASS[0m] format-mixed-string-exponential          [2m(618.58µs)[0m

[1mtests/multi_index_dml.sqltest[0m
  [[32mPASS[0m] update-multi-index-compound-or-null-residuals [2m(10.48ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] pow-float-int                            [2m(2.79ms)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_subtype_strip_patch_subquery_patch  [2m(3.12ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-mixed-exponential-float           [2m(473.41µs)[0m
  [[32mPASS[0m] format-mixed-octal-hex                   [2m(437.05µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-timezone-utc                   [2m(441.84µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-equals-printf-string              [2m(540.29µs)[0m
  [[32mPASS[0m] format-equals-printf-integer             [2m(530.87µs)[0m
  [[32mPASS[0m] format-equals-printf-float               [2m(535.15µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_blob_number_3                 [2m(435.28µs)[0m
  [[32mPASS[0m] json-remove-8                            [2m(474.24µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-equals-printf-mixed               [2m(566.54µs)[0m
  [[32mPASS[0m] format-equals-printf-hex                 [2m(513.58µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-2arg-start-at-object-inside-array [2m(746.38µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-timezone-positive              [2m(453.86µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_malformed_json_1            [2m(6.29ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-octal-large-number                [2m(3.51ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-int-null                     [2m(2.64ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_array_element                [2m(501.25µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] bitwise-and-int-agg-int                  [2m(4.36ms)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-max-page-count-set-large          [2m(2.88ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-not-text-int-2                   [2m(2.57ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-string-replacement                [2m(1.98ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-remove-9                            [2m(386.09µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] right-join-all-match                     [2m(7.44ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] pow-float-float                          [2m(2.78ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-basic-string                      [2m(2.78ms)[0m

[1mtests/rollback.sqltest[0m
  [[32mPASS[0m] schema-create-index-rollback             [2m(5.87ms)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_subtype_strip_patch_subquery_doc    [2m(2.99ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-multiple-strings                  [2m(2.73ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_blob_negative_number          [2m(469.95µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-null-string                       [2m(2.83ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-int-int-overflow             [2m(2.73ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-2arg-nonexistent-path-returns-no-rows [2m(673.31µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-timezone-negative              [2m(478.64µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-multiple-rows           [2m(5.66ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-integer-arithmetic                [2m(1.75ms)[0m
  [[32mPASS[0m] printf-integer-basic                     [2m(2.51ms)[0m
  [[32mPASS[0m] printf-integer-as-string                 [2m(2.90ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_type                            [2m(371.45µs)[0m
  [[32mPASS[0m] json_remove_array_negative_index         [2m(510.59µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-between-upsert             [2m(8.88ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-text-as-integer                   [2m(2.90ms)[0m
  [[32mPASS[0m] printf-float-as-integer                  [2m(2.81ms)[0m
  [[32mPASS[0m] printf-i-specifier                       [2m(2.79ms)[0m
  [[32mPASS[0m] printf-unsigned-basic                    [2m(2.76ms)[0m
  [[32mPASS[0m] printf-unsigned-negative                 [2m(2.74ms)[0m
  [[32mPASS[0m] printf-text-as-float                     [2m(1.74ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-r                       [2m(16.99ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-int-int-underflow            [2m(2.73ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_shift_blob                    [2m(464.92µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-unsigned-text                     [2m(2.86ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_postive-expo                    [2m(351.63µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-float-basic                       [2m(2.83ms)[0m
  [[32mPASS[0m] printf-float-negative                    [2m(2.78ms)[0m
  [[32mPASS[0m] printf-integer-as-float                  [2m(2.83ms)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-max-page-count-set-zero-ignored   [2m(2.79ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-timezone-day-change-positive   [2m(460.07µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-percent-escape                    [2m(5.85ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-2arg-empty-array               [2m(691.87µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-integer-negative                  [2m(5.83ms)[0m
  [[32mPASS[0m] printf-float-precision-2                 [2m(2.86ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] bitwise-and-int-agg-float                [2m(719.59µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_array_multiple_elements      [2m(512.45µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] pow-float-str                            [2m(2.83ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-float-width                       [2m(1.76ms)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_subtype_strip_direct_call           [2m(2.82ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_negative-expo                   [2m(357.82µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-int-float-overflow           [2m(2.58ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-float-precision-0                 [2m(2.71ms)[0m
  [[32mPASS[0m] printf-float-precision-10                [2m(2.75ms)[0m
  [[32mPASS[0m] printf-float-width-precision             [2m(2.50ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-Z                       [2m(463.72µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_object_2                    [2m(535.44µs)[0m

[1mtests/multi_index_dml.sqltest[0m
  [[32mPASS[0m] delete-multi-index-compound-or-not-residual [2m(10.04ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-float-zero-pad                    [2m(2.77ms)[0m
  [[32mPASS[0m] printf-float-left-justify                [2m(2.89ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-timezone-day-change-negative   [2m(468.56µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-int-width                         [2m(1.80ms)[0m
  [[32mPASS[0m] printf-float-force-sign-negative         [2m(2.86ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_notation                        [2m(387.18µs)[0m
  [[32mPASS[0m] json-each-2arg-empty-object              [2m(694.27µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-float-space-sign                  [2m(2.82ms)[0m
  [[32mPASS[0m] printf-float-space-sign-negative         [2m(2.62ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_array_out_of_bounds          [2m(460.34µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-int-float-underflow          [2m(2.69ms)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-integrity-check-empty             [2m(2.46ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-int-zero-pad                      [2m(2.66ms)[0m
  [[32mPASS[0m] printf-int-left-justify                  [2m(2.64ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-multiple-rows-expressions [2m(5.64ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-z                       [2m(469.77µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_object_3                    [2m(513.19µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-int-force-sign                    [2m(3.09ms)[0m
  [[32mPASS[0m] printf-int-precision                     [2m(2.12ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-basic                    [2m(7.87ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-int-space-sign                    [2m(2.82ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-not-scalar-float                 [2m(5.90ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-timezone-iso8601               [2m(479.70µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-int-precision-zero-value          [2m(2.78ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] pow-str-str                              [2m(2.85ms)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_subtype_strip_from_subquery         [2m(2.92ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-float-int                    [2m(2.57ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-int-width-and-precision           [2m(2.71ms)[0m
  [[32mPASS[0m] printf-string-width                      [2m(2.63ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_nested_object                [2m(535.56µs)[0m
  [[32mPASS[0m] json-each-2arg-bools-and-null            [2m(1.02ms)[0m

[1mtests/rollback.sqltest[0m
  [[32mPASS[0m] schema-drop-table-rollback               [2m(9.49ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-string-precision-truncate         [2m(2.56ms)[0m
  [[32mPASS[0m] printf-string-left-justify               [2m(2.87ms)[0m
  [[32mPASS[0m] printf-string-width-and-precision        [2m(2.53ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-colonz                  [2m(459.65µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_with_escaping               [2m(463.02µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-dynamic-width                     [2m(3.02ms)[0m
  [[32mPASS[0m] printf-dynamic-precision                 [2m(2.78ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_malformed_json_2            [2m(10.28ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-hex-basic                         [2m(2.73ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-float-neg_int                [2m(2.73ms)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-cache-spill-off                   [2m(2.41ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-dynamic-width-and-precision       [2m(2.99ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-timezone-and-milliseconds      [2m(458.11µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-hex-upper                         [2m(2.65ms)[0m
  [[32mPASS[0m] printf-hex-width-zero-pad                [2m(2.89ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_nested_array                 [2m(517.46µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-hex-negative                      [2m(2.70ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-2arg-primitive-key-null        [2m(774.16µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-pointer-basic                     [2m(2.65ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-doublecolonz            [2m(455.96µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_blob_value                      [2m(5.84ms)[0m
  [[32mPASS[0m] json_extract_with_escaping_2             [2m(477.04µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-dynamic-width-negative            [2m(5.95ms)[0m
  [[32mPASS[0m] printf-hex-precision                     [2m(2.48ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-float-float                  [2m(2.51ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-octal-basic                       [2m(2.77ms)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_subtype_strip_cte                   [2m(2.99ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-between-bulk               [2m(15.05ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-invalid-timezone               [2m(462.19µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-octal-alternate                   [2m(2.80ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-null-values             [2m(5.41ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] pow-null-int                             [2m(3.85ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-hex-alternate                     [2m(5.84ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_deep_nesting                 [2m(520.70µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-octal-width                       [2m(2.79ms)[0m
  [[32mPASS[0m] printf-exp-basic                         [2m(2.68ms)[0m
  [[32mPASS[0m] printf-exp-uppercase                     [2m(2.70ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-triplecolonz            [2m(439.77µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-float-text                   [2m(2.70ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-2arg-negative-index-root       [2m(755.64µs)[0m
  [[32mPASS[0m] json_set_field_empty_object              [2m(485.20µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-float-force-sign                  [2m(14.54ms)[0m

[1mtests/pragma/memory.sqltest[0m
  [[32mPASS[0m] pragma-page-size-set-uninitialized-db    [2m(3.21ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-exp-zero                          [2m(2.77ms)[0m
  [[32mPASS[0m] printf-pointer-zero                      [2m(5.94ms)[0m
  [[32mPASS[0m] printf-exp-small                         [2m(2.75ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] bitwise-and-int-agg-int-agg              [2m(748.41µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-modifier-start-of-day          [2m(459.89µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-g-basic                           [2m(2.83ms)[0m
  [[32mPASS[0m] printf-g-small                           [2m(2.78ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-no-matches               [2m(7.42ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-g-very-small                      [2m(2.71ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_empty_object                 [2m(454.14µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-g-large                           [2m(2.55ms)[0m
  [[32mPASS[0m] printf-exp-negative                      [2m(4.19ms)[0m
  [[32mPASS[0m] printf-g-precision-2                     [2m(2.64ms)[0m
  [[32mPASS[0m] printf-g-trailing-zeros                  [2m(2.64ms)[0m
  [[32mPASS[0m] printf-exp-precision                     [2m(5.69ms)[0m
  [[32mPASS[0m] printf-g-precision-3                     [2m(2.83ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-hashz                   [2m(436.77µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-g-trailing-zeros-2                [2m(2.49ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-not-scalar-int                   [2m(5.84ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_null_path                   [2m(455.85µs)[0m
  [[32mPASS[0m] json_set_replace_field                   [2m(473.48µs)[0m

[1mtests/multi_index_intersection.sqltest[0m
  [[32mPASS[0m] multi-index-and-2-way                    [2m(12.63ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-no-arguments                   [2m(571.24µs)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_subtype_strip_union_all             [2m(3.04ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-g-integer                         [2m(2.72ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] pow-int-null                             [2m(2.98ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-g-zero                            [2m(2.83ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-modifier-add-hours             [2m(495.31µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-char-from-string                  [2m(2.81ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-float-text_float             [2m(2.60ms)[0m

[1mtests/pragma/require_where.sqltest[0m
  [[32mPASS[0m] pragma-require-where-alias               [2m(2.40ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-char-from-integer-single-digit    [2m(2.53ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_empty_array                  [2m(487.06µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-c                       [2m(432.03µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-q-null                            [2m(2.69ms)[0m
  [[32mPASS[0m] printf-Q-basic                           [2m(2.77ms)[0m
  [[32mPASS[0m] printf-char-from-integer                 [2m(3.80ms)[0m
  [[32mPASS[0m] printf-Q-with-quotes                     [2m(2.75ms)[0m

[1mtests/row-value-in.sqltest[0m
  [[32mPASS[0m] row-value-in-literal                     [2m(7.83ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-G-basic                           [2m(5.73ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-null-expression         [2m(5.69ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-join-table-column-order-by-issue-5904 [2m(1.14ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-Q-null                            [2m(2.93ms)[0m
  [[32mPASS[0m] printf-w-basic                           [2m(2.77ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] bitwise-or-int-agg                       [2m(748.41µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_malformed_json_3            [2m(7.22ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-comma-integer                     [2m(2.43ms)[0m

[1mtests/offset/default.sqltest[0m
  [[32mPASS[0m] select-offset-0-groupby                  [2m(176.32ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-w-with-double-quotes              [2m(2.69ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-modifier-add-minutes           [2m(479.71µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-comma-negative                    [2m(2.59ms)[0m
  [[32mPASS[0m] printf-comma-float                       [2m(2.51ms)[0m
  [[32mPASS[0m] printf-q-basic                           [2m(5.65ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_quoted_key                   [2m(517.70µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-plus                    [2m(438.66µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-char-width                        [2m(5.97ms)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_subtype_strip_scalar_subquery       [2m(2.84ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_null_path                     [2m(425.82µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-alt-float-precision-0             [2m(2.89ms)[0m
  [[32mPASS[0m] printf-mixed-basic                       [2m(2.86ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] power-int-int                            [2m(2.73ms)[0m
  [[32mPASS[0m] shift-right-float-text_int               [2m(2.35ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-mixed-hex-pad                     [2m(2.62ms)[0m
  [[32mPASS[0m] printf-format-integer                    [2m(2.65ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-modifier-add-seconds           [2m(521.46µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-join-table-column-two-arg-issue-5904 [2m(1.11ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-format-null                       [2m(2.53ms)[0m
  [[32mPASS[0m] printf-format-float                      [2m(2.81ms)[0m
  [[32mPASS[0m] printf-empty-format-null                 [2m(2.68ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-t                       [2m(435.40µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_all_properties               [2m(509.43µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-all-match                [2m(7.66ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] format-alias                             [2m(2.52ms)[0m
  [[32mPASS[0m] printf-format-blob                       [2m(2.72ms)[0m
  [[32mPASS[0m] printf-mixed-all-types                   [2m(5.73ms)[0m
  [[32mPASS[0m] printf-mixed-complex                     [2m(5.92ms)[0m
  [[32mPASS[0m] printf-empty-format                      [2m(2.73ms)[0m
  [[32mPASS[0m] printf-only-percent-escapes              [2m(2.73ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] bitwise-and-int-agg-int-agg-2            [2m(707.77µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-modifier-subtract-hours        [2m(495.31µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-consecutive-specifiers            [2m(2.54ms)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_quote_json_typed_arg                [2m(2.71ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-n                       [2m(430.57µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-text-numeric-prefix               [2m(2.74ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-not-agg-int                      [2m(5.91ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-text-as-float-coerce              [2m(2.85ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-join-complex-expression-where-issue-5904 [2m(1.16ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-float-null                   [2m(2.51ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-rowid                   [2m(5.42ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_all_array_elements           [2m(572.25µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] power-int-float                          [2m(2.83ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-altform2-float-trailing           [2m(2.73ms)[0m
  [[32mPASS[0m] printf-altform2-float-zero               [2m(2.72ms)[0m
  [[32mPASS[0m] printf-altform2-float-precision-4        [2m(2.52ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-modifier-subtract-minutes      [2m(461.20µs)[0m
  [[32mPASS[0m] strftime-invalid-dashquestion            [2m(428.03µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-altform2-float-negative           [2m(2.87ms)[0m
  [[32mPASS[0m] printf-altform2-float-width              [2m(2.79ms)[0m
  [[32mPASS[0m] printf-altform2-exp-basic                [2m(2.82ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-between-update-multi       [2m(12.23ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-altform2-exp-precision-0          [2m(2.79ms)[0m

[1mtests/multi_index_intersection.sqltest[0m
  [[32mPASS[0m] multi-index-and-3-way                    [2m(12.54ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_root                         [2m(455.24µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-altform2-float-sign               [2m(3.56ms)[0m
  [[32mPASS[0m] printf-altform2-float-basic              [2m(5.90ms)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_quote_plain_string_arg              [2m(2.75ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-altform2-exp-precision-2          [2m(2.53ms)[0m
  [[32mPASS[0m] printf-altform2-exp-zero                 [2m(2.77ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-underscorequestion      [2m(444.85µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-altform2-float-precision-0        [2m(5.95ms)[0m
  [[32mPASS[0m] printf-comma-small                       [2m(14.37ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-modifier-subtract-seconds      [2m(452.87µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] power-int-str                            [2m(2.74ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-altform2-g-small-exp              [2m(2.61ms)[0m
  [[32mPASS[0m] printf-altform2-g-zero                   [2m(2.78ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] jsonb_remove_root                        [2m(480.69µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-altform2-g-integer                [2m(3.50ms)[0m
  [[32mPASS[0m] printf-altform2-g-has-decimal            [2m(2.72ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-duplicates               [2m(7.95ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-rowid-expression        [2m(4.90ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-q-with-quotes                     [2m(20.28ms)[0m
  [[32mPASS[0m] printf-altform2-g-negative               [2m(2.52ms)[0m
  [[32mPASS[0m] printf-float-large                       [2m(2.78ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] sin-products-id                          [2m(3.79ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-invalid-zeroquestion            [2m(432.25µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_malformed_json_4            [2m(10.28ms)[0m
  [[32mPASS[0m] json_set_replace_field_2                 [2m(584.54µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-float-neg-zero                    [2m(2.78ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-multiple-modifiers             [2m(455.40µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-float-int-overflow           [2m(5.52ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-trailing-percent                  [2m(2.75ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_complex_1                    [2m(568.09µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-float-infinity                    [2m(2.56ms)[0m
  [[32mPASS[0m] printf-float-left-zero-combined          [2m(2.90ms)[0m

[1mtests/pragma/require_where.sqltest[0m
  [[32mPASS[0m] pragma-require-where-delete-with-where-ok [2m(18.40ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-exp-infinity                      [2m(2.65ms)[0m

[1mtests/row-value-in.sqltest[0m
  [[32mPASS[0m] row-value-in-literal-no-match            [2m(5.65ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-empty-result-not-null-constraint  [2m(17.46ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-not-agg-and-agg                  [2m(6.00ms)[0m
  [[32mPASS[0m] power-float-int                          [2m(2.77ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-round-half-away-1                 [2m(2.75ms)[0m
  [[32mPASS[0m] printf-round-half-away-0                 [2m(3.01ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-julianday-2                     [2m(489.02µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_shift_null_path               [2m(435.55µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-invalid-modifier               [2m(480.99µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_set_deeply_nested_key           [2m(537.58µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-round-half-away-neg               [2m(2.77ms)[0m

[1mtests/join/memory.sqltest[0m
  [[32mPASS[0m] test-spill-hash-join-rows                [2m(448.71ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_complex_1                   [2m(653.73µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-round-half-away-1dp               [2m(2.53ms)[0m
  [[32mPASS[0m] printf-round-half-away-2dp               [2m(2.57ms)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_quote_view_boundary_consistency     [2m(6.49ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] trim                                     [2m(581.19µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-float-neg-infinity                [2m(5.83ms)[0m
  [[32mPASS[0m] printf-alt-hex-zeropad-8                 [2m(2.78ms)[0m
  [[32mPASS[0m] printf-alt-hex-zeropad-4                 [2m(2.65ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] trim-number                              [2m(463.77µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_after_remove                [2m(529.27µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-g-infinity                        [2m(5.96ms)[0m
  [[32mPASS[0m] printf-round-half-away-2                 [2m(5.58ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] timediff-basic-positive                  [2m(484.64µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-alt-octal-zeropad                 [2m(2.79ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] trim-null                                [2m(458.13µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_null                        [2m(452.34µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] mod-products-id                          [2m(748.36µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-alt-hex-space-pad                 [2m(3.00ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] trim-leading-whitespace                  [2m(462.22µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-invalid-modifier-2             [2m(462.58µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-round-half-away-exp               [2m(3.98ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] trim-pattern                             [2m(439.12µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_after_replace                [2m(526.43µs)[0m
  [[32mPASS[0m] json_each_3_arguments                    [2m(12.94ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-with-where              [2m(5.22ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_add_value_to_empty_array        [2m(522.68µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] timediff-basic-negative                  [2m(463.86µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] trim-pattern-number                      [2m(468.09µs)[0m
  [[32mPASS[0m] trim-pattern-null                        [2m(471.47µs)[0m
  [[32mPASS[0m] trim-pattern-null-2                      [2m(480.86µs)[0m
  [[32mPASS[0m] trim-no-match-pattern                    [2m(453.17µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-alt-exp-precision-0               [2m(2.86ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] ltrim                                    [2m(472.40µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] mod-products-price-id                    [2m(691.32µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] ltrim-number                             [2m(447.43µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-auto                           [2m(443.28µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_idempotence                 [2m(503.41µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-not-between                [2m(9.99ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] timediff-with-milliseconds-positive      [2m(475.09µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_add_value_to_nonexistent_array  [2m(533.76µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] ltrim-leading-whitespace                 [2m(436.41µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-alt-hex-upper-zeropad             [2m(5.18ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] ltrim-no-leading-whitespace              [2m(463.53µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] power-float-float                        [2m(2.77ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-1arg-root-object-and-children-preorder [2m(1.04ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] ltrim-pattern                            [2m(470.47µs)[0m
  [[32mPASS[0m] ltrim-pattern-number                     [2m(469.91µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-empty-left               [2m(6.78ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] ltrim-pattern-null                       [2m(492.22µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-unixepoch                      [2m(488.82µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] trim-trailing-whitespace                 [2m(2.32ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_float                       [2m(505.19µs)[0m
  [[32mPASS[0m] json_remove_idempotence                  [2m(511.52µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] ltrim-no-match-pattern                   [2m(450.07µs)[0m
  [[32mPASS[0m] rtrim                                    [2m(460.44µs)[0m
  [[32mPASS[0m] rtrim-null                               [2m(421.38µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] timediff-same-time                       [2m(465.53µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] rtrim-number                             [2m(459.46µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_add_value_to_array              [2m(514.04µs)[0m
  [[32mPASS[0m] json_extract_json_null_type              [2m(501.25µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] rtrim-no-trailing-whitespace             [2m(541.92µs)[0m
  [[32mPASS[0m] rtrim-pattern                            [2m(468.01µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-unixepoch-modifiers            [2m(488.95µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] ltrim-null                               [2m(2.02ms)[0m
  [[32mPASS[0m] rtrim-pattern-number                     [2m(473.47µs)[0m
  [[32mPASS[0m] rtrim-pattern-null                       [2m(454.20µs)[0m

[1mtests/pragma/require_where.sqltest[0m
  [[32mPASS[0m] pragma-require-where-update-with-where-ok [2m(5.11ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-1arg-root-array-and-children-preorder [2m(955.33µs)[0m
  [[32mPASS[0m] json_remove_with_extract                 [2m(523.01µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] rtrim-no-match-pattern                   [2m(457.31µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] timediff-across-dates                    [2m(477.09µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_json_null_type                [2m(491.70µs)[0m
  [[32mPASS[0m] json_set_add_value_to_array_out_of_bounds [2m(509.74µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] rtrim-trailing-whitespace                [2m(1.44ms)[0m

[1mtests/multi_index_intersection.sqltest[0m
  [[32mPASS[0m] multi-index-and-4-way                    [2m(12.86ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-alt-g-precision-0                 [2m(2.79ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] trim-integer-returns-text                [2m(503.91µs)[0m
  [[32mPASS[0m] trim-float-returns-text                  [2m(504.97µs)[0m
  [[32mPASS[0m] ltrim-integer-returns-text               [2m(461.21µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-julianday                      [2m(459.56µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] rtrim-integer-returns-text               [2m(473.16µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_float                         [2m(452.14µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] round-float-no-precision                 [2m(471.95µs)[0m
  [[32mPASS[0m] round-float-with-precision               [2m(471.86µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_with_extract                [2m(516.50µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] timediff-across-dates-negative           [2m(465.82µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] round-float-with-text-precision          [2m(480.71µs)[0m
  [[32mPASS[0m] round-text-parsable                      [2m(457.79µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-1arg-primitive-root-null-single-row [2m(723.14µs)[0m
  [[32mPASS[0m] json_set_replace_value_in_array          [2m(513.52µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] round-text-non-parsable                  [2m(471.76µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-julianday-2                    [2m(485.44µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] round-integer-with-precision             [2m(491.56µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-float-int-underflow          [2m(5.57ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_shift_float                   [2m(476.52µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] round-float-negative-precision           [2m(468.74µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-not-agg-int-2                    [2m(4.86ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] round-float-zero-precision               [2m(481.43µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] power-float-str                          [2m(2.84ms)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_quote_view_boundary_where_consistent_with_select [2m(6.31ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] round-null-precision                     [2m(475.60µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] timediff-different-formats               [2m(480.58µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_replace_with_arrow                  [2m(531.69µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] length-text-utf8-chars                   [2m(442.78µs)[0m
  [[32mPASS[0m] length-text                              [2m(518.98µs)[0m
  [[32mPASS[0m] length-integer                           [2m(458.35µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_null_path                       [2m(510.01µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] length-float                             [2m(474.11µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-1arg-primitive-root-true-single-row [2m(732.05µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] length-null                              [2m(457.12µs)[0m
  [[32mPASS[0m] length-empty-text                        [2m(451.49µs)[0m
  [[32mPASS[0m] octet-length-text                        [2m(457.21µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-julianday-modifiers            [2m(509.05µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-no-matches              [2m(5.12ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_true                        [2m(464.84µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] octet-length-text-utf8-chars             [2m(465.25µs)[0m
  [[32mPASS[0m] octet-length-integer                     [2m(434.21µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] timediff-with-timezone                   [2m(452.90µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] octet-length-float                       [2m(445.74µs)[0m
  [[32mPASS[0m] octet-length-empty-text                  [2m(429.78µs)[0m
  [[32mPASS[0m] octet-length-null                        [2m(460.90µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_remove_with_arrow                   [2m(519.72µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-alt-g-trailing                    [2m(2.78ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_multiple_keys                   [2m(549.38µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] remainder-float-agg                      [2m(3.72ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-at-start                       [2m(451.88µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] octet-length-date-binary-expr            [2m(510.28µs)[0m
  [[32mPASS[0m] min-number                               [2m(480.51µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-1arg-primitive-root-false-single-row [2m(740.68µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] trim-integer-with-pattern-returns-text   [2m(3.52ms)[0m
  [[32mPASS[0m] min-str                                  [2m(474.97µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_true_type                   [2m(748.46µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] min-blob-number                          [2m(468.05µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] timediff-large-difference                [2m(469.21µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] max-str-number                           [2m(464.86µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_shift_json_null_type          [2m(492.87µs)[0m
  [[32mPASS[0m] json_set_add_array_in_nested_object      [2m(538.91µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] max-blob-number                          [2m(536.63µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-at-1-second-before-epochtime   [2m(457.95µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] min-null                                 [2m(466.27µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_arrays_heterogeneous_primitives [2m(1.07ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] min-with-null                            [2m(446.71µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_true                          [2m(437.32µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] max-number                               [2m(471.43µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-1arg-primitive-root-integer-single-row [2m(745.94µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] max-str                                  [2m(462.23µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] timediff-with-seconds-precision          [2m(463.39µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] power-str-str                            [2m(2.75ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_empty                       [2m(431.13µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] min-int-float                            [2m(438.03µs)[0m
  [[32mPASS[0m] max-int-float                            [2m(446.11µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_add_array_in_array_in_nested_object [2m(516.88µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] min-float-int                            [2m(465.95µs)[0m
  [[32mPASS[0m] max-null                                 [2m(453.57µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-at-future                      [2m(469.94µs)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_subtype_strip_unary_plus_in_subquery [2m(2.93ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] max-with-null                            [2m(453.72µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_true_type                     [2m(485.66µs)[0m
  [[32mPASS[0m] json_each_arrays_parent_is_always_null   [2m(717.35µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] nullif-2                                 [2m(473.19µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] timediff-null-first-arg                  [2m(453.32µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] nullif-3                                 [2m(454.38µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-alt-g-precision-1                 [2m(2.82ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] nullif-complex-first-arg                 [2m(491.00µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-1arg-primitive-root-real-single-row [2m(737.03µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] substr-3-args                            [2m(467.49µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-empty-right              [2m(7.19ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] substr-3-args-exceed-length              [2m(489.73µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-at-start-of-time               [2m(474.13µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] substr-3-args-start-exceed-length        [2m(493.63µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_set_add_array_in_array_in_nested_object_out_of_bounds [2m(569.54µs)[0m

[1mtests/row-value-in.sqltest[0m
  [[32mPASS[0m] row-value-in-where-clause                [2m(11.48ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] substr-2-args                            [2m(478.56µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_shift_true                    [2m(432.90µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] timediff-null-second-arg                 [2m(454.16µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] substr-large-float-clamp                 [2m(473.96µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-not-zero                         [2m(4.83ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_arrays_id_uniqueness           [2m(840.76µs)[0m
  [[32mPASS[0m] json-tree-1arg-primitive-root-text-single-row [2m(717.09µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-real-overflow-clamp                 [2m(483.45µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_insert_type                         [2m(532.32µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-real-overflow-clamp-rounded         [2m(462.76µs)[0m
  [[32mPASS[0m] min-str-number                           [2m(4.19ms)[0m
  [[32mPASS[0m] substring-3-args                         [2m(459.95µs)[0m
  [[32mPASS[0m] substr-cases                             [2m(1.47ms)[0m
  [[32mPASS[0m] substring-3-args-exceed-length           [2m(455.94µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-float-float-overflow         [2m(5.73ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] timediff-invalid-first-arg               [2m(450.87µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] substring-3-args-start-exceed-length     [2m(477.94µs)[0m

[1mtests/pragma/require_where.sqltest[0m
  [[32mPASS[0m] pragma-require-where-delete-without-where [2m(7.69ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] power-null-int                           [2m(2.78ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] substring-2-args                         [2m(475.54µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_arrays_empty_container_yields_zero_rows [2m(639.05µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] substring-2-args-exceed-length           [2m(468.00µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_insert_postive-expo                 [2m(509.03µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] substring-with-blob                      [2m(474.66µs)[0m
  [[32mPASS[0m] nullif                                   [2m(3.49ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-string-concat           [2m(5.56ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-atom-null-for-containers       [2m(849.66µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-g-threshold-round-up              [2m(2.81ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] substring-with-blob-size                 [2m(476.81µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] timediff-invalid-second-arg              [2m(453.38µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-between-null               [2m(10.48ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] substring-with-numbers                   [2m(485.63µs)[0m
  [[32mPASS[0m] substr-utf8                              [2m(2.64ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_single_param                [2m(465.37µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] substring-with-numbers-2                 [2m(469.69µs)[0m
  [[32mPASS[0m] substring-with-blob-sanity               [2m(789.50µs)[0m
  [[32mPASS[0m] substring-emoji-1                        [2m(473.15µs)[0m
  [[32mPASS[0m] substring-emoji-2                        [2m(501.21µs)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_subtype_strip_materialized_cte      [2m(3.55ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_insert_negative-expo                [2m(511.62µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-at-millisecond-precision-input-produces-seconds-precision-output [2m(483.50µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] substring-utf8-emoji-1                   [2m(486.41µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] timediff-julian-day                      [2m(471.44µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] substring-utf8-emoji-2                   [2m(468.13µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_objects_simple_integer_values  [2m(890.89µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] substring-utf8-emoji-3                   [2m(493.73µs)[0m
  [[32mPASS[0m] substr-null-length-returns-null          [2m(521.56µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-value-minified-for-containers  [2m(898.65µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] substr-extreme-negative-start-and-length [2m(3.64ms)[0m
  [[32mPASS[0m] substring-emoji-3                        [2m(1.42ms)[0m
  [[32mPASS[0m] typeof-null                              [2m(475.43µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-modifier-start-of-day     [2m(450.38µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] typeof-null-case                         [2m(458.63µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_insert_notation                     [2m(517.62µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] typeof-text                              [2m(447.90µs)[0m
  [[32mPASS[0m] substr-embedded-nul                      [2m(1.20ms)[0m
  [[32mPASS[0m] typeof-text-empty                        [2m(470.02µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_objects_nested_containers_value_is_valid_json [2m(889.81µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-modifier-start-of-month   [2m(463.35µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] power-int-null                           [2m(2.77ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-key-types-by-parent-kind       [2m(941.52µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] timediff-different-time-formats          [2m(463.52µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-g-threshold-1sig                  [2m(2.84ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_insert_new_key_in_nested_object     [2m(527.72µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] datetime-numeric-string-whitespace       [2m(5.09ms)[0m
  [[32mPASS[0m] typeof-integer                           [2m(440.03µs)[0m
  [[32mPASS[0m] typeof-real                              [2m(434.48µs)[0m
  [[32mPASS[0m] typeof-blob                              [2m(451.92µs)[0m
  [[32mPASS[0m] typeof-blob-empty                        [2m(445.98µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-not-empty-blob                   [2m(4.69ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_shift_true_type               [2m(470.03µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] typeof-sum-real                          [2m(753.17µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-modifier-start-of-year    [2m(470.85µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_objects_empty_container_yields_zero_rows [2m(637.74µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unicode-a                                [2m(427.73µs)[0m
  [[32mPASS[0m] unicode-emoji                            [2m(460.00µs)[0m
  [[32mPASS[0m] unistr-case-insensitive-hex              [2m(485.59µs)[0m
  [[32mPASS[0m] typeof-group_concat                      [2m(757.66µs)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_subtype_strip_not_materialized_cte  [2m(3.03ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-multiple-escapes                  [2m(463.51µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_null_invalid_path           [2m(445.48µs)[0m

[1mtests/multi_index_intersection.sqltest[0m
  [[32mPASS[0m] multi-index-and-3-way-no-match           [2m(12.44ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_false                       [2m(469.36µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-mixed                             [2m(475.23µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-modifier-add-months       [2m(474.83µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-no-escapes                        [2m(449.31µs)[0m
  [[32mPASS[0m] unistr-empty-string                      [2m(473.01µs)[0m
  [[32mPASS[0m] unistr-null-input                        [2m(455.61µs)[0m
  [[32mPASS[0m] unistr-nul-codepoint                     [2m(454.07µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-parent-links-self-join         [2m(1.70ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unicode-empty                            [2m(462.04µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_objects_keys_require_quoting_in_json_path [2m(866.36µs)[0m
  [[32mPASS[0m] json_extract_null_invalid_path_2         [2m(496.86µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-float-float-underflow        [2m(5.85ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-max-valid                         [2m(505.32µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-g-threshold-2sig                  [2m(2.88ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-substring               [2m(5.45ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] log-int                                  [2m(2.76ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unicode-number                           [2m(461.86µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-count                    [2m(7.43ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-quote-plain-text                  [2m(438.96µs)[0m

[1mtests/pragma/require_where.sqltest[0m
  [[32mPASS[0m] pragma-require-where-update-without-where [2m(7.47ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-quote-null                        [2m(464.97µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_false_type                  [2m(3.48ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-string-zeropad-ignored            [2m(2.92ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-quote-integer                     [2m(433.63µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_false                         [2m(434.03µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-quote-float                       [2m(453.68µs)[0m

[1mtests/row-value-in.sqltest[0m
  [[32mPASS[0m] row-value-in-where-no-match              [2m(8.43ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-not-cast-blob                    [2m(5.70ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_false_type                    [2m(459.15µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-quote-with-newline                [2m(513.26µs)[0m
  [[32mPASS[0m] unistr-quote-backslash-no-ctrl           [2m(438.02µs)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_subtype_plain_table_no_subtype      [2m(6.37ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_shift_false                   [2m(432.62µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-char-zeropad-ignored              [2m(2.92ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-between-nested             [2m(10.86ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-quote-backslash-with-ctrl         [2m(1.50ms)[0m
  [[32mPASS[0m] unistr-quote-single-quote-with-ctrl      [2m(1.55ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] order-by-qualified                       [2m(152.12ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-not-blob                         [2m(2.68ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_shift_false_type              [2m(481.80µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-surrogate-rejected                [2m(8.63ms)[0m
  [[32mPASS[0m] unistr-quote-matches-quote-no-ctrl       [2m(540.57µs)[0m
  [[32mPASS[0m] unistr-quote-with-tab                    [2m(3.56ms)[0m
  [[32mPASS[0m] unistr-above-max-rejected                [2m(8.58ms)[0m
  [[32mPASS[0m] unistr-error-invalid-escape              [2m(8.41ms)[0m

[1mtests/pragma/table_list.sqltest[0m
  [[32mPASS[0m] pragma-table-list-single-table           [2m(5.90ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-quote-ignores-control-after-nul   [2m(497.27µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-modifier-subtract-months  [2m(503.08µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-quote-blob                        [2m(1.13ms)[0m

[1mtests/multi_index_intersection.sqltest[0m
  [[32mPASS[0m] multi-index-and-correlated-exists        [2m(9.00ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-error-non-hex-plus                [2m(8.30ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-parent-null-at-top             [2m(795.03µs)[0m
  [[32mPASS[0m] json_each_top_level_integer_single_row_key_null [2m(776.91µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-with-where               [2m(7.61ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-q-width                           [2m(2.95ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-error-non-hex-capital-U           [2m(8.87ms)[0m
  [[32mPASS[0m] unicode-float                            [2m(505.60µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] bitwise-not-blob-2                       [2m(2.65ms)[0m
  [[32mPASS[0m] log-float                                [2m(2.64ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] func7-pg-180                             [2m(432.46µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-q-left-justify                    [2m(2.91ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] func2-1-2-1                              [2m(5.30ms)[0m
  [[32mPASS[0m] func2-1-2-2                              [2m(5.03ms)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_subtype_view_where_clause_strips_subtype [2m(6.10ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] func9-120                                [2m(4.63ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_string                      [2m(506.36µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] func3-5-8                                [2m(5.38ms)[0m
  [[32mPASS[0m] func7-pg-401                             [2m(514.54µs)[0m
  [[32mPASS[0m] func2-1-2-3                              [2m(5.56ms)[0m
  [[32mPASS[0m] func7-pg-411                             [2m(488.31µs)[0m
  [[32mPASS[0m] func3-5-10                               [2m(5.28ms)[0m
  [[32mPASS[0m] func3-5-9                                [2m(5.47ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-modifier-add-years        [2m(502.29µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] log-str                                  [2m(2.65ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_string_type                 [2m(486.01µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] func9-150                                [2m(4.69ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-q-precision                       [2m(2.88ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] concat-chars                             [2m(469.18µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_top_level_true_single_row_key_null [2m(729.19µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] concat-char-and-number                   [2m(478.26µs)[0m
  [[32mPASS[0m] concat-char-and-decimal                  [2m(488.11µs)[0m
  [[32mPASS[0m] unicode-null                             [2m(471.15µs)[0m
  [[32mPASS[0m] unistr-error-short-hex                   [2m(15.43ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-2arg-start-at-object           [2m(949.31µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-modifier-subtract-years   [2m(473.53µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] concat-char-null-char                    [2m(452.11µs)[0m
  [[32mPASS[0m] unistr-error-non-hex-u                   [2m(15.15ms)[0m
  [[32mPASS[0m] func9-160                                [2m(4.78ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_string                        [2m(450.25µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] concat_ws-numbers                        [2m(490.04µs)[0m
  [[32mPASS[0m] concat_ws-single-number                  [2m(467.49µs)[0m
  [[32mPASS[0m] concat_ws-null                           [2m(462.91µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_top_level_null_single_row_key_null [2m(725.24µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] concat_ws-multiple                       [2m(528.68µs)[0m
  [[32mPASS[0m] unicode-char0                            [2m(489.67µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-default-ceiling                [2m(455.43µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_string_type                   [2m(468.10µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] concat_ws-complex-args                   [2m(532.41µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-2arg-start-at-array            [2m(904.21µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] concat-complex-first-arg                 [2m(547.11µs)[0m
  [[32mPASS[0m] abs                                      [2m(457.82µs)[0m
  [[32mPASS[0m] abs-negative                             [2m(461.88µs)[0m
  [[32mPASS[0m] abs-char                                 [2m(469.78µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-default-ceiling-fractional     [2m(459.80µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] quote-string-embedded-nul                [2m(516.20µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_shift_string                  [2m(452.27µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-Q-width                           [2m(2.89ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] log-negative                             [2m(2.69ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] abs-null                                 [2m(460.05µs)[0m
  [[32mPASS[0m] ifnull-1                                 [2m(478.46µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_atom_equals_value_for_primitives_containers_are_json_text [2m(1.27ms)[0m
  [[32mPASS[0m] json-tree-2arg-start-at-primitive-yields-single-row-and-path-to-self [2m(762.79µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] ifnull-2                                 [2m(465.81µs)[0m
  [[32mPASS[0m] iif-true                                 [2m(462.68µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-floor                          [2m(465.93µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] iif-false                                [2m(478.82µs)[0m
  [[32mPASS[0m] quote-string                             [2m(460.10µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] pi                                       [2m(2.81ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] instr-str                                [2m(451.06µs)[0m
  [[32mPASS[0m] instr-str-not-found                      [2m(451.15µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_shift_string_type             [2m(472.26µs)[0m

[1mtests/pragma/table_list.sqltest[0m
  [[32mPASS[0m] pragma-table-list-equals-syntax          [2m(5.48ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] instr-blob                               [2m(448.72µs)[0m
  [[32mPASS[0m] instr-blob-not-found                     [2m(468.31µs)[0m
  [[32mPASS[0m] instr-null                               [2m(546.57µs)[0m
  [[32mPASS[0m] instr-integer                            [2m(475.83µs)[0m
  [[32mPASS[0m] instr-integer-not-found                  [2m(460.34µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-2arg-nonexistent-path-returns-no-rows [2m(663.07µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] instr-integer-leading-zeros              [2m(488.66µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-floor-keeps-time               [2m(476.55µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_multiple_nulls              [2m(4.00ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] quote-escape                             [2m(466.49µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_typeof_key_array_indices_integer [2m(927.36µs)[0m
  [[32mPASS[0m] json_arrow_implicit_root_path            [2m(448.29µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] instr-real                               [2m(459.72µs)[0m
  [[32mPASS[0m] instr-real-not-found                     [2m(473.29µs)[0m
  [[32mPASS[0m] instr-real-trailing-zeros                [2m(496.21µs)[0m
  [[32mPASS[0m] instr-blob-2                             [2m(496.84µs)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_subtype_case_when_json_patch_view   [2m(6.14ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] instr-blob-not-found-2                   [2m(491.37µs)[0m
  [[32mPASS[0m] upper                                    [2m(481.16µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-ceiling-keeps-time             [2m(472.01µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_number                      [2m(477.45µs)[0m
  [[32mPASS[0m] json-tree-2arg-empty-array               [2m(687.13µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] quote-null                               [2m(464.23µs)[0m
  [[32mPASS[0m] upper-number                             [2m(442.81µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_shift_implicit_root_path      [2m(454.93µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-error-trailing-backslash          [2m(20.80ms)[0m
  [[32mPASS[0m] upper-char                               [2m(461.17µs)[0m
  [[32mPASS[0m] lower                                    [2m(458.26µs)[0m
  [[32mPASS[0m] lower-number                             [2m(482.61µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_typeof_key_object_keys_text    [2m(956.54µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] lower-char                               [2m(464.29µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] log-null                                 [2m(2.76ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-ceiling-floor-2                [2m(463.99µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-qualified-column-in-function [2m(12.96ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] quote-integer                            [2m(460.32µs)[0m
  [[32mPASS[0m] lower-null                               [2m(443.33µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_implicit_root_path_undefined_key [2m(443.66µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-coalesce                 [2m(7.87ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-2arg-empty-object              [2m(668.58µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] replace                                  [2m(450.03µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-Q-left-justify                    [2m(2.93ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] acos-int                                 [2m(2.80ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] replace-number                           [2m(494.26µs)[0m
  [[32mPASS[0m] replace-null-byte-pattern                [2m(495.04µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_parent_column_always_null      [2m(652.79µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] hex                                      [2m(635.81µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-case-expression         [2m(7.02ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] replace-null-byte-prefix                 [2m(679.61µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-ceiling-floor-5                [2m(638.03µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] hex-number                               [2m(454.53µs)[0m
  [[32mPASS[0m] replace-null                             [2m(1.21ms)[0m
  [[32mPASS[0m] likely                                   [2m(447.67µs)[0m
  [[32mPASS[0m] hex-null                                 [2m(473.85µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-2arg-bools-and-null-under-array [2m(898.71µs)[0m

[1mtests/multi_index_intersection.sqltest[0m
  [[32mPASS[0m] multi-index-and-correlated-not-exists    [2m(9.19ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] likely-int                               [2m(448.75µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-ceiling-floor-6                [2m(475.23µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] likely-decimal                           [2m(440.83µs)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_subtype_unary_plus_where_consistent [2m(3.12ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] nullif-3                                 [2m(468.19µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-text-int                     [2m(3.23ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] nullif-complex-first-arg                 [2m(508.64µs)[0m
  [[32mPASS[0m] likely-null                              [2m(717.44µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] log-int-int                              [2m(2.62ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] upper-null                               [2m(3.87ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-ceiling-floor-7                [2m(482.35µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] substr-3-args                            [2m(524.35µs)[0m
  [[32mPASS[0m] substr-3-args-start-exceed-length        [2m(425.64µs)[0m
  [[32mPASS[0m] substr-3-args-exceed-length              [2m(460.24µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_number_type                 [2m(495.40µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] substr-2-args                            [2m(479.50µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] acos-float                               [2m(2.62ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] likelihood-string                        [2m(459.18µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-Q-precision                       [2m(2.85ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-ceiling-floor-8                [2m(477.42µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] substr-large-float-clamp                 [2m(469.84µs)[0m

[1mtests/pragma/table_list.sqltest[0m
  [[32mPASS[0m] pragma-table-list-view                   [2m(6.57ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] substr-extreme-negative-start-and-length [2m(599.28µs)[0m
  [[32mPASS[0m] likelihood-string-high-probability       [2m(465.40µs)[0m
  [[32mPASS[0m] substr-cases                             [2m(1.46ms)[0m
  [[32mPASS[0m] cast-real-overflow-clamp                 [2m(443.16µs)[0m
  [[32mPASS[0m] replace-wrong-arg-count-0                [2m(4.90ms)[0m
  [[32mPASS[0m] replace-wrong-arg-count-2                [2m(4.78ms)[0m
  [[32mPASS[0m] replace-wrong-arg-count-4                [2m(4.80ms)[0m
  [[32mPASS[0m] datetime-numeric-string-whitespace       [2m(1.10ms)[0m
  [[32mPASS[0m] cast-real-overflow-clamp-rounded         [2m(439.18µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_number                        [2m(436.68µs)[0m
  [[32mPASS[0m] json_each_malformed_json_raises_error    [2m(4.28ms)[0m
  [[32mPASS[0m] json_arrow_shift_implicit_root_path_undefined_key [2m(510.86µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] quote-integer-typeof                     [2m(515.45µs)[0m

[1mtests/json/json_subtype_strip.sqltest[0m
  [[32mPASS[0m] json_subtype_iif_preserves_subtype       [2m(2.77ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-modifier-weekday          [2m(465.71µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] likelihood-integer                       [2m(456.07µs)[0m
  [[32mPASS[0m] substring-3-args                         [2m(461.46µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] log-int-float                            [2m(2.62ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] substring-3-args-exceed-length           [2m(483.19µs)[0m
  [[32mPASS[0m] substring-2-args                         [2m(482.97µs)[0m
  [[32mPASS[0m] substring-2-args-exceed-length           [2m(446.64µs)[0m
  [[32mPASS[0m] substring-3-args-start-exceed-length     [2m(498.11µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-text-float                   [2m(2.87ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] substring-with-blob                      [2m(449.24µs)[0m
  [[32mPASS[0m] substring-with-blob-size                 [2m(465.20µs)[0m
  [[32mPASS[0m] substr-utf8                              [2m(2.47ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_number_type                   [2m(472.46µs)[0m
  [[32mPASS[0m] json_arrow_implicit_root_path_array      [2m(459.66µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] quote-negative-integer                   [2m(465.44µs)[0m
  [[32mPASS[0m] substring-with-numbers                   [2m(490.70µs)[0m
  [[32mPASS[0m] likelihood-integer-probability-1         [2m(459.54µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-subsec                    [2m(477.77µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_object_member_order_preserved  [2m(701.74µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] substring-with-blob-sanity               [2m(782.06µs)[0m
  [[32mPASS[0m] substring-emoji-1                        [2m(470.17µs)[0m
  [[32mPASS[0m] substring-emoji-2                        [2m(472.10µs)[0m
  [[32mPASS[0m] substring-emoji-3                        [2m(466.00µs)[0m
  [[32mPASS[0m] substring-utf8-emoji-1                   [2m(453.27µs)[0m
  [[32mPASS[0m] substring-utf8-emoji-2                   [2m(471.30µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_shift_implicit_root_path_array [2m(447.73µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] quote-float                              [2m(483.05µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-w-width                           [2m(2.88ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] likelihood-decimal                       [2m(464.70µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-modifier-add-months-31    [2m(479.40µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] substring-utf8-emoji-3                   [2m(470.48µs)[0m
  [[32mPASS[0m] substr-null-length-returns-null          [2m(531.21µs)[0m
  [[32mPASS[0m] typeof-null                              [2m(448.97µs)[0m
  [[32mPASS[0m] typeof-null-case                         [2m(438.41µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_json_extract_on_value          [2m(976.24µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] remainder-int-agg                        [2m(36.39ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_implicit_root_path_array_negative_idx [2m(445.18µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] quote-float-scientific-format            [2m(511.53µs)[0m
  [[32mPASS[0m] likelihood-null                          [2m(462.19µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-modifier-subtract-months-31 [2m(467.45µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] typeof-text                              [2m(471.70µs)[0m
  [[32mPASS[0m] typeof-text-empty                        [2m(497.94µs)[0m
  [[32mPASS[0m] substr-embedded-nul                      [2m(1.24ms)[0m
  [[32mPASS[0m] typeof-integer                           [2m(442.68µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] acos-str                                 [2m(2.78ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] typeof-real                              [2m(459.52µs)[0m
  [[32mPASS[0m] typeof-blob                              [2m(457.56µs)[0m
  [[32mPASS[0m] typeof-blob-empty                        [2m(454.57µs)[0m
  [[32mPASS[0m] typeof-sum-integer                       [2m(33.28ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_shift_implicit_root_path_array_negative_idx [2m(436.25µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] log-int-str                              [2m(2.65ms)[0m
  [[32mPASS[0m] shift-right-text-text                    [2m(2.92ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-modifier-add-months-large [2m(462.91µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] quote-float-typeof                       [2m(497.07µs)[0m
  [[32mPASS[0m] likelihood-blob                          [2m(488.61µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-2arg-array-basic               [2m(899.78µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-whitespace-now-rejected         [2m(491.64µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] substring-with-numbers-2                 [2m(3.52ms)[0m
  [[32mPASS[0m] unicode-empty                            [2m(447.63µs)[0m
  [[32mPASS[0m] unicode-a                                [2m(489.42µs)[0m
  [[32mPASS[0m] unicode-emoji                            [2m(483.05µs)[0m
  [[32mPASS[0m] typeof-sum-real                          [2m(743.53µs)[0m
  [[32mPASS[0m] unicode-number                           [2m(494.70µs)[0m
  [[32mPASS[0m] unicode-float                            [2m(473.41µs)[0m
  [[32mPASS[0m] unicode-null                             [2m(454.19µs)[0m
  [[32mPASS[0m] typeof-group_concat                      [2m(752.32µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_shift_number                  [2m(448.14µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] likelihood-zero-probability              [2m(468.08µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-modifier-subtract-months-large [2m(520.72µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_implicit_real_cast            [2m(496.29µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sign-positive-integer                    [2m(494.85µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-whitespace-now-rejected-leading [2m(478.57µs)[0m

[1mtests/row-value-in.sqltest[0m
  [[32mPASS[0m] row-value-in-single-element-list         [2m(24.06ms)[0m

[1mtests/pragma/table_list.sqltest[0m
  [[32mPASS[0m] pragma-table-list-strict                 [2m(5.81ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unicode-char0                            [2m(495.35µs)[0m
  [[32mPASS[0m] quote-string-embedded-nul                [2m(504.41µs)[0m
  [[32mPASS[0m] quote-string                             [2m(455.98µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-2arg-object-basic              [2m(964.75µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] quote-escape                             [2m(468.99µs)[0m
  [[32mPASS[0m] quote-null                               [2m(467.86µs)[0m
  [[32mPASS[0m] quote-integer                            [2m(449.37µs)[0m
  [[32mPASS[0m] quote-integer-typeof                     [2m(496.83µs)[0m
  [[32mPASS[0m] quote-negative-integer                   [2m(438.38µs)[0m

[1mtests/multi_index_intersection.sqltest[0m
  [[32mPASS[0m] multi-index-and-correlated-exists-rows   [2m(9.22ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_shift_number_type             [2m(480.31µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-str-ab                             [2m(484.93µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-modifier-february-leap-year [2m(479.26µs)[0m
  [[32mPASS[0m] datetime-whitespace-now-rejected-trailing [2m(456.68µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-w-precision                       [2m(2.80ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_shift_implicit_real_cast      [2m(481.18µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sign-negative-integer                    [2m(480.49µs)[0m
  [[32mPASS[0m] quote-float                              [2m(501.38µs)[0m

[1mtests/json/json_tree.sqltest[0m
  [[32mPASS[0m] json-tree-correlated-subquery-stale-cursor [2m(5.74ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] quote-float-typeof                       [2m(487.57µs)[0m
  [[32mPASS[0m] quote-float-scientific-format            [2m(518.11µs)[0m
  [[32mPASS[0m] sign-positive-integer                    [2m(437.26µs)[0m
  [[32mPASS[0m] sign-positive-float                      [2m(438.71µs)[0m
  [[32mPASS[0m] sign-negative-integer                    [2m(450.13µs)[0m
  [[32mPASS[0m] sign-zero                                [2m(441.68µs)[0m
  [[32mPASS[0m] sign-negative-float                      [2m(462.20µs)[0m
  [[32mPASS[0m] sign-zero-float                          [2m(461.81µs)[0m
  [[32mPASS[0m] sign-text-positive-integer               [2m(439.50µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_object_1                    [2m(465.84µs)[0m
  [[32mPASS[0m] json-each-2arg-root-dollar-array         [2m(803.01µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-int-ab                             [2m(455.22µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-negative-year                   [2m(473.65µs)[0m
  [[32mPASS[0m] unixepoch-with-modifier-february-non-leap-year [2m(473.71µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_implicit_true_cast            [2m(456.57µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sign-zero                                [2m(464.74µs)[0m
  [[32mPASS[0m] sign-text-zero                           [2m(470.56µs)[0m
  [[32mPASS[0m] sign-text-non-numeric                    [2m(436.07µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-doubly-qualified-column-in-function [2m(11.78ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-text_int-text_int            [2m(2.92ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sign-null                                [2m(458.89µs)[0m
  [[32mPASS[0m] randomblob-int-2                         [2m(478.73µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] log-float-int                            [2m(2.87ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] randomblob-int-0                         [2m(480.28µs)[0m
  [[32mPASS[0m] randomblob-int-negative                  [2m(486.89µs)[0m
  [[32mPASS[0m] unhex-dot-uppercase                      [2m(448.14µs)[0m
  [[32mPASS[0m] randomblob-str-2                         [2m(499.08µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-negative-year-with-time         [2m(477.83µs)[0m
  [[32mPASS[0m] unixepoch-with-modifier-date             [2m(463.33µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_shift_implicit_true_cast      [2m(456.55µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] zeroblob-int-0                           [2m(521.71µs)[0m
  [[32mPASS[0m] zeroblob-int-1                           [2m(476.50µs)[0m
  [[32mPASS[0m] sign-positive-float                      [2m(462.25µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-2arg-start-at-primitive        [2m(719.92µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] zeroblob-str-3                           [2m(461.69µs)[0m
  [[32mPASS[0m] cast-text-to-integer                     [2m(453.99µs)[0m
  [[32mPASS[0m] zeroblob-blob                            [2m(482.63µs)[0m
  [[32mPASS[0m] zeroblob-str-a                           [2m(490.94µs)[0m
  [[32mPASS[0m] cast-text-with-spaces-to-integer         [2m(469.96µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-nested-expressions      [2m(12.44ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-text-with-trailing-junk-to-integer  [2m(461.37µs)[0m
  [[32mPASS[0m] unhex-dot-lowercase                      [2m(441.02µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_implicit_false_cast           [2m(460.53µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-modifier-datetime-pos     [2m(475.75µs)[0m
  [[32mPASS[0m] date-negative-year                       [2m(476.39µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sign-negative-float                      [2m(475.17µs)[0m
  [[32mPASS[0m] cast-invalid-text-to-integer             [2m(467.68µs)[0m
  [[32mPASS[0m] cast-text-prefix-to-integer              [2m(465.40µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-q-zeropad-ignored                 [2m(2.80ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-2arg-start-at-object-inside-array [2m(756.88µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-float-to-integer                    [2m(448.58µs)[0m
  [[32mPASS[0m] cast-large-float-to-integer              [2m(464.14µs)[0m
  [[32mPASS[0m] cast-small-float-to-integer              [2m(470.26µs)[0m

[1mtests/json/memory.sqltest[0m
  [[32mPASS[0m] non-string-path                          [2m(2.96ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-no-hex                             [2m(451.55µs)[0m

[1mtests/pragma/table_list.sqltest[0m
  [[32mPASS[0m] pragma-table-list-nonexistent            [2m(4.10ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-large-text-to-integer               [2m(445.03µs)[0m
  [[32mPASS[0m] cast-text-exp-to-integer                 [2m(454.75µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_shift_implicit_false_cast     [2m(446.24µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sign-zero-float                          [2m(460.44µs)[0m
  [[32mPASS[0m] cast-text-with-spaces-to-real            [2m(461.52µs)[0m
  [[32mPASS[0m] cast-invalid-text-to-real                [2m(456.62µs)[0m
  [[32mPASS[0m] sign-text-negative-integer               [2m(3.48ms)[0m
  [[32mPASS[0m] cast-float-to-integer-rounding           [2m(1.34ms)[0m
  [[32mPASS[0m] cast-text-prefix-to-real                 [2m(459.97µs)[0m
  [[32mPASS[0m] cast-integer-to-real                     [2m(411.91µs)[0m
  [[32mPASS[0m] cast-integer-to-text                     [2m(433.93µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-modifier-sub              [2m(481.53µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-text-to-real                        [2m(473.84µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-2arg-nonexistent-path-returns-no-rows [2m(667.48µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-null                               [2m(442.15µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] log-float-float                          [2m(2.75ms)[0m

[1mtests/row-value-in.sqltest[0m
  [[32mPASS[0m] row-value-not-in                         [2m(5.11ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_object                        [2m(454.61µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-date-overflow-feb30             [2m(468.63µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sign-text-positive-integer               [2m(427.82µs)[0m
  [[32mPASS[0m] cast-real-to-text                        [2m(458.25µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-modifier-add              [2m(481.07µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-text_int-text_float          [2m(2.98ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] func7-pg-180                             [2m(478.31µs)[0m
  [[32mPASS[0m] func7-pg-401                             [2m(489.37µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-const-1                           [2m(423.48µs)[0m
  [[32mPASS[0m] select-const-2                           [2m(440.68µs)[0m
  [[32mPASS[0m] select-const-3                           [2m(444.71µs)[0m
  [[32mPASS[0m] select-const-4                           [2m(429.47µs)[0m
  [[32mPASS[0m] select-true                              [2m(405.72µs)[0m
  [[32mPASS[0m] select-false                             [2m(434.49µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-x-y-prefix                         [2m(458.00µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-2arg-empty-array               [2m(655.80µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-date-overflow-feb31             [2m(453.57µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-blob-to-text                        [2m(447.14µs)[0m
  [[32mPASS[0m] sign-text-negative-integer               [2m(466.39µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-multiple-modifiers        [2m(477.77µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-text-escape-1                     [2m(427.35µs)[0m
  [[32mPASS[0m] select-blob-empty                        [2m(415.41µs)[0m
  [[32mPASS[0m] select-blob-ascii                        [2m(427.99µs)[0m
  [[32mPASS[0m] select-blob-emoji                        [2m(430.06µs)[0m
  [[32mPASS[0m] select-limit-0                           [2m(597.01µs)[0m
  [[32mPASS[0m] select-doubly-qualified                  [2m(570.96µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-x-y-suffix                         [2m(453.49µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-Q-zeropad-ignored                 [2m(2.81ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-date-overflow-apr31             [2m(439.85µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-text-to-blob                        [2m(483.65µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-2arg-empty-object              [2m(661.68µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sign-text-zero                           [2m(477.80µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-with-multiple-modifiers-datetime [2m(493.24µs)[0m

[1mtests/json/memory.sqltest[0m
  [[32mPASS[0m] invalid-path                             [2m(2.86ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-limit-expression                  [2m(625.79µs)[0m
  [[32mPASS[0m] select-limit-true                        [2m(666.36µs)[0m
  [[32mPASS[0m] realify                                  [2m(573.65µs)[0m
  [[32mPASS[0m] select-limit-false                       [2m(681.36µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-date-overflow-feb29-non-leap    [2m(435.54µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-subtract                          [2m(763.54µs)[0m
  [[32mPASS[0m] select-add                               [2m(817.30µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-integer-to-blob                     [2m(484.28µs)[0m
  [[32mPASS[0m] sign-text-non-numeric                    [2m(443.28µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] log-float-str                            [2m(2.73ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-modifier-start-of-day          [2m(469.17µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-text_int-text                [2m(2.89ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-2arg-bools-and-null            [2m(880.80µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] case-insensitive-columns                 [2m(747.38µs)[0m
  [[32mPASS[0m] table-star                               [2m(651.44µs)[0m
  [[32mPASS[0m] select_with_quoting                      [2m(688.69µs)[0m
  [[32mPASS[0m] select_with_quoting_2                    [2m(712.65µs)[0m
  [[32mPASS[0m] table-star-2                             [2m(984.96µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-date-overflow-feb30                 [2m(462.32µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-rowid                             [2m(737.07µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_blob_object                   [2m(492.53µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sign-null                                [2m(419.20µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-rowid-2                           [2m(803.95µs)[0m
  [[32mPASS[0m] seekrowid                                [2m(781.27µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-integer-text-to-numeric             [2m(537.33µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-modifier-start-of-month        [2m(451.53µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select_case_noelse_null                  [2m(413.16µs)[0m
  [[32mPASS[0m] select_case_base_else                    [2m(469.02µs)[0m
  [[32mPASS[0m] select_parenthesized                     [2m(626.09µs)[0m
  [[32mPASS[0m] select-doubly-qualified-wrong-table      [2m(3.05ms)[0m

[1mtests/multi_index_intersection.sqltest[0m
  [[32mPASS[0m] multi-index-and-correlated-scalar        [2m(9.41ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-doubly-qualified-wrong-column     [2m(3.11ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] randomblob-int-2                         [2m(448.33µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select_base_case_else                    [2m(497.39µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-2arg-primitive-key-null        [2m(771.44µs)[0m
  [[32mPASS[0m] json_arrow_blob_array                    [2m(457.34µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-fractional-days                 [2m(483.87µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-x-y-prefix-suffix                  [2m(488.39µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select_base_case_noelse_null             [2m(497.72µs)[0m
  [[32mPASS[0m] select_base_case_null_result             [2m(829.29µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-float-text-to-numeric               [2m(530.27µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-like-expression                   [2m(442.80µs)[0m
  [[32mPASS[0m] select-is-null                           [2m(727.07µs)[0m

[1mtests/json/memory.sqltest[0m
  [[32mPASS[0m] json-tree-non-string-path                [2m(2.94ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-modifier-start-of-year         [2m(469.57µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-is-not-null                       [2m(815.75µs)[0m
  [[32mPASS[0m] select-not-like-expression               [2m(464.44µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-x-y-incorrect-suffix               [2m(458.64µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-like-expression-2                 [2m(434.39µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] randomblob-int-0                         [2m(449.49µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select_bin_shl                           [2m(1.16ms)[0m
  [[32mPASS[0m] select_positive_infinite_float           [2m(478.60µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] log-str-str                              [2m(2.76ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-small-float-to-numeric              [2m(526.56µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select_negative_infinite_float           [2m(442.52µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-2arg-negative-index-root       [2m(720.17µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-fractional-hours                [2m(480.64µs)[0m
  [[32mPASS[0m] date-with-modifier-add-months            [2m(469.63µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-multiple-column-references [2m(8.89ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select_shl_large_negative_float          [2m(718.02µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-x-y-long-prefix                    [2m(470.72µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-text_float-text_int          [2m(2.94ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select_shl_basic                         [2m(700.68µs)[0m
  [[32mPASS[0m] select_shl_negative_numbers              [2m(727.54µs)[0m
  [[32mPASS[0m] select_shl_negative_shifts               [2m(749.62µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] randomblob-int-negative                  [2m(442.16µs)[0m

[1mtests/pragma/table_list.sqltest[0m
  [[32mPASS[0m] pragma-table-list-all-user-tables        [2m(8.18ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select_shl_large_shifts                  [2m(668.50µs)[0m
  [[32mPASS[0m] select_shl_chained                       [2m(634.91µs)[0m
  [[32mPASS[0m] select_shl_text_conversion               [2m(786.97µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-inf-zeropad-f                     [2m(2.92ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-invalid-numeric-text              [2m(404.16µs)[0m
  [[32mPASS[0m] select_shl_numeric_types                 [2m(780.52µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-signed-edgecase-int-to-numeric      [2m(534.39µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-invalid-numeric-text-2            [2m(415.40µs)[0m
  [[32mPASS[0m] select_fuzz_failure_case                 [2m(703.68µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-no-arguments                   [2m(554.87µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-default-ceiling                 [2m(472.81µs)[0m
  [[32mPASS[0m] datetime-fractional-days-negative        [2m(617.29µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] randomblob-str-2                         [2m(444.28µs)[0m
  [[32mPASS[0m] unhex-x-y-shorter-suffix                 [2m(455.72µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-or-where-clause            [2m(11.62ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-values-column-name                [2m(544.97µs)[0m
  [[32mPASS[0m] select-binary-collation                  [2m(594.14µs)[0m
  [[32mPASS[0m] limit-complex-exprs-limit-const-1        [2m(616.39µs)[0m
  [[32mPASS[0m] limit-complex-exprs-limit-text-2         [2m(691.41µs)[0m

[1mtests/json/memory.sqltest[0m
  [[32mPASS[0m] json-tree-invalid-path                   [2m(2.92ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-fractional-minutes              [2m(455.99µs)[0m
  [[32mPASS[0m] datetime-floor-keeps-time                [2m(463.21µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] zeroblob-int-0                           [2m(443.15µs)[0m
  [[32mPASS[0m] unhex-x-y-shorter-prefix                 [2m(462.09µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] limit-complex-exprs-limit-expr-sub       [2m(545.96µs)[0m
  [[32mPASS[0m] limit-complex-exprs-limit-bool-true      [2m(1.11ms)[0m
  [[32mPASS[0m] limit-complex-exprs-limit-expr-add       [2m(701.60µs)[0m
  [[32mPASS[0m] limit-complex-exprs-limit-expr-paren     [2m(705.76µs)[0m
  [[32mPASS[0m] select_bin_shr                           [2m(4.93ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] left-hash-join-larger                    [2m(21.56ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] limit-complex-exprs-limit-bool-add       [2m(658.59µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] log-negative-negative                    [2m(2.87ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] limit-complex-exprs-limit-text-math      [2m(722.80µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-fractional-seconds              [2m(539.36µs)[0m
  [[32mPASS[0m] datetime-julianday-negative              [2m(436.38µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_blob_array_2                  [2m(461.83µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] zeroblob-int-1                           [2m(450.21µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] limit-complex-exprs-limit-bool-false-add [2m(693.81µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-text_float-text_float        [2m(2.85ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-x-y-random-order                   [2m(442.00µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] remainder-agg-int                        [2m(18.62ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] limit-complex-exprs-limit-mixed-math     [2m(725.27µs)[0m
  [[32mPASS[0m] limit-complex-exprs-limit-text-bool      [2m(686.69µs)[0m
  [[32mPASS[0m] limit-complex-exprs-limit-coalesce       [2m(685.78µs)[0m
  [[32mPASS[0m] explain-64-bit-integer-no-overflow       [2m(568.47µs)[0m
  [[32mPASS[0m] select-const-1                           [2m(406.32µs)[0m
  [[32mPASS[0m] select-const-2                           [2m(435.04µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-trailing-garbage-rejected       [2m(452.59µs)[0m
  [[32mPASS[0m] datetime-unixepoch-negative              [2m(447.37µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] zeroblob-str-3                           [2m(452.64µs)[0m
  [[32mPASS[0m] unhex-x-y-char-in-the-middle             [2m(420.38µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-const-3                           [2m(424.65µs)[0m
  [[32mPASS[0m] select-const-4                           [2m(416.54µs)[0m
  [[32mPASS[0m] select-true                              [2m(413.34µs)[0m
  [[32mPASS[0m] select-false                             [2m(406.91µs)[0m

[1mtests/pragma/table_list.sqltest[0m
  [[32mPASS[0m] pragma-table-list-includes-sqlite-schema [2m(4.49ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-trailing-garbage-date-rejected  [2m(446.92µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] zeroblob-str-a                           [2m(426.22µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-unixepoch-4decimal              [2m(454.41µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-x-y-character-outside-set          [2m(438.95µs)[0m

[1mtests/json/memory.sqltest[0m
  [[32mPASS[0m] json-malformed-empty-string              [2m(2.76ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-text-escape-1                     [2m(420.89µs)[0m
  [[32mPASS[0m] select-blob-empty                        [2m(407.36µs)[0m
  [[32mPASS[0m] select-blob-ascii                        [2m(410.01µs)[0m
  [[32mPASS[0m] select-blob-emoji                        [2m(415.77µs)[0m
  [[32mPASS[0m] select-limit-0                           [2m(570.39µs)[0m
  [[32mPASS[0m] select-doubly-qualified                  [2m(579.91µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] arithmetic-overflow                      [2m(459.50µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] zeroblob-blob                            [2m(426.93µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-fractional-month                [2m(460.74µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] log-float-negative                       [2m(2.78ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-dash-separated                     [2m(481.40µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-limit-expression                  [2m(657.32µs)[0m
  [[32mPASS[0m] select-limit-false                       [2m(577.98µs)[0m
  [[32mPASS[0m] select-limit-true                        [2m(628.41µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-fractional-year                 [2m(471.49µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-multi-dash-separated               [2m(478.46µs)[0m

[1mtests/multi_index_intersection.sqltest[0m
  [[32mPASS[0m] composite-index-preferred-over-intersection [2m(9.72ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] realify                                  [2m(611.87µs)[0m
  [[32mPASS[0m] select-star-no-from                      [2m(7.77ms)[0m
  [[32mPASS[0m] select-param-zero-invalid                [2m(4.70ms)[0m
  [[32mPASS[0m] select-add                               [2m(868.34µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-ceiling-keeps-time              [2m(474.10µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-subtract                          [2m(863.99µs)[0m
  [[32mPASS[0m] select-star-and-constant-no-from         [2m(8.00ms)[0m

[1mtests/json/memory.sqltest[0m
  [[32mPASS[0m] json-malformed-plus-only                 [2m(2.71ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] func7-pg-411                             [2m(16.22ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-star-subquery                     [2m(8.14ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-space-separated                    [2m(480.43µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-doubly-qualified-wrong-table      [2m(3.06ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-edgecase-int-to-numeric             [2m(556.38µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-doubly-qualified-wrong-column     [2m(3.19ms)[0m
  [[32mPASS[0m] case-insensitive-columns                 [2m(870.66µs)[0m
  [[32mPASS[0m] table-star                               [2m(639.77µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] log-null-int                             [2m(2.46ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] sqlite-version-should-return-valid-output [2m(6.32ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] out-of-bounds-unixepoch                  [2m(500.62µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-text-to-integer                     [2m(434.01µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-ceiling-floor-2                     [2m(495.15µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] table-star-2                             [2m(1.07ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-join-table-column-order-by-issue-5904 [2m(1.19ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select_with_quoting                      [2m(824.44µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_blob_number                   [2m(509.74µs)[0m

[1mtests/pragma/table_list.sqltest[0m
  [[32mPASS[0m] pragma-table-list-includes-temp-schema   [2m(4.53ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select_with_quoting_2                    [2m(734.48µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] trim                                     [2m(438.15µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-rowid                             [2m(718.04µs)[0m
  [[32mPASS[0m] select-rowid-2                           [2m(674.34µs)[0m
  [[32mPASS[0m] select_case_base_else                    [2m(487.92µs)[0m
  [[32mPASS[0m] select_case_noelse_null                  [2m(447.60µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-edgecase-int-to-numeric-2           [2m(551.60µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select_parenthesized                     [2m(662.61µs)[0m
  [[32mPASS[0m] seekrowid                                [2m(910.96µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-text-with-spaces-to-integer         [2m(430.10µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] arithmetic-overflow-julianday            [2m(485.59µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select_base_case_else                    [2m(495.60µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-inf-zeropad-f-prefix              [2m(2.89ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_blob_number_2                 [2m(459.55µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-ceiling-floor-3                     [2m(464.92µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select_base_case_noelse_null             [2m(442.93µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] trim-number                              [2m(456.75µs)[0m

[1mtests/row-value-in.sqltest[0m
  [[32mPASS[0m] row-value-in-three-columns               [2m(2.67ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select_base_case_null_result             [2m(819.48µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-text_float-text              [2m(3.28ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-join-table-column-two-arg-issue-5904 [2m(1.07ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-is-null                           [2m(793.58µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-text-with-trailing-junk-to-integer  [2m(440.56µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-like-expression                   [2m(459.07µs)[0m
  [[32mPASS[0m] select-not-like-expression               [2m(452.43µs)[0m
  [[32mPASS[0m] select-is-not-null                       [2m(814.15µs)[0m
  [[32mPASS[0m] select-like-expression-2                 [2m(477.54µs)[0m

[1mtests/json/memory.sqltest[0m
  [[32mPASS[0m] json-malformed-minus-only                [2m(2.67ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select_positive_infinite_float           [2m(454.99µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] overflows-with-large-days-modifier       [2m(473.65µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select_negative_infinite_float           [2m(424.46µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] trim-null                                [2m(430.60µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select_bin_shl                           [2m(1.15ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-invalid-text-to-integer             [2m(419.86µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] log-int-null                             [2m(2.56ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select_shl_large_negative_float          [2m(760.91µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-ceiling-floor-4                     [2m(494.16µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select_shl_basic                         [2m(698.38µs)[0m
  [[32mPASS[0m] select_shl_negative_numbers              [2m(690.23µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-column-aliases          [2m(12.81ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-each-join-complex-expression-where-issue-5904 [2m(1.14ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select_shl_negative_shifts               [2m(674.14µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time_date-overflow-issue-5251            [2m(492.09µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] trim-leading-whitespace                  [2m(444.56µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select_shl_large_shifts                  [2m(844.79µs)[0m
  [[32mPASS[0m] select_shl_text_conversion               [2m(776.48µs)[0m
  [[32mPASS[0m] select_shl_chained                       [2m(643.97µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-text-prefix-to-integer              [2m(421.52µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-invalid-numeric-text              [2m(441.59µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-not-between-existing-rows  [2m(11.43ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select_fuzz_failure_case                 [2m(566.30µs)[0m
  [[32mPASS[0m] select_shl_numeric_types                 [2m(800.38µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-ceiling-floor-5                     [2m(485.08µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-invalid-numeric-text-2            [2m(405.43µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] trim-trailing-whitespace                 [2m(453.84µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] invalid-date                             [2m(452.15µs)[0m

[1mtests/row-value-in.sqltest[0m
  [[32mPASS[0m] row-value-in-three-columns-no-match      [2m(2.66ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-float-to-integer                    [2m(424.33µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-inf-zeropad-e                     [2m(2.83ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-text-int-text-edge           [2m(2.86ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sqlite-version-should-return-valid-output [2m(3.44ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-binary-collation                  [2m(621.25µs)[0m
  [[32mPASS[0m] select-values-column-name                [2m(619.08µs)[0m
  [[32mPASS[0m] limit-complex-exprs-limit-const-1        [2m(638.20µs)[0m

[1mtests/json/memory.sqltest[0m
  [[32mPASS[0m] json-malformed-double-minus              [2m(2.67ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] limit-complex-exprs-limit-text-2         [2m(704.81µs)[0m
  [[32mPASS[0m] limit-complex-exprs-limit-bool-true      [2m(727.98µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] invalid-time                             [2m(474.61µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] limit-complex-exprs-limit-expr-add       [2m(770.42µs)[0m
  [[32mPASS[0m] limit-complex-exprs-limit-expr-sub       [2m(595.75µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] remainder-int-null                       [2m(2.46ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-large-text-to-numeric               [2m(546.44µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] limit-complex-exprs-limit-expr-paren     [2m(727.26µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-float-to-integer-rounding           [2m(1.18ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] invalid-datetime                         [2m(484.36µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] limit-complex-exprs-limit-bool-add       [2m(730.35µs)[0m
  [[32mPASS[0m] limit-complex-exprs-limit-text-math      [2m(750.28µs)[0m
  [[32mPASS[0m] limit-complex-exprs-limit-bool-false-add [2m(662.80µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-max-i64-text-to-numeric             [2m(452.31µs)[0m
  [[32mPASS[0m] cast-large-float-to-integer              [2m(418.71µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] limit-complex-exprs-limit-mixed-math     [2m(771.61µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-multibyte-utf8-emoji            [2m(443.68µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] limit-complex-exprs-limit-text-bool      [2m(723.15µs)[0m
  [[32mPASS[0m] select_bin_shr                           [2m(7.08ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-inf-zeropad-g                     [2m(2.78ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] limit-complex-exprs-limit-coalesce       [2m(656.91µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-text-signed-text-edge        [2m(2.87ms)[0m

[1mtests/json/memory.sqltest[0m
  [[32mPASS[0m] json-malformed-double-plus               [2m(2.54ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_each_3_arguments                    [2m(4.61ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-negative-large-text-to-numeric      [2m(493.43µs)[0m
  [[32mPASS[0m] cast-small-float-to-integer              [2m(430.08µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] explain-64-bit-integer-no-overflow       [2m(584.38µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] remainder-int-0                          [2m(2.51ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_blob_number_3                 [2m(1.51ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-where-in                [2m(6.47ms)[0m

[1mtests/row-value-in.sqltest[0m
  [[32mPASS[0m] row-value-in-or-condition                [2m(5.51ms)[0m

[1mtests/pragma/table_list.sqltest[0m
  [[32mPASS[0m] pragma-table-list-unqualified-includes-main-and-temp-matches [2m(10.46ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] is-true-integer                          [2m(2.65ms)[0m
  [[32mPASS[0m] is-true-null                             [2m(2.64ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] sqlite-version-should-return-valid-output [2m(3.40ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-inf-neg-zeropad-e                 [2m(2.71ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] is-true-float                            [2m(2.96ms)[0m
  [[32mPASS[0m] is-true-text                             [2m(2.94ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-large-text-to-integer               [2m(433.70µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-star-and-constant-no-from         [2m(7.74ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] is-false-integer                         [2m(2.56ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-multibyte-utf8-chinese              [2m(489.98µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] remainder-agg-float                      [2m(18.24ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-null-to-any                         [2m(528.22µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-ceiling-floor-6                     [2m(497.35µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-1arg-root-object-and-children-preorder [2m(1.09ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-param-zero-invalid                [2m(4.54ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] is-false-text                            [2m(2.91ms)[0m
  [[32mPASS[0m] is-false-null                            [2m(2.71ms)[0m
  [[32mPASS[0m] is-not-true-integer                      [2m(2.51ms)[0m

[1mtests/multi_index_or_adversarial.sqltest[0m
  [[32mPASS[0m] or-join-self-loop-dedup                  [2m(15.13ms)[0m

[1mtests/json/memory.sqltest[0m
  [[32mPASS[0m] json-malformed-incomplete-exponent       [2m(2.69ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] is-not-true-null                         [2m(2.76ms)[0m
  [[32mPASS[0m] is-not-false-integer                     [2m(2.55ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] remainder-int-int                        [2m(2.87ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] is-not-false-null                        [2m(2.79ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-text-exp-to-integer                 [2m(427.10µs)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-star-subquery                     [2m(10.96ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-multibyte-utf8-japanese             [2m(477.98µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] trim-pattern                             [2m(498.50µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-text-null                    [2m(2.98ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] is-false-float                           [2m(6.93ms)[0m

[1mtests/select/default.sqltest[0m
  [[32mPASS[0m] select-star-no-from                      [2m(13.24ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-inf-no-zeropad-e                  [2m(2.89ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_blob_negative_number          [2m(554.23µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] is-false-where                           [2m(5.18ms)[0m
  [[32mPASS[0m] is-true-where                            [2m(5.32ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-without-type-integer                [2m(462.75µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] add-agg-int                              [2m(728.31µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] typeof-sum-integer                       [2m(45.91ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-union-1                           [2m(7.52ms)[0m
  [[32mPASS[0m] select-union-all-1                       [2m(8.89ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-ceiling-floor-7                     [2m(567.40µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-union-all-with-filters            [2m(9.17ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-1arg-root-array-and-children-preorder [2m(1.02ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] acos-null                                [2m(2.86ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-correlated-in-selfread  [2m(11.16ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-union-all-union                   [2m(10.86ms)[0m
  [[32mPASS[0m] select-union-all-with-offset             [2m(8.95ms)[0m
  [[32mPASS[0m] select-union-with-offset                 [2m(7.98ms)[0m

[1mtests/offset/default.sqltest[0m
  [[32mPASS[0m] select-offset-1-groupby                  [2m(120.29ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-or-multiple-existing       [2m(10.09ms)[0m

[1mtests/json/memory.sqltest[0m
  [[32mPASS[0m] json-malformed-incomplete-exponent-plus  [2m(2.77ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-inf-no-zeropad-g                  [2m(2.61ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-intersect-union                   [2m(9.71ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-null-int                     [2m(2.94ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] add-int-agg                              [2m(765.96µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-union-all-union-2                 [2m(14.93ms)[0m
  [[32mPASS[0m] select-union-all-union-3                 [2m(14.24ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] trim-pattern-number                      [2m(503.63µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-intersect-2                       [2m(13.00ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-without-type-text-numeric           [2m(450.71µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-union-intersect                   [2m(9.93ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] remainder-int-float                      [2m(2.12ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-union-all-intersect               [2m(9.01ms)[0m
  [[32mPASS[0m] select-intersect-with-limit              [2m(7.95ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-ceiling-floor-8                     [2m(543.95µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-1arg-primitive-root-null-single-row [2m(617.21µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-intersect-with-offset             [2m(5.60ms)[0m

[1mtests/offset/default.sqltest[0m
  [[32mPASS[0m] select-offset-subquery                   [2m(913.69µs)[0m

[1mtests/row-value-in.sqltest[0m
  [[32mPASS[0m] row-value-correlated-subquery-in         [2m(20.37ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-union-3                           [2m(18.96ms)[0m

[1mtests/pragma/table_list.sqltest[0m
  [[32mPASS[0m] pragma-table-info-prefers-temp-shadow    [2m(20.50ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-text-to-real                        [2m(486.97µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-except-2                          [2m(6.55ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] left-hash-join-larger-null-count         [2m(41.64ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-intersect-union-with-limit        [2m(8.91ms)[0m
  [[32mPASS[0m] select-intersect-union-all               [2m(12.72ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-sigdigits-f-20                    [2m(2.62ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-except-1                          [2m(10.19ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-target-selfread-reruns-per-row [2m(5.66ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-intersect-1                       [2m(21.52ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_shift_blob                    [2m(540.83µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-values-except                     [2m(5.07ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] acosh-int                                [2m(2.84ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-values-union-all-limit            [2m(4.54ms)[0m
  [[32mPASS[0m] select-except-limit                      [2m(10.96ms)[0m
  [[32mPASS[0m] select-except-union-all                  [2m(9.14ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-text-with-spaces-to-real            [2m(491.94µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-union-4                           [2m(24.68ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] trim-pattern-null                        [2m(480.53µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-intersect-except                  [2m(6.84ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] julianday-multibyte-utf8-cyrillic        [2m(514.50µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-except-union                      [2m(8.66ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] add-agg-float                            [2m(850.23µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-union-all-except                  [2m(9.28ms)[0m

[1mtests/json/memory.sqltest[0m
  [[32mPASS[0m] json-malformed-incomplete-exponent-minus [2m(2.81ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-except-intersect                  [2m(8.09ms)[0m
  [[32mPASS[0m] select-values-union-2                    [2m(6.82ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-without-type-text-integer-string    [2m(461.06µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-union-except                      [2m(8.92ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-1arg-primitive-root-true-single-row [2m(772.67µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-except-3                          [2m(13.53ms)[0m
  [[32mPASS[0m] select-values-union                      [2m(8.07ms)[0m
  [[32mPASS[0m] select-values-union-all-limit-1          [2m(4.70ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-modifier-subtract-months       [2m(497.01µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-values-union-all-offset           [2m(5.15ms)[0m

[1mtests/join/memory.sqltest[0m
  [[32mPASS[0m] test-spill-hash-join-count               [2m(573.14ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] remainder-float-int                      [2m(2.87ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-sigdigits-e-20                    [2m(2.25ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-null-float                   [2m(2.57ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-in-simple                         [2m(3.09ms)[0m
  [[32mPASS[0m] select-in-with-nulls                     [2m(2.92ms)[0m
  [[32mPASS[0m] select-in-with-nulls-2                   [2m(3.00ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-without-type-text-real-string       [2m(576.68µs)[0m

[1mtests/offset/memory.sqltest[0m
  [[32mPASS[0m] select-limit-comma-offset-equivalence    [2m(4.54ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-1arg-primitive-root-false-single-row [2m(762.79µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-modifier-add-years             [2m(473.13µs)[0m

[1mtests/multi_index_or_adversarial.sqltest[0m
  [[32mPASS[0m] or-join-null-columns                     [2m(24.53ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] glob-is-not-null-precedence              [2m(2.76ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] remainder-0-float                        [2m(2.68ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-values-union-all-offset-1         [2m(11.45ms)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] simple-count-scalar-subquery             [2m(5.95ms)[0m
  [[32mPASS[0m] unindexed-min-does-not-early-exit        [2m(6.36ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-target-selfread-correlated-group-count [2m(8.28ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_object_2                    [2m(621.97µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-without-type-null                   [2m(447.16µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] right-join-larger                        [2m(12.50ms)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] indexed-min-all-null-returns-null        [2m(7.00ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-cte-union-all-column-resolution   [2m(13.85ms)[0m
  [[32mPASS[0m] select-in-complex                        [2m(10.35ms)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] indexed-min-result-skips-null            [2m(8.37ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-range-search-scan-desc-index      [2m(13.45ms)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] unindexed-max-does-not-early-exit        [2m(10.05ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-modifier-subtract-years        [2m(506.87µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] acosh-float                              [2m(2.21ms)[0m

[1mtests/json/memory.sqltest[0m
  [[32mPASS[0m] json-malformed-hex-no-digits             [2m(1.90ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] where-variable-predicate                 [2m(12.08ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-null-text                    [2m(2.72ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] limit-expr-can-be-cast-losslessly-1      [2m(3.28ms)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] indexed-min-empty-table-returns-null     [2m(8.75ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-range-search-scan-asc-index       [2m(17.73ms)[0m

[1mtests/row-value-in.sqltest[0m
  [[32mPASS[0m] row-value-correlated-subquery-not-in     [2m(16.87ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-sigdigits-g-20                    [2m(2.67ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-range-search-count-asc-index      [2m(18.85ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] add-float-agg                            [2m(771.44µs)[0m

[1mtests/offset/memory.sqltest[0m
  [[32mPASS[0m] select-ungrouped-aggregate-with-offset-limit [2m(8.31ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] trim-pattern-null-2                      [2m(552.68µs)[0m

[1mtests/pragma/table_list.sqltest[0m
  [[32mPASS[0m] pragma-table-xinfo-prefers-temp-shadow   [2m(18.98ms)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] indexed-max-result                       [2m(14.82ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-or-unique                  [2m(23.89ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] remainder-float-0                        [2m(2.84ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-1arg-primitive-root-integer-single-row [2m(753.79µs)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] indexed-max-real-with-negatives          [2m(5.69ms)[0m
  [[32mPASS[0m] indexed-min-max-extreme-integers         [2m(5.43ms)[0m
  [[32mPASS[0m] indexed-min-max-text                     [2m(10.06ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-without-type-in-expression          [2m(357.13µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] acosh-str                                [2m(1.92ms)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] indexed-min-with-equality-prefix-skips-null [2m(12.14ms)[0m

[1mtests/json/memory.sqltest[0m
  [[32mPASS[0m] json-malformed-leading-zeros             [2m(2.44ms)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] indexed-min-max-real-with-negatives      [2m(9.66ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-range-search-count-desc-index     [2m(23.71ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_object_3                    [2m(616.94µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-modifier-weekday               [2m(513.67µs)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] indexed-max-extreme-integers             [2m(7.67ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-multibyte-utf8-greek            [2m(522.34µs)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] indexed-max-single-row                   [2m(5.99ms)[0m
  [[32mPASS[0m] indexed-max-text                         [2m(12.95ms)[0m
  [[32mPASS[0m] indexed-max-all-null-returns-null        [2m(5.91ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] add-agg-int-agg-int-2                    [2m(739.83µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] limit-expr-can-be-cast-losslessly-2      [2m(9.37ms)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] unordered-materialized-cte-max-without-where [2m(6.17ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-sigdigits-f-15                    [2m(2.55ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-1arg-primitive-root-real-single-row [2m(755.51µs)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] indexed-max-empty-table-returns-null     [2m(8.67ms)[0m
  [[32mPASS[0m] indexed-min-single-row                   [2m(9.59ms)[0m
  [[32mPASS[0m] indexed-max-with-equality-prefix         [2m(17.10ms)[0m
  [[32mPASS[0m] indexed-min-nocase-column-collation      [2m(5.76ms)[0m
  [[32mPASS[0m] unordered-materialized-cte-correlated-max [2m(7.03ms)[0m
  [[32mPASS[0m] materialized-cte-max-without-where       [2m(9.30ms)[0m
  [[32mPASS[0m] materialized-cte-min-with-where-skips-null [2m(10.12ms)[0m
  [[32mPASS[0m] materialized-cte-max-with-where          [2m(10.35ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-in-where                            [2m(883.66µs)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] indexed-max-nocase-column-collation      [2m(6.40ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] shift-right-null-null                    [2m(5.88ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] trim-no-match-pattern                    [2m(542.45µs)[0m

[1mtests/offset/memory.sqltest[0m
  [[32mPASS[0m] offset-expr-can-be-cast-losslessly-1     [2m(2.83ms)[0m

[1mtests/multi_index_or_adversarial.sqltest[0m
  [[32mPASS[0m] or-join-extra-where-filter               [2m(18.07ms)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] indexed-max-desc-index                   [2m(6.34ms)[0m
  [[32mPASS[0m] materialized-cte-correlated-max-with-implicit-inner-order [2m(10.35ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-multiple-modifiers-2           [2m(500.64µs)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] indexed-min-multi-equality-prefix        [2m(6.96ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] acosh-invalid                            [2m(2.79ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-multibyte-modifier-arithmetic   [2m(503.36µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-no-match-in-leaf-page             [2m(32.51ms)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] indexed-min-desc-index                   [2m(8.11ms)[0m
  [[32mPASS[0m] indexed-max-multi-equality-prefix        [2m(6.75ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-invalid-text-to-real                [2m(474.04µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_with_escaping               [2m(558.31µs)[0m

[1mtests/json/memory.sqltest[0m
  [[32mPASS[0m] json-malformed-leading-zero-digit        [2m(2.75ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] add-agg-float-agg-float                  [2m(735.00µs)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] negative-min-plus-constant               [2m(6.01ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] remainder-float-float                    [2m(2.82ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] order-by-qualified-2                     [2m(121.86ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-1arg-primitive-root-text-single-row [2m(763.87µs)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] negative-max-times-one                   [2m(6.34ms)[0m
  [[32mPASS[0m] negative-min-and-max-together            [2m(5.73ms)[0m

[1mtests/pragma/table_list.sqltest[0m
  [[32mPASS[0m] pragma-table-list-function-syntax        [2m(6.73ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] parse-large-integral-numeric-string-as-number [2m(489.90µs)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] negative-length-of-min                   [2m(6.85ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-or-and-combined            [2m(8.24ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-where-between           [2m(10.28ms)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] negative-min-expression-argument         [2m(6.91ms)[0m
  [[32mPASS[0m] negative-min-partial-index-prefix        [2m(6.26ms)[0m
  [[32mPASS[0m] negative-max-partial-index-prefix        [2m(6.11ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-text-prefix-to-real                 [2m(439.87µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] limit-expr-can-be-cast-losslessly-3      [2m(11.36ms)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] negative-negated-min                     [2m(9.55ms)[0m
  [[32mPASS[0m] negative-coalesce-wrapping-max           [2m(9.53ms)[0m
  [[32mPASS[0m] negative-min-cast-argument               [2m(6.28ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] ltrim                                    [2m(529.32µs)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] negative-max-with-inequality-where       [2m(7.45ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] add-agg-int-agg-float-2                  [2m(758.59µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] acosh-null                               [2m(2.80ms)[0m
  [[32mPASS[0m] remainder-text-1                         [2m(1.80ms)[0m

[1mtests/offset/memory.sqltest[0m
  [[32mPASS[0m] offset-expr-can-be-cast-losslessly-2     [2m(5.27ms)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] negative-min-correlated-subquery         [2m(8.45ms)[0m
  [[32mPASS[0m] negative-min-with-group-by               [2m(12.24ms)[0m
  [[32mPASS[0m] negative-max-with-limit-zero             [2m(10.08ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-larger                   [2m(19.11ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-atom-null-for-containers       [2m(1.02ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-subsec                         [2m(575.61µs)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] negative-min-with-inequality-where       [2m(11.03ms)[0m

[1mtests/simple-count-optimization.sqltest[0m
  [[32mPASS[0m] simple-count                             [2m(7.68ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] limit-expr-int-and-string                [2m(4.79ms)[0m

[1mtests/snapshot_tests/analyze/stat1_rows.sqltest[0m
  [[32mPASS[0m] analyze-indexed-table-does-not-insert-null-stat1-row [2m(7.63ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_with_escaping_2             [2m(554.36µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] parse-large-integral-numeric-string-as-number-2 [2m(513.62µs)[0m

[1mtests/row-value-in.sqltest[0m
  [[32mPASS[0m] row-value-left-join-correlated-subquery-in [2m(18.17ms)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] negative-min-abs-expression              [2m(14.32ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] comp-float-float                         [2m(2.76ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-leapyear-floor                      [2m(499.54µs)[0m

[1mtests/snapshot_tests/subqueries/subquery-sort-elision.sqltest[0m
  [[32mPASS[0m] outer-orderby-elided-by-cte              [2m(7.67ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-integer-to-real                     [2m(490.45µs)[0m

[1mtests/json/memory.sqltest[0m
  [[32mPASS[0m] json_valid_blob_check_constraint_rejects_invalid_blob [2m(9.46ms)[0m

[1mtests/snapshot_tests/subqueries/subquery-sort-elision.sqltest[0m
  [[32mPASS[0m] three-hop-passthrough                    [2m(7.91ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] subtract-agg-int                         [2m(769.26µs)[0m

[1mtests/snapshot_tests/subqueries/subquery-sort-elision.sqltest[0m
  [[32mPASS[0m] multi-hop-cte-window                     [2m(8.57ms)[0m
  [[32mPASS[0m] asc-vs-desc-not-elided                   [2m(7.38ms)[0m
  [[32mPASS[0m] multi-col-direction-mismatch             [2m(7.68ms)[0m
  [[32mPASS[0m] different-orderby-column                 [2m(7.38ms)[0m

[1mtests/multi_index_or_adversarial.sqltest[0m
  [[32mPASS[0m] or-join-left-join-no-match               [2m(15.73ms)[0m

[1mtests/snapshot_tests/subqueries/subquery-sort-elision.sqltest[0m
  [[32mPASS[0m] multi-hop-outer-orderby-elided           [2m(11.74ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-modifier-add-hours-2           [2m(504.95µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-value-minified-for-containers  [2m(983.28µs)[0m
  [[32mPASS[0m] json_extract_null_path                   [2m(511.54µs)[0m

[1mtests/select/simple_min_max.sqltest[0m
  [[32mPASS[0m] indexed-min-in-scalar-subquery           [2m(25.40ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] ltrim-number                             [2m(466.19µs)[0m

[1mtests/snapshot_tests/subqueries/subquery-sort-elision.sqltest[0m
  [[32mPASS[0m] limit-breaks-chain                       [2m(7.19ms)[0m
  [[32mPASS[0m] outer-orderby-elided-by-group            [2m(12.52ms)[0m
  [[32mPASS[0m] cte-group-feeds-window                   [2m(10.80ms)[0m
  [[32mPASS[0m] offset-breaks-chain                      [2m(7.99ms)[0m
  [[32mPASS[0m] partial-order-match-not-elided           [2m(10.02ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-nan-f                             [2m(2.63ms)[0m

[1mtests/snapshot_tests/subqueries/subquery-sort-elision.sqltest[0m
  [[32mPASS[0m] no-cte-order-not-elided                  [2m(9.13ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-leapyear-floor2                     [2m(519.67µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] length-999123                            [2m(510.30µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] comp-int-float                           [2m(2.00ms)[0m
  [[32mPASS[0m] asin-int                                 [2m(2.60ms)[0m

[1mtests/pragma/table_list.sqltest[0m
  [[32mPASS[0m] pragma-table-list-attached-database      [2m(12.43ms)[0m

[1mtests/offset/memory.sqltest[0m
  [[32mPASS[0m] offset-expr-can-be-cast-losslessly-3     [2m(6.07ms)[0m

[1mtests/snapshot_tests/subqueries/subquery-sort-elision.sqltest[0m
  [[32mPASS[0m] cte-order-feeds-window                   [2m(16.48ms)[0m
  [[32mPASS[0m] window-in-intermediate-cte               [2m(8.45ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] subtract-int-agg                         [2m(764.41µs)[0m

[1mtests/snapshot_tests/subqueries/subquery-sort-elision.sqltest[0m
  [[32mPASS[0m] distinct-in-cte                          [2m(11.76ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] limit-expr-cannot-be-cast-losslessly-1   [2m(15.38ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-integer-to-text                     [2m(497.33µs)[0m

[1mtests/snapshot_tests/subqueries/subquery-sort-elision.sqltest[0m
  [[32mPASS[0m] aggregate-in-cte                         [2m(11.97ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] remainder-text-2                         [2m(3.10ms)[0m

[1mtests/snapshot_tests/subqueries/subquery-sort-elision.sqltest[0m
  [[32mPASS[0m] desc-inner-asc-outer                     [2m(9.16ms)[0m
  [[32mPASS[0m] join-in-cte                              [2m(9.40ms)[0m

[1mtests/strict.sqltest[0m
  [[32mPASS[0m] strict-null-integer-column               [2m(4.98ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-modifier-subtract-hours-2      [2m(507.08µs)[0m

[1mtests/snapshot_tests/subqueries/subquery-sort-elision.sqltest[0m
  [[32mPASS[0m] collation-mismatch                       [2m(9.86ms)[0m

[1mtests/snapshot_tests/joins/joins.sqltest[0m
  [[32mPASS[0m] three-table-join-rowid-seek-correctness  [2m(24.96ms)[0m

[1mtests/snapshot_tests/subqueries/subquery-sort-elision.sqltest[0m
  [[32mPASS[0m] nulls-in-order-column                    [2m(7.69ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-larger-verify-rows       [2m(8.30ms)[0m

[1mtests/snapshot_tests/subqueries/subquery-sort-elision.sqltest[0m
  [[32mPASS[0m] empty-result-set                         [2m(7.53ms)[0m
  [[32mPASS[0m] duplicate-values-stability               [2m(7.71ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-key-types-by-parent-kind       [2m(1.01ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sum-1                                    [2m(479.47µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-leapyear-ceiling                    [2m(500.66µs)[0m

[1mtests/strict.sqltest[0m
  [[32mPASS[0m] strict-null-real-column                  [2m(4.85ms)[0m

[1mtests/snapshot_tests/subqueries/subquery-sort-elision.sqltest[0m
  [[32mPASS[0m] window-partial-not-elided                [2m(9.02ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-nan-e                             [2m(1.82ms)[0m

[1mtests/snapshot_tests/subqueries/subquery-sort-elision.sqltest[0m
  [[32mPASS[0m] window-no-cte-order-not-elided           [2m(8.39ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] subtract-agg-float                       [2m(762.90µs)[0m

[1mtests/strict.sqltest[0m
  [[32mPASS[0m] strict-null-blob-column                  [2m(5.07ms)[0m

[1mtests/json/memory.sqltest[0m
  [[32mPASS[0m] json_valid_blob_check_constraint_ignore_does_not_insert [2m(4.82ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-real-to-text                        [2m(501.54µs)[0m

[1mtests/snapshot_tests/joins/joins.sqltest[0m
  [[32mPASS[0m] bigass-three-table-join-correctness      [2m(27.40ms)[0m

[1mtests/strict.sqltest[0m
  [[32mPASS[0m] strict-not-null-still-enforced           [2m(4.29ms)[0m
  [[32mPASS[0m] strict-null-text-column                  [2m(9.88ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] comp-float-int                           [2m(2.92ms)[0m
  [[32mPASS[0m] asin-float                               [2m(2.84ms)[0m

[1mtests/strict.sqltest[0m
  [[32mPASS[0m] strict-explicit-null-insert              [2m(6.37ms)[0m

[1mtests/snapshot_tests/subqueries/subquery-sort-elision.sqltest[0m
  [[32mPASS[0m] expression-ordering                      [2m(15.84ms)[0m

[1mtests/strict.sqltest[0m
  [[32mPASS[0m] strict-insert-select-any-to-blob-rejects-xfer [2m(7.54ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-modifier-add-minutes-2         [2m(526.99µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] ltrim-null                               [2m(458.47µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] limit-expr-cannot-be-cast-losslessly-2   [2m(10.79ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-where-like              [2m(5.91ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] remainder-text-3                         [2m(2.95ms)[0m

[1mtests/pragma/table_list.sqltest[0m
  [[32mPASS[0m] pragma-table-list-explicit-main-vs-temp  [2m(10.62ms)[0m

[1mtests/subquery/compound_select_in_where.sqltest[0m
  [[32mPASS[0m] in-except                                [2m(7.80ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] sort-elision-cte-limit-parent-order      [2m(5.26ms)[0m

[1mtests/offset/memory.sqltest[0m
  [[32mPASS[0m] offset-expr-int-and-string               [2m(5.82ms)[0m

[1mtests/subquery/compound_select_in_where.sqltest[0m
  [[32mPASS[0m] in-union                                 [2m(10.47ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-nan-g                             [2m(1.84ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] sort-elision-multi-col-same              [2m(5.58ms)[0m

[1mtests/subquery/compound_select_in_where.sqltest[0m
  [[32mPASS[0m] row-value-in-union                       [2m(9.37ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] ltrim-leading-whitespace                 [2m(504.71µs)[0m

[1mtests/subquery/compound_select_in_where.sqltest[0m
  [[32mPASS[0m] in-union-all                             [2m(11.61ms)[0m

[1mtests/row-value-in.sqltest[0m
  [[32mPASS[0m] row-value-left-join-correlated-subquery-not-in [2m(19.97ms)[0m

[1mtests/subquery/compound_select_in_where.sqltest[0m
  [[32mPASS[0m] in-intersect                             [2m(11.75ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-parent-links-self-join         [2m(1.76ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-leapyear-ceiling2                   [2m(510.81µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] asin-str                                 [2m(2.82ms)[0m

[1mtests/subquery/compound_select_in_where.sqltest[0m
  [[32mPASS[0m] not-in-union                             [2m(10.91ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sum-2                                    [2m(585.86µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] subtract-float-agg                       [2m(757.26µs)[0m

[1mtests/subquery/compound_select_in_where.sqltest[0m
  [[32mPASS[0m] correlated-in-union                      [2m(11.37ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-blob-to-text                        [2m(467.52µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] sort-elision-multi-col-different         [2m(5.81ms)[0m

[1mtests/snapshot_tests/subqueries/subquery-sort-elision.sqltest[0m
  [[32mPASS[0m] partial-partition-match                  [2m(25.44ms)[0m

[1mtests/multi_index_or_adversarial.sqltest[0m
  [[32mPASS[0m] or-join-double-or-three-way              [2m(16.48ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] sort-elision-group-by                    [2m(5.78ms)[0m
  [[32mPASS[0m] sort-elision-cte-desc-parent-asc         [2m(3.70ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-null-keys-both-sides     [2m(10.93ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] sort-elision-mixed-asc-desc              [2m(6.41ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_null_path                     [2m(465.36µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-blob-s                            [2m(1.77ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] sort-elision-expression-order            [2m(5.60ms)[0m

[1mtests/json/memory.sqltest[0m
  [[32mPASS[0m] json101-8-1                              [2m(6.00ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] sort-elision-cte-asc-parent-desc         [2m(11.97ms)[0m
  [[32mPASS[0m] sort-elision-nulls-in-order-by           [2m(5.31ms)[0m
  [[32mPASS[0m] sort-elision-limit-offset                [2m(5.14ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] remainder-text-4                         [2m(2.75ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] ltrim-no-leading-whitespace              [2m(498.16µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] limit-expr-cannot-be-cast-losslessly-3   [2m(10.78ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-modifier-subtract-seconds-2    [2m(507.47µs)[0m

[1mtests/pragma/user_version_10.sqltest[0m
  [[32mPASS[0m] pragma-user-version-user-set             [2m(3.31ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] leapyear-plus-month                      [2m(482.96µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-parent-null-at-top             [2m(774.56µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] sort-elision-cte-order-total-parent-order-a [2m(6.19ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sum-3                                    [2m(506.68µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] sort-elision-collate-nocase-vs-default   [2m(8.17ms)[0m
  [[32mPASS[0m] range-scan-is-null                       [2m(3.64ms)[0m
  [[32mPASS[0m] sort-elision-group-by-opposite-dir       [2m(10.68ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] printf-blob-s-nul                        [2m(1.80ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] range-scan-strict-inequality             [2m(9.65ms)[0m
  [[32mPASS[0m] sort-elision-distinct                    [2m(12.34ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-where-complex           [2m(11.58ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] range-scan-equality                      [2m(12.68ms)[0m
  [[32mPASS[0m] range-scan-empty-table                   [2m(8.75ms)[0m
  [[32mPASS[0m] range-scan-double-filter                 [2m(8.32ms)[0m
  [[32mPASS[0m] range-scan-between                       [2m(12.05ms)[0m
  [[32mPASS[0m] range-scan-negative-numbers              [2m(8.51ms)[0m
  [[32mPASS[0m] range-scan-composite-eq-plus-range       [2m(8.56ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] limit-expr-invalid-data-type-1           [2m(8.81ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-text-to-blob                        [2m(487.81µs)[0m
  [[32mPASS[0m] ltrim-pattern                            [2m(573.66µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] subtract-agg-int-agg-int-2               [2m(866.37µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-subsec                         [2m(561.57µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] range-scan-inclusive                     [2m(12.74ms)[0m
  [[32mPASS[0m] range-scan-not-in                        [2m(9.34ms)[0m
  [[32mPASS[0m] range-scan-in-clause                     [2m(9.99ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-2arg-start-at-object           [2m(885.34µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] invalid-date-format                      [2m(496.60µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] range-scan-aliased-columns               [2m(10.02ms)[0m

[1mtests/pragma_query_only.sqltest[0m
  [[32mPASS[0m] pragma-query-only-float-enable           [2m(3.34ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sum-4                                    [2m(545.12µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] range-scan-text-vs-int-affinity          [2m(8.08ms)[0m
  [[32mPASS[0m] min-max-basic-min                        [2m(4.64ms)[0m
  [[32mPASS[0m] range-scan-duplicates                    [2m(13.11ms)[0m
  [[32mPASS[0m] range-scan-nulls-excluded                [2m(15.87ms)[0m
  [[32mPASS[0m] min-max-basic-max                        [2m(4.64ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_shift_null_path               [2m(496.26µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-integer-to-blob                     [2m(625.66µs)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] func7-pg-301                             [2m(2.67ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] min-max-all-null                         [2m(6.07ms)[0m
  [[32mPASS[0m] min-max-empty                            [2m(6.21ms)[0m
  [[32mPASS[0m] min-max-with-nulls                       [2m(6.35ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] ltrim-pattern-number                     [2m(526.33µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] remainder-text-5                         [2m(2.72ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] min-max-both-together                    [2m(5.07ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] subtract-agg-float-agg-float             [2m(767.41µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-with-modifier-add                   [2m(484.81µs)[0m

[1mtests/last_insert_rowid.sqltest[0m
  [[32mPASS[0m] last-insert-rowid-unchanged-after-update [2m(8.16ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-2arg-start-at-array            [2m(839.85µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] min-max-with-where                       [2m(5.96ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] invalid-datetime-modifier-format         [2m(458.70µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] asin-null                                [2m(10.32ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-where-modulo            [2m(5.36ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] limit-expr-invalid-data-type-2           [2m(10.31ms)[0m

[1mtests/offset/memory.sqltest[0m
  [[32mPASS[0m] offset-expr-cannot-be-cast-losslessly-1  [2m(17.16ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] min-max-text-column                      [2m(7.61ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-all-nulls                [2m(10.26ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] min-max-group-by-min-on-cte              [2m(6.64ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sum-5                                    [2m(671.45µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_float                       [2m(631.62µs)[0m

[1mtests/multi_index_or_adversarial.sqltest[0m
  [[32mPASS[0m] or-join-rowset-reset-across-outer        [2m(15.16ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] multi-or-basic                           [2m(7.57ms)[0m
  [[32mPASS[0m] multi-or-range                           [2m(7.64ms)[0m
  [[32mPASS[0m] complex-cte-chain                        [2m(5.44ms)[0m
  [[32mPASS[0m] min-max-on-cte-with-limit                [2m(7.95ms)[0m
  [[32mPASS[0m] multi-or-with-limit                      [2m(7.13ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] ltrim-pattern-null                       [2m(516.75µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] complex-cte-self-join                    [2m(6.93ms)[0m
  [[32mPASS[0m] multi-or-update                          [2m(8.29ms)[0m
  [[32mPASS[0m] multi-or-different-columns               [2m(8.77ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-2arg-start-at-primitive-yields-single-row-and-path-to-self [2m(761.62µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-utc-basic                       [2m(548.75µs)[0m
  [[32mPASS[0m] time-with-modifier-sub                   [2m(507.73µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] min-max-correlated-from-cte              [2m(10.93ms)[0m
  [[32mPASS[0m] multi-or-delete                          [2m(9.41ms)[0m

[1mtests/scalar-functions-printf.sqltest[0m
  [[32mPASS[0m] func7-pg-311                             [2m(2.91ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] complex-cte-with-having                  [2m(5.67ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-integer-text-to-numeric             [2m(580.98µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] multi-or-dedup                           [2m(9.51ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] subtract-agg-int-agg-float-2             [2m(787.73µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] complex-cte-chain-aggregation            [2m(5.92ms)[0m

[1mtests/pragma_query_only.sqltest[0m
  [[32mPASS[0m] pragma-query-only-float-disable          [2m(3.00ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] complex-cte-in-subquery-and-main         [2m(9.72ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sum-6                                    [2m(508.40µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] complex-cte-union                        [2m(7.75ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] remainder-text-7                         [2m(3.11ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-where-false             [2m(5.15ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] cte-join-table                           [2m(8.30ms)[0m
  [[32mPASS[0m] cte-left-join-table                      [2m(7.74ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] limit-expr-invalid-data-type-3           [2m(10.22ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_float                         [2m(638.42µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] sin-int                                  [2m(4.87ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] evil-two-ctes-different-orders-joined    [2m(5.94ms)[0m
  [[32mPASS[0m] evil-cte-groupby-having-order-flip       [2m(5.69ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-single-row-each          [2m(7.02ms)[0m
VDBE Trace:
0     Init               0     2     0                    0   Start at 2
2     Goto               0     1     0                    0   
1     Halt               0     0     0                    0   
VDBE Trace:
0     Init               0     2     0                    0   Start at 2
2     Goto               0     1     0                    0   
1     Halt               0     0     0                    0   
VDBE Trace:
0     Init               0     2     0                    0   Start at 2
2     Goto               0     1     0                    0   
1     Halt               0     0     0                    0   
VDBE Trace:
0     Init               0     2     0                    0   Start at 2
2     Goto               0     1     0                    0   
1     Halt               0     0     0                    0   
VDBE Trace:
0     Init               0     2     0                    0   Start at 2
2     Goto               0     1     0                    0   
1     Halt               0     0     0                    0   

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] cte-cross-join-self                      [2m(8.36ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] subtract-agg-float-agg-int-2             [2m(806.80µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] evil-cte-limit-then-different-order      [2m(7.40ms)[0m

[1mtests/subquery/cte_chain_regression.sqltest[0m
  [[32mPASS[0m] cte-chain-large-linear-regression        [2m(46.86ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] cte-left-join-order-preserved            [2m(8.24ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-2arg-nonexistent-path-returns-no-rows [2m(708.75µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-modifier-add-months-2          [2m(544.11µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-float-text-to-numeric               [2m(735.53µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] evil-cte-correlated-scalar-plus-range    [2m(8.37ms)[0m

[1mtests/pragma_vdbe_trace.sqltest[0m
  [[32mPASS[0m] pragma-vdbe-trace                        [2m(3.17ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] comp-int-string                          [2m(2.35ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] cte-exists-subquery                      [2m(11.83ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sum-7                                    [2m(552.23µs)[0m
  [[32mPASS[0m] ltrim-no-match-pattern                   [2m(465.74µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] evil-cte-with-internal-filter-parent-range [2m(6.66ms)[0m
  [[32mPASS[0m] evil-cte-aggregate-then-range            [2m(9.08ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_arrow_shift_float                   [2m(583.27µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] from-sub-desc-parent-asc                 [2m(5.21ms)[0m
  [[32mPASS[0m] from-sub-basic-elision                   [2m(5.73ms)[0m

[1mtests/offset/memory.sqltest[0m
  [[32mPASS[0m] offset-expr-cannot-be-cast-losslessly-2  [2m(10.94ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] multiply-agg-int                         [2m(728.04µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] from-sub-nested                          [2m(5.25ms)[0m
  [[32mPASS[0m] evil-chained-ctes-flip-order             [2m(10.38ms)[0m
  [[32mPASS[0m] cte-order-by-join-preserves-order        [2m(14.43ms)[0m
  [[32mPASS[0m] from-sub-aggregate-different-order       [2m(5.65ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] remainder-text-8                         [2m(2.76ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] limit-expr-invalid-data-type-4           [2m(10.16ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-2arg-empty-array               [2m(692.94µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-utc-with-z-suffix               [2m(524.88µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] evil-self-join-offset-seek               [2m(7.92ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-modifier-subtract-months-2     [2m(521.67µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] group-by-cte-order-feeds-groupby-no-parent-order [2m(5.54ms)[0m
  [[32mPASS[0m] group-by-count-star                      [2m(5.65ms)[0m

[1mtests/last_insert_rowid.sqltest[0m
  [[32mPASS[0m] last-insert-rowid-unchanged-after-upsert-update [2m(15.48ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] group-by-cte-order-on-different-col      [2m(5.45ms)[0m
  [[32mPASS[0m] from-sub-joined                          [2m(7.71ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sum-8                                    [2m(518.50µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] evil-cte-from-where-subquery             [2m(16.59ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-small-float-to-numeric              [2m(560.64µs)[0m
  [[32mPASS[0m] concat-chars                             [2m(446.28µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] sin-float                                [2m(2.86ms)[0m
  [[32mPASS[0m] comp-string-int                          [2m(2.76ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-rowid-and-pk            [2m(5.25ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] join-cte-self-non-equi                   [2m(5.97ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] multiply-int-agg                         [2m(723.11µs)[0m

[1mtests/multi_index_or_adversarial.sqltest[0m
  [[32mPASS[0m] or-join-asymmetric-branches              [2m(16.44ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] join-cte-self-graph-walk                 [2m(6.43ms)[0m
  [[32mPASS[0m] join-two-ctes-different-cardinalities    [2m(10.80ms)[0m
  [[32mPASS[0m] min-max-min-on-desc-cte                  [2m(4.93ms)[0m
  [[32mPASS[0m] join-anti-join-left-join-is-null         [2m(8.79ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] iif-3-args-true                          [2m(505.10µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] min-max-group-by-both                    [2m(5.42ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] rtrim                                    [2m(652.44µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] join-aggregate-cte-to-table              [2m(8.08ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-single-row-no-match      [2m(9.35ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] min-max-limit-zero                       [2m(5.07ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-signed-edgecase-int-to-numeric      [2m(580.21µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] join-distinct-cte-to-table               [2m(8.39ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-2arg-empty-object              [2m(655.73µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] remainder-text-9                         [2m(2.84ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-modifier-add-months-large      [2m(514.58µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] join-three-way-cte-table-cte             [2m(12.89ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] limit-expression-invalid-type            [2m(10.81ms)[0m

[1mtests/reindex.sqltest[0m
  [[32mPASS[0m] reindex-all                              [2m(9.29ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json_extract_true                        [2m(536.28µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] sin-str                                  [2m(2.79ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] compound-cte-in-case-expr                [2m(5.72ms)[0m
  [[32mPASS[0m] join-correlated-sub-from-cte-in-select   [2m(12.04ms)[0m
  [[32mPASS[0m] compound-cte-except                      [2m(7.70ms)[0m
  [[32mPASS[0m] compound-cte-intersect                   [2m(7.87ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-utc-idempotent                  [2m(526.98µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] compound-double-aggregate                [2m(5.42ms)[0m
  [[32mPASS[0m] min-max-multiple-aggregates              [2m(10.56ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] iif-3-args-false                         [2m(485.63µs)[0m
  [[32mPASS[0m] concat-char-and-number                   [2m(506.72µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] compound-not-exists-with-cte             [2m(8.08ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-long-between-chain         [2m(73.14ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] multiply-agg-float                       [2m(765.64µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] comp-string-blog                         [2m(2.84ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] internals-equality-non-first-col         [2m(6.91ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] rtrim-number                             [2m(461.61µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] internals-materialized-cte-probed-per-outer-row [2m(7.28ms)[0m
  [[32mPASS[0m] internals-null-in-join-col               [2m(8.36ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-2arg-bools-and-null-under-array [2m(880.89µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] min-max-no-order-in-cte                  [2m(15.34ms)[0m
  [[32mPASS[0m] internals-range-desc-on-ephemeral        [2m(9.11ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-edgecase-int-to-numeric             [2m(572.59µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] internals-delete-via-cte-in              [2m(9.25ms)[0m
  [[32mPASS[0m] cost-cte-order-plus-limit                [2m(4.92ms)[0m

[1mtests/offset/memory.sqltest[0m
  [[32mPASS[0m] offset-expr-cannot-be-cast-losslessly-3  [2m(11.23ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] join-order-cte-ordered-parent-order-different-table [2m(8.54ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] iif-2-args-true                          [2m(501.47µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] join-order-cte-agg-join-order-across     [2m(8.34ms)[0m
  [[32mPASS[0m] join-order-left-join-order-by-nullable   [2m(9.02ms)[0m
  [[32mPASS[0m] internals-update-via-correlated-cte      [2m(10.64ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-modifier-subtract-months-large [2m(524.74µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-expr-order-elision                   [2m(6.29ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] remainder-overflow-1                     [2m(2.77ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-empty-cte-left-join                  [2m(6.98ms)[0m
  [[32mPASS[0m] join-order-3table-order-across           [2m(11.04ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] multiply-float-agg                       [2m(785.76µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] sin-null                                 [2m(2.85ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-cte-self-join-sum-condition          [2m(7.41ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-join-using-with-both-limits     [2m(1.70ms)[0m
  [[32mPASS[0m] subquery-join-using-with-both-limits-cte [2m(1.20ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] cost-cte-multi-col-groupby-different-order [2m(8.58ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-containing-join                 [2m(1.24ms)[0m
  [[32mPASS[0m] subquery-containing-join-cte             [2m(1.33ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] multiply-agg-int-agg-int-2               [2m(740.64µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-duplicate-keys-both-sides [2m(6.98ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-case-in-order-by                     [2m(6.00ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-autoincrement           [2m(12.85ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] remainder-overflow-2                     [2m(2.84ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-cte-chain-conflicting-orders         [2m(7.91ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] comp-blog-string                         [2m(3.13ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] iif-2-args-false-is-null                 [2m(486.81µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] cost-large-table-small-cte               [2m(13.43ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-edgecase-int-to-numeric-2           [2m(571.45µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-union-all-partition-aggregate        [2m(7.58ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] asinh-int                                [2m(2.92ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-localtime-utc-roundtrip         [2m(506.51µs)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-inner-filter                    [2m(1.08ms)[0m
  [[32mPASS[0m] subquery-inner-filter-cte                [2m(894.43µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] concat-char-and-decimal                  [2m(4.39ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-outer-filter                    [2m(978.09µs)[0m

[1mtests/multi_index_or_adversarial.sqltest[0m
  [[32mPASS[0m] or-join-aggregate-no-phantom-dupes       [2m(15.47ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] iif-multi-args-finds-first-true          [2m(490.62µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-modifier-february-leap-year    [2m(4.86ms)[0m

[1mtests/last_insert_rowid.sqltest[0m
  [[32mPASS[0m] last-insert-rowid-after-insert-returning [2m(10.31ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-without-alias                   [2m(926.29µs)[0m
  [[32mPASS[0m] subquery-outer-filter-cte                [2m(1.18ms)[0m
  [[32mPASS[0m] subquery-without-alias-cte               [2m(964.42µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-localtime-basic                 [2m(467.56µs)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-no-alias-on-col-cte             [2m(846.67µs)[0m
  [[32mPASS[0m] subquery-no-alias-on-col                 [2m(920.05µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-all-nulls-cte-aggregate              [2m(12.67ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] plan-partial-index-and-where-eq-seek     [2m(6.92ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-no-alias-on-col-named-cte       [2m(932.01µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] concat-char-null-char                    [2m(466.98µs)[0m
  [[32mPASS[0m] iif-multi-args-falls-to-else             [2m(478.07µs)[0m

[1mtests/reindex.sqltest[0m
  [[32mPASS[0m] reindex-table                            [2m(9.40ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] rtrim-null                               [2m(462.01µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-cte-four-ways                        [2m(11.78ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-select-star                     [2m(894.62µs)[0m
  [[32mPASS[0m] subquery-no-alias-on-col-named           [2m(1.69ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-julianday-modifier              [2m(456.12µs)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-select-table-star               [2m(944.99µs)[0m
  [[32mPASS[0m] subquery-select-star-cte                 [2m(1.04ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] comp-blog-blog                           [2m(2.90ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-select-table-star-cte           [2m(899.93µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-fullkey-remains-absolute-under-subpath [2m(822.65µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-modifier-february-non-leap-year [2m(496.01µs)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] nested-subquery                          [2m(1.05ms)[0m
  [[32mPASS[0m] nested-subquery-cte                      [2m(1.16ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-triple-self-join-combos              [2m(10.06ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] if-alias-3-args-true                     [2m(472.40µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] rowid-references                         [2m(8.33ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] rtrim-trailing-whitespace                [2m(456.96µs)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-orderby-limit                   [2m(1.04ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] asinh-float                              [2m(2.82ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sqlite-version-should-return-valid-output [2m(3.49ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-orderby-limit-cte               [2m(1.11ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-julianday-modifier              [2m(482.91µs)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] table-join-subquery                      [2m(1.78ms)[0m
  [[32mPASS[0m] subquery-join-subquery                   [2m(1.32ms)[0m
  [[32mPASS[0m] subquery-join-subquery-cte               [2m(1.32ms)[0m
  [[32mPASS[0m] subquery-join-table-cte                  [2m(1.97ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] rtrim-no-trailing-whitespace             [2m(456.15µs)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-path-points-to-container       [2m(823.26µs)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] select-star-table-subquery               [2m(1.74ms)[0m
  [[32mPASS[0m] select-star-table-subquery-cte           [2m(1.79ms)[0m
  [[32mPASS[0m] subquery-join-table                      [2m(2.39ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] if-alias-3-args-false                    [2m(458.47µs)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] select-star-subquery-table               [2m(1.68ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] concat_ws-numbers                        [2m(3.50ms)[0m
  [[32mPASS[0m] cast-large-text-to-numeric               [2m(547.88µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-julianday-modifier             [2m(465.10µs)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] table-join-subquery-cte                  [2m(3.71ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] unary-plus-noop-string                   [2m(2.83ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-count-all                       [2m(8.43ms)[0m
  [[32mPASS[0m] select-star-subquery-subquery            [2m(1.64ms)[0m
  [[32mPASS[0m] select-star-subquery-table-cte           [2m(1.70ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-modifier-invalid-date          [2m(704.73µs)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] select-star-subquery-subquery-cte        [2m(1.67ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] rtrim-pattern                            [2m(453.20µs)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-ignore-unused-cte               [2m(10.38ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-count-includes-containers-and-leaves [2m(640.56µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] if-alias-2-args-true                     [2m(457.67µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] asinh-str                                [2m(2.77ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] multiply-agg-float-agg-float             [2m(917.77µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] plan-partial-index-and-where-reversed-order [2m(6.36ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-join-using-with-inner-limit     [2m(1.29ms)[0m
  [[32mPASS[0m] subquery-join-using-with-outer-limit-cte [2m(1.45ms)[0m
  [[32mPASS[0m] subquery-join-using-with-inner-limit-cte [2m(1.36ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] rtrim-pattern-number                     [2m(439.72µs)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-join-using-with-both-limits     [2m(1.41ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-escapes-in-fullkey             [2m(693.71µs)[0m

[1mtests/reindex.sqltest[0m
  [[32mPASS[0m] reindex-index                            [2m(6.79ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] if-alias-multi-args-finds-first-true     [2m(504.20µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] remainder-overflow-3                     [2m(2.79ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-join-using-with-both-limits-cte [2m(1.50ms)[0m

[1mtests/offset/memory.sqltest[0m
  [[32mPASS[0m] offset-expr-invalid-data-type-1          [2m(10.91ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-containing-join                 [2m(1.49ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] multiply-agg-int-agg-float-2             [2m(769.91µs)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-containing-join-cte             [2m(1.59ms)[0m

[1mtests/last_insert_rowid.sqltest[0m
  [[32mPASS[0m] last-insert-rowid-unchanged-by-cdc-internal-inserts [2m(9.84ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] rtrim-pattern-null                       [2m(427.59µs)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-join-using-with-outer-limit     [2m(4.50ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-huge-limit-offset                    [2m(6.38ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-modifier-date                  [2m(574.18µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] asinh-null                               [2m(3.01ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-in-abs                          [2m(3.21ms)[0m
  [[32mPASS[0m] subquery-in-length                       [2m(3.06ms)[0m
  [[32mPASS[0m] subquery-in-instr                        [2m(3.04ms)[0m
  [[32mPASS[0m] subquery-in-substr                       [2m(3.11ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] multiply-agg-float-agg-int-2             [2m(787.84µs)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-in-replace                      [2m(3.11ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] rtrim-no-match-pattern                   [2m(461.64µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] oid-column-overrides-rowid-alias         [2m(10.70ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-in-upper-lower                  [2m(3.13ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] plan-partial-index-qualified-where-matches [2m(6.16ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-count-distinct                  [2m(18.12ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-in-trim                         [2m(5.60ms)[0m
  [[32mPASS[0m] subquery-in-printf                       [2m(3.96ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] atan-int                                 [2m(3.19ms)[0m

[1mtests/multi_index_or_adversarial.sqltest[0m
  [[32mPASS[0m] or-join-customer-cte-pattern             [2m(17.81ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-with-modifier-datetime-pos      [2m(987.49µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] divide-agg-int                           [2m(749.69µs)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-in-typeof                       [2m(4.30ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-ignore-unused-cte               [2m(9.80ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-in-coalesce-first-non-null      [2m(3.06ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] trim-integer-returns-text                [2m(482.19µs)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-count-all                       [2m(10.42ms)[0m

[1mtests/json/default.sqltest[0m
  [[32mPASS[0m] json-tree-deeply-nested-mixed-types      [2m(839.08µs)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-in-coalesce                     [2m(5.97ms)[0m
  [[32mPASS[0m] subquery-in-nullif-not-equal             [2m(2.97ms)[0m
  [[32mPASS[0m] subquery-in-nullif-equal                 [2m(3.09ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-typeof                  [2m(5.57ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-in-ifnull                       [2m(2.84ms)[0m

[1mtests/left-join-case-iif-null-masking.sqltest[0m
  [[32mPASS[0m] left-join-case-when-is-null-not-rejecting [2m(7.82ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-correlated-not-exists                [2m(7.57ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-in-iif                          [2m(2.86ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] divide-int-agg                           [2m(751.99µs)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-in-iif-false                    [2m(2.89ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] trim-float-returns-text                  [2m(476.82µs)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-in-ifnull-first-not-null        [2m(3.87ms)[0m
  [[32mPASS[0m] cte-chain-with-subquery                  [2m(3.32ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-with-modifier-datetime-neg      [2m(659.95µs)[0m

[1mtests/offset/memory.sqltest[0m
  [[32mPASS[0m] offset-expr-invalid-data-type-2          [2m(11.13ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] atan-float                               [2m(2.90ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-comparison-with-null            [2m(2.89ms)[0m
  [[32mPASS[0m] scalar-subquery-rowid-derived-table      [2m(2.64ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] plan-partial-index-full-scan-when-where-matched [2m(6.10ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-comparison-operators            [2m(3.66ms)[0m
  [[32mPASS[0m] subquery-is-distinct-from                [2m(3.15ms)[0m
  [[32mPASS[0m] cte-union-with-subquery-ref              [2m(4.21ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] divide-agg-float                         [2m(756.57µs)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-count-distinct                  [2m(17.91ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] if-alias-multi-args-falls-to-else        [2m(533.18µs)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-in-max-scalar                   [2m(2.93ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] rowid-column-overrides-rowid-alias       [2m(7.95ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-null-coalesce           [2m(5.76ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-coalesce-nullif-iif-typeof           [2m(7.04ms)[0m

[1mtests/left-join-case-iif-null-masking.sqltest[0m
  [[32mPASS[0m] left-join-iif-is-null-not-rejecting      [2m(8.05ms)[0m

[1mtests/multi_index_or_adversarial_extra.sqltest[0m
  [[32mPASS[0m] left-join-or-with-weight-filter          [2m(13.14ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-correlated-outer-ref-in-group-by-rowid-alias-oid [2m(10.66ms)[0m
  [[32mPASS[0m] subquery-correlated-outer-ref-in-group-by-rowid-alias-underscore [2m(10.58ms)[0m
  [[32mPASS[0m] subquery-correlated-in-group-by-with-join [2m(13.51ms)[0m
  [[32mPASS[0m] subquery-correlated-in-having-grouped    [2m(5.24ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] divide-float-agg                         [2m(810.05µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-with-modifier-datetime-large    [2m(655.92µs)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-uncorrelated-in-having          [2m(6.97ms)[0m
  [[32mPASS[0m] subquery-correlated-outer-ref-in-group-by-in-subquery [2m(14.78ms)[0m
  [[32mPASS[0m] subquery-correlated-outer-ref-in-group-by-function-call [2m(14.99ms)[0m
  [[32mPASS[0m] subquery-correlated-in-order-by          [2m(11.33ms)[0m
  [[32mPASS[0m] subquery-correlated-in-having-grouped-in-subquery [2m(5.73ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] plan-partial-index-not-used-when-where-not-implied [2m(5.45ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-uncorrelated-in-order-by-in     [2m(14.21ms)[0m
  [[32mPASS[0m] subquery-correlated-in-having-grouped-scalar-null [2m(8.93ms)[0m
  [[32mPASS[0m] subquery-correlated-outer-ref-in-nested-group-by-subquery [2m(14.83ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] atan-str                                 [2m(3.04ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] if-alias-multi-args-no-else-is-null      [2m(490.69µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-cte-order-ignored-by-parent-group    [2m(6.05ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-correlated-in-order-by-grouped-desc [2m(5.84ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-blob                    [2m(5.47ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] divide-agg-int-agg-int-2                 [2m(756.18µs)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-in-min-scalar                   [2m(2.86ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-correlated-in-order-by-grouped-null [2m(8.33ms)[0m
  [[32mPASS[0m] subquery-multiple-columns-in-select      [2m(4.73ms)[0m
  [[32mPASS[0m] subquery-uncorrelated-in-having-in       [2m(7.06ms)[0m

[1mtests/subquery/materialized_cte_seek.sqltest[0m
  [[32mPASS[0m] cte-aggregation-results                  [2m(26.32ms)[0m
  [[32mPASS[0m] cte-referenced-multiple-times-results    [2m(26.18ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-in-offset                       [2m(6.65ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] ltrim-integer-returns-text               [2m(515.29µs)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-in-limit                        [2m(7.15ms)[0m

[1mtests/left-join-case-iif-null-masking.sqltest[0m
  [[32mPASS[0m] left-join-case-when-is-not-null-not-rejecting [2m(7.57ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] replace-test-zeroblob-quote              [2m(679.42µs)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-vector-in-where                 [2m(6.31ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] _rowid_-column-overrides-rowid-alias     [2m(11.04ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-vector-in-having                [2m(6.87ms)[0m
  [[32mPASS[0m] subquery-vector-in-limit                 [2m(6.55ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] divide-agg-float-agg-float               [2m(782.52µs)[0m

[1mtests/multi_index_or_adversarial_extra.sqltest[0m
  [[32mPASS[0m] left-join-compound-or-branch-residuals   [2m(12.04ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] atan-null                                [2m(2.97ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-vector-in-offset                [2m(7.50ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-concat                          [2m(3.02ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] plan-partial-index-equality-predicate    [2m(6.49ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-cte-scalar-sub-everywhere            [2m(7.97ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-printf                  [2m(5.71ms)[0m

[1mtests/offset/memory.sqltest[0m
  [[32mPASS[0m] offset-expr-invalid-data-type-3          [2m(19.32ms)[0m

[1mtests/temp_tables.sqltest[0m
  [[32mPASS[0m] create-temp-table-qualified-main-is-rejected [2m(2.58ms)[0m
  [[32mPASS[0m] create-temp-table-behaves-like-temp-schema [2m(9.85ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-empty-string-null       [2m(6.02ms)[0m

[1mtests/temp_tables.sqltest[0m
  [[32mPASS[0m] create-table-explicit-temp-schema        [2m(9.44ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] order-by-column-number                   [2m(174.76ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-in-like                         [2m(2.83ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-count-distinct-age              [2m(74.60ms)[0m

[1mtests/temp_tables.sqltest[0m
  [[32mPASS[0m] temp-table-shadows-main-for-unqualified-lookups [2m(13.41ms)[0m
  [[32mPASS[0m] temp-table-shadowing-applies-to-update-and-delete [2m(12.37ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-mixed-types              [2m(7.99ms)[0m

[1mtests/temp_tables.sqltest[0m
  [[32mPASS[0m] temp-and-main-can-share-table-name-both-directions [2m(21.49ms)[0m
  [[32mPASS[0m] insert-into-temp-select-from-temp-self   [2m(12.87ms)[0m
  [[32mPASS[0m] create-temp-table-if-not-exists          [2m(28.18ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-in-like-no-match                [2m(2.95ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-abort-notnull                  [2m(4.50ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-string-concat-numbers   [2m(7.24ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-column-is-correlated-subquery        [2m(21.18ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] plan-partial-index-range-predicate       [2m(6.30ms)[0m

[1mtests/subquery/subquery_cte_equivalence_tests.sqltest[0m
  [[32mPASS[0m] banana-report-digest-sections-subquery   [2m(37.25ms)[0m

[1mtests/temp_tables.sqltest[0m
  [[32mPASS[0m] create-temp-table-as-select              [2m(11.31ms)[0m

[1mtests/subquery/subquery_cte_equivalence_tests.sqltest[0m
  [[32mPASS[0m] banana-report-streak-longest-cte         [2m(42.42ms)[0m
  [[32mPASS[0m] banana-report-digest-sections-cte        [2m(39.96ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] rtrim-integer-returns-text               [2m(536.42µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] tan-int                                  [2m(3.03ms)[0m

[1mtests/temp_tables.sqltest[0m
  [[32mPASS[0m] create-index-on-temp-table-uses-temp-schema [2m(11.99ms)[0m
  [[32mPASS[0m] temp-schema-visible-only-via-temp-sqlite-schema [2m(15.57ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-where-filters-both-sides [2m(8.54ms)[0m

[1mtests/temp_tables.sqltest[0m
  [[32mPASS[0m] sqlite-temp-schema-aliases-are-supported [2m(15.60ms)[0m
  [[32mPASS[0m] sqlite-master-remains-main-schema-with-temp-present [2m(16.46ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-in-glob                         [2m(3.82ms)[0m

[1mtests/temp_tables.sqltest[0m
  [[32mPASS[0m] temp-and-main-can-share-index-name       [2m(14.06ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-abort-notnull-verify-rollback  [2m(4.93ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-cte-as-config-variable               [2m(6.98ms)[0m

[1mtests/multi_index_or_adversarial_extra.sqltest[0m
  [[32mPASS[0m] left-join-compound-or-null-extended-antijoin [2m(17.74ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-round-precision         [2m(5.73ms)[0m

[1mtests/temp_tables.sqltest[0m
  [[32mPASS[0m] drop-table-prefers-temp-schema           [2m(12.58ms)[0m
  [[32mPASS[0m] drop-index-prefers-temp-schema           [2m(15.89ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] trim-integer-with-pattern-returns-text   [2m(510.74µs)[0m

[1mtests/temp_tables.sqltest[0m
  [[32mPASS[0m] drop-table-explicit-temp-schema          [2m(10.76ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] plan-partial-index-range-not-implied     [2m(6.35ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-count-distinct-age              [2m(101.32ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] tan-float                                [2m(2.99ms)[0m

[1mtests/temp_tables.sqltest[0m
  [[32mPASS[0m] pragma-temp-wal-checkpoint-without-temp-db-matches-sqlite [2m(2.86ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] round-float-no-precision                 [2m(586.37µs)[0m

[1mtests/subquery/subquery_cte_equivalence_tests.sqltest[0m
  [[32mPASS[0m] banana-activity-streak-subquery          [2m(60.02ms)[0m

[1mtests/temp_tables.sqltest[0m
  [[32mPASS[0m] alter-temp-table-add-column              [2m(11.08ms)[0m
  [[32mPASS[0m] temp-store-change-survives-rollback-before-temp-db-exists [2m(3.05ms)[0m
  [[32mPASS[0m] temp-store-memory-still-supports-temp-tables [2m(9.40ms)[0m

[1mtests/reindex.sqltest[0m
  [[32mPASS[0m] reindex-table-without-indexes            [2m(5.08ms)[0m

[1mtests/temp_tables.sqltest[0m
  [[32mPASS[0m] temp-table-join-can-distinguish-main-and-temp [2m(11.49ms)[0m
  [[32mPASS[0m] database-list-includes-temp-after-temp-init [2m(8.98ms)[0m
  [[32mPASS[0m] temp-store-file-still-supports-temp-tables [2m(9.38ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-in-glob-no-match                [2m(2.77ms)[0m

[1mtests/subquery/subquery_cte_equivalence_tests.sqltest[0m
  [[32mPASS[0m] banana-report-streak-current-cte         [2m(66.90ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] tan-str                                  [2m(3.06ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] replace-test-zeroblob-quote-2            [2m(475.88µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-with-expressions-in-select [2m(8.24ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-vector-in-order-by              [2m(6.86ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-abort-unique                   [2m(5.21ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-distinct-modulo-cte                  [2m(6.73ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-glob-pattern            [2m(5.66ms)[0m

[1mtests/temp_tables.sqltest[0m
  [[32mPASS[0m] temp-store-change-inside-active-temp-transaction-errors [2m(9.87ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] round-float-with-precision               [2m(507.78µs)[0m

[1mtests/temp_tables.sqltest[0m
  [[32mPASS[0m] temp-store-change-outside-transaction-resets-temp-schema [2m(15.63ms)[0m
  [[32mPASS[0m] multi-ddl-in-temp-transaction-preserves-data [2m(12.10ms)[0m

[1mtests/subquery/subquery_cte_equivalence_tests.sqltest[0m
  [[32mPASS[0m] banana-report-streak-current-cte-rich    [2m(75.96ms)[0m

[1mtests/temp_tables.sqltest[0m
  [[32mPASS[0m] full-txn-rollback-restores-temp-schema   [2m(11.95ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] divide-agg-int-agg-float-2               [2m(879.15µs)[0m

[1mtests/subquery/subquery_cte_equivalence_tests.sqltest[0m
  [[32mPASS[0m] banana-report-streak-longest-subquery    [2m(79.78ms)[0m

[1mtests/temp_tables_mvcc.sqltest[0m
  [[32mPASS[0m] pragma-temp-journal-mode-mvcc-is-ignored [2m(12.12ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-lhs-in-like                     [2m(2.95ms)[0m

[1mtests/temp_tables_mvcc.sqltest[0m
  [[32mPASS[0m] begin-concurrent-still-fails-after-temp-journal-mode-mvcc [2m(14.34ms)[0m

[1mtests/multi_index_or_adversarial_extra.sqltest[0m
  [[32mPASS[0m] left-join-compound-or-correlated-residuals [2m(15.64ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] tan-null                                 [2m(3.00ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-seek-correctness           [2m(7.03ms)[0m

[1mtests/temp_trigger.sqltest[0m
  [[32mPASS[0m] temp-trigger-in-temp-schema              [2m(11.09ms)[0m
  [[32mPASS[0m] temp-trigger-insert-on-main-table        [2m(12.24ms)[0m
  [[32mPASS[0m] temp-trigger-on-temp-table               [2m(12.22ms)[0m

[1mtests/reindex.sqltest[0m
  [[32mPASS[0m] reindex-qualified-table                  [2m(7.19ms)[0m

[1mtests/temp_trigger.sqltest[0m
  [[32mPASS[0m] temp-trigger-update-on-main-table        [2m(14.03ms)[0m

[1mtests/temp_tables_mvcc.sqltest[0m
  [[32mPASS[0m] temp-autoincrement-is-allowed-when-main-db-is-mvcc [2m(18.58ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] uuid-str-empty                           [2m(5.03ms)[0m

[1mtests/temp_trigger.sqltest[0m
  [[32mPASS[0m] temp-trigger-delete-on-main-table        [2m(14.43ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-abort-pk                       [2m(4.96ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] round-float-with-text-precision          [2m(580.14µs)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-vector-in-group-by              [2m(6.28ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-limit-chop-then-group-by             [2m(6.69ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-operator-precedence     [2m(5.62ms)[0m

[1mtests/total-changes.sqltest[0m
  [[32mPASS[0m] total-changes-on-basic-insert            [2m(5.54ms)[0m

[1mtests/temp_trigger.sqltest[0m
  [[32mPASS[0m] temp-trigger-cross-db-body               [2m(15.28ms)[0m

[1mtests/total-changes.sqltest[0m
  [[32mPASS[0m] total-changes-on-multiple-row-insert     [2m(4.70ms)[0m

[1mtests/temp_trigger.sqltest[0m
  [[32mPASS[0m] temp-trigger-orphan-on-drop-table        [2m(12.06ms)[0m
  [[32mPASS[0m] temp-trigger-if-not-exists               [2m(12.13ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-non-equi                 [2m(8.03ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] round-text-parsable                      [2m(857.65µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] divide-agg-float-agg-int-2               [2m(924.22µs)[0m

[1mtests/temp_trigger.sqltest[0m
  [[32mPASS[0m] temp-trigger-drop                        [2m(13.65ms)[0m

[1mtests/total-changes.sqltest[0m
  [[32mPASS[0m] total-changes-on-multiple-inserts        [2m(5.26ms)[0m

[1mtests/temp_trigger.sqltest[0m
  [[32mPASS[0m] temp-trigger-multiple-on-same-table      [2m(14.46ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] atanh-int                                [2m(2.68ms)[0m

[1mtests/temp_trigger.sqltest[0m
  [[32mPASS[0m] temp-trigger-when-clause                 [2m(12.81ms)[0m

[1mtests/total-changes.sqltest[0m
  [[32mPASS[0m] total-changes-on-update-single-row       [2m(5.64ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-arithmetic-all-ops              [2m(3.30ms)[0m

[1mtests/total-changes.sqltest[0m
  [[32mPASS[0m] total-changes-insert-in-transaction      [2m(4.88ms)[0m
  [[32mPASS[0m] total-changes-on-update-multiple-rows    [2m(5.78ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] bitwise-and-int-agg-int                  [2m(794.44µs)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-vector-in-case-when             [2m(6.84ms)[0m

[1mtests/temp_trigger.sqltest[0m
  [[32mPASS[0m] drop-table-main-preserves-temp-trigger-on-temp-shadow [2m(14.36ms)[0m

[1mtests/total-changes.sqltest[0m
  [[32mPASS[0m] total-changes-on-update-no-match         [2m(6.49ms)[0m

[1mtests/subquery/subquery_cte_equivalence_tests.sqltest[0m
  [[32mPASS[0m] banana-report-streak-current-subquery    [2m(101.83ms)[0m

[1mtests/temp_trigger.sqltest[0m
  [[32mPASS[0m] drop-table-main-preserves-temp-shadow-table-and-indexes [2m(14.25ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] round-text-non-parsable                  [2m(618.94µs)[0m

[1mtests/total-changes.sqltest[0m
  [[32mPASS[0m] total-changes-insert-in-transaction-after-commit [2m(4.98ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-text-between-range-scan              [2m(7.20ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-large-numbers           [2m(5.84ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-full-scan-correctness      [2m(6.95ms)[0m

[1mtests/reindex.sqltest[0m
  [[32mPASS[0m] reindex-qualified-index                  [2m(6.91ms)[0m

[1mtests/total-changes.sqltest[0m
  [[32mPASS[0m] total-changes-multiple-stmts-in-transaction [2m(5.62ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] atanh-float                              [2m(2.86ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] uuid-blob-empty                          [2m(5.02ms)[0m

[1mtests/transactions.sqltest[0m
  [[32mPASS[0m] basic-tx-3                               [2m(2.71ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-ignore-notnull                 [2m(4.98ms)[0m

[1mtests/subquery/materialized_cte_seek.sqltest[0m
  [[32mPASS[0m] cte-self-join-results                    [2m(28.72ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-bitwise-ops                     [2m(3.32ms)[0m

[1mtests/total-changes.sqltest[0m
  [[32mPASS[0m] total-changes-includes-fk-cascade-rows   [2m(7.47ms)[0m
  [[32mPASS[0m] total-changes-includes-trigger-body-rows [2m(7.70ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] bitwise-and-int-agg-float                [2m(811.58µs)[0m

[1mtests/multi_index_or_adversarial_extra.sqltest[0m
  [[32mPASS[0m] left-join-compound-or-correlated-residuals-antijoin [2m(15.87ms)[0m

[1mtests/transactions.sqltest[0m
  [[32mPASS[0m] basic-tx-2                               [2m(7.64ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-using-errors             [2m(10.93ms)[0m

[1mtests/transactions.sqltest[0m
  [[32mPASS[0m] basic-tx-1                               [2m(9.44ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-nullif-coalesce         [2m(5.96ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] round-integer-with-precision             [2m(595.75µs)[0m

[1mtests/transactions.sqltest[0m
  [[32mPASS[0m] temp-table-in-immediate-tx               [2m(9.67ms)[0m
  [[32mPASS[0m] temp-table-persists-after-immediate-tx   [2m(9.43ms)[0m

[1mtests/left-join-case-iif-null-masking.sqltest[0m
  [[32mPASS[0m] left-join-case-without-null-check-is-rejecting [2m(12.73ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-vector-in-case-then             [2m(6.20ms)[0m

[1mtests/transactions.sqltest[0m
  [[32mPASS[0m] temp-table-in-exclusive-tx               [2m(9.98ms)[0m

[1mtests/trigger-before-insert-affinity.sqltest[0m
  [[32mPASS[0m] basic-integer-to-real-affinity           [2m(7.81ms)[0m

[1mtests/transactions.sqltest[0m
  [[32mPASS[0m] temp-table-in-deferred-tx                [2m(9.69ms)[0m
  [[32mPASS[0m] temp-table-insert-rollback               [2m(9.53ms)[0m

[1mtests/trigger-before-insert-affinity.sqltest[0m
  [[32mPASS[0m] multi-column-affinity                    [2m(8.16ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-cross-join-two-ctes                  [2m(6.68ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] atanh-str                                [2m(2.91ms)[0m

[1mtests/trigger-before-insert-affinity.sqltest[0m
  [[32mPASS[0m] multi-column-values                      [2m(7.93ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] bitwise-and-int-agg-int-agg              [2m(775.08µs)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-unary-minus                     [2m(2.87ms)[0m

[1mtests/transactions.sqltest[0m
  [[32mPASS[0m] temp-and-main-table-in-immediate-tx      [2m(11.97ms)[0m

[1mtests/trigger-before-insert-affinity.sqltest[0m
  [[32mPASS[0m] null-value                               [2m(7.66ms)[0m
  [[32mPASS[0m] zero-value                               [2m(7.60ms)[0m

[1mtests/reindex.sqltest[0m
  [[32mPASS[0m] reindex-collation-nocase                 [2m(6.96ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] uuid7-timestamp-ms-empty                 [2m(5.31ms)[0m

[1mtests/trigger-before-insert-affinity.sqltest[0m
  [[32mPASS[0m] negative-integer                         [2m(8.78ms)[0m
  [[32mPASS[0m] when-clause-sees-affinity                [2m(7.50ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] round-float-negative-precision           [2m(572.66µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-left-join-on-predicate-can-prove-index [2m(8.59ms)[0m

[1mtests/trigger-before-insert-affinity.sqltest[0m
  [[32mPASS[0m] before-update-affinity                   [2m(8.18ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-ignore-unique                  [2m(5.51ms)[0m

[1mtests/trigger-before-insert-affinity.sqltest[0m
  [[32mPASS[0m] after-trigger-affinity                   [2m(7.55ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-iif-null-handling       [2m(5.58ms)[0m

[1mtests/trigger-before-insert-affinity.sqltest[0m
  [[32mPASS[0m] when-rowid-alias-keeps-integer-affinity  [2m(7.56ms)[0m
  [[32mPASS[0m] rowid-keeps-integer-affinity             [2m(7.57ms)[0m

[1mtests/trigger-last-insert-rowid.sqltest[0m
  [[32mPASS[0m] trigger-last-insert-rowid-basic          [2m(7.03ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-u-escape                          [2m(479.15µs)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-between-all-subqueries          [2m(2.96ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-vector-in-case-else             [2m(6.13ms)[0m

[1mtests/left-join-case-iif-null-masking.sqltest[0m
  [[32mPASS[0m] left-join-iif-is-not-null-not-rejecting  [2m(7.53ms)[0m

[1mtests/trigger-before-insert-affinity.sqltest[0m
  [[32mPASS[0m] new-columns-have-no-affinity             [2m(11.74ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-inner-grouping                  [2m(175.09ms)[0m

[1mtests/multi_index_or_adversarial_extra.sqltest[0m
  [[32mPASS[0m] mixed-and-or-on-clause                   [2m(11.85ms)[0m

[1mtests/trigger-last-insert-rowid.sqltest[0m
  [[32mPASS[0m] trigger-last-insert-rowid-side-effect    [2m(8.07ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] concat_ws-single-number                  [2m(525.66µs)[0m

[1mtests/trigger-before-insert-affinity.sqltest[0m
  [[32mPASS[0m] old-columns-have-no-affinity             [2m(13.28ms)[0m

[1mtests/trigger-last-insert-rowid.sqltest[0m
  [[32mPASS[0m] trigger-last-insert-rowid-multiple       [2m(9.58ms)[0m
  [[32mPASS[0m] trigger-last-insert-rowid-autoincrement  [2m(7.52ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] round-float-zero-precision               [2m(500.61µs)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] bitwise-or-int-agg                       [2m(781.63µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] atanh-null                               [2m(2.83ms)[0m

[1mtests/trigger-last-insert-rowid.sqltest[0m
  [[32mPASS[0m] trigger-last-insert-rowid-before-insert  [2m(7.61ms)[0m

[1mtests/reindex.sqltest[0m
  [[32mPASS[0m] reindex-collation-binary                 [2m(7.22ms)[0m

[1mtests/trigger-quoted-identifiers.sqltest[0m
  [[32mPASS[0m] trigger-with-hyphenated-identifiers      [2m(7.39ms)[0m
  [[32mPASS[0m] trigger-with-quoted-identifiers          [2m(7.52ms)[0m
  [[32mPASS[0m] trigger-with-reserved-word-identifiers   [2m(7.30ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-ignore-pk                      [2m(5.40ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-left-join-where-predicate-cannot-prove-index [2m(7.94ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-string-functions        [2m(5.46ms)[0m

[1mtests/trigger-quoted-identifiers.sqltest[0m
  [[32mPASS[0m] trigger-update-of-quoted-columns         [2m(8.57ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-five-cte-chain-join-all              [2m(13.47ms)[0m

[1mtests/trigger-quoted-identifiers.sqltest[0m
  [[32mPASS[0m] trigger-persists-across-multiple-inserts [2m(8.53ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-between-out-of-range            [2m(2.96ms)[0m

[1mtests/trigger-last-insert-rowid.sqltest[0m
  [[32mPASS[0m] trigger-last-insert-rowid-nested         [2m(10.80ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] concat_ws-null                           [2m(486.08µs)[0m
  [[32mPASS[0m] unistr-bare-hex                          [2m(470.93µs)[0m

[1mtests/trigger-virtual-table-innocuous.sqltest[0m
  [[32mPASS[0m] json-tree-in-trigger                     [2m(8.43ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-outer-grouping-cte              [2m(185.35ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] bitwise-and-int-agg-int-agg-2            [2m(737.87µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-no-on-clause             [2m(22.81ms)[0m

[1mtests/trigger-virtual-table-innocuous.sqltest[0m
  [[32mPASS[0m] json-each-nested-array-in-trigger        [2m(9.62ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-vector-in-aggregate-arg         [2m(6.27ms)[0m

[1mtests/subquery/materialized_cte_seek.sqltest[0m
  [[32mPASS[0m] cte-with-correlated-subquery-results     [2m(26.76ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] unary-plus-noop-blob                     [2m(3.00ms)[0m

[1mtests/trigger-virtual-table-innocuous.sqltest[0m
  [[32mPASS[0m] json-each-in-trigger                     [2m(11.84ms)[0m

[1mtests/left-join-case-iif-null-masking.sqltest[0m
  [[32mPASS[0m] left-join-case-multiple-when-branches    [2m(7.65ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] round-null-precision                     [2m(542.76µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-create-if-not-exists             [2m(6.52ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] ceil-int                                 [2m(2.88ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-drop-basic                       [2m(6.81ms)[0m
  [[32mPASS[0m] trigger-drop-if-exists                   [2m(6.30ms)[0m

[1mtests/trigger-virtual-table-innocuous.sqltest[0m
  [[32mPASS[0m] json-each-with-subquery-in-trigger       [2m(10.59ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-before-insert                    [2m(6.53ms)[0m

[1mtests/trigger-virtual-table-innocuous.sqltest[0m
  [[32mPASS[0m] json-each-before-trigger                 [2m(9.09ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] concat_ws-multiple                       [2m(530.87µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-after-insert                     [2m(7.14ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-plus-form                         [2m(457.28µs)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-not-between                     [2m(2.93ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-after-update                     [2m(8.62ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-replace-function        [2m(5.57ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-after-delete                     [2m(7.69ms)[0m
  [[32mPASS[0m] trigger-when-clause                      [2m(7.99ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-empty-zero-null-group-by             [2m(6.88ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-ignore-continue-after-skip     [2m(6.43ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-new-old-references               [2m(9.32ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] add-text-1                               [2m(2.77ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-drop-table-drops-triggers        [2m(10.29ms)[0m

[1mtests/reindex.sqltest[0m
  [[32mPASS[0m] reindex-partial-expression-index         [2m(8.82ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-create-basic                     [2m(16.81ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] length-text                              [2m(473.42µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-new-reference-in-subquery-orderby-limit [2m(9.57ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] ceil-float                               [2m(2.95ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-before-delete                    [2m(15.04ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-capital-U                         [2m(488.95µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-multiple-same-event              [2m(13.83ms)[0m
  [[32mPASS[0m] trigger-new-reference-in-subquery-offset [2m(9.99ms)[0m
  [[32mPASS[0m] trigger-new-reference-in-subquery-cte    [2m(8.18ms)[0m
  [[32mPASS[0m] trigger-new-reference-in-subquery        [2m(12.37ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] concat_ws-complex-args                   [2m(563.21µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-new-reference-in-values-subquery [2m(7.72ms)[0m
  [[32mPASS[0m] trigger-new-reference-in-subquery-join-on [2m(8.34ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] right-join-duplicate-keys                [2m(10.50ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-case-when-condition             [2m(2.71ms)[0m

[1mtests/multi_index_or_adversarial_extra.sqltest[0m
  [[32mPASS[0m] two-independent-or-joins                 [2m(23.44ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-recursive-after-insert           [2m(8.23ms)[0m

[1mtests/subquery/subquery_cte_equivalence_tests.sqltest[0m
  [[32mPASS[0m] banana-report-streak-current-subquery-rich [2m(165.72ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-before-update                    [2m(25.02ms)[0m
  [[32mPASS[0m] trigger-old-new-reference-in-subquery    [2m(11.19ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-backslash-escape                  [2m(516.37µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-update-of                        [2m(9.38ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] length-text-utf8-chars                   [2m(505.91µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-new-reference-in-subquery-compound [2m(12.86ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-abs-sign                [2m(10.21ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-new-reference-in-from-subquery   [2m(14.32ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] sin-products-id                          [2m(2.62ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-recursive-before-insert          [2m(7.15ms)[0m
  [[32mPASS[0m] trigger-update-of-multiple               [2m(9.98ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] ceil-str                                 [2m(2.91ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] concat-complex-first-arg                 [2m(552.93µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] add-text-2                               [2m(2.81ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-cte-bounds-filter                    [2m(9.84ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] rowid-alias-still-works-without-user-column [2m(4.85ms)[0m

[1mtests/left-join-ifnull-optimization.sqltest[0m
  [[32mPASS[0m] left-join-ifnull-not-null-rejecting      [2m(13.72ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-vector-in-binary-expr           [2m(14.36ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-when-complex                     [2m(11.18ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-ignore-no-conflict             [2m(14.27ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-indexed-by-left-join-where-predicate-rejected [2m(20.22ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-inner-grouping-cte              [2m(220.92ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-update-of-when                   [2m(9.45ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-max-i64-text-to-numeric             [2m(539.70µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-multiple-statements-before-insert [2m(9.40ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-case-insensitive-hex              [2m(523.38µs)[0m
  [[32mPASS[0m] length-integer                           [2m(530.76µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-multiple-statements-after-insert [2m(8.82ms)[0m

[1mtests/subquery/materialized_cte_seek.sqltest[0m
  [[32mPASS[0m] derived-table-join-results               [2m(27.34ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-multiple-statements-mixed-before-insert [2m(9.00ms)[0m
  [[32mPASS[0m] trigger-multiple-statements-after-update [2m(9.99ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] right-join-with-where                    [2m(7.76ms)[0m

[1mtests/reindex.sqltest[0m
  [[32mPASS[0m] reindex-composite-desc-index             [2m(11.16ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-nested                           [2m(19.04ms)[0m
  [[32mPASS[0m] trigger-multiple-statements-conditional  [2m(9.37ms)[0m
  [[32mPASS[0m] trigger-multiple-statements-before-update [2m(11.97ms)[0m
  [[32mPASS[0m] trigger-delete-inserted-row              [2m(9.37ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] mod-products-id                          [2m(858.75µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-recursive-insert                 [2m(6.98ms)[0m
  [[32mPASS[0m] trigger-multiple-statements-before-delete [2m(12.43ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] ceil-null                                [2m(2.91ms)[0m

[1mtests/multi_index_or_adversarial_extra.sqltest[0m
  [[32mPASS[0m] cte-dedup-both-branches-match            [2m(13.57ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] abs                                      [2m(471.66µs)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-case-simple                     [2m(2.88ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-coalesce-arithmetic     [2m(5.74ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-constraint-violation             [2m(7.96ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-multiple-escapes                  [2m(491.07µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-multiple-interdependent          [2m(8.26ms)[0m
  [[32mPASS[0m] trigger-multiple-statements-after-delete [2m(15.09ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] length-float                             [2m(512.19µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-update-old-reference             [2m(11.30ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-cte-array-indexing                   [2m(7.99ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-update-same-row                  [2m(15.94ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-negative-large-text-to-numeric      [2m(503.67µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-when-clause-modification         [2m(8.00ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] add-text-3                               [2m(2.94ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-delete-on-insert-condition       [2m(9.09ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] order-by-column-number-2                 [2m(162.10ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-indexed-by-left-join-null-rejecting-where-can-prove-after-rewrite [2m(8.51ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-old-new-arithmetic               [2m(12.21ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-vector-in-between               [2m(9.77ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-before-modify-primary-key        [2m(8.45ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] null-in-search                           [2m(11.11ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] ceiling-int                              [2m(1.88ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-circular-dependency              [2m(11.17ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] mod-products-price-id                    [2m(812.75µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-update-of-modify-same-column     [2m(11.41ms)[0m
  [[32mPASS[0m] trigger-update-of-nonexistent-column     [2m(6.71ms)[0m
  [[32mPASS[0m] trigger-chain-insert-update-delete       [2m(20.04ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-with-group-by            [2m(7.51ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-before-modify-new-in-when        [2m(10.90ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] length-null                              [2m(504.03µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-insert-new-rowid-alias           [2m(6.11ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-outer-grouping                  [2m(243.93ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-unixepoch-modifier                  [2m(507.90µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-rowid-alias-old-new              [2m(9.52ms)[0m
  [[32mPASS[0m] trigger-delete-old-rowid-alias           [2m(9.12ms)[0m
  [[32mPASS[0m] trigger-before-update-rowid-alias-when   [2m(9.77ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] limit-column-reference-error             [2m(2.94ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-multiple-modify-same-column      [2m(16.36ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-mixed                             [2m(504.02µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-update-rowid-complex-where       [2m(9.18ms)[0m
  [[32mPASS[0m] trigger-rowid-and-alias-mixed            [2m(8.20ms)[0m
  [[32mPASS[0m] trigger-modify-rowid-indirect            [2m(14.05ms)[0m

[1mtests/left-join-ifnull-optimization.sqltest[0m
  [[32mPASS[0m] left-join-coalesce-not-null-rejecting    [2m(14.87ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-modify-when-clause-column        [2m(19.68ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] remainder-float-agg                      [2m(780.01µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-unique-violation-ignore          [2m(19.79ms)[0m
  [[32mPASS[0m] trigger-before-modify-update-of-column   [2m(20.39ms)[0m
  [[32mPASS[0m] trigger-modify-column-multiple-times     [2m(15.44ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-null-to-any                         [2m(547.70µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-fake-factorial-self-join             [2m(9.98ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] ceiling-float                            [2m(2.87ms)[0m
  [[32mPASS[0m] add-text-4                               [2m(2.85ms)[0m

[1mtests/reindex.sqltest[0m
  [[32mPASS[0m] reindex-rtrim-and-implicit-column-collation [2m(17.36ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-ignore-omitted-notnull-no-default [2m(10.81ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-case-then-result                [2m(2.83ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-before-insert-modify-new         [2m(6.65ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] julianday-unixepoch-modifier             [2m(486.39µs)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-vector-in-cast                  [2m(6.47ms)[0m

[1mtests/multi_index_or_adversarial_extra.sqltest[0m
  [[32mPASS[0m] all-rows-match-both-branches             [2m(13.60ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-modify-unique-column             [2m(10.86ms)[0m
  [[32mPASS[0m] trigger-cannot-create-on-system-table    [2m(2.68ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] length-empty-text                        [2m(455.37µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-cannot-create-on-system-table-2  [2m(3.18ms)[0m
  [[32mPASS[0m] trigger-cascading-updates                [2m(12.62ms)[0m
  [[32mPASS[0m] trigger-recursive-different-rowid        [2m(7.16ms)[0m
  [[32mPASS[0m] trigger-update-of-multiple-modify-one    [2m(11.27ms)[0m
  [[32mPASS[0m] trigger-before-update-primary-key        [2m(7.81ms)[0m
  [[32mPASS[0m] trigger-before-update-parent-expression-old-values [2m(7.86ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-without-type-integer                [2m(467.74µs)[0m
  [[32mPASS[0m] abs-negative                             [2m(9.75ms)[0m

[1mtests/subquery/materialized_cte_seek.sqltest[0m
  [[32mPASS[0m] subquery-within-cte-results              [2m(22.44ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-modify-composite-unique          [2m(11.31ms)[0m
  [[32mPASS[0m] trigger-table-qualified-where-clause     [2m(6.93ms)[0m
  [[32mPASS[0m] trigger-multiple-before-insert-lifo      [2m(8.40ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-no-escapes                        [2m(496.92µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-unixepoch-modifier              [2m(501.06µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-indexed-by-without-predicate-rejected [2m(13.97ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] limit-hex-literal                        [2m(2.66ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-new-rowid-alias-when-clause      [2m(8.20ms)[0m
  [[32mPASS[0m] trigger-col-name-trigger-subquery        [2m(9.19ms)[0m
  [[32mPASS[0m] trigger-when-rowid-alias-string-literal-affinity [2m(7.52ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] ceiling-str                              [2m(2.79ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] octet-length-text                        [2m(480.31µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-with-aggregate-no-group-by [2m(7.57ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-case-else-result                [2m(2.83ms)[0m

[1mtests/reindex.sqltest[0m
  [[32mPASS[0m] reindex-unique-autoindex-and-null-keys   [2m(10.60ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-raise-abort-conditional-pass     [2m(6.15ms)[0m
  [[32mPASS[0m] trigger-when-becomes-false-after-before  [2m(18.30ms)[0m
  [[32mPASS[0m] trigger-after-insert-scalar-count-subquery [2m(7.62ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-case-expression-2       [2m(19.51ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-new-rowid-alias-when-clause-before [2m(15.03ms)[0m
  [[32mPASS[0m] trigger-drop-make-same-table             [2m(17.70ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-linear-regression-cte                [2m(10.63ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-raise-ignore-before-delete       [2m(6.41ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-vector-in-collate               [2m(9.60ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-raise-ignore-conditional         [2m(5.10ms)[0m
  [[32mPASS[0m] trigger-update-set-subquery              [2m(11.83ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] abs-char                                 [2m(465.79µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-raise-abort-after-insert         [2m(11.76ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] add-text-5                               [2m(2.92ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-unixepoch-negative-2            [2m(414.91µs)[0m

[1mtests/left-join-ifnull-optimization.sqltest[0m
  [[32mPASS[0m] left-join-nested-null-masking-not-null-rejecting [2m(5.43ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-indexed-by-count-star-without-predicate-counts-table [2m(6.18ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-case-all-subqueries             [2m(2.90ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-raise-ignore-multi-row           [2m(4.61ms)[0m
  [[32mPASS[0m] trigger-raise-ignore-before-insert       [2m(12.13ms)[0m
  [[32mPASS[0m] trigger-raise-ignore-after-insert        [2m(6.28ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] octet-length-text-utf8-chars             [2m(491.45µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] fail-raise-fail-outside-trigger          [2m(3.43ms)[0m
  [[32mPASS[0m] trigger-raise-abort-conditional-fail     [2m(15.09ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-auto-julianday                  [2m(363.51µs)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-ignore-omitted-notnull-continues [2m(22.45ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-when-strict-any-no-affinity      [2m(18.44ms)[0m
  [[32mPASS[0m] trigger-raise-abort-before-insert        [2m(17.47ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-without-type-text-numeric           [2m(513.85µs)[0m
  [[32mPASS[0m] unistr-empty-string                      [2m(534.95µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-raise-in-case-pass               [2m(6.30ms)[0m
  [[32mPASS[0m] fail-raise-rollback-outside-trigger      [2m(5.83ms)[0m
  [[32mPASS[0m] fail-raise-ignore-outside-trigger        [2m(5.34ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] abs-null                                 [2m(470.05µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] limit-underscore-literal                 [2m(2.75ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-raise-ignore-after-no-skip-postwork [2m(8.45ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-type-cast               [2m(6.26ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-raise-ignore-before-update       [2m(18.41ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-cte-union-with-itself                [2m(7.07ms)[0m

[1mtests/reindex.sqltest[0m
  [[32mPASS[0m] reindex-table-with-multiple-index-types  [2m(11.55ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] raise-shorthand-normal-query             [2m(5.73ms)[0m

[1mtests/left-join-ifnull-optimization.sqltest[0m
  [[32mPASS[0m] left-join-bare-column-null-rejecting     [2m(5.79ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-vector-in-is-null               [2m(6.24ms)[0m

[1mtests/multi_index_or_adversarial_extra.sqltest[0m
  [[32mPASS[0m] distinct-over-or-join                    [2m(16.97ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-raise-ignore-after-multi         [2m(8.62ms)[0m
  [[32mPASS[0m] trigger-raise-ignore-upsert              [2m(6.31ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-without-type-text-integer-string    [2m(484.33µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-multi-index-or-can-use-proven-partial-index [2m(8.56ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-omitted-notnull-no-default        [2m(4.51ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] remainder-int-agg                        [2m(32.07ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-raise-nested-success             [2m(10.13ms)[0m
  [[32mPASS[0m] trigger-raise-abort-before-update        [2m(15.68ms)[0m
  [[32mPASS[0m] trigger-raise-nested-ignore              [2m(10.55ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] ifnull-1                                 [2m(484.79µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-raise-shorthand                  [2m(14.26ms)[0m
  [[32mPASS[0m] trigger-raise-abort-before-delete        [2m(15.34ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-auto-unixepoch                  [2m(510.08µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-raise-rollback                   [2m(28.70ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] octet-length-integer                     [2m(522.72µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-no-column-reference     [2m(5.20ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-cast-to-integer                 [2m(2.80ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-raise-fail-conditional           [2m(6.34ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] multi-table-left-join-chain              [2m(11.86ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-raise-fail-basic                 [2m(11.72ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] ceiling-null                             [2m(3.38ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-raise-nested                     [2m(17.40ms)[0m
  [[32mPASS[0m] trigger-raise-ignore-preserves-trigger-body [2m(8.59ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-null-input                        [2m(535.74µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-raise-expr-concat                [2m(6.93ms)[0m
  [[32mPASS[0m] trigger-raise-expr-concat-pass           [2m(5.64ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] ifnull-2                                 [2m(551.95µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] julianday-auto                           [2m(465.90µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-raise-expr-case                  [2m(6.45ms)[0m
  [[32mPASS[0m] trigger-raise-fail-rolls-back-trigger-body [2m(14.60ms)[0m
  [[32mPASS[0m] trigger-raise-fail-after-trigger         [2m(13.59ms)[0m
  [[32mPASS[0m] trigger-raise-fail-delete                [2m(11.83ms)[0m

[1mtests/subquery/materialized_cte_seek.sqltest[0m
  [[32mPASS[0m] cte-referencing-previous-results         [2m(33.73ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-cast-to-text                    [2m(2.81ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] rowid-select-from-clause-subquery        [2m(12.57ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-agg-of-agg-with-subquery             [2m(7.51ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-multi-index-or-rejects-unproven-partial-index [2m(7.96ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-when-between-bug                 [2m(8.77ms)[0m
  [[32mPASS[0m] trigger-raise-fail-expr                  [2m(12.51ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] octet-length-float                       [2m(517.47µs)[0m

[1mtests/left-join-ifnull-optimization.sqltest[0m
  [[32mPASS[0m] left-join-is-null-not-null-rejecting     [2m(5.77ms)[0m

[1mtests/multi_index_or_adversarial_extra.sqltest[0m
  [[32mPASS[0m] or-join-group-by-having-degree           [2m(12.48ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-update-pk-change-bug             [2m(8.95ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-vector-in-not-null              [2m(6.29ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-nul-codepoint                     [2m(521.15µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-raise-rollback-expr              [2m(14.27ms)[0m
  [[32mPASS[0m] trigger-when-bare-column-error           [2m(13.01ms)[0m
  [[32mPASS[0m] trigger-returning-update-rejected        [2m(9.49ms)[0m
  [[32mPASS[0m] trigger-returning-insert-rejected        [2m(10.03ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] cos-int                                  [2m(3.59ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-auto                           [2m(500.01µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] iif-true                                 [2m(474.24µs)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-raise-fail-keeps-prior-rows      [2m(25.41ms)[0m
  [[32mPASS[0m] trigger-raise-in-case-fail               [2m(38.15ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-no-column-reference     [2m(5.39ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-raise-fail-update                [2m(25.24ms)[0m
  [[32mPASS[0m] trigger-before-insert-rowid-alias-auto-neg1 [2m(7.75ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] add-text-7                               [2m(2.93ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-when-exists-correlated           [2m(11.38ms)[0m
  [[32mPASS[0m] trigger-when-in-subquery                 [2m(12.48ms)[0m
  [[32mPASS[0m] trigger-when-not-in-subquery             [2m(13.18ms)[0m
  [[32mPASS[0m] trigger-before-insert-rowid-alias-explicit-vs-auto [2m(9.41ms)[0m
  [[32mPASS[0m] trigger-when-not-exists                  [2m(12.45ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] multi-table-left-join-t3-depends-on-t2   [2m(10.17ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-when-scalar-subquery             [2m(13.22ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] octet-length-null                        [2m(475.34µs)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-replace-unique                 [2m(5.99ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] rowid-select-from-clause-subquery-explicit-works [2m(4.80ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] remainder-agg-int                        [2m(18.91ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-when-subquery-old-ref            [2m(14.71ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-vector-in-like                  [2m(6.34ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-group-expr-having-order-diff-agg     [2m(6.54ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-before-insert-rowid-alias-multiple [2m(13.06ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-cast-to-real                    [2m(5.87ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-before-insert-rowid-alias-explicit-null [2m(9.43ms)[0m
  [[32mPASS[0m] trigger-when-subquery-update-old-new     [2m(16.08ms)[0m
  [[32mPASS[0m] trigger-autoincrement-before-insert-delete-first-row [2m(7.91ms)[0m
  [[32mPASS[0m] trigger-raise-fail-conditional-fires     [2m(34.55ms)[0m
  [[32mPASS[0m] trigger-before-insert-rowid-alias-in-expr [2m(14.04ms)[0m

[1mtests/reindex.sqltest[0m
  [[32mPASS[0m] reindex-attached-qualified-targets       [2m(13.25ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-returning-delete-rejected        [2m(20.56ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-without-type-text-real-string       [2m(564.40µs)[0m

[1mtests/partial_index_affinity.sqltest[0m
  [[32mPASS[0m] partial-index-affinity-insert            [2m(7.58ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] add-text-8                               [2m(2.81ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-before-insert-rowid-alias-when-neg1 [2m(17.01ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-auto                            [2m(3.55ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] before-update-trigger-panic-on-update    [2m(5.88ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] iif-false                                [2m(561.35µs)[0m

[1mtests/left-join-ifnull-optimization.sqltest[0m
  [[32mPASS[0m] left-join-is-between-rhs-columns-not-null-rejecting [2m(5.73ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-autoincrement-before-insert-delete-multiple-rows [2m(8.99ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-surrogate-rejected                [2m(10.60ms)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] trigger-explicit-main-qualification      [2m(6.38ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] cos-float                                [2m(6.04ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] trigger-autoincrement-before-insert-delete-sequence [2m(9.08ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] octet-length-empty-text                  [2m(502.35µs)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-is-null-true                    [2m(2.69ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-replace-pk                     [2m(5.10ms)[0m

[1mtests/trigger.sqltest[0m
  [[32mPASS[0m] selectC-2-1                              [2m(11.19ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] delete-returning-no-column-reference     [2m(9.64ms)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] drop-trigger-finds-attached-trigger-without-qualifier [2m(9.34ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-without-type-null                   [2m(463.98µs)[0m

[1mtests/multi_index_or_adversarial_extra.sqltest[0m
  [[32mPASS[0m] or-join-type-coercion                    [2m(10.87ms)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] trigger-same-db-allowed                  [2m(13.11ms)[0m

[1mtests/subquery/materialized_cte_seek.sqltest[0m
  [[32mPASS[0m] cte-correlated-range-probe-results       [2m(12.52ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] ambiguous-self-join                      [2m(4.86ms)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] trigger-cross-db-when-subquery-reject    [2m(15.90ms)[0m
  [[32mPASS[0m] trigger-temp-db-can-reference-main       [2m(13.56ms)[0m
  [[32mPASS[0m] trigger-cross-db-subquery-reject         [2m(19.20ms)[0m
  [[32mPASS[0m] trigger-cross-db-compound                [2m(12.30ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] add-text-9                               [2m(2.81ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] octet-length-date-binary-expr            [2m(559.13µs)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-vector-in-unary                 [2m(5.99ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] four-way-left-join-chain                 [2m(16.44ms)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] trigger-attached-db-cross-to-main-reject [2m(17.67ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-without-subsec                  [2m(414.31µs)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] trigger-cross-db-nested-subquery-in-insert [2m(18.39ms)[0m
  [[32mPASS[0m] trigger-attached-db-same-db              [2m(22.51ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] instr-str                                [2m(510.43µs)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] trigger-temp-db-same-db                  [2m(20.44ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-pure-literal-cte-aggregate           [2m(11.26ms)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] trigger-cross-db-from-subquery           [2m(15.24ms)[0m
  [[32mPASS[0m] trigger-cross-db-cte                     [2m(18.04ms)[0m
  [[32mPASS[0m] trigger-attached-db-explicit-self-qualification [2m(10.17ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-is-null-false                   [2m(2.83ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] cos-str                                  [2m(2.75ms)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] trigger-cross-attached-dbs               [2m(15.43ms)[0m
  [[32mPASS[0m] trigger-doubly-qualified-when-runtime-error [2m(18.13ms)[0m
  [[32mPASS[0m] trigger-cross-db-update-set-subquery     [2m(25.30ms)[0m
  [[32mPASS[0m] trigger-cross-db-in-select               [2m(20.22ms)[0m
  [[32mPASS[0m] trigger-cross-db-delete-where-subquery   [2m(26.35ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] cast-without-type-in-expression          [2m(525.28µs)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] trigger-doubly-qualified-expr-runtime-error [2m(22.12ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] ambiguous-column-error-message-format    [2m(8.98ms)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] trigger-cross-db-delete-trigger          [2m(12.75ms)[0m
  [[32mPASS[0m] trigger-cross-db-update-of-trigger       [2m(13.33ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-with-subsec                     [2m(519.94µs)[0m

[1mtests/left-join-ifnull-optimization.sqltest[0m
  [[32mPASS[0m] left-join-is-not-literal-not-null-rejecting [2m(6.26ms)[0m

[1mtests/math/default.sqltest[0m
  [[32mPASS[0m] remainder-agg-float                      [2m(18.33ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-sum-expression          [2m(4.88ms)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] trigger-cross-db-second-cmd              [2m(18.78ms)[0m

[1mtests/reindex.sqltest[0m
  [[32mPASS[0m] reindex-temp-shadows-main-unqualified    [2m(16.64ms)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] trigger-cross-db-case-subquery           [2m(13.16ms)[0m
  [[32mPASS[0m] trigger-cross-db-deep-nested             [2m(13.72ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] add-overflow-1                           [2m(2.85ms)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] trigger-cross-db-update-trigger          [2m(20.71ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-is-not-null-true                [2m(2.78ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] multi-table-left-join-all-unmatched      [2m(9.52ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] cos-null                                 [2m(2.85ms)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] trigger-cross-db-coalesce-subquery       [2m(12.21ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-above-max-rejected                [2m(13.04ms)[0m
  [[32mPASS[0m] cast-in-where                            [2m(960.79µs)[0m

[1mtests/subquery/materialized_cte_seek.sqltest[0m
  [[32mPASS[0m] cte-materialized-scalar-probe-results    [2m(16.84ms)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] trigger-cross-db-before-trigger          [2m(25.54ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] min-number                               [2m(568.55µs)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] trigger-cross-db-except                  [2m(23.25ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] instr-str-not-found                      [2m(504.01µs)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] trigger-cross-db-intersect               [2m(23.58ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-vector-in-function-call         [2m(6.22ms)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] trigger-cross-db-in-join-on-subquery     [2m(14.40ms)[0m
  [[32mPASS[0m] trigger-cross-db-join                    [2m(39.44ms)[0m
  [[32mPASS[0m] trigger-cross-db-select-where-subquery   [2m(12.79ms)[0m
  [[32mPASS[0m] trigger-cross-db-having-subquery         [2m(14.15ms)[0m
  [[32mPASS[0m] trigger-cross-db-limit-subquery          [2m(13.85ms)[0m
  [[32mPASS[0m] trigger-cross-db-orderby-subquery        [2m(14.24ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-floor-localtime                 [2m(633.50µs)[0m

[1mtests/math/degrees-radians-precision.sqltest[0m
  [[32mPASS[0m] degrees-pi                               [2m(2.57ms)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] trigger-cross-db-insert-select-from      [2m(13.64ms)[0m

[1mtests/partial_index_affinity.sqltest[0m
  [[32mPASS[0m] partial-index-affinity-insert-text-column [2m(30.94ms)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] trigger-attached-db-before-same-db       [2m(13.16ms)[0m

[1mtests/multi_index_or_adversarial_extra.sqltest[0m
  [[32mPASS[0m] or-of-and-pairs-bidirectional            [2m(25.24ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-replace-notnull-with-default   [2m(7.01ms)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] trigger-attached-db-delete-same-db       [2m(11.95ms)[0m
  [[32mPASS[0m] trigger-attached-db-update-same-db       [2m(12.30ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-coalesce-join-negation               [2m(9.07ms)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] trigger-attached-db-multi-cmd-same-db    [2m(12.97ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-sum-expression          [2m(7.12ms)[0m

[1mtests/left-join-ifnull-optimization.sqltest[0m
  [[32mPASS[0m] left-join-is-not-expression-not-null-rejecting [2m(6.52ms)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] trigger-attached-db-cte-same-db          [2m(12.77ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] duplicate-alias-ambiguous-column         [2m(10.43ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-is-not-null-false               [2m(2.91ms)[0m

[1mtests/trigger_fk_cascade.sqltest[0m
  [[32mPASS[0m] fk-cascade-after-trigger-sees-parent-deleted [2m(11.89ms)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] trigger-attached-db-subquery-same-db     [2m(14.71ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] cosh-int                                 [2m(2.83ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] instr-blob                               [2m(520.70µs)[0m
  [[32mPASS[0m] unistr-max-valid                         [2m(532.96µs)[0m

[1mtests/trigger_fk_cascade.sqltest[0m
  [[32mPASS[0m] fk-cascade-trigger-specific-parent-gone  [2m(12.03ms)[0m

[1mtests/trigger_attach_regression.sqltest[0m
  [[32mPASS[0m] trigger-attached-db-explicit-qual-subquery [2m(15.25ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] add-overflow-2                           [2m(5.75ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] parse-large-integral-numeric-string-as-number [2m(558.26µs)[0m

[1mtests/reindex.sqltest[0m
  [[32mPASS[0m] reindex-view-is-noop                     [2m(7.63ms)[0m

[1mtests/trigger_fk_cascade.sqltest[0m
  [[32mPASS[0m] fk-cascade-trigger-sees-parent-deleted   [2m(16.16ms)[0m

[1mtests/subquery/materialized_cte_seek.sqltest[0m
  [[32mPASS[0m] cte-seek-index-preserves-result-column-order [2m(4.29ms)[0m

[1mtests/trigger_fk_cascade.sqltest[0m
  [[32mPASS[0m] fk-cascade-trigger-multiple-parents      [2m(12.31ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-exists-in-aggregate-group-concat [2m(2.97ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] min-str                                  [2m(554.37µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] datetime-auto-start-of-month             [2m(504.32µs)[0m

[1mtests/trigger_fk_cascade.sqltest[0m
  [[32mPASS[0m] fk-cascade-final-state                   [2m(10.18ms)[0m
  [[32mPASS[0m] fk-set-null-trigger-sees-parent-deleted  [2m(12.10ms)[0m
  [[32mPASS[0m] fk-cascade-delete-writeback-trigger-no-rows-skipped [2m(10.60ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] multi-table-left-join-duplicate-hash-keys [2m(9.83ms)[0m

[1mtests/trigger_fk_cascade.sqltest[0m
  [[32mPASS[0m] fk-cascade-delete-trigger-inserted-rows-not-consumed [2m(10.27ms)[0m

[1mtests/trigger_namespace.sqltest[0m
  [[32mPASS[0m] trigger-same-name-as-table               [2m(7.34ms)[0m
  [[32mPASS[0m] trigger-if-not-exists-same-name-as-table [2m(7.56ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] instr-blob-not-found                     [2m(477.88µs)[0m

[1mtests/trigger_namespace.sqltest[0m
  [[32mPASS[0m] trigger-same-name-as-other-table         [2m(8.84ms)[0m

[1mtests/math/degrees-radians-precision.sqltest[0m
  [[32mPASS[0m] degrees-pi-half                          [2m(2.85ms)[0m

[1mtests/trigger_namespace.sqltest[0m
  [[32mPASS[0m] trigger-same-name-as-view                [2m(8.71ms)[0m
  [[32mPASS[0m] trigger-same-name-as-index               [2m(9.42ms)[0m

[1mtests/trigger_fk_cascade.sqltest[0m
  [[32mPASS[0m] fk-cascade-delete-two-hop-trigger-no-rows-skipped [2m(12.89ms)[0m

[1mtests/trigger_on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-ignore-pk-fires-before-trigger [2m(8.21ms)[0m

[1mtests/trigger_fk_cascade.sqltest[0m
  [[32mPASS[0m] fk-cascade-delete-grandchild-writeback-trigger-no-rows-skipped [2m(12.52ms)[0m
  [[32mPASS[0m] fk-cascade-delete-grandchild-trigger-inserted-rows-not-consumed [2m(12.60ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-replace-notnull-no-default     [2m(4.73ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-fn                                  [2m(835.91µs)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-empty-in-comparison             [2m(2.91ms)[0m

[1mtests/trigger_on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-ignore-unique-fires-before-trigger [2m(8.32ms)[0m
  [[32mPASS[0m] insert-or-ignore-no-conflict-fires-trigger [2m(7.90ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] unambiguous-self-join                    [2m(5.36ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] cosh-float                               [2m(2.86ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-julianday-floor                     [2m(524.85µs)[0m

[1mtests/trigger_namespace.sqltest[0m
  [[32mPASS[0m] multiple-triggers-sharing-names-with-tables [2m(11.48ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] parse-large-integral-numeric-string-as-number-2 [2m(541.72µs)[0m

[1mtests/trigger_on_conflict.sqltest[0m
  [[32mPASS[0m] upsert-do-update-trigger-insert-or-ignore-errors [2m(8.04ms)[0m
  [[32mPASS[0m] nested-insert-or-ignore-pk-in-trigger    [2m(11.07ms)[0m

[1mtests/partial_index_affinity.sqltest[0m
  [[32mPASS[0m] partial-index-affinity-update            [2m(10.10ms)[0m

[1mtests/trigger_on_conflict.sqltest[0m
  [[32mPASS[0m] nested-insert-or-ignore-unique-in-trigger [2m(11.82ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-column-same-as-table-name [2m(5.42ms)[0m

[1mtests/multi_index_or_adversarial_extra.sqltest[0m
  [[32mPASS[0m] chained-ctes-or-join-neighbor-extract    [2m(10.00ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] min-str-number                           [2m(523.29µs)[0m

[1mtests/trigger_on_conflict.sqltest[0m
  [[32mPASS[0m] upsert-do-update-trigger-insert-no-conflict [2m(8.18ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-divide-by-null-cte                   [2m(6.59ms)[0m

[1mtests/trigger_namespace.sqltest[0m
  [[32mPASS[0m] drop-trigger-with-same-name-as-table     [2m(15.53ms)[0m

[1mtests/subquery/materialized_cte_seek.sqltest[0m
  [[32mPASS[0m] cte-seek-index-limit-with-order-by       [2m(4.84ms)[0m

[1mtests/reindex.sqltest[0m
  [[32mPASS[0m] reindex-bracket-quoted-target            [2m(6.61ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] instr-null                               [2m(467.87µs)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-exists-in-aggregate-sum         [2m(2.80ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-error-invalid-escape              [2m(8.52ms)[0m

[1mtests/trigger_on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-ignore-pk-skips-after-trigger  [2m(8.51ms)[0m
  [[32mPASS[0m] trigger-upsert-new-column                [2m(8.25ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] add-overflow-3                           [2m(5.49ms)[0m
  [[32mPASS[0m] cosh-str                                 [2m(1.80ms)[0m

[1mtests/trigger_on_conflict.sqltest[0m
  [[32mPASS[0m] trigger-upsert-new-in-where              [2m(10.37ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-format-integer                  [2m(558.77µs)[0m

[1mtests/trigger_on_conflict.sqltest[0m
  [[32mPASS[0m] multi-level-triggers-insert-or-ignore    [2m(12.64ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-then-left-join                [2m(9.70ms)[0m

[1mtests/trigger_on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-replace-no-delete-trigger-by-default [2m(11.79ms)[0m
  [[32mPASS[0m] trigger-chain-insert-or-ignore-skipped   [2m(12.73ms)[0m
  [[32mPASS[0m] update-or-ignore-notnull-fires-before-trigger [2m(10.28ms)[0m
  [[32mPASS[0m] default-abort-does-not-override-trigger  [2m(6.85ms)[0m
  [[32mPASS[0m] update-abort-notnull-trigger-fires-then-errors [2m(8.28ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] length-999123                            [2m(560.75µs)[0m

[1mtests/trigger_on_conflict.sqltest[0m
  [[32mPASS[0m] trigger-upsert-new-in-where-no-update    [2m(8.14ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] where-like                               [2m(827.96µs)[0m

[1mtests/trigger_on_conflict.sqltest[0m
  [[32mPASS[0m] outer-replace-overrides-trigger-ignore   [2m(8.20ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] min-blob-number                          [2m(518.39µs)[0m

[1mtests/trigger_on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-ignore-unique-fires-before-trigger [2m(9.43ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-exists-in-aggregate-count       [2m(2.83ms)[0m

[1mtests/trigger_on_conflict.sqltest[0m
  [[32mPASS[0m] plain-update-trigger-insert-or-ignore-works [2m(18.92ms)[0m
  [[32mPASS[0m] outer-ignore-overrides-trigger-replace   [2m(10.79ms)[0m
  [[32mPASS[0m] update-or-ignore-notnull-no-violation    [2m(12.27ms)[0m
  [[32mPASS[0m] update-or-ignore-notnull-multiple-rows   [2m(12.24ms)[0m
  [[32mPASS[0m] outer-replace-propagates-before-trigger  [2m(11.82ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] order-by-limit-0                         [2m(4.86ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] instr-integer                            [2m(478.94µs)[0m

[1mtests/subquery/materialized_cte_seek.sqltest[0m
  [[32mPASS[0m] cte-materialized-hint-limit-with-order-by [2m(4.56ms)[0m

[1mtests/union_all.sqltest[0m
  [[32mPASS[0m] union-all-no-in-subquery                 [2m(7.65ms)[0m

[1mtests/trigger_on_conflict.sqltest[0m
  [[32mPASS[0m] outer-update-replace-overrides-trigger-ignore [2m(8.08ms)[0m

[1mtests/math/degrees-radians-precision.sqltest[0m
  [[32mPASS[0m] degrees-pi-quarter                       [2m(2.86ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-replace-multiple-conflicts     [2m(7.31ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-empty-aggregate-sum             [2m(12.18ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] not-exists-basic                         [2m(7.59ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] add-overflow-4                           [2m(2.79ms)[0m
  [[32mPASS[0m] cosh-null                                [2m(2.89ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] max-str-number                           [2m(496.03µs)[0m

[1mtests/union_all.sqltest[0m
  [[32mPASS[0m] union-all-with-in-subquery-two-cols      [2m(9.51ms)[0m
  [[32mPASS[0m] union-all-with-in-subquery-three-cols    [2m(9.07ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sum-1                                    [2m(552.12µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-column-same-as-table-name-unqualified [2m(6.79ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] exists-with-inner-filter                 [2m(7.82ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-running-total-correlated-join        [2m(10.73ms)[0m

[1mtests/trigger_on_conflict.sqltest[0m
  [[32mPASS[0m] outer-fail-overrides-trigger-ignore      [2m(18.25ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] order-by-column-number-3                 [2m(170.00ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] not-exists-with-inner-filter             [2m(7.07ms)[0m
  [[32mPASS[0m] exists-empty-inner                       [2m(6.73ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] instr-integer-not-found                  [2m(464.52µs)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] not-exists-empty-inner                   [2m(7.06ms)[0m
  [[32mPASS[0m] exists-basic                             [2m(12.50ms)[0m

[1mtests/partial_index_affinity.sqltest[0m
  [[32mPASS[0m] partial-index-affinity-upsert            [2m(9.40ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] exists-multi-correlation                 [2m(7.60ms)[0m
  [[32mPASS[0m] exists-in-and                            [2m(7.88ms)[0m

[1mtests/reindex.sqltest[0m
  [[32mPASS[0m] reindex-expression-index-after-rename-column [2m(11.69ms)[0m

[1mtests/union_all.sqltest[0m
  [[32mPASS[0m] union-all-where-literal                  [2m(16.06ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-exists-in-aggregate-with-table  [2m(4.92ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-format-integer-positive         [2m(499.76µs)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] not-exists-in-and                        [2m(8.05ms)[0m

[1mtests/multi_index_or_adversarial_extra.sqltest[0m
  [[32mPASS[0m] cross-table-or-in-on-clause              [2m(14.92ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] exists-duplicate-inner                   [2m(7.59ms)[0m

[1mtests/math/degrees-radians-precision.sqltest[0m
  [[32mPASS[0m] radians-180                              [2m(2.86ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] where-like-case-insensitive              [2m(3.85ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] max-blob-number                          [2m(494.23µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] left-hash-join-multi-predicate           [2m(8.10ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] exists-indexed                           [2m(8.07ms)[0m
  [[32mPASS[0m] not-exists-count                         [2m(7.24ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] group-by-limit-0                         [2m(6.85ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sum-2                                    [2m(463.81µs)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] exists-null-keys                         [2m(9.78ms)[0m
  [[32mPASS[0m] not-exists-null-keys                     [2m(9.81ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-empty-aggregate-count           [2m(3.07ms)[0m

[1mtests/subquery/materialized_cte_seek.sqltest[0m
  [[32mPASS[0m] cte-materialized-order-reused-same-direction [2m(7.00ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] not-exists-sum                           [2m(7.24ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] instr-integer-leading-zeros              [2m(445.89µs)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-replace-no-conflict            [2m(5.74ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] add-overflow-5                           [2m(2.86ms)[0m
  [[32mPASS[0m] degrees-int                              [2m(2.89ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] not-exists-group-by                      [2m(6.99ms)[0m
  [[32mPASS[0m] exists-self-join                         [2m(5.94ms)[0m
  [[32mPASS[0m] not-exists-self-join                     [2m(5.93ms)[0m
  [[32mPASS[0m] exists-order-by                          [2m(7.37ms)[0m
  [[32mPASS[0m] exists-group-by                          [2m(10.00ms)[0m
  [[32mPASS[0m] exists-count                             [2m(12.57ms)[0m

[1mtests/math/degrees-radians-precision.sqltest[0m
  [[32mPASS[0m] radians-90                               [2m(2.76ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-exists-false-in-aggregate       [2m(2.88ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] two-exists                               [2m(9.19ms)[0m
  [[32mPASS[0m] not-exists-order-by                      [2m(10.87ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] where-like-underscore                    [2m(796.34µs)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] two-not-exists                           [2m(10.14ms)[0m
  [[32mPASS[0m] not-exists-and-exists                    [2m(8.77ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-format-integer-negative         [2m(533.79µs)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] exists-and-not-exists                    [2m(10.13ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] min-null                                 [2m(458.85µs)[0m
  [[32mPASS[0m] unistr-error-short-hex                   [2m(16.50ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-cte-chain-filter-back                [2m(9.79ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] two-exists-count                         [2m(9.82ms)[0m
  [[32mPASS[0m] exists-and-not-exists-count              [2m(9.59ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sum-3                                    [2m(533.00µs)[0m

[1mtests/reindex.sqltest[0m
  [[32mPASS[0m] reindex-expression-index-explicit-collate [2m(7.43ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] not-exists-indexed                       [2m(20.25ms)[0m

[1mtests/partial_index_affinity.sqltest[0m
  [[32mPASS[0m] expression-index-affinity                [2m(7.99ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] duplicate-with-cte-name                  [2m(4.59ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] join-with-exists                         [2m(12.84ms)[0m
  [[32mPASS[0m] join-with-not-exists                     [2m(12.29ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] instr-real                               [2m(511.27µs)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-scalar-count-fast-path          [2m(5.17ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] join-with-two-not-exists                 [2m(13.51ms)[0m
  [[32mPASS[0m] left-join-with-exists                    [2m(12.83ms)[0m
  [[32mPASS[0m] left-join-on-exists-correlated           [2m(8.70ms)[0m
  [[32mPASS[0m] join-with-exists-and-not-exists          [2m(13.51ms)[0m
  [[32mPASS[0m] left-join-on-not-exists-correlated-both-sides [2m(8.92ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] degrees-float                            [2m(2.78ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] left-join-on-not-exists-correlated-with-rhs-filter [2m(9.65ms)[0m
  [[32mPASS[0m] left-join-on-exists-correlated-with-rhs-filter [2m(10.05ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-exists-in-aggregate-expression  [2m(2.89ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] add-overflow-6                           [2m(2.76ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] min-with-null                            [2m(520.48µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-format-float                    [2m(528.27µs)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] exists-on-join-aggregate                 [2m(12.89ms)[0m

[1mtests/subquery/materialized_cte_seek.sqltest[0m
  [[32mPASS[0m] cte-materialized-order-reused-opposite-direction [2m(6.76ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-replace-multi-notnull-default  [2m(5.24ms)[0m

[1mtests/math/degrees-radians-precision.sqltest[0m
  [[32mPASS[0m] radians-45                               [2m(2.89ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] not-exists-volatile-function-not-unnested [2m(7.54ms)[0m
  [[32mPASS[0m] three-not-exists-with-gaps               [2m(11.89ms)[0m
  [[32mPASS[0m] three-not-exists                         [2m(12.46ms)[0m

[1mtests/multi_index_or_adversarial_extra.sqltest[0m
  [[32mPASS[0m] nested-subquery-or-join                  [2m(19.64ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sum-4                                    [2m(549.08µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-rowid-and-pk-alias      [2m(11.48ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] three-exists-mix                         [2m(12.32ms)[0m
  [[32mPASS[0m] exists-no-matching-outer-rows            [2m(11.87ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-table-info-equal-syntax           [2m(482.93µs)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-empty-in-coalesce               [2m(2.93ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] not-exists-all-match                     [2m(11.81ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] instr-real-not-found                     [2m(461.80µs)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] left-join-on-exists-correlated-rhs       [2m(18.80ms)[0m
  [[32mPASS[0m] not-exists-multi-table-cross-join        [2m(9.25ms)[0m
  [[32mPASS[0m] exists-multi-table-inner-join            [2m(9.58ms)[0m

[1mtests/reindex.sqltest[0m
  [[32mPASS[0m] reindex-expression-index-outer-collate-target [2m(5.93ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] left-join-not-exists-on-right-table      [2m(10.26ms)[0m
  [[32mPASS[0m] exists-multi-table-inner-join-count      [2m(9.75ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] cte-multiple-refs-basic                  [2m(3.75ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-format-float-negative           [2m(567.98µs)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] not-exists-multi-table-inner-join        [2m(10.21ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-tier-cross-join-comparison           [2m(11.05ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] left-join-on-not-exists-correlated       [2m(21.49ms)[0m
  [[32mPASS[0m] not-exists-with-inner-left-join          [2m(9.65ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] max-number                               [2m(511.67µs)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] exists-with-inner-left-join              [2m(9.39ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] where-like-underscore-case-insensitive   [2m(751.82µs)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-scalar-in-aggregate-sum         [2m(2.84ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] add-overflow-7                           [2m(2.74ms)[0m
  [[32mPASS[0m] degrees-str                              [2m(2.88ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] not-exists-limit-0-always-true           [2m(7.37ms)[0m
  [[32mPASS[0m] not-exists-inner-constant-false          [2m(6.62ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] instr-real-trailing-zeros                [2m(499.86µs)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] exists-limit-0-always-false              [2m(8.32ms)[0m
  [[32mPASS[0m] exists-with-outer-left-join-hash         [2m(11.59ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-table-info-call-syntax            [2m(471.00µs)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] not-exists-with-outer-left-join-hash     [2m(11.72ms)[0m
  [[32mPASS[0m] two-not-exists-with-outer-left-join-hash [2m(11.53ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-replace-multi-notnull-default-many-cols [2m(4.94ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sum-5                                    [2m(518.82µs)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] not-exists-inner-constant-null           [2m(7.29ms)[0m

[1mtests/subquery/materialized_cte_seek.sqltest[0m
  [[32mPASS[0m] cte-materialized-order-multi-col-same-direction [2m(6.04ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-limit-zero                      [2m(3.00ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] exists-under-or                          [2m(7.27ms)[0m
  [[32mPASS[0m] not-exists-under-or                      [2m(7.20ms)[0m
  [[32mPASS[0m] exists-is-true-wrapper                   [2m(7.15ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-format-null                     [2m(505.15µs)[0m

[1mtests/math/degrees-radians-precision.sqltest[0m
  [[32mPASS[0m] roundtrip-degrees-radians                [2m(2.92ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] not-exists-is-false-wrapper              [2m(7.19ms)[0m
  [[32mPASS[0m] exists-inner-outer-only-false            [2m(6.95ms)[0m
  [[32mPASS[0m] not-exists-is-true-wrapper               [2m(8.29ms)[0m
  [[32mPASS[0m] exists-case-wrapper                      [2m(7.26ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] where-like-fn                            [2m(865.15µs)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] not-exists-inner-outer-only-false        [2m(6.88ms)[0m
  [[32mPASS[0m] exists-is-false-wrapper                  [2m(10.92ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] instr-blob-2                             [2m(492.07µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] degrees-null                             [2m(2.65ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] max-str                                  [2m(514.48µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] add-overflow-8                           [2m(2.70ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] not-exists-non-equality-correlation      [2m(6.81ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-error-trailing-backslash          [2m(14.58ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] exists-inner-nested-exists               [2m(7.83ms)[0m

[1mtests/reindex.sqltest[0m
  [[32mPASS[0m] reindex-nocase-embedded-nul              [2m(7.45ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-scalar-in-aggregate-group-concat [2m(4.76ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] exists-and-not-exists-with-outer-left-join-hash [2m(20.20ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sum-6                                    [2m(522.43µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] cte-multiple-refs-self-join              [2m(7.64ms)[0m

[1mtests/multi_index_or_adversarial_extra.sqltest[0m
  [[32mPASS[0m] or-join-empty-inner-table                [2m(9.83ms)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-nested-aggregate-cte                 [2m(8.60ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-replace-notnull-default-respects-check [2m(4.98ms)[0m

[1mtests/unnest-exists.sqltest[0m
  [[32mPASS[0m] exists-non-equality-correlation          [2m(13.33ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-table-xinfo-call-syntax           [2m(609.46µs)[0m

[1mtests/update-fk-cascade-trigger-skips-rows.sqltest[0m
  [[32mPASS[0m] update-fk-set-null-trigger-mutates-parent [2m(13.09ms)[0m
  [[32mPASS[0m] update-fk-cascade-trigger-mutates-parent [2m(13.28ms)[0m
  [[32mPASS[0m] update-fk-set-null-after-update-trigger-on-child [2m(12.80ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-rowid-and-pk-alias      [2m(6.10ms)[0m

[1mtests/update-fk-cascade-trigger-skips-rows.sqltest[0m
  [[32mPASS[0m] update-fk-set-default-trigger-mutates-parent [2m(13.31ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-format-blob                     [2m(541.37µs)[0m

[1mtests/update-fk-cascade-trigger-skips-rows.sqltest[0m
  [[32mPASS[0m] update-fk-no-cascade-when-key-untouched  [2m(9.65ms)[0m
  [[32mPASS[0m] update-fk-cascade-virtual-gencol-before-key [2m(9.60ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] min-int-float                            [2m(458.34µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-basic-equijoin               [2m(8.49ms)[0m
  [[32mPASS[0m] update-from-unqualified-set-columns      [2m(8.08ms)[0m

[1mtests/update-fk-cascade-trigger-skips-rows.sqltest[0m
  [[32mPASS[0m] update-fk-cascade-virtual-gencol-before-key-multicol [2m(9.87ms)[0m

[1mtests/math/degrees-radians-precision.sqltest[0m
  [[32mPASS[0m] roundtrip-radians-degrees                [2m(2.95ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-limit-negative                  [2m(3.75ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-no-match-leaves-unchanged    [2m(7.60ms)[0m

[1mtests/update-fk-cascade-trigger-skips-rows.sqltest[0m
  [[32mPASS[0m] update-fk-cascade-before-update-trigger-on-child [2m(12.01ms)[0m
  [[32mPASS[0m] update-fk-multicol-cascade               [2m(13.33ms)[0m
  [[32mPASS[0m] update-fk-cascade-rowid-alias-parent     [2m(11.91ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] instr-blob-not-found-2                   [2m(472.75µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-empty-source                 [2m(6.81ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] where-not-like-and                       [2m(892.78µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] subtract-text-1                          [2m(2.83ms)[0m

[1mtests/subquery/materialized_cte_seek.sqltest[0m
  [[32mPASS[0m] cte-materialized-order-multi-col-all-flipped [2m(5.96ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] exp-int                                  [2m(2.79ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-null-join-key-no-match       [2m(8.35ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-table-xinfo-vtable                [2m(780.06µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-expression-mixing-sides      [2m(14.60ms)[0m
  [[32mPASS[0m] update-from-self-reference-unaliased-errors [2m(4.53ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sum-7                                    [2m(526.31µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-aggregated-subquery          [2m(9.04ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-replace-notnull-default-unique-index-consistent [2m(6.12ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-values-cte-source            [2m(6.53ms)[0m
  [[32mPASS[0m] update-from-self-reference-unaliased-case-insensitive-errors [2m(4.80ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] upper                                    [2m(484.36µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-extra-where-filter           [2m(16.92ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-error-non-hex-u                   [2m(8.15ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-self-join-aliased            [2m(6.88ms)[0m

[1mtests/reindex.sqltest[0m
  [[32mPASS[0m] reindex-unset-is-not-a-collation         [2m(9.18ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] where-like-or                            [2m(942.26µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-explicit-inner-join          [2m(11.25ms)[0m
  [[32mPASS[0m] update-from-with-cte                     [2m(9.24ms)[0m
  [[32mPASS[0m] update-from-join-using                   [2m(10.70ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-format-bool-true                [2m(534.79µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] max-int-float                            [2m(475.87µs)[0m

[1mtests/subquery/cte_materialization_adversarial.sqltest[0m
  [[32mPASS[0m] wtf-insert-select-cte                    [2m(10.57ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-natural-join                 [2m(11.02ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] subtract-text-2                          [2m(5.84ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-reference-updated-column [2m(5.80ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-returning-target             [2m(7.83ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-order-by-in-from-subquery       [2m(3.17ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-table-info-alt-name-equal-syntax  [2m(494.24µs)[0m

[1mtests/subquery/materialized_cte_seek.sqltest[0m
  [[32mPASS[0m] cte-materialized-order-multi-col-mixed-flips-cleanly [2m(5.92ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-returning-star               [2m(7.90ms)[0m

[1mtests/math/degrees-radians-precision.sqltest[0m
  [[32mPASS[0m] sin-radians-30                           [2m(3.24ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] sum-8                                    [2m(502.88µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] cte-multiple-refs-scalar-subquery        [2m(6.87ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-returning-complex-expressions [2m(8.33ms)[0m
  [[32mPASS[0m] update-from-returning-correlated-subquery-can-see-target [2m(8.51ms)[0m
  [[32mPASS[0m] update-from-set-subquery                 [2m(8.36ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] upper-number                             [2m(480.98µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-row-value-set                [2m(8.10ms)[0m
  [[32mPASS[0m] update-from-row-value-set-extra-parens   [2m(7.88ms)[0m
  [[32mPASS[0m] update-from-three-tables                 [2m(14.65ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] strftime-format-bool-false               [2m(515.56µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-set-subquery-may-reference-source-table [2m(7.86ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-inner-filter                    [2m(840.52µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-row-value-set-subquery       [2m(10.08ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] min-float-int                            [2m(473.23µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] exp-float                                [2m(2.79ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-target-alias                 [2m(7.85ms)[0m
  [[32mPASS[0m] update-from-target-alias-allows-source-alias-matching-base-name [2m(7.99ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-table-info-alt-name-call-syntax   [2m(493.28µs)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-replace-select-multi-notnull-default [2m(6.46ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-target-alias-allows-different-table-same-alias [2m(7.76ms)[0m
  [[32mPASS[0m] update-from-source-alias-collision-still-allows-source-only-column [2m(8.31ms)[0m
  [[32mPASS[0m] update-from-multiple-columns             [2m(32.34ms)[0m
  [[32mPASS[0m] update-from-returning-reuses-cte-snapshot-after-trigger-mutation [2m(11.40ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] iif-3-args-true                          [2m(479.91µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-returning-reuses-row-cte-snapshot-after-trigger-mutation [2m(11.31ms)[0m

[1mtests/math/degrees-radians-precision.sqltest[0m
  [[32mPASS[0m] cos-radians-60                           [2m(2.83ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-target-not-visible-in-join-on [2m(8.31ms)[0m

[1mtests/reindex.sqltest[0m
  [[32mPASS[0m] reindex-non-ascii-identifiers-use-ascii-case-folding [2m(8.72ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] upper-char                               [2m(463.83µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-target-not-visible-in-join-on-subquery [2m(8.56ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-offset-without-limit            [2m(3.22ms)[0m

[1mtests/subquery/materialized_cte_seek.sqltest[0m
  [[32mPASS[0m] cte-materialized-order-multi-col-partial-mismatch-still-sorts [2m(5.95ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-basic-string                      [2m(485.01µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-target-alias-allows-unaliased-self-reference [2m(6.94ms)[0m
  [[32mPASS[0m] update-from-target-alias-not-visible-in-join-on [2m(8.02ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] max-null                                 [2m(475.24µs)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-scalar-in-aggregate-with-table-ref [2m(5.22ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-inner-filter-cte                [2m(882.29µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-returning-may-not-reference-source-columns [2m(20.80ms)[0m
  [[32mPASS[0m] update-from-returning-subquery-can-see-cte [2m(17.26ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-reference-multiple-updated-columns [2m(5.67ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-duplicate-match-keeps-set-columns-consistent [2m(7.86ms)[0m
  [[32mPASS[0m] update-from-duplicate-match-updates-once [2m(8.52ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-function-table-info-alt-name      [2m(791.77µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-vtab-source-arguments        [2m(6.38ms)[0m
  [[32mPASS[0m] update-from-updates-indexed-column       [2m(10.67ms)[0m
  [[32mPASS[0m] update-from-source-alias-collision-still-allows-source-only-column-in-subquery [2m(17.45ms)[0m

[1mtests/math/degrees-radians-precision.sqltest[0m
  [[32mPASS[0m] tan-radians-45                           [2m(2.89ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] exp-str                                  [2m(2.81ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-rollback-autocommit            [2m(4.79ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] subtract-text-3                          [2m(5.82ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-attached-constant-where-zero [2m(10.48ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] upper-null                               [2m(486.45µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-attached-constant-where-literal-set [2m(11.22ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-error-non-hex-plus                [2m(8.32ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-before-trigger-raise-ignore-skips-row [2m(10.30ms)[0m
  [[32mPASS[0m] update-from-attached-constant-where-noop-set [2m(11.12ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] max-with-null                            [2m(510.36µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-attached-constant-where-one  [2m(12.03ms)[0m
  [[32mPASS[0m] update-from-left-join-source-graph       [2m(10.70ms)[0m
  [[32mPASS[0m] update-from-trigger-derived-source-limit-sees-old [2m(11.83ms)[0m
  [[32mPASS[0m] update-from-trigger-derived-source-sees-new [2m(12.32ms)[0m
  [[32mPASS[0m] update-from-right-join-source-graph      [2m(10.41ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-outer-filter                    [2m(915.78µs)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-function-table-info               [2m(816.76µs)[0m

[1mtests/reindex.sqltest[0m
  [[32mPASS[0m] reindex-unquoted-compound-keyword-target-rejected [2m(5.40ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-exists-in-order-by-aggregate    [2m(2.76ms)[0m

[1mtests/subquery/materialized_cte_seek.sqltest[0m
  [[32mPASS[0m] cte-materialized-order-prefix-match      [2m(5.85ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-scalar-limit-zero               [2m(5.24ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-nullif                  [2m(5.13ms)[0m

[1mtests/multi_index_or_adversarial_extra.sqltest[0m
  [[32mPASS[0m] four-way-or-join-two-hop-neighbors       [2m(45.81ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] cte-multiple-refs-where-subquery         [2m(13.65ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-triggers-see-updated-values  [2m(18.78ms)[0m
  [[32mPASS[0m] update-from-right-join-using-preserves-right-row-payload [2m(11.85ms)[0m
  [[32mPASS[0m] update-from-maintains-partial-index      [2m(11.83ms)[0m
  [[32mPASS[0m] update-from-indexed-by-target-hint-is-preserved [2m(22.87ms)[0m
  [[32mPASS[0m] update-from-or-ignore-skips-conflicting-row [2m(9.08ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-string-replacement                [2m(533.00µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] iif-3-args-false                         [2m(551.43µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] left-hash-join-spill-unmatched           [2m(92.19ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-composite-key-update-cascades-foreign-keys [2m(11.33ms)[0m
  [[32mPASS[0m] update-from-or-replace-deletes-conflicting-row [2m(9.49ms)[0m
  [[32mPASS[0m] update-from-source-null-violates-not-null [2m(7.16ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] lower                                    [2m(474.02µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-rowid-update-cascades-foreign-keys [2m(14.01ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] nullif                                   [2m(484.99µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] exp-null                                 [2m(2.74ms)[0m

[1mtests/math/degrees-radians-precision.sqltest[0m
  [[32mPASS[0m] degrees-zero                             [2m(2.82ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-outer-filter-cte                [2m(964.28µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-source-value-violates-check  [2m(7.42ms)[0m
  [[32mPASS[0m] update-from-strict-table-coerces-compatible-values [2m(7.85ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-vtab-table-info                   [2m(822.27µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-maintains-expression-index   [2m(15.99ms)[0m
  [[32mPASS[0m] update-from-updates-generated-columns    [2m(7.91ms)[0m
  [[32mPASS[0m] update-from-from-subquery-reads-target-snapshot [2m(6.46ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-string-substitution               [2m(549.52µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-snapshot-shift-predecessor   [2m(8.80ms)[0m
  [[32mPASS[0m] update-from-strict-table-rejects-incompatible-values [2m(11.19ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] subtract-text-4                          [2m(5.67ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-error-non-hex-capital-U           [2m(8.33ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-or-replace-cascades-foreign-key-deletes [2m(11.07ms)[0m
  [[32mPASS[0m] update-from-self-join-linked-list        [2m(6.63ms)[0m

[1mtests/reindex.sqltest[0m
  [[32mPASS[0m] reindex-unknown-target                   [2m(5.61ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-rollback-autocommit-no-partial [2m(5.28ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-affinity-coercion-at-insert  [2m(7.53ms)[0m
  [[32mPASS[0m] update-from-self-join-preserves-non-set-columns [2m(6.78ms)[0m
  [[32mPASS[0m] update-from-snapshot-reverse-three-rows  [2m(10.38ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] lower-number                             [2m(456.55µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-type-preservation-mixed-types [2m(9.01ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-without-alias                   [2m(852.80µs)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-scalar-in-order-by-aggregate    [2m(2.87ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-self-join-using-preserves-non-set-columns [2m(7.09ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-table-info-invalid-table          [2m(404.37µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-i64-extremes-through-ephemeral [2m(9.00ms)[0m

[1mtests/subquery/materialized_cte_seek.sqltest[0m
  [[32mPASS[0m] cte-materialized-order-aggregate-with-order-by [2m(6.07ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] nullif-2                                 [2m(501.30µs)[0m

[1mtests/math/degrees-radians-precision.sqltest[0m
  [[32mPASS[0m] radians-zero                             [2m(2.83ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-nullif-no-match         [2m(5.14ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-scalar-limit-negative           [2m(4.98ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-triple-self-join             [2m(12.20ms)[0m
  [[32mPASS[0m] update-from-snapshot-shift-successor     [2m(16.52ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] cte-multiple-refs-with-data              [2m(6.83ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-or-replace-cascading-unique-chain [2m(9.02ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-quote-plain-text                  [2m(495.70µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] floor-int                                [2m(3.16ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-or-replace-swap-unique-keys  [2m(9.27ms)[0m
  [[32mPASS[0m] update-from-cartesian-updates-once       [2m(7.60ms)[0m
  [[32mPASS[0m] update-from-or-ignore-skips-check-violation [2m(8.59ms)[0m
  [[32mPASS[0m] update-from-expression-index-partial-column-update [2m(11.12ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] lower-char                               [2m(488.10µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-window-function-in-from-subquery [2m(8.47ms)[0m
  [[32mPASS[0m] update-from-rowid-mutation               [2m(8.49ms)[0m
  [[32mPASS[0m] update-from-correlated-subquery-refs-from-source [2m(9.88ms)[0m
  [[32mPASS[0m] update-from-or-replace-rowid-collision   [2m(7.92ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] subtract-text-5                          [2m(2.89ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-three-indexes-maintained     [2m(15.30ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-multiple-strings                  [2m(496.24µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-before-trigger-modifies-target-row [2m(12.68ms)[0m
  [[32mPASS[0m] update-from-natural-join-after-join-is-ambiguous [2m(6.57ms)[0m

[1mtests/math/degrees-radians-precision.sqltest[0m
  [[32mPASS[0m] degrees-negative-pi                      [2m(2.85ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-without-alias-cte               [2m(1.01ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-rowid-mutation-conflict      [2m(11.94ms)[0m
  [[32mPASS[0m] update-from-snapshot-set-to-pre-update-aggregate [2m(24.25ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] iif-2-args-true                          [2m(555.96µs)[0m

[1mtests/returning-fk-constraint.sqltest[0m
  [[32mPASS[0m] insert-fk-violation-returning-star       [2m(7.09ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-no-pk-duplicate-target-rows  [2m(14.81ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-function-table-info-invalid-table [2m(666.78µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-or-ignore-skips-notnull-violation [2m(18.04ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] cte-transitive-deps-multi-ref            [2m(4.14ms)[0m

[1mtests/multi_index_or_adversarial_extra.sqltest[0m
  [[32mPASS[0m] subquery-driving-or-join                 [2m(11.82ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-duplicate-source-alias-using-is-ambiguous [2m(12.00ms)[0m
  [[32mPASS[0m] update-from-duplicate-source-alias-natural-is-ambiguous [2m(11.96ms)[0m
  [[32mPASS[0m] update-from-unqualified-set-rhs-is-ambiguous [2m(13.64ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-exists-in-order-by-aggregate-with-group [2m(12.94ms)[0m

[1mtests/subquery/materialized_cte_seek.sqltest[0m
  [[32mPASS[0m] cte-materialized-order-aggregate-opposite [2m(9.86ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] lower-null                               [2m(482.97µs)[0m
  [[32mPASS[0m] unistr-quote-null                        [2m(535.07µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] floor-float                              [2m(2.59ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-group-concat                 [2m(8.43ms)[0m
  [[32mPASS[0m] update-from-case-expression              [2m(11.25ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-no-alias-on-col                 [2m(899.57µs)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-join-two-subqueries             [2m(12.25ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-multi-tables                 [2m(11.04ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-nullif                  [2m(10.00ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-sequential-visibility        [2m(9.26ms)[0m
  [[32mPASS[0m] update-from-mixed-target-source-set      [2m(12.39ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-rollback-in-transaction        [2m(7.98ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-like-join                    [2m(8.15ms)[0m
  [[32mPASS[0m] update-from-dup-unaliased-comma          [2m(15.28ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-null-string                       [2m(508.84µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] subtract-text-7                          [2m(2.91ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-repeated-accumulate          [2m(10.02ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-vtab-table-info-invalid-table     [2m(694.89µs)[0m

[1mtests/math/degrees-radians-precision.sqltest[0m
  [[32mPASS[0m] radians-negative-180                     [2m(3.25ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] cte-multiple-deps                        [2m(3.28ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-double-natural-join-ok       [2m(12.33ms)[0m
  [[32mPASS[0m] update-from-numeric-affinity             [2m(8.41ms)[0m

[1mtests/returning-fk-constraint.sqltest[0m
  [[32mPASS[0m] insert-fk-violation-returning-specific-cols [2m(6.39ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-all-types                    [2m(9.55ms)[0m
  [[32mPASS[0m] update-from-multi-using-cols             [2m(10.36ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-quote-integer                     [2m(468.39µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] floor-str                                [2m(2.64ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-json-extract                 [2m(8.06ms)[0m
  [[32mPASS[0m] update-from-integer-affinity             [2m(10.75ms)[0m
  [[32mPASS[0m] update-from-mixed-types-union            [2m(12.88ms)[0m
  [[32mPASS[0m] update-from-complex-multi-col            [2m(19.62ms)[0m
  [[32mPASS[0m] update-from-savepoint-rollback           [2m(9.17ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] replace                                  [2m(489.12µs)[0m
  [[32mPASS[0m] iif-2-args-false-is-null                 [2m(482.94µs)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-no-alias-on-col-cte             [2m(946.15µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-snapshot-accumulate-stays-one [2m(7.75ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-legacy-file-format                [2m(349.54µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-trigger-cascading-update     [2m(11.73ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-column-affinity-inheritance     [2m(4.47ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] where-like-another-column                [2m(76.35ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-before-after-triggers        [2m(12.74ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] subtract-text-8                          [2m(2.84ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-before-trigger-raise-ignore-conditional [2m(9.64ms)[0m
  [[32mPASS[0m] update-from-fk-cascade-extra             [2m(13.05ms)[0m
  [[32mPASS[0m] update-from-trigger-deletes-source-mid-flight [2m(10.15ms)[0m

[1mtests/math/degrees-radians-precision.sqltest[0m
  [[32mPASS[0m] radians-360                              [2m(2.85ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-trigger-inserts-target-not-matched [2m(9.38ms)[0m

[1mtests/multi_index_or_adversarial_extra.sqltest[0m
  [[32mPASS[0m] three-branch-or-self-join                [2m(11.06ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-left-join-two-subqueries        [2m(4.14ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-coalesce-multiple       [2m(5.23ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-where-false                  [2m(15.67ms)[0m
  [[32mPASS[0m] update-from-glob-pattern-join            [2m(9.60ms)[0m
  [[32mPASS[0m] update-from-pragma-table-info-source     [2m(6.34ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-integer-as-string                 [2m(492.79µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-quote-float                       [2m(582.05µs)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-no-alias-on-col-named           [2m(907.42µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] cte-fan-out-refs                         [2m(3.85ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-rollback-rolls-back-entire-tx  [2m(5.46ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-source-rowid-in-set-expr     [2m(8.65ms)[0m
  [[32mPASS[0m] update-from-like-escape-join             [2m(10.96ms)[0m
  [[32mPASS[0m] update-from-left-join-coalesce           [2m(23.71ms)[0m
  [[32mPASS[0m] update-from-between-source-bounds        [2m(8.41ms)[0m
  [[32mPASS[0m] update-from-chain-rowid-remap            [2m(8.28ms)[0m
  [[32mPASS[0m] update-from-explicit-cross-join          [2m(10.85ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] floor-null                               [2m(2.86ms)[0m

[1mtests/returning-fk-constraint.sqltest[0m
  [[32mPASS[0m] delete-fk-violation-returning-star       [2m(7.19ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-rowid-alias-plus-column      [2m(7.88ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] subtract-text-9                          [2m(2.60ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-or-ignore-mutual-unique-conflict [2m(7.69ms)[0m
  [[32mPASS[0m] update-from-cte-shadows-source-is-circular [2m(6.72ms)[0m
  [[32mPASS[0m] update-from-dup-unaliased-cross-join     [2m(14.63ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-function-legacy-file-format       [2m(5.25ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-quote-with-tab                    [2m(553.16µs)[0m

[1mtests/update-returning-correlated-subquery.sqltest[0m
  [[32mPASS[0m] update-returning-correlated-subquery-basic [2m(7.40ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-is-not-distinct-from-join    [2m(15.32ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-no-alias-on-col-named-cte       [2m(934.10µs)[0m

[1mtests/update-returning-correlated-subquery.sqltest[0m
  [[32mPASS[0m] update-returning-correlated-subquery-multi-row [2m(8.47ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-column-affinity-inheritance-text [2m(4.79ms)[0m

[1mtests/update-returning-correlated-subquery.sqltest[0m
  [[32mPASS[0m] update-returning-correlated-subquery-unmodified-column [2m(8.32ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update_primary_key_unique_constraint_error [2m(2.62ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-before-trigger-new-is-source-value [2m(11.27ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update_primary_key_constraint_error_4    [2m(2.71ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-left-join-null-in-set        [2m(11.33ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-quote-with-newline                [2m(543.59µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-row-value-equality-join      [2m(18.15ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] replace-number                           [2m(495.11µs)[0m
  [[32mPASS[0m] iif-multi-args-finds-first-true          [2m(3.52ms)[0m

[1mtests/update-returning-correlated-subquery.sqltest[0m
  [[32mPASS[0m] update-returning-direct-and-subquery     [2m(7.39ms)[0m
  [[32mPASS[0m] update-returning-correlated-subquery-null-result [2m(7.39ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update_primary_key_constraint_error_2    [2m(7.18ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] cte-deep-chain-multi-ref                 [2m(4.15ms)[0m

[1mtests/update-returning-correlated-subquery.sqltest[0m
  [[32mPASS[0m] update-returning-multiple-correlated-subqueries [2m(7.93ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-set-expression-missing-identifier [2m(4.01ms)[0m
  [[32mPASS[0m] update-single-rowid                      [2m(5.59ms)[0m

[1mtests/subquery/materialized_cte_seek.sqltest[0m
  [[32mPASS[0m] cte-materialized-order-outer-beyond-cte-order [2m(11.59ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-cross-join-subqueries           [2m(4.52ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update_primary_key_constraint_error_3    [2m(7.68ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-rollback-verify-state-after-error [2m(5.09ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] ln-int                                   [2m(2.77ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-select-star                     [2m(967.54µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-coalesce-all-null       [2m(4.88ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] subtract-overflow-1                      [2m(2.78ms)[0m

[1mtests/multi_index_or_adversarial_extra.sqltest[0m
  [[32mPASS[0m] or-join-with-in-and-between              [2m(9.71ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-returning-basic-column            [2m(5.82ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-function-too-many-arguments       [2m(8.16ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-quote-backslash-no-ctrl           [2m(2.10ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-returning-literal                 [2m(5.69ms)[0m
  [[32mPASS[0m] update-returning-arithmetic              [2m(5.78ms)[0m

[1mtests/returning-fk-constraint.sqltest[0m
  [[32mPASS[0m] delete-fk-violation-returning-specific-cols [2m(7.29ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-not-in-text-primary-key-null    [2m(6.70ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-returning-mixed-expressions       [2m(5.86ms)[0m
  [[32mPASS[0m] update-returning-old-vs-new-values       [2m(5.99ms)[0m
  [[32mPASS[0m] update-returning-with-where              [2m(6.15ms)[0m
  [[32mPASS[0m] basic-row-values                         [2m(5.53ms)[0m
  [[32mPASS[0m] update-returning-multiple-rows           [2m(6.40ms)[0m
  [[32mPASS[0m] update-returning-null-values             [2m(5.95ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-select-star-cte                 [2m(1.03ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] parse-error-row-values                   [2m(4.76ms)[0m
  [[32mPASS[0m] update-returning-multiple-columns        [2m(11.71ms)[0m
  [[32mPASS[0m] row-values-repeated-values-should-take-latter [2m(4.59ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] cte-join-on-clause                       [2m(4.31ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] replace-null                             [2m(499.11µs)[0m
  [[32mPASS[0m] unistr-quote-backslash-with-ctrl         [2m(435.65µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] subtract-overflow-2                      [2m(2.70ms)[0m
  [[32mPASS[0m] ln-float                                 [2m(2.85ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] basic-update                             [2m(5.23ms)[0m
  [[32mPASS[0m] update-returning-function-call           [2m(10.24ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-fail-autocommit                [2m(5.04ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-null-arithmetic         [2m(4.86ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] row-values-from-subquery-star            [2m(7.22ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-function-update                   [2m(4.97ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] row-values-from-subquery                 [2m(7.88ms)[0m
  [[32mPASS[0m] row-values-from-volatile-subquery-project-same-row [2m(5.59ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-natural-join-subqueries         [2m(3.63ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-select-table-star               [2m(950.13µs)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] can-update-rowid-directly                [2m(5.31ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-cascade-afters-through-three-tables [2m(30.12ms)[0m
  [[32mPASS[0m] update-from-dup-unaliased-triple         [2m(15.32ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-percent-escape                    [2m(502.61µs)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-alias-visibility-in-where-clause  [2m(5.93ms)[0m
  [[32mPASS[0m] update-with-trigger-and-index-scan-preserves-unchanged-indexed-columns [2m(7.11ms)[0m
  [[32mPASS[0m] update-returning-complex-expression      [2m(16.71ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-quote-single-quote-with-ctrl      [2m(460.45µs)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-non-indexed-column                [2m(7.21ms)[0m
  [[32mPASS[0m] update-returning-all-columns             [2m(20.42ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] subtract-overflow-3                      [2m(2.73ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] iif-multi-args-falls-to-else             [2m(530.80µs)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] row-values-from-subquery-star-width-mismatch [2m(9.41ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] ln-str                                   [2m(2.82ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-indexed-column                    [2m(7.59ms)[0m
  [[32mPASS[0m] update-both-indexed-and-non-indexed      [2m(7.67ms)[0m

[1mtests/returning-fk-constraint.sqltest[0m
  [[32mPASS[0m] update-fk-violation-returning-star       [2m(7.22ms)[0m

[1mtests/multi_index_or_adversarial_extra.sqltest[0m
  [[32mPASS[0m] delete-with-or-condition                 [2m(14.12ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] collation-rtrim-1                        [2m(2.92ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-range-non-indexed                 [2m(7.32ms)[0m
  [[32mPASS[0m] update-all-rows-with-index               [2m(8.24ms)[0m
  [[32mPASS[0m] update-multiple-indexes                  [2m(8.97ms)[0m
  [[32mPASS[0m] rowid-update-updates-all-indexes         [2m(13.53ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-select-table-star-cte           [2m(962.80µs)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-mul                               [2m(5.57ms)[0m
  [[32mPASS[0m] update-integer-pk-underflow-error        [2m(5.09ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-function-nontext-argument         [2m(781.41µs)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-range-both-columns                [2m(8.19ms)[0m
  [[32mPASS[0m] update-range-indexed-column              [2m(8.94ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-integer-replacement               [2m(471.78µs)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-fail-keeps-prior-changes       [2m(5.84ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-null-arithmetic         [2m(5.65ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-deeply-nested                   [2m(3.25ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] subtract-overflow-4                      [2m(2.88ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-range-multiple-indexes            [2m(9.71ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] order-by-column-number-4                 [2m(188.07ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-in-subquery-1                     [2m(8.35ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-quote-blob                        [2m(566.21µs)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-integer-pk-overflow-error         [2m(9.29ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] replace-wrong-arg-count-0                [2m(4.93ms)[0m
  [[32mPASS[0m] if-alias-3-args-true                     [2m(553.36µs)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-in-subquery-empty-1               [2m(7.23ms)[0m
  [[32mPASS[0m] update-not-in-subquery-empty-1           [2m(7.70ms)[0m
  [[32mPASS[0m] update-not-in-subquery-1                 [2m(8.31ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] correlated-left-join-exists              [2m(13.08ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-correlated-self-in-where          [2m(6.77ms)[0m
  [[32mPASS[0m] update-exists-subquery-1                 [2m(7.69ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-vtab-nontext-argument             [2m(825.13µs)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-correlated-self-not-in-where      [2m(6.58ms)[0m
  [[32mPASS[0m] update-correlated-self-not-exists-where  [2m(6.62ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] nested-subquery                          [2m(978.23µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-integer-negative                  [2m(490.69µs)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-correlated-self-rowvalue-where    [2m(6.68ms)[0m
  [[32mPASS[0m] update-correlated-self-scalar-cmp-where  [2m(7.27ms)[0m
  [[32mPASS[0m] update-not-exists-subquery-1             [2m(8.06ms)[0m
  [[32mPASS[0m] update-correlated-self-exists-where      [2m(11.74ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-nested-with-operations          [2m(3.09ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-in-text-primary-key-null        [2m(22.74ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] subtract-overflow-5                      [2m(2.73ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] collate-compound-1                       [2m(8.47ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-scalar-gt-subquery-1              [2m(10.31ms)[0m
  [[32mPASS[0m] update-scalar-eq-subquery-1              [2m(10.41ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-fail-partial-insert            [2m(5.96ms)[0m

[1mtests/returning-fk-constraint.sqltest[0m
  [[32mPASS[0m] update-fk-violation-returning-specific-cols [2m(8.46ms)[0m

[1mtests/update_expression_index.sqltest[0m
  [[32mPASS[0m] substr-expression-index                  [2m(10.16ms)[0m
  [[32mPASS[0m] real-column-cast-expression              [2m(8.39ms)[0m
  [[32mPASS[0m] update-to-same-value                     [2m(7.69ms)[0m
  [[32mPASS[0m] update-with-subquery-where               [2m(11.32ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-function-no-arguments             [2m(584.57µs)[0m

[1mtests/update_expression_index.sqltest[0m
  [[32mPASS[0m] insert-or-replace-expression-index       [2m(7.67ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-quote-matches-quote-no-ctrl       [2m(635.50µs)[0m

[1mtests/update_expression_index.sqltest[0m
  [[32mPASS[0m] modulo-expression-index                  [2m(7.91ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-string-func-null        [2m(7.32ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-integer-arithmetic-expression     [2m(495.19µs)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] nested-subquery-cte                      [2m(1.13ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] replace-wrong-arg-count-2                [2m(4.71ms)[0m

[1mtests/multi_index_or_adversarial_extra.sqltest[0m
  [[32mPASS[0m] update-with-or-condition                 [2m(14.21ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-natural-plus-join-ambiguous  [2m(18.37ms)[0m

[1mtests/update_expression_index_affinity.sqltest[0m
  [[32mPASS[0m] cast-expr-index-int-affinity             [2m(7.74ms)[0m

[1mtests/update_expression_index.sqltest[0m
  [[32mPASS[0m] update-unrelated-column                  [2m(15.25ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-correlated-from-clause-nested   [2m(5.59ms)[0m

[1mtests/update_expression_index_affinity.sqltest[0m
  [[32mPASS[0m] comparison-expr-index-text-affinity      [2m(8.00ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] if-alias-3-args-false                    [2m(508.10µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] left-hash-join-spill-verify-order        [2m(101.10ms)[0m

[1mtests/update_expression_index_affinity.sqltest[0m
  [[32mPASS[0m] cast-expr-index-multiple-updates         [2m(8.68ms)[0m
  [[32mPASS[0m] comparison-expr-index-numeric-affinity   [2m(8.15ms)[0m
  [[32mPASS[0m] expr-index-null-values                   [2m(8.47ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-where                             [2m(11.31ms)[0m

[1mtests/update_expression_index_affinity.sqltest[0m
  [[32mPASS[0m] expr-index-int-affinity-float-text       [2m(9.17ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-from-union                      [2m(3.63ms)[0m

[1mtests/update_expression_index_affinity.sqltest[0m
  [[32mPASS[0m] expr-index-multi-row                     [2m(9.91ms)[0m
  [[32mPASS[0m] comparison-expr-index-truth-value-changes [2m(8.52ms)[0m

[1mtests/update_index_affinity.sqltest[0m
  [[32mPASS[0m] update-index-affinity-nullif-expression  [2m(7.74ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-vtab-no-arguments                 [2m(563.53µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-text-as-integer                   [2m(485.81µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] subtract-overflow-6                      [2m(2.87ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-not-in-integer-primary-key-null [2m(5.53ms)[0m

[1mtests/update_index_affinity.sqltest[0m
  [[32mPASS[0m] update-index-affinity-text-to-real       [2m(10.15ms)[0m
  [[32mPASS[0m] update-index-affinity-non-unique         [2m(7.35ms)[0m

[1mtests/update_expression_index_affinity.sqltest[0m
  [[32mPASS[0m] cast-expr-index-numeric-literal-lookup   [2m(10.58ms)[0m

[1mtests/update_or_replace_rowid_secondary_index.sqltest[0m
  [[32mPASS[0m] update-or-replace-conflict-delete-changes-count [2m(6.45ms)[0m

[1mtests/update_index_affinity.sqltest[0m
  [[32mPASS[0m] update-index-affinity-multi-column       [2m(9.98ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-quote-ignores-control-after-nul   [2m(665.74µs)[0m

[1mtests/update_or_replace_rowid_secondary_index.sqltest[0m
  [[32mPASS[0m] update-rowid-through-secondary-index-scan [2m(8.41ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-fail-in-transaction            [2m(5.00ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-orderby-limit                   [2m(1.11ms)[0m

[1mtests/update_or_replace_rowid_secondary_index.sqltest[0m
  [[32mPASS[0m] update-or-replace-rowid-conflict-preserves-secondary-index [2m(8.65ms)[0m

[1mtests/upsert-expr-index.sqltest[0m
  [[32mPASS[0m] upsert-expr-index-basic                  [2m(7.87ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] collate-compound-3                       [2m(7.68ms)[0m

[1mtests/upsert-expr-index.sqltest[0m
  [[32mPASS[0m] upsert-expr-index-do-nothing             [2m(6.77ms)[0m

[1mtests/returning-fk-constraint.sqltest[0m
  [[32mPASS[0m] insert-fk-ok-returning                   [2m(6.64ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] replace-wrong-arg-count-4                [2m(4.83ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-scalar-lt-subquery-1              [2m(8.13ms)[0m

[1mtests/upsert-expr-index.sqltest[0m
  [[32mPASS[0m] upsert-expr-index-no-conflict            [2m(6.84ms)[0m
  [[32mPASS[0m] upsert-expr-index-do-nothing-no-target   [2m(6.98ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-string-func-null        [2m(5.69ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-unsigned-basic                    [2m(525.57µs)[0m

[1mtests/upsert-expr-index.sqltest[0m
  [[32mPASS[0m] upsert-expr-index-mixed-columns          [2m(7.56ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-from-union-all                  [2m(3.15ms)[0m

[1mtests/upsert-expr-index.sqltest[0m
  [[32mPASS[0m] upsert-expr-index-basic-no-target        [2m(7.29ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-pk-update                         [2m(5.16ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-vtab-join                         [2m(1.26ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-orderby-limit-cte               [2m(1.14ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] subtract-overflow-7                      [2m(2.79ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-pk-do-nothing                     [2m(5.00ms)[0m

[1mtests/upsert-expr-index.sqltest[0m
  [[32mPASS[0m] upsert-expr-index-no-conflict-no-target  [2m(7.42ms)[0m
  [[32mPASS[0m] upsert-expr-index-mixed-columns-no-target [2m(7.88ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-where-guard-no-change             [2m(5.67ms)[0m

[1mtests/update_or_replace_rowid_secondary_index.sqltest[0m
  [[32mPASS[0m] update-or-replace-rowid-no-conflict-preserves-secondary-index [2m(17.37ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-where-guard-apply                 [2m(5.80ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-exists-in-select-with-aggregate-empty-table [2m(4.69ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-unique-do-nothing                 [2m(6.78ms)[0m
  [[32mPASS[0m] upsert-unique-update                     [2m(7.38ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] if-alias-2-args-true                     [2m(538.51µs)[0m
  [[32mPASS[0m] replace-null-byte-pattern                [2m(517.52µs)[0m

[1mtests/upsert-expr-index.sqltest[0m
  [[32mPASS[0m] upsert-expr-index-binary-op              [2m(16.35ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-do-update-reads-added-column-default-for-prealter-row [2m(6.15ms)[0m
  [[32mPASS[0m] upsert-values-mixed-insert-update        [2m(5.91ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] collate-compound-4                       [2m(4.84ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-select-single                     [2m(5.55ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-fail-tx-still-active           [2m(5.06ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-vtab-reversed-join-order          [2m(1.19ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] func2-1-2-1                              [2m(5.93ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-composite-target-orderless        [2m(7.11ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] table-join-subquery                      [2m(1.90ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-case-with-null          [2m(5.12ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-returning-insert                  [2m(4.98ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-from-intersect                  [2m(4.54ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-collate-nocase                    [2m(6.04ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-scalar-agg-subquery-1             [2m(7.03ms)[0m

[1mtests/returning-fk-constraint.sqltest[0m
  [[32mPASS[0m] delete-fk-ok-returning                   [2m(5.98ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] replace-null-byte-prefix                 [2m(556.68µs)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] subtract-overflow-8                      [2m(2.79ms)[0m

[1mtests/multi_index_or_compound.sqltest[0m
  [[32mPASS[0m] compound-or-basic                        [2m(17.48ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-rowid-in-set                      [2m(5.41ms)[0m
  [[32mPASS[0m] upsert-selfref-and-excluded              [2m(13.54ms)[0m
  [[32mPASS[0m] upsert-notnull-violation                 [2m(5.17ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-unsigned-arithmetic-expression    [2m(507.17µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-using-plus-join-ambiguous    [2m(18.38ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-target-mismatch-errors            [2m(4.84ms)[0m
  [[32mPASS[0m] upsert-returning-update                  [2m(8.67ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] if-alias-multi-args-finds-first-true     [2m(495.61µs)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-omitted-target-matches-pk         [2m(5.34ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-correlated-exists-in-select-with-aggregate-empty-table [2m(4.42ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-module-list-nonempty              [2m(2.43ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-returning-do-nothing-empty        [2m(8.96ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] table-join-subquery-cte                  [2m(1.99ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-correlated-derived-in-select    [2m(22.51ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-invalid-qualified-lhs             [2m(5.28ms)[0m
  [[32mPASS[0m] upsert-rowvalue-set                      [2m(5.50ms)[0m
  [[32mPASS[0m] upsert-where-excluded-vs-target          [2m(6.26ms)[0m
  [[32mPASS[0m] upsert-values-returning-mixed            [2m(5.68ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-where-2                           [2m(11.54ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-ignore-notnull                 [2m(5.53ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-collate-implicit-match            [2m(6.72ms)[0m
  [[32mPASS[0m] upsert-composite-target-too-few          [2m(5.56ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-unsigned-negative                 [2m(516.43µs)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-updates-other-unique-key          [2m(11.56ms)[0m
  [[32mPASS[0m] upsert-qualified-target                  [2m(5.51ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] multiply-text-1                          [2m(2.77ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-from-except                     [2m(3.92ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] if-alias-multi-args-falls-to-else        [2m(522.84µs)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-pk-where-false                    [2m(4.96ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-case-nested             [2m(5.33ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] collate-compound-5                       [2m(5.19ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-where-null-3vl-no-update          [2m(5.77ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-temp-store                        [2m(369.83µs)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-where-combo                       [2m(5.92ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-join-table                      [2m(2.22ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-invalid-target-expression         [2m(7.76ms)[0m

[1mtests/returning-fk-constraint.sqltest[0m
  [[32mPASS[0m] update-fk-ok-returning                   [2m(7.43ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-update-conflicting-key            [2m(6.03ms)[0m
  [[32mPASS[0m] upsert-unknown-qualified-column-do-update [2m(13.26ms)[0m
  [[32mPASS[0m] upsert-noop-update-ok                    [2m(5.04ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-exists-empty-1                    [2m(7.39ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] hex                                      [2m(506.98µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-text-as-unsigned                  [2m(476.47µs)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-cte-available-in-arbitrary-depth [2m(3.66ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-multilple-conflict-targets        [2m(12.92ms)[0m
  [[32mPASS[0m] upsert-multirow-mixed-where              [2m(6.06ms)[0m
  [[32mPASS[0m] upsert-current-qualified_1               [2m(6.07ms)[0m
  [[32mPASS[0m] upsert-multicol-set_1                    [2m(6.04ms)[0m
  [[32mPASS[0m] upsert-where-predicate_1                 [2m(5.46ms)[0m
  [[32mPASS[0m] upsert-omitted-target-updates-unique     [2m(6.62ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-temp-store-set-memory             [2m(415.75µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] func2-1-2-2                              [2m(7.74ms)[0m
  [[32mPASS[0m] if-alias-multi-args-no-else-is-null      [2m(497.61µs)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-scalar-in-select-with-aggregate-empty-table [2m(4.56ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-update-causes-second-unique-violation [2m(8.04ms)[0m
  [[32mPASS[0m] upsert-invalid-excluded-column           [2m(11.10ms)[0m
  [[32mPASS[0m] upsert-doubly-qualified-target           [2m(5.83ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-ignore-unique                  [2m(6.03ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-nested-compound-in-scalar       [2m(3.78ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-non-rowid-pk-target               [2m(5.64ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] multiply-text-2                          [2m(2.86ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-join-table-cte                  [2m(2.36ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-collate-omitted-target-matches    [2m(6.35ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-float-basic                       [2m(503.94µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-type-conversion         [2m(5.18ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] collate-compound-7                       [2m(5.72ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] where-like-another-column-prefix         [2m(102.97ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-targets-chain                     [2m(11.78ms)[0m
  [[32mPASS[0m] upsert-unique-update-rowid-change        [2m(8.48ms)[0m

[1mtests/multi_index_or_compound.sqltest[0m
  [[32mPASS[0m] compound-or-node2                        [2m(17.56ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-temp-store-set-file               [2m(392.09µs)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-check-all-valid-with-conflict     [2m(5.45ms)[0m
  [[32mPASS[0m] upsert-check-valid-insert-no-conflict    [2m(4.82ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-float-negative                    [2m(492.15µs)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-unresolved-table-reference        [2m(9.77ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-join-subquery                   [2m(1.17ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-unique-update-rowid-change-preserves-unique [2m(10.54ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] replace-test-zeroblob-quote              [2m(643.37µs)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-unresolved-table-in-where         [2m(10.55ms)[0m
  [[32mPASS[0m] upsert-check-insert-violates-check-conflict-exists [2m(9.37ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-natural-plus-left-join-ambiguous [2m(13.22ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-check-insert-violates-check-no-conflict [2m(9.40ms)[0m
  [[32mPASS[0m] upsert-composite-collate-orderless       [2m(13.86ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-or-ignore-rowid-change-preserves-index [2m(9.37ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-check-excluded-ref-valid          [2m(5.82ms)[0m
  [[32mPASS[0m] upsert-check-do-nothing-insert-violates  [2m(8.96ms)[0m
  [[32mPASS[0m] upsert-overrides-pk-on-conflict-ignore   [2m(5.34ms)[0m

[1mtests/returning-fk-constraint.sqltest[0m
  [[32mPASS[0m] insert-multi-row-fk-violation-returning  [2m(6.53ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-overrides-unique-on-conflict-ignore [2m(5.82ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] func2-1-2-3                              [2m(4.29ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-collate-target-mismatch           [2m(17.72ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-group-by-inside                 [2m(3.37ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] hex-number                               [2m(501.51µs)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-overrides-pk-on-conflict-ignore-deferred-fk [2m(6.81ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-scalar-null-in-select-with-aggregate-empty-table [2m(5.90ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-integer-as-float                  [2m(478.03µs)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-function-list-has-abs             [2m(2.65ms)[0m

[1mtests/vacuum_into.sqltest[0m
  [[32mPASS[0m] vacuum-into-empty-path-rejected          [2m(5.39ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-ignore-pk                      [2m(5.49ms)[0m

[1mtests/values.sqltest[0m
  [[32mPASS[0m] values-1                                 [2m(2.63ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-all-many                          [2m(16.62ms)[0m

[1mtests/values.sqltest[0m
  [[32mPASS[0m] values-3                                 [2m(2.45ms)[0m
  [[32mPASS[0m] values-2                                 [2m(2.75ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] hex-null                                 [2m(348.04µs)[0m

[1mtests/values.sqltest[0m
  [[32mPASS[0m] values-in-from                           [2m(2.84ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-exists-basic                    [2m(7.45ms)[0m

[1mtests/values.sqltest[0m
  [[32mPASS[0m] values-in-join                           [2m(2.83ms)[0m
  [[32mPASS[0m] values-unquoted-identifier-mixed         [2m(1.65ms)[0m
  [[32mPASS[0m] values-unquoted-identifier-single        [2m(2.59ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-float-arithmetic                  [2m(472.40µs)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-check-update-violates-check       [2m(16.95ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] likely                                   [2m(339.69µs)[0m

[1mtests/using-dedup-outer-ref.sqltest[0m
  [[32mPASS[0m] correlated-subquery-using-dedup-outer-ref [2m(10.26ms)[0m

[1mtests/vacuum_into.sqltest[0m
  [[32mPASS[0m] vacuum-into-nonexistent-schema           [2m(6.56ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-having-inside                   [2m(3.08ms)[0m

[1mtests/values.sqltest[0m
  [[32mPASS[0m] values-illegal-column-ref                [2m(4.20ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-join-subquery-cte               [2m(1.33ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] collate-compound-8                       [2m(7.80ms)[0m

[1mtests/values.sqltest[0m
  [[32mPASS[0m] values-between                           [2m(4.57ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] func3-5-8                                [2m(5.15ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] multiply-text-3                          [2m(2.71ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-in-in-select-with-aggregate-empty-table [2m(4.57ms)[0m

[1mtests/view-rowid.sqltest[0m
  [[32mPASS[0m] view-rowid-select                        [2m(3.66ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] likely-int                               [2m(371.98µs)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-function-list-has-count           [2m(2.67ms)[0m

[1mtests/view-rowid.sqltest[0m
  [[32mPASS[0m] view-rowid-order-by-index                [2m(4.72ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-text-as-float                     [2m(462.83µs)[0m

[1mtests/using-dedup-outer-ref.sqltest[0m
  [[32mPASS[0m] qualified-ref-to-using-hidden-col-in-subquery [2m(15.05ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] replace-test-zeroblob-quote-2            [2m(501.23µs)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-ignore-no-conflict             [2m(5.57ms)[0m

[1mtests/returning-fk-constraint.sqltest[0m
  [[32mPASS[0m] insert-fk-null-child-returning           [2m(6.55ms)[0m

[1mtests/views.sqltest[0m
  [[32mPASS[0m] view-basic-filtering                     [2m(6.54ms)[0m
  [[32mPASS[0m] view-aggregation-groupby                 [2m(6.54ms)[0m
  [[32mPASS[0m] view-composition-with-functions          [2m(6.12ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] select-star-table-subquery               [2m(1.95ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-natural-plus-inner-join-ambiguous [2m(13.09ms)[0m

[1mtests/views.sqltest[0m
  [[32mPASS[0m] view-referencing-view                    [2m(7.74ms)[0m
  [[32mPASS[0m] view-drop-and-recreate                   [2m(6.43ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-distinct-inside                 [2m(3.25ms)[0m

[1mtests/views.sqltest[0m
  [[32mPASS[0m] view-case-expression                     [2m(7.14ms)[0m
  [[32mPASS[0m] view-arithmetic-expression               [2m(5.65ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] likely-decimal                           [2m(492.63µs)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-large-small                       [2m(11.29ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-char-single                       [2m(469.54µs)[0m

[1mtests/views.sqltest[0m
  [[32mPASS[0m] view-with-having                         [2m(6.71ms)[0m
  [[32mPASS[0m] view-filter-clause                       [2m(6.30ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] multiply-text-4                          [2m(2.79ms)[0m

[1mtests/views.sqltest[0m
  [[32mPASS[0m] view-recreate-after-drop                 [2m(9.17ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] func3-5-9                                [2m(5.38ms)[0m

[1mtests/values.sqltest[0m
  [[32mPASS[0m] values-correlated-values-in-select       [2m(14.97ms)[0m

[1mtests/upsert.sqltest[0m
  [[32mPASS[0m] upsert-check-unique-index-insert-violates [2m(30.93ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] column-is-null-in-aggregate-empty-table  [2m(4.35ms)[0m

[1mtests/vacuum_into.sqltest[0m
  [[32mPASS[0m] vacuum-into-rejected-in-transaction      [2m(21.56ms)[0m

[1mtests/views.sqltest[0m
  [[32mPASS[0m] view-rejects-aggregate-order-by          [2m(8.56ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-function-list-scalar              [2m(2.59ms)[0m

[1mtests/views.sqltest[0m
  [[32mPASS[0m] view-bracket-column-list                 [2m(5.52ms)[0m
  [[32mPASS[0m] view-if-not-exists-idempotent            [2m(5.86ms)[0m
  [[32mPASS[0m] view-reserved-prefix-sqlite              [2m(6.62ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] uuid-str-empty                           [2m(4.94ms)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-char-first-from-string            [2m(477.55µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] likely-null                              [2m(434.95µs)[0m

[1mtests/views.sqltest[0m
  [[32mPASS[0m] view-with-join                           [2m(16.34ms)[0m
  [[32mPASS[0m] view-self-circle-detection               [2m(8.00ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-clause-eq                          [2m(683.93µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-char-empty-string                 [2m(445.52µs)[0m

[1mtests/views.sqltest[0m
  [[32mPASS[0m] view-quoted-column-list                  [2m(5.61ms)[0m
  [[32mPASS[0m] view-mutual-circle-detection             [2m(9.21ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-ignore-continue                [2m(6.17ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] select-star-table-subquery-cte           [2m(1.79ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-type-conversion         [2m(5.56ms)[0m

[1mtests/virtual-table-left-join.sqltest[0m
  [[32mPASS[0m] left-join-virtual-table-null             [2m(5.46ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-not-in-with-null-in-list        [2m(3.44ms)[0m

[1mtests/virtual-table-left-join.sqltest[0m
  [[32mPASS[0m] left-join-virtual-table-on-left-matching [2m(5.53ms)[0m

[1mtests/math/memory.sqltest[0m
  [[32mPASS[0m] multiply-text-5                          [2m(2.72ms)[0m

[1mtests/returning-fk-constraint.sqltest[0m
  [[32mPASS[0m] insert-fk-violation-no-persist           [2m(7.06ms)[0m

[1mtests/virtual-table-left-join.sqltest[0m
  [[32mPASS[0m] left-join-virtual-table-on-left-null     [2m(5.63ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-function-list-aggregate           [2m(2.55ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] exists-in-ungrouped-aggregate-with-data  [2m(4.88ms)[0m

[1mtests/virtual-table-left-join.sqltest[0m
  [[32mPASS[0m] left-join-virtual-table-matching         [2m(12.16ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-exists-uncorrelated             [2m(12.65ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-set-subquery                      [2m(12.59ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-subquery-source              [2m(8.74ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] uuid-blob-empty                          [2m(5.18ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-clause-unary-false                 [2m(545.22µs)[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-true [2m(463.58µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] collate-compound-9                       [2m(10.77ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-identifier-true [2m(513.98µs)[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-true-2 [2m(460.60µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] func3-5-10                               [2m(5.65ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-clause-lt                          [2m(7.90ms)[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-true-3 [2m(462.75µs)[0m
  [[32mPASS[0m] where-clause-lte                         [2m(7.60ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] uuid7-timestamp-ms-empty                 [2m(4.99ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-true-4 [2m(552.36µs)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-function-list-window-row-number   [2m(2.60ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-u-escape                          [2m(478.86µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-true-5 [2m(448.03µs)[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-true-6 [2m(463.40µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-bare-hex                          [2m(452.22µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-false [2m(426.52µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-plus-form                         [2m(416.39µs)[0m
  [[32mPASS[0m] func9-120                                [2m(4.67ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-clause-gt                          [2m(15.22ms)[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-identifier-false [2m(400.91µs)[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-false-3 [2m(452.80µs)[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-false-2 [2m(483.04µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-capital-U                         [2m(437.34µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where_not_in_empty_list                  [2m(843.53µs)[0m
  [[32mPASS[0m] where_name_in_list_and_price_gt_70_or_name_exactly_boots [2m(948.14µs)[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-false-4 [2m(427.60µs)[0m
  [[32mPASS[0m] where-clause-isnull-or-false             [2m(19.33ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-function-list-omits-unimplemented-window-functions [2m(3.79ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unistr-backslash-escape                  [2m(433.62µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-clause-gte                         [2m(16.24ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] exists-in-ungrouped-aggregate-empty-main-table [2m(6.02ms)[0m

[1mtests/multi_index_or_compound.sqltest[0m
  [[32mPASS[0m] compound-or-type-filter                  [2m(45.83ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where_name_in_list_or_price_gt_70_and_name_like_shirt [2m(908.21µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-three-way-join               [2m(11.27ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where_multiple                           [2m(819.47µs)[0m
  [[32mPASS[0m] where_name_not_in_list_or_name_eq_shirt  [2m(1.03ms)[0m
  [[32mPASS[0m] where_multiple_flipped                   [2m(826.97µs)[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-false-5 [2m(428.73µs)[0m
  [[32mPASS[0m] where-parentheses-and                    [2m(882.72µs)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] test-pragma-journal-mode-unsupported     [2m(381.69µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-one-operand-null-and-eq     [2m(731.61µs)[0m
  [[32mPASS[0m] where-nested-parentheses                 [2m(1.14ms)[0m
  [[32mPASS[0m] where-complex-parentheses                [2m(1.13ms)[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-false-6 [2m(453.66µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] collate-compound-10                      [2m(12.61ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-one-operand-null-and-gt     [2m(762.71µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] func9-150                                [2m(4.78ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-false-7 [2m(434.03µs)[0m
  [[32mPASS[0m] where-binary-one-operand-null-and-lt     [2m(823.53µs)[0m

[1mtests/views.sqltest[0m
  [[32mPASS[0m] view-duplicate-without-if-not-exists-errors [2m(33.74ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-ignore-partial-update          [2m(6.45ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-set-correlated-subquery           [2m(17.02ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] exists-in-ungrouped-aggregate-empty-subquery-table [2m(5.68ms)[0m

[1mtests/returning-fk-constraint.sqltest[0m
  [[32mPASS[0m] delete-fk-violation-no-persist           [2m(7.14ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-one-operand-null-and-gte    [2m(709.48µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-cross-join-no-where          [2m(7.66ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] correlated-exists-in-ungrouped-aggregate-empty-main-table [2m(5.66ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-one-operand-null-and-lte    [2m(712.10µs)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-null                              [2m(8.71ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-clause-ne                          [2m(35.90ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] where-like-another-column-prefix-2       [2m(65.60ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-spill                    [2m(110.74ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-one-operand-null-and-ne     [2m(753.50µs)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] from-subquery-inner-join                 [2m(6.75ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-clause-isnull                      [2m(41.16ms)[0m
  [[32mPASS[0m] where-literal-string                     [2m(531.78µs)[0m

[1mtests/scalar-functions-format.sqltest[0m
  [[32mPASS[0m] format-char-from-integer                 [2m(553.00µs)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-not-in-with-null-and-value      [2m(3.67ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-cast-float-to-int                  [2m(695.25µs)[0m
  [[32mPASS[0m] where-clause-unary-true                  [2m(36.94ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] subquery-in-with-null-match              [2m(3.45ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-clause-notnull-or-false            [2m(45.90ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] from-subquery-left-join                  [2m(6.48ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-clause-notnull                     [2m(48.87ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] select-star-subquery-table               [2m(1.75ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-rowid-returning                   [2m(6.05ms)[0m

[1mtests/multi_index_or_compound.sqltest[0m
  [[32mPASS[0m] compound-or-no-match                     [2m(35.24ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-not-exists-uncorrelated         [2m(12.39ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-clause-eq-string                   [2m(59.19ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] likelihood-string                        [2m(480.27µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-cast-string-to-int                 [2m(19.61ms)[0m
  [[32mPASS[0m] where-binary-one-operand-null-gt         [2m(42.98ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-table-info-equal-syntax           [2m(482.20µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-one-operand-null-gte        [2m(44.51ms)[0m
  [[32mPASS[0m] where-binary-one-operand-null-lte        [2m(44.92ms)[0m
  [[32mPASS[0m] where-binary-one-operand-null-or-gt      [2m(51.64ms)[0m
  [[32mPASS[0m] select-where-and                         [2m(49.37ms)[0m
  [[32mPASS[0m] where-id-index-seek-regression-test      [2m(811.81µs)[0m
  [[32mPASS[0m] where-binary-one-operand-null-ne         [2m(40.60ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-abort-notnull                  [2m(5.18ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-unary-bitnot                       [2m(18.95ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-single-column           [2m(4.97ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] likelihood-string-high-probability       [2m(486.47µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-add                         [2m(19.26ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-not-null-1                        [2m(5.20ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-one-operand-null-or-lt      [2m(62.84ms)[0m
  [[32mPASS[0m] where-case-simple                        [2m(39.18ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] func9-160                                [2m(4.94ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-clause-gte                         [2m(11.04ms)[0m
  [[32mPASS[0m] where-clause-isnull-or-false             [2m(17.07ms)[0m
  [[32mPASS[0m] where-clause-gt                          [2m(12.21ms)[0m
  [[32mPASS[0m] where-unary-minus                        [2m(36.07ms)[0m
  [[32mPASS[0m] where-id-index-seek-regression-test-opposite [2m(1.88ms)[0m
  [[32mPASS[0m] where-binary-one-operand-null-or-lte     [2m(64.80ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-not-null-2                        [2m(4.63ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] from-subquery-multiple-joins             [2m(9.99ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-clause-unary-false                 [2m(543.57µs)[0m
  [[32mPASS[0m] where-case-searched                      [2m(47.95ms)[0m
  [[32mPASS[0m] where-clause-lt                          [2m(8.46ms)[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-true [2m(438.94µs)[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-false-6 [2m(433.39µs)[0m
  [[32mPASS[0m] where-clause-lte                         [2m(7.07ms)[0m
  [[32mPASS[0m] where-unary-not                          [2m(47.28ms)[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-identifier-true [2m(420.94µs)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] select-star-subquery-table-cte           [2m(1.82ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-false-7 [2m(426.44µs)[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-true-2 [2m(436.38µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-no-matches-extra             [2m(7.77ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-unary-plus                         [2m(46.78ms)[0m
  [[32mPASS[0m] where-binary-one-operand-null-or-ne      [2m(64.53ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] where-like-impossible                    [2m(633.35µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-function-length                    [2m(59.25ms)[0m
  [[32mPASS[0m] where-float-int                          [2m(854.21µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-with-backslash                      [2m(473.93µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-reference-hidden-column [2m(4.88ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-one-operand-null-lt         [2m(83.66ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] from-subquery-expression-join            [2m(8.92ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-multiple-or                        [2m(921.23µs)[0m
  [[32mPASS[0m] where-multiple-and                       [2m(939.02µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-with-dollar                         [2m(463.39µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where_in_list                            [2m(736.51µs)[0m
  [[32mPASS[0m] where_not_in_list                        [2m(952.56µs)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] cte-in-scalar-subquery-select            [2m(3.50ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-with-dot                            [2m(477.11µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where_in_list_or_another_list            [2m(877.39µs)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-exists-correlated               [2m(16.38ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where_not_in_list_and_not_in_another_list [2m(1.12ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-embedded-nul                        [2m(754.42µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where_in_list_or_not_in_another_list     [2m(1.08ms)[0m
  [[32mPASS[0m] where_in_empty_list                      [2m(499.05µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-fn-esc-1                            [2m(485.12µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-subtract                    [2m(19.45ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-reference-hidden-column [2m(5.47ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where_not_in_empty_list                  [2m(854.43µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-fn-esc-2                            [2m(471.79µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where_name_in_list_and_price_gt_70_or_name_exactly_boots [2m(1.07ms)[0m
  [[32mPASS[0m] where_name_in_list_or_price_gt_70_and_name_like_shirt [2m(956.19µs)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] cte-in-exists-subquery                   [2m(3.97ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where_name_not_in_list_or_name_eq_shirt  [2m(1.10ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] collate-compound-11                      [2m(7.77ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where_multiple                           [2m(918.46µs)[0m
  [[32mPASS[0m] where-clause-unary-true                  [2m(17.99ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-fn-esc-3                            [2m(482.92µs)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] from-subquery-as-outer-table             [2m(7.85ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-parentheses-and                    [2m(1.02ms)[0m
  [[32mPASS[0m] where-clause-eq-string                   [2m(50.80ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-fn-esc-4                            [2m(465.23µs)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-table-info-call-syntax            [2m(488.95µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-nested-parentheses                 [2m(1.36ms)[0m
  [[32mPASS[0m] where-clause-ne                          [2m(39.28ms)[0m
  [[32mPASS[0m] where-id-index-seek-regression-test      [2m(826.74µs)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] cte-in-from-subquery                     [2m(2.97ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] likelihood-integer                       [2m(515.26µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-complex-parentheses                [2m(1.44ms)[0m
  [[32mPASS[0m] where-clause-isnull                      [2m(43.70ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-table-xinfo-call-syntax           [2m(460.64µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-fn-esc-5                            [2m(475.99µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where_multiple_flipped                   [2m(3.96ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-duplicate-column-names  [2m(5.15ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-id-index-seek-regression-test-2    [2m(18.04ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-not-null-3                        [2m(4.90ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-id-index-seek-regression-test-opposite [2m(2.01ms)[0m
  [[32mPASS[0m] where-simple-between                     [2m(814.98µs)[0m
  [[32mPASS[0m] between-price-range-with-names           [2m(867.51µs)[0m
  [[32mPASS[0m] where-age-index-seek-regression-test     [2m(1.06ms)[0m
  [[32mPASS[0m] where-between-true-and-2                 [2m(806.05µs)[0m

[1mtests/multi_index_or_compound.sqltest[0m
  [[32mPASS[0m] compound-or-all-entities                 [2m(18.29ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] cte-used-in-both-direct-and-subquery     [2m(3.45ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] from-subquery-multi-column-join          [2m(7.91ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-clause-notnull                     [2m(49.31ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-mixed-types                       [2m(6.36ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-age-index-seek-regression-test     [2m(9.39ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-table-xinfo-vtable                [2m(755.38µs)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-not-exists-correlated           [2m(18.64ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-one-operand-null-or-gte     [2m(104.36ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-abort-unique                   [2m(5.87ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-multiply                    [2m(18.99ms)[0m
  [[32mPASS[0m] where-binary-one-operand-null-and-eq     [2m(920.51µs)[0m
  [[32mPASS[0m] where-id-index-seek-regression-test-2    [2m(14.76ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] likelihood-integer-probability-1         [2m(466.87µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] collate-compound-13                      [2m(16.13ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-one-operand-null-and-gt     [2m(859.06µs)[0m
  [[32mPASS[0m] where-age-index-seek-regression-test-2   [2m(14.27ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-abort-pk                       [2m(5.39ms)[0m

[1mtests/update_attached_db_index.sqltest[0m
  [[32mPASS[0m] update-attached-db-rowid-change-with-unique-index [2m(10.61ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] multi-table-left-join-spill-with-inner   [2m(102.26ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-one-operand-null-and-lt     [2m(917.59µs)[0m
  [[32mPASS[0m] where-clause-notnull-or-false            [2m(66.24ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] collate-compound-14                      [2m(10.42ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-multiple-columns        [2m(4.96ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-one-operand-null-and-gte    [2m(916.76µs)[0m
  [[32mPASS[0m] select-where-or                          [2m(48.36ms)[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-true-3 [2m(471.41µs)[0m
  [[32mPASS[0m] select-where-and                         [2m(50.14ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] select-star-subquery-subquery            [2m(1.91ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-all-columns             [2m(4.77ms)[0m

[1mtests/update_attached_db_index.sqltest[0m
  [[32mPASS[0m] update-attached-db-rowid-with-where-subquery [2m(12.50ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-one-operand-null-and-lte    [2m(894.97µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-table-qualified         [2m(4.77ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] select-where-or                          [2m(52.07ms)[0m
  [[32mPASS[0m] where-binary-one-operand-null-and-ne     [2m(954.56µs)[0m
  [[32mPASS[0m] where-age-index-seek-regression-test-2   [2m(52.88ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-fn-esc-6                            [2m(520.14µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-one-operand-null-eq         [2m(43.80ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-table-info-alt-name-equal-syntax  [2m(482.56µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-literal-string                     [2m(663.54µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-fn-esc-7                            [2m(466.85µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-cast-float-to-int                  [2m(659.23µs)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-fn-esc-8                            [2m(465.33µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-one-operand-null-gt         [2m(43.83ms)[0m
  [[32mPASS[0m] select-where-and-or                      [2m(76.51ms)[0m
  [[32mPASS[0m] where-binary-one-operand-null-lt         [2m(44.62ms)[0m
  [[32mPASS[0m] where-age-index-seek-regression-test-3   [2m(862.70µs)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-null-source-value            [2m(8.23ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-one-operand-null-gte        [2m(45.10ms)[0m
  [[32mPASS[0m] nested-parens-and-inside-or-regression-test [2m(61.13ms)[0m
  [[32mPASS[0m] where-cast-string-to-int                 [2m(19.26ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-table-info-alt-name-call-syntax   [2m(464.88µs)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-with-expression                   [2m(6.02ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-divide                      [2m(19.77ms)[0m
  [[32mPASS[0m] where-binary-one-operand-null-or-lt      [2m(60.80ms)[0m
  [[32mPASS[0m] where-binary-one-operand-null-ne         [2m(44.29ms)[0m
  [[32mPASS[0m] where-binary-one-operand-null-or-gt      [2m(68.06ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] likelihood-decimal                       [2m(506.00µs)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-abort-rollback-statement       [2m(5.69ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] right-join-with-inner-join               [2m(16.77ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-one-operand-null-or-eq      [2m(75.35ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] order-by-case-insensitive-aggregate      [2m(293.99ms)[0m

[1mtests/update_attached_db_index.sqltest[0m
  [[32mPASS[0m] update-attached-db-integrity-after-rowid-change [2m(11.52ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-unary-plus                         [2m(17.35ms)[0m
  [[32mPASS[0m] where-binary-one-operand-null-lte        [2m(61.27ms)[0m
  [[32mPASS[0m] nested-parens-conditionals-and-or-and    [2m(89.57ms)[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-true-4 [2m(454.90µs)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-exists-with-conditions          [2m(8.00ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-one-operand-null-or-gte     [2m(67.57ms)[0m
  [[32mPASS[0m] where-binary-one-operand-null-or-lte     [2m(60.88ms)[0m
  [[32mPASS[0m] where-unary-minus                        [2m(20.15ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] select-star-subquery-subquery-cte        [2m(1.96ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] cte-materialized-multiple-scalar-refs    [2m(4.09ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-self-reference                    [2m(5.70ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] collate-compound-15                      [2m(4.93ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-constant-condition-no-tables       [2m(445.26µs)[0m
  [[32mPASS[0m] nested-parens-conditionals-or-and-or     [2m(97.38ms)[0m
  [[32mPASS[0m] where-binary-one-operand-null-or-ne      [2m(60.13ms)[0m
  [[32mPASS[0m] where-unary-bitnot                       [2m(19.26ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] likelihood-null                          [2m(492.47µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-constant-condition-no-tables-2     [2m(455.39µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] likelihood-blob                          [2m(489.36µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-add                         [2m(19.16ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-literal                 [2m(4.78ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] affinity-conversion-1                    [2m(6.45ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-multiply                    [2m(19.41ms)[0m
  [[32mPASS[0m] where-binary-modulo                      [2m(16.78ms)[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-true-5 [2m(436.09µs)[0m
  [[32mPASS[0m] where-binary-divide                      [2m(19.38ms)[0m

[1mtests/where/memory.sqltest[0m
  [[32mPASS[0m] where_alias_precedence                   [2m(4.93ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-age-index-seek-regression-test-3   [2m(112.53ms)[0m
  [[32mPASS[0m] where-binary-shift-left                  [2m(18.74ms)[0m
  [[32mPASS[0m] where-function-length                    [2m(58.71ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-fn-esc-9                            [2m(540.99µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-bitwise-or                  [2m(17.10ms)[0m

[1mtests/where/memory.sqltest[0m
  [[32mPASS[0m] where_alias_precedence_2                 [2m(5.04ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-shift-right                 [2m(18.52ms)[0m
  [[32mPASS[0m] where-binary-modulo                      [2m(19.16ms)[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-true-6 [2m(457.75µs)[0m
  [[32mPASS[0m] where-binary-bitwise-and                 [2m(18.98ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] affinity-conversion-2                    [2m(6.34ms)[0m

[1mtests/multi_index_or_compound.sqltest[0m
  [[32mPASS[0m] compound-or-benchmark-shape              [2m(19.41ms)[0m

[1mtests/subquery/expressions.sqltest[0m
  [[32mPASS[0m] cte-inside-from-subquery                 [2m(3.24ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-replace-unique                 [2m(15.68ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-subtract                    [2m(35.18ms)[0m

[1mtests/where/memory.sqltest[0m
  [[32mPASS[0m] where_alias_precedence_3                 [2m(6.03ms)[0m

[1mtests/where/small.sqltest[0m
  [[32mPASS[0m] where-is-a-with-nulls                    [2m(902.13µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-duplicate-column-names  [2m(8.84ms)[0m

[1mtests/where/small.sqltest[0m
  [[32mPASS[0m] where-is-null                            [2m(4.44ms)[0m
  [[32mPASS[0m] where-equals-null                        [2m(4.41ms)[0m
  [[32mPASS[0m] where-equals-a-with-nulls                [2m(812.74µs)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-self-ref-all                      [2m(5.98ms)[0m

[1mtests/where/small.sqltest[0m
  [[32mPASS[0m] where-is-not-null                        [2m(3.42ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-fn-esc-10                           [2m(511.67µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-false [2m(454.45µs)[0m

[1mtests/where/small.sqltest[0m
  [[32mPASS[0m] where-is-not-a-with-nulls                [2m(1.09ms)[0m

[1mtests/update_attached_db_index.sqltest[0m
  [[32mPASS[0m] update-attached-db-non-rowid-column      [2m(10.70ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-nested-exists                   [2m(9.63ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-without-partition-by-or-order-by  [2m(1.53ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-fn-esc-11                           [2m(471.12µs)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-order-by-multiple-columns         [2m(2.76ms)[0m
  [[32mPASS[0m] window-with-aggregate                    [2m(1.45ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] select-where-and-or                      [2m(71.93ms)[0m
  [[32mPASS[0m] where-unary-not                          [2m(52.00ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-function-table-info-alt-name      [2m(863.81µs)[0m

[1mtests/where/small.sqltest[0m
  [[32mPASS[0m] where-not-equal-null                     [2m(6.94ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] left-join-where-filters-to-inner         [2m(7.12ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] affinity-conversion-4                    [2m(6.87ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-column-name-as-string   [2m(5.12ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-case-searched                      [2m(61.48ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-duplicate-name                    [2m(2.33ms)[0m
  [[32mPASS[0m] window-name-with-space                   [2m(2.46ms)[0m

[1mtests/update_expression_index.sqltest[0m
  [[32mPASS[0m] basic-update-deletes-old-entry           [2m(9.16ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-nonexistent-name                  [2m(4.46ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-replace-notnull-default        [2m(13.12ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] left-join-order-by-nullable-column       [2m(6.90ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] affinity-conversion-5                    [2m(6.65ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-excluded-not-upsert     [2m(4.28ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-function-in-having                [2m(4.54ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-null-join-key-extra          [2m(8.37ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-shift-left                  [2m(19.30ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-scalar-function                   [2m(3.95ms)[0m
  [[32mPASS[0m] window-nested-function                   [2m(4.45ms)[0m
  [[32mPASS[0m] window-function-in-group-by              [2m(7.63ms)[0m
  [[32mPASS[0m] window-function-without-over             [2m(4.43ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-case-simple                        [2m(80.29ms)[0m
  [[32mPASS[0m] where-simple-between                     [2m(801.70µs)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-scalar-function-star              [2m(4.00ms)[0m

[1mtests/multi_index_or_compound.sqltest[0m
  [[32mPASS[0m] compound-or-correlated-subquery          [2m(18.06ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-self-referential-regression        [2m(39.30ms)[0m

[1mtests/update_expression_index.sqltest[0m
  [[32mPASS[0m] update-via-expression-index-scan         [2m(8.32ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-aggregate-in-order-by             [2m(1.28ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-both-empty               [2m(5.75ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-nonexistent-function              [2m(10.24ms)[0m
  [[32mPASS[0m] window-aggregate-in-partition-by         [2m(3.21ms)[0m
  [[32mPASS[0m] window-aggregate-as-argument             [2m(1.27ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-null-comparison-index-seek-regression-test [2m(45.70ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-updated-to-null         [2m(7.93ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] likelihood-zero-probability              [2m(497.81µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-constant-expression     [2m(4.93ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-is-join                      [2m(8.07ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-aggregate-with-group-by-as-argument [2m(2.80ms)[0m
  [[32mPASS[0m] window-nonexistent-function-star         [2m(9.39ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] between-price-range-with-names           [2m(875.71µs)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-function-as-aggregate-argument    [2m(4.53ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-str-ab                             [2m(479.61µs)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-function-nested-in-partition-by   [2m(4.27ms)[0m
  [[32mPASS[0m] window-function-nested-in-order-by       [2m(4.17ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] affinity-conversion-6                    [2m(6.94ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-with-limit               [2m(6.97ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-replace-notnull-no-default     [2m(14.29ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-rowid-in-result                   [2m(2.37ms)[0m
  [[32mPASS[0m] window-rowid-in-order-by                 [2m(2.52ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-between-true-and-2                 [2m(764.87µs)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-int-ab                             [2m(485.89µs)[0m

[1mtests/update_expression_index.sqltest[0m
  [[32mPASS[0m] update-multiple-rows-via-expr-index      [2m(10.28ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-distinct                          [2m(2.10ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-updated-from-null       [2m(5.53ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-rowid-as-argument                 [2m(2.92ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-string-literal          [2m(4.91ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-distinct-as-argument-2            [2m(4.64ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-shift-right                 [2m(18.75ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-range-join                   [2m(8.29ms)[0m

[1mtests/multi_index_or_compound.sqltest[0m
  [[32mPASS[0m] compound-or-rowid-branch                 [2m(15.30ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-empty-partition                   [2m(1.20ms)[0m
  [[32mPASS[0m] window-distinct-as-argument              [2m(8.72ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-composite-keys           [2m(7.14ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] affinity-conversion-7                    [2m(7.06ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-complex-argument                  [2m(1.68ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] affinity-conversion-8                    [2m(6.66ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-row-number-order-by               [2m(1.47ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-replace-pk                     [2m(12.18ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-row-number-no-order-by            [2m(1.38ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] affinity-conversion-9                    [2m(6.70ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-bitwise-and                 [2m(19.67ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-row-number-with-peers             [2m(2.07ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] affinity-conversion-10                   [2m(6.55ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-replace-no-conflict            [2m(15.45ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-single-row-partition              [2m(53.79ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-view-source                  [2m(7.82ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-order-by                          [2m(3.13ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-aggregate-subquery-extra     [2m(8.85ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-order-by-duplicate-column         [2m(2.56ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-having-subquery              [2m(11.39ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-order-by-multiple-columns         [2m(4.40ms)[0m
  [[32mPASS[0m] window-partition-by-and-order-by         [2m(150.83ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-window-function-extra        [2m(17.94ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-inner-grouping                  [2m(160.04ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-large-text                        [2m(5.31ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-without-partition-by-or-order-by  [2m(1.64ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] nested-parens-conditionals-or-and-or     [2m(129.07ms)[0m
  [[32mPASS[0m] nested-parens-conditionals-and-double-or [2m(291.24ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] from-subquery-distinct                   [2m(8.33ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-group-by-with-aggregate           [2m(197.89ms)[0m
  [[32mPASS[0m] window-group-by-having                   [2m(195.73ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-function-table-info               [2m(1.89ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-float-int                          [2m(23.17ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-order-by-limit-offset             [2m(212.12ms)[0m

[1mtests/update_expression_index.sqltest[0m
  [[32mPASS[0m] update-implicit-rowid-table              [2m(8.94ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-partition-by-duplicate-column     [2m(188.90ms)[0m
  [[32mPASS[0m] window-limit-offset                      [2m(237.67ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-dot-uppercase                      [2m(639.57µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] affinity-conversion-11                   [2m(30.82ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-duplicate-name                    [2m(2.80ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-inner-grouping-cte              [2m(138.31ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-name-with-space                   [2m(2.62ms)[0m
  [[32mPASS[0m] window-with-aggregate                    [2m(95.25ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-vtab-table-info                   [2m(963.17µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] affinity-conversion-12                   [2m(7.39ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-nonexistent-name                  [2m(4.36ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-table-info-invalid-table          [2m(353.18µs)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-function-in-having                [2m(4.25ms)[0m
  [[32mPASS[0m] window-function-in-group-by              [2m(4.51ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-function-table-info-invalid-table [2m(654.73µs)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] affinity-conversion-13                   [2m(5.07ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-partition-by-and-order-by         [2m(157.93ms)[0m
  [[32mPASS[0m] window-scalar-function                   [2m(3.92ms)[0m
  [[32mPASS[0m] window-nested-function                   [2m(4.37ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-vtab-table-info-invalid-table     [2m(673.02µs)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-partition-by                      [2m(252.41ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-union-all                    [2m(7.06ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] affinity-conversion-15                   [2m(6.53ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-function-without-over             [2m(4.39ms)[0m
  [[32mPASS[0m] window-scalar-function-star              [2m(3.86ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] affinity-conversion-16                   [2m(6.41ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-bitwise-or                  [2m(17.27ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-partition-by-multiple-columns     [2m(260.14ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-intersect                    [2m(11.24ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-aggregate-in-partition-by         [2m(1.39ms)[0m
  [[32mPASS[0m] window-aggregate-in-order-by             [2m(1.26ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-constant-condition-no-tables       [2m(433.97µs)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-aggregate-as-argument             [2m(1.25ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-constant-condition-no-tables-2     [2m(426.77µs)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-aggregate-with-group-by-as-argument [2m(2.46ms)[0m
  [[32mPASS[0m] window-nonexistent-function-star         [2m(9.57ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-null-comparison-index-seek-regression-test [2m(713.36µs)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-nonexistent-function              [2m(25.92ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-legacy-file-format                [2m(339.99µs)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-replace-multiple-conflicts     [2m(8.59ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-function-as-aggregate-argument    [2m(4.61ms)[0m
  [[32mPASS[0m] window-function-nested-in-partition-by   [2m(4.13ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-except                       [2m(10.69ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-function-nested-in-order-by       [2m(4.34ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] affinity-conversion-17                   [2m(6.61ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-rowid-in-result                   [2m(2.82ms)[0m
  [[32mPASS[0m] window-rowid-in-order-by                 [2m(2.74ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-function-legacy-file-format       [2m(4.93ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-rollback-autocommit            [2m(5.75ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-rowid-as-argument                 [2m(2.59ms)[0m
  [[32mPASS[0m] window-distinct                          [2m(2.53ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-rollback-in-transaction        [2m(5.87ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-function-too-many-arguments       [2m(8.15ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-distinct-as-argument              [2m(4.73ms)[0m
  [[32mPASS[0m] window-distinct-as-argument-2            [2m(4.68ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] order-by-agg-not-mentioned-in-select     [2m(378.86ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] affinity-conversion-18                   [2m(18.21ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-function-update                   [2m(4.90ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-self-referential-regression        [2m(34.76ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-cte-chain                    [2m(32.27ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-empty-partition                   [2m(1.07ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-self-join-shift              [2m(6.48ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-rollback-entire-tx-rolled-back [2m(23.32ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-single-row-partition              [2m(53.10ms)[0m
  [[32mPASS[0m] window-group-by-with-aggregate           [2m(183.85ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-rotate-values                [2m(7.01ms)[0m

[1mtests/update_expression_index.sqltest[0m
  [[32mPASS[0m] multiple-sequential-updates              [2m(11.10ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-complex-argument                  [2m(1.76ms)[0m
  [[32mPASS[0m] window-group-by-having                   [2m(180.03ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-dot-lowercase                      [2m(607.61µs)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-row-number-order-by               [2m(1.61ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-no-hex                             [2m(460.91µs)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-row-number-no-order-by            [2m(1.35ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-null                               [2m(449.02µs)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-row-number-with-peers             [2m(2.01ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-x-y-prefix                         [2m(475.27µs)[0m

[1mtests/update_expression_index.sqltest[0m
  [[32mPASS[0m] self-referencing-set-expression          [2m(8.22ms)[0m

[1mtests/window/filter-over.sqltest[0m
  [[32mPASS[0m] filter-over-full-window-aggregate-matrix [2m(7.04ms)[0m
  [[32mPASS[0m] filter-over-empty-filter-results         [2m(6.75ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-x-y-suffix                         [2m(475.63µs)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-outer-grouping                  [2m(177.60ms)[0m

[1mtests/update_expression_index.sqltest[0m
  [[32mPASS[0m] multi-column-expression-index            [2m(8.30ms)[0m

[1mtests/window/filter-over.sqltest[0m
  [[32mPASS[0m] filter-over-partition-and-running-default-frame [2m(8.39ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-x-y-prefix-suffix                  [2m(479.15µs)[0m

[1mtests/window/filter-over.sqltest[0m
  [[32mPASS[0m] filter-over-same-row-columns-and-case    [2m(6.85ms)[0m
  [[32mPASS[0m] filter-over-group-concat-collation-order [2m(6.88ms)[0m
  [[32mPASS[0m] filter-over-identical-window-different-filters [2m(7.28ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-x-y-incorrect-suffix               [2m(473.49µs)[0m

[1mtests/update_expression_index.sqltest[0m
  [[32mPASS[0m] string-concat-expression-index           [2m(8.51ms)[0m

[1mtests/window/filter-over.sqltest[0m
  [[32mPASS[0m] filter-over-cte                          [2m(7.12ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-x-y-long-prefix                    [2m(487.26µs)[0m

[1mtests/window/filter-over.sqltest[0m
  [[32mPASS[0m] filter-over-subquery-outer-where-alias   [2m(6.88ms)[0m
  [[32mPASS[0m] filter-over-order-by-expression          [2m(6.79ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-x-y-shorter-suffix                 [2m(474.56µs)[0m

[1mtests/window/filter-over.sqltest[0m
  [[32mPASS[0m] filter-over-grouped-result               [2m(6.51ms)[0m

[1mtests/update_expression_index.sqltest[0m
  [[32mPASS[0m] cast-expression-index                    [2m(8.23ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-x-y-shorter-prefix                 [2m(484.04µs)[0m

[1mtests/window/filter-over.sqltest[0m
  [[32mPASS[0m] filter-over-filter-expression-subquery   [2m(6.61ms)[0m
  [[32mPASS[0m] filter-over-between-and-null-predicates  [2m(7.12ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-x-y-random-order                   [2m(457.86µs)[0m

[1mtests/window/filter-over.sqltest[0m
  [[32mPASS[0m] filter-over-count-star-is-true-false     [2m(6.12ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-x-y-char-in-the-middle             [2m(440.87µs)[0m

[1mtests/update_expression_index.sqltest[0m
  [[32mPASS[0m] null-values-expression-index             [2m(8.56ms)[0m

[1mtests/window/filter-over.sqltest[0m
  [[32mPASS[0m] filter-over-empty-prefix-defaults        [2m(6.12ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-x-y-character-outside-set          [2m(450.80µs)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-with-group-by                     [2m(527.35ms)[0m

[1mtests/window/filter-over.sqltest[0m
  [[32mPASS[0m] filter-over-peer-group-default-range     [2m(6.13ms)[0m

[1mtests/like.sqltest[0m
  [[32mPASS[0m] like-fn-esc-12                           [2m(498.92µs)[0m

[1mtests/window/filter-over.sqltest[0m
  [[32mPASS[0m] filter-over-repeated-expression-cache    [2m(6.08ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-dash-separated                     [2m(493.38µs)[0m

[1mtests/window/filter-over.sqltest[0m
  [[32mPASS[0m] filter-over-grouped-filter-aggregate-predicate [2m(6.64ms)[0m

[1mtests/update_expression_index.sqltest[0m
  [[32mPASS[0m] update-all-rows                          [2m(9.91ms)[0m

[1mtests/window/filter-over.sqltest[0m
  [[32mPASS[0m] filter-over-repeated-deterministic-correlated-subquery-filter [2m(7.15ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-multi-dash-separated               [2m(464.76µs)[0m

[1mtests/window/filter-over.sqltest[0m
  [[32mPASS[0m] filter-over-multiple-window-layers-with-filters [2m(9.30ms)[0m
  [[32mPASS[0m] filter-over-scalar-function-rejected     [2m(8.79ms)[0m
  [[32mPASS[0m] filter-over-repeated-nondeterministic-call [2m(5.80ms)[0m

[1mtests/scalar-functions.sqltest[0m
  [[32mPASS[0m] unhex-space-separated                    [2m(469.44µs)[0m

[1mtests/window/filter-over.sqltest[0m
  [[32mPASS[0m] filter-over-empty-filtered-partition     [2m(6.93ms)[0m
  [[32mPASS[0m] filter-over-mixed-deterministic-and-nondeterministic-dedup [2m(5.99ms)[0m

[1mtests/update_expression_index.sqltest[0m
  [[32mPASS[0m] mixed-regular-and-expression-index       [2m(10.52ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-limit-offset                      [2m(194.73ms)[0m

[1mtests/window/filter-over.sqltest[0m
  [[32mPASS[0m] filter-over-distinct-argument-rejected   [2m(9.16ms)[0m
  [[32mPASS[0m] filter-over-multi-key-partition          [2m(6.99ms)[0m

[1mtests/window/group-by-order-by-partition.sqltest[0m
  [[32mPASS[0m] window-group-by-order-by-partition-basic [2m(6.64ms)[0m
  [[32mPASS[0m] window-group-by-partition-sum-desc       [2m(6.84ms)[0m
  [[32mPASS[0m] window-group-by-partition-max            [2m(7.01ms)[0m

[1mtests/update_expression_index.sqltest[0m
  [[32mPASS[0m] coalesce-expression-index                [2m(8.11ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-order-by-limit-offset             [2m(206.58ms)[0m

[1mtests/window/filter-over.sqltest[0m
  [[32mPASS[0m] filter-over-non-boolean-truthy-predicates [2m(6.57ms)[0m
  [[32mPASS[0m] filter-over-nested-window-in-filter-rejected [2m(8.74ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] affinity-conversion-19                   [2m(6.34ms)[0m

[1mtests/window/group-by-order-by-partition.sqltest[0m
  [[32mPASS[0m] window-group-by-partition-with-aggregate [2m(6.84ms)[0m
  [[32mPASS[0m] window-group-by-partition-empty-table    [2m(4.86ms)[0m
  [[32mPASS[0m] window-group-by-partition-nulls          [2m(6.84ms)[0m
  [[32mPASS[0m] window-group-by-partition-desc-outer-order [2m(7.16ms)[0m

[1mtests/update_expression_index.sqltest[0m
  [[32mPASS[0m] abs-expression-index                     [2m(8.01ms)[0m

[1mtests/window/filter-over.sqltest[0m
  [[32mPASS[0m] filter-over-correlated-subquery-in-filter [2m(7.52ms)[0m
  [[32mPASS[0m] filter-over-non-aggregate-window-rejected [2m(9.02ms)[0m

[1mtests/window/memory.sqltest[0m
  [[32mPASS[0m] window-partition-by-duplicate-columns    [2m(5.06ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] affinity-conversion-20                   [2m(6.47ms)[0m

[1mtests/window/memory.sqltest[0m
  [[32mPASS[0m] window-partition-by-duplicate-columns-multi-row [2m(6.10ms)[0m
  [[32mPASS[0m] window-partition-by-duplicate-columns-collation [2m(5.65ms)[0m
  [[32mPASS[0m] window-collate-partition-by              [2m(5.39ms)[0m
  [[32mPASS[0m] window-with-correlated-scalar-subquery-partition [2m(6.51ms)[0m
  [[32mPASS[0m] window-row-number-no-order               [2m(5.31ms)[0m
  [[32mPASS[0m] window-with-uncorrelated-scalar-subquery [2m(5.91ms)[0m
  [[32mPASS[0m] window-row-number-all-null-partition     [2m(5.95ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] affinity-conversion-21                   [2m(6.92ms)[0m

[1mtests/window/memory.sqltest[0m
  [[32mPASS[0m] window-row-number-with-where             [2m(6.06ms)[0m
  [[32mPASS[0m] window-row-number-with-limit-offset      [2m(5.80ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] case-insensitive-alias                   [2m(50.00ms)[0m

[1mtests/window/memory.sqltest[0m
  [[32mPASS[0m] window-collate-order-by                  [2m(5.41ms)[0m
  [[32mPASS[0m] window-row-number-top-n-per-group        [2m(6.82ms)[0m
  [[32mPASS[0m] window-row-number-nulls-in-order-by      [2m(5.75ms)[0m
  [[32mPASS[0m] window-in-exists-subquery-referencing-outer-column [2m(6.34ms)[0m
  [[32mPASS[0m] window-row-number-empty-table            [2m(5.33ms)[0m
  [[32mPASS[0m] window-row-number-combined-with-aggregate-window [2m(6.08ms)[0m
  [[32mPASS[0m] window-row-number-single-row             [2m(5.31ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] affinity-conversion-22                   [2m(6.49ms)[0m

[1mtests/window/memory.sqltest[0m
  [[32mPASS[0m] window-row-number-multiple-partition-keys [2m(6.45ms)[0m
  [[32mPASS[0m] window-with-correlated-scalar-subquery   [2m(5.85ms)[0m
  [[32mPASS[0m] window-row-number-nulls-in-partition-by  [2m(6.00ms)[0m
  [[32mPASS[0m] window-row-number-multiple-different-windows [2m(6.90ms)[0m
  [[32mPASS[0m] window-row-number-partition-order        [2m(6.13ms)[0m
  [[32mPASS[0m] window-row-number-in-expression          [2m(5.73ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] large-values-comparison                  [2m(2.90ms)[0m

[1mtests/window/memory.sqltest[0m
  [[32mPASS[0m] window-row-number-alias-in-order-by      [2m(5.66ms)[0m
  [[32mPASS[0m] window-row-number-with-named-window      [2m(6.20ms)[0m
  [[32mPASS[0m] window-row-number-with-join              [2m(8.40ms)[0m
  [[32mPASS[0m] window-row-number-distinct               [2m(5.84ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] values-select-empty-db                   [2m(3.40ms)[0m

[1mtests/window/memory.sqltest[0m
  [[32mPASS[0m] named-window-with-order-by               [2m(5.80ms)[0m
  [[32mPASS[0m] window-row-number-wrong-arg-count        [2m(7.12ms)[0m
  [[32mPASS[0m] window-row-number-in-check-constraint    [2m(7.26ms)[0m
  [[32mPASS[0m] window-row-number-over-group-by          [2m(5.68ms)[0m
  [[32mPASS[0m] named-window-in-subquery                 [2m(6.08ms)[0m
  [[32mPASS[0m] named-window-case-insensitive            [2m(5.33ms)[0m
  [[32mPASS[0m] named-window-ntile                       [2m(12.59ms)[0m
  [[32mPASS[0m] named-window-rank-dense-rank             [2m(13.98ms)[0m
  [[32mPASS[0m] named-window-multiple-distinct           [2m(7.02ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] select-sqlite-schema-empty-db            [2m(3.82ms)[0m

[1mtests/window/memory.sqltest[0m
  [[32mPASS[0m] named-window-row-number                  [2m(5.75ms)[0m
  [[32mPASS[0m] named-window-different-windows-per-function [2m(6.88ms)[0m
  [[32mPASS[0m] named-window-lag-lead                    [2m(12.74ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-outer-grouping-cte              [2m(183.65ms)[0m

[1mtests/window/memory.sqltest[0m
  [[32mPASS[0m] named-window-three-windows               [2m(7.91ms)[0m
  [[32mPASS[0m] named-window-reused-by-many-functions    [2m(6.28ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] between-is-not-null-precedence           [2m(2.89ms)[0m

[1mtests/window/memory.sqltest[0m
  [[32mPASS[0m] named-window-empty-spec                  [2m(5.41ms)[0m
  [[32mPASS[0m] named-window-with-group-by               [2m(13.17ms)[0m
  [[32mPASS[0m] window-same-window-fn-used-multiple-times [2m(6.64ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-join-using-with-outer-limit     [2m(1.12ms)[0m

[1mtests/window/memory.sqltest[0m
  [[32mPASS[0m] window-order-by-position-references-window-fn [2m(6.34ms)[0m
  [[32mPASS[0m] window-same-window-fn-used-three-times   [2m(5.66ms)[0m
  [[32mPASS[0m] window-same-window-fn-inside-coalesce    [2m(5.47ms)[0m
  [[32mPASS[0m] window-same-star-window-fn-twice         [2m(5.37ms)[0m

[1mtests/select/memory.sqltest[0m
  [[32mPASS[0m] like-is-not-null-precedence              [2m(2.83ms)[0m

[1mtests/window/memory.sqltest[0m
  [[32mPASS[0m] window-same-row-number-twice             [2m(5.15ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-join-using-with-outer-limit-cte [2m(1.18ms)[0m

[1mtests/window/memory.sqltest[0m
  [[32mPASS[0m] window-named-window-same-fn-multiple-references [2m(5.94ms)[0m

[1mtests/window/multi-window-ordering.sqltest[0m
  [[32mPASS[0m] three-row-number-windows-distinct-defs   [2m(7.41ms)[0m

[1mtests/window-agg-row-value.sqltest[0m
  [[32mPASS[0m] window-agg-case-else-row-value           [2m(4.37ms)[0m

[1mtests/window/multi-window-ordering.sqltest[0m
  [[32mPASS[0m] three-windows-aggregate-between          [2m(7.47ms)[0m
  [[32mPASS[0m] empty-over-first-then-ordered-then-partitioned [2m(7.48ms)[0m

[1mtests/window-agg-row-value.sqltest[0m
  [[32mPASS[0m] window-agg-case-then-row-value           [2m(4.74ms)[0m

[1mtests/subquery/default.sqltest[0m
  [[32mPASS[0m] subquery-join-using-with-inner-limit     [2m(1.19ms)[0m
  [[32mPASS[0m] subquery-join-using-with-inner-limit-cte [2m(1.14ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-timezone-day-change-negative   [2m(455.11µs)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-function-list-has-count           [2m(2.68ms)[0m
  [[32mPASS[0m] pragma-function-nontext-argument         [2m(800.78µs)[0m
  [[32mPASS[0m] pragma-vtab-nontext-argument             [2m(806.51µs)[0m
  [[32mPASS[0m] pragma-function-no-arguments             [2m(566.48µs)[0m
  [[32mPASS[0m] pragma-vtab-no-arguments                 [2m(553.74µs)[0m
  [[32mPASS[0m] pragma-vtab-join                         [2m(1.30ms)[0m
  [[32mPASS[0m] pragma-vtab-reversed-join-order          [2m(1.26ms)[0m

[1mtests/alter-rename-column-temp-trigger.sqltest[0m
  [[31mFAIL[0m] rename-column-with-temp-trigger          [2m(10.05ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-module-list-nonempty              [2m(2.32ms)[0m
  [[32mPASS[0m] pragma-temp-store                        [2m(365.20µs)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-clause-eq                          [2m(911.87µs)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-temp-store-set-memory             [2m(429.16µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] unixepoch-after-start-of-day             [2m(483.58µs)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-temp-store-set-file               [2m(428.56µs)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-timezone-iso8601               [2m(469.89µs)[0m
  [[32mPASS[0m] date-with-timezone-and-milliseconds      [2m(453.84µs)[0m
  [[32mPASS[0m] date-with-invalid-timezone               [2m(454.25µs)[0m
  [[32mPASS[0m] date-with-modifier-add-days              [2m(457.60µs)[0m
  [[32mPASS[0m] date-with-modifier-subtract-days         [2m(465.08µs)[0m
  [[32mPASS[0m] date-with-multiple-modifiers             [2m(473.79µs)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-function-list-has-abs             [2m(2.58ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-invalid-modifier               [2m(452.38µs)[0m
  [[32mPASS[0m] julianday-null-input                     [2m(433.54µs)[0m
  [[32mPASS[0m] julianday-invalid-input                  [2m(457.27µs)[0m
  [[32mPASS[0m] julianday-empty-string                   [2m(438.23µs)[0m
  [[32mPASS[0m] julianday-negative-timezone              [2m(454.38µs)[0m
  [[32mPASS[0m] julianday-z-suffix                       [2m(450.79µs)[0m
  [[32mPASS[0m] julianday-iso8601-t-separator            [2m(452.43µs)[0m
  [[32mPASS[0m] julianday-out-of-range                   [2m(448.78µs)[0m
  [[32mPASS[0m] julianday-string-with-modifier           [2m(466.48µs)[0m
  [[32mPASS[0m] julianday-string-with-start-of-month     [2m(463.39µs)[0m
  [[32mPASS[0m] julianday-string-with-minutes-modifier   [2m(462.28µs)[0m
  [[32mPASS[0m] timediff-one-arg                         [2m(3.92ms)[0m
  [[32mPASS[0m] date-floor-then-add-day                  [2m(501.21µs)[0m
  [[32mPASS[0m] datetime-floor-then-add-hours            [2m(478.18µs)[0m
  [[32mPASS[0m] date-fractional-month-floor              [2m(475.30µs)[0m
  [[32mPASS[0m] datetime-fractional-month-floor          [2m(472.28µs)[0m
  [[32mPASS[0m] unixepoch-not-first-modifier             [2m(511.92µs)[0m
  [[32mPASS[0m] julianday-not-first-modifier             [2m(460.06µs)[0m

[1mtests/not_between.sqltest[0m
  [[32mPASS[0m] not-between-subquery-bound-orderby-offset [2m(20.33ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] inner-then-full-outer-join               [2m(10.73ms)[0m
  [[32mPASS[0m] full-outer-join-expression-in-on         [2m(7.61ms)[0m

[1mtests/multi_index_or_compound.sqltest[0m
  [[32mPASS[0m] compound-or-compound-branch-seek         [2m(11.50ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-null-probe-key           [2m(7.27ms)[0m

[1mtests/multi_index_or_compound.sqltest[0m
  [[32mPASS[0m] compound-or-mixed-residual               [2m(18.56ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-many-duplicates          [2m(16.69ms)[0m

[1mtests/multi_index_or_join.sqltest[0m
  [[32mPASS[0m] multi-index-or-join-basic                [2m(12.45ms)[0m
  [[32mPASS[0m] multi-index-or-join-cte                  [2m(12.42ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-where-references-both    [2m(20.81ms)[0m

[1mtests/window-selfjoin-reset-sorter.sqltest[0m
  [[32mPASS[0m] window-over-triple-selfjoin-select       [2m(94.52ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-auto-unixepoch-bottom          [2m(571.30µs)[0m
  [[32mPASS[0m] julianday-after-hours-modifier           [2m(448.34µs)[0m
  [[32mPASS[0m] date-negative-date-offset                [2m(440.74µs)[0m
  [[32mPASS[0m] date-negative-date-offset-2              [2m(453.32µs)[0m

[1mtests/window-selfjoin-reset-sorter.sqltest[0m
  [[32mPASS[0m] window-over-triple-selfjoin-update       [2m(95.61ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-1200-offset                    [2m(431.95µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-self-join-equi                [2m(5.88ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-12-offset                      [2m(459.38µs)[0m
  [[32mPASS[0m] datetime-huge-number                     [2m(457.43µs)[0m
  [[32mPASS[0m] datetime-negative-date-offset            [2m(459.97µs)[0m
  [[32mPASS[0m] unixepoch-negative-date-offset           [2m(453.13µs)[0m
  [[32mPASS[0m] date-with-auto-julianday-top             [2m(441.12µs)[0m

[1mtests/negative_zero.sqltest[0m
  [[32mPASS[0m] negative-zero-comparison-less-than       [2m(2.81ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-auto-unixepoch-top             [2m(445.82µs)[0m
  [[32mPASS[0m] date-with-auto-julianday-bottom          [2m(449.60µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-self-join-non-equi            [2m(5.72ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] age_idx_order_desc                       [2m(172.30ms)[0m
  [[32mPASS[0m] rowid_or_integer_pk_desc                 [2m(784.74µs)[0m

[1mtests/mvcc-update-noop.sqltest[0m
  [[32mPASS[0m] mvcc-update-self-multiple-rows           [2m(11.35ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-then-inner-equi               [2m(9.29ms)[0m

[1mtests/multi_index_or_join.sqltest[0m
  [[32mPASS[0m] multi-index-or-join-all-nodes            [2m(27.09ms)[0m

[1mtests/negative_zero.sqltest[0m
  [[32mPASS[0m] negative-zero-comparison-greater-than    [2m(2.32ms)[0m

[1mtests/multi_index_or_join.sqltest[0m
  [[32mPASS[0m] multi-index-or-join-parenthesized        [2m(21.32ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-then-inner-non-equi           [2m(9.27ms)[0m

[1mtests/negative_zero.sqltest[0m
  [[32mPASS[0m] negative-zero-in-table                   [2m(3.24ms)[0m
  [[32mPASS[0m] negative-zero-in-table-comparison        [2m(3.28ms)[0m

[1mtests/mvcc-begin-within-concurrent.sqltest[0m
  [[32mPASS[0m] begin-immediate-inside-begin-concurrent-errors [2m(18.19ms)[0m

[1mtests/mvcc-update-noop.sqltest[0m
  [[32mPASS[0m] mvcc-update-different-value              [2m(10.54ms)[0m

[1mtests/negative_zero.sqltest[0m
  [[32mPASS[0m] negative-zero-arithmetic-result          [2m(4.53ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] inner-then-full-outer-reorderable        [2m(10.33ms)[0m

[1mtests/negative_zero.sqltest[0m
  [[32mPASS[0m] negative-zero-literal                    [2m(2.76ms)[0m
  [[32mPASS[0m] negative-zero-multiplication             [2m(2.65ms)[0m
  [[32mPASS[0m] negative-zero-order-by                   [2m(5.46ms)[0m
  [[32mPASS[0m] negative-zero-tiny-times-zero            [2m(2.79ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-then-left                     [2m(9.73ms)[0m

[1mtests/negative_zero.sqltest[0m
  [[32mPASS[0m] negative-zero-comparison-equals          [2m(2.76ms)[0m
  [[32mPASS[0m] negative-zero-distinct                   [2m(5.32ms)[0m
  [[32mPASS[0m] negative-zero-comparison-not-equals      [2m(2.59ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-of-datetime                         [2m(443.44µs)[0m
  [[32mPASS[0m] date-with-unixepoch                      [2m(425.67µs)[0m
  [[32mPASS[0m] date-with-julianday                      [2m(435.19µs)[0m
  [[32mPASS[0m] time-no-arg                              [2m(428.45µs)[0m
  [[32mPASS[0m] time-current-time                        [2m(452.97µs)[0m
  [[32mPASS[0m] time-specific-time                       [2m(430.05µs)[0m

[1mtests/mvcc-begin-within-concurrent.sqltest[0m
  [[32mPASS[0m] begin-deferred-inside-begin-concurrent-errors [2m(18.25ms)[0m

[1mtests/negative_zero.sqltest[0m
  [[32mPASS[0m] negative-zero-group-by                   [2m(5.50ms)[0m

[1mtests/not_between.sqltest[0m
  [[32mPASS[0m] not-between-self-ref-group-by            [2m(5.30ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-then-two-inner                [2m(13.00ms)[0m

[1mtests/nested-subquery-scope-depth.sqltest[0m
  [[32mPASS[0m] three-level-outer-ref                    [2m(8.98ms)[0m

[1mtests/not_between.sqltest[0m
  [[32mPASS[0m] not-between-self-ref-no-group-by         [2m(4.90ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-group-by-having          [2m(8.13ms)[0m

[1mtests/not_between.sqltest[0m
  [[32mPASS[0m] between-self-ref-group-by                [2m(5.07ms)[0m

[1mtests/nested-subquery-scope-depth.sqltest[0m
  [[32mPASS[0m] nested-correlated-same-column-name       [2m(8.24ms)[0m

[1mtests/mvcc-begin-within-concurrent.sqltest[0m
  [[32mPASS[0m] begin-concurrent-inside-begin-concurrent-errors [2m(17.90ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] pk-on-conflict-rollback                  [2m(4.69ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-sum-with-nulls           [2m(7.80ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-rollback-verify-state          [2m(4.93ms)[0m

[1mtests/nested-subquery-scope-depth.sqltest[0m
  [[32mPASS[0m] nested-explicit-outer-ref                [2m(8.73ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-julian-day-integer                  [2m(460.93µs)[0m
  [[32mPASS[0m] time-iso8601                             [2m(437.23µs)[0m
  [[32mPASS[0m] time-with-milliseconds                   [2m(451.26µs)[0m

[1mtests/mvcc-update-noop.sqltest[0m
  [[32mPASS[0m] mvcc-update-self-does-not-delete-rows    [2m(10.55ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-fail-autocommit                [2m(5.07ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] right-join-null-keys                     [2m(7.95ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-fail-in-transaction            [2m(5.32ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] left-then-full-outer-chain-errors        [2m(10.59ms)[0m

[1mtests/mvcc-update-noop.sqltest[0m
  [[32mPASS[0m] mvcc-update-self-data-intact             [2m(10.32ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-fail-partial-update            [2m(5.35ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] right-then-full-outer-chain-errors       [2m(11.67ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update_cache_full_regression_test_1625   [2m(6.19ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-fail-tx-stays-active           [2m(5.79ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-count-verification       [2m(12.33ms)[0m
  [[32mPASS[0m] right-join-aggregate-counts              [2m(7.76ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-with-null-condition               [2m(5.87ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-ignore-fk-still-fails          [2m(6.51ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-to-null                           [2m(5.85ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-with-offset              [2m(7.29ms)[0m
  [[32mPASS[0m] full-outer-case-expressions              [2m(8.08ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-replace-fk-still-fails         [2m(6.43ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-with-randomblob                   [2m(5.46ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-distinct                 [2m(7.61ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-ignore-fk-still-fails          [2m(6.75ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-multiple-columns                  [2m(5.39ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] three-way-full-outer-join-errors         [2m(10.72ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-true-expr                         [2m(5.87ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-replace-fk-still-fails         [2m(7.10ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] right-join-where-filters-nulls           [2m(7.71ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-fail-fk-still-fails            [2m(9.77ms)[0m
  [[32mPASS[0m] insert-or-rollback-fk-still-fails        [2m(6.26ms)[0m
  [[32mPASS[0m] multiple-constraints-first-wins          [2m(5.39ms)[0m
  [[32mPASS[0m] update-multiple-rows-one-fails           [2m(5.53ms)[0m
  [[32mPASS[0m] insert-select-or-ignore                  [2m(7.65ms)[0m
  [[32mPASS[0m] insert-select-or-replace                 [2m(7.91ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-with-group-by                     [2m(730.55ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-ignore-empty                   [2m(5.44ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-multiple-and                       [2m(1.27ms)[0m
  [[32mPASS[0m] where-multiple-or                        [2m(928.69µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-stress-matched-bits      [2m(56.64ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where_in_list                            [2m(812.63µs)[0m
  [[32mPASS[0m] where_not_in_list                        [2m(934.54µs)[0m
  [[32mPASS[0m] where_in_list_or_another_list            [2m(936.85µs)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] self-ref-update-or-ignore                [2m(5.91ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where_not_in_list_and_not_in_another_list [2m(1.11ms)[0m
  [[32mPASS[0m] where_in_list_or_not_in_another_list     [2m(1.11ms)[0m
  [[32mPASS[0m] where_in_empty_list                      [2m(509.62µs)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-ignore-returning               [2m(5.63ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] right-then-left-chain                    [2m(9.74ms)[0m

[1mtests/null/memory.sqltest[0m
  [[32mPASS[0m] composite-index-null-second-col-le       [2m(6.65ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-replace-returning              [2m(5.62ms)[0m

[1mtests/null/default.sqltest[0m
  [[32mPASS[0m] isnull                                   [2m(2.83ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-one-operand-null-eq         [2m(68.16ms)[0m

[1mtests/null/default.sqltest[0m
  [[32mPASS[0m] is-null                                  [2m(2.99ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-ignore-returning               [2m(5.87ms)[0m

[1mtests/null/default.sqltest[0m
  [[32mPASS[0m] notnull                                  [2m(2.84ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] nested-parens-and-inside-or-regression-test [2m(72.53ms)[0m

[1mtests/null/default.sqltest[0m
  [[32mPASS[0m] not-null                                 [2m(2.82ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] right-join-select-star-with-aggregate    [2m(7.90ms)[0m

[1mtests/null/default.sqltest[0m
  [[32mPASS[0m] sel-true                                 [2m(2.80ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] update-or-replace-returning              [2m(6.18ms)[0m

[1mtests/null/default.sqltest[0m
  [[32mPASS[0m] sel-false                                [2m(2.80ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-ignore-autoincrement           [2m(6.87ms)[0m

[1mtests/null/memory.sqltest[0m
  [[32mPASS[0m] not-null-just-cuz-unique                 [2m(5.21ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] left-then-right-join-chain-errors        [2m(10.96ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] insert-or-replace-autoincrement          [2m(6.50ms)[0m

[1mtests/null/memory.sqltest[0m
  [[32mPASS[0m] null-comparison-asc-gt                   [2m(6.30ms)[0m

[1mtests/on_conflict.sqltest[0m
  [[32mPASS[0m] mvcc-upsert-set-pk-on-conflict-target-resolves [2m(8.88ms)[0m

[1mtests/null/memory.sqltest[0m
  [[32mPASS[0m] null-comparison-desc-gt                  [2m(10.44ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] right-then-right-join-chain-errors       [2m(16.83ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] pk-on-conflict-replace                   [2m(6.20ms)[0m
  [[32mPASS[0m] pk-on-conflict-ignore                    [2m(5.98ms)[0m

[1mtests/null/memory.sqltest[0m
  [[32mPASS[0m] null-comparison-asc-lt                   [2m(13.50ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-function-in-order-by              [2m(979.41ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] pk-on-conflict-abort                     [2m(4.98ms)[0m
  [[32mPASS[0m] pk-on-conflict-fail                      [2m(4.94ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-then-right-join-errors        [2m(19.82ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-nested-expressions      [2m(4.85ms)[0m

[1mtests/null/memory.sqltest[0m
  [[32mPASS[0m] null-comparison-desc-lt                  [2m(12.47ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-with-not-exists-subquery      [2m(11.39ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-column-arithmetic       [2m(4.76ms)[0m
  [[32mPASS[0m] insert-returning-complex-expression      [2m(4.96ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-with-in-subquery              [2m(8.94ms)[0m
  [[32mPASS[0m] full-outer-join-expr-both-sides-on       [2m(13.60ms)[0m

[1mtests/null/memory.sqltest[0m
  [[32mPASS[0m] null-comparison-asc-ge                   [2m(12.51ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-multiple-column-expression [2m(5.05ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-with-where-equijoin           [2m(6.80ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-function-call           [2m(4.80ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-with-rhs-self-filter          [2m(6.79ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-function-multiple-columns [2m(5.13ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] right-join-select-star-column-order      [2m(14.07ms)[0m

[1mtests/null/memory.sqltest[0m
  [[32mPASS[0m] null-comparison-desc-ge                  [2m(15.29ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-function-with-expression [2m(4.89ms)[0m
  [[32mPASS[0m] insert-returning-mixed-expressions       [2m(4.82ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] materialized-build-where-filter-not-dropped [2m(15.41ms)[0m

[1mtests/null/memory.sqltest[0m
  [[32mPASS[0m] null-comparison-asc-le                   [2m(10.78ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-multiple-rows           [2m(5.04ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] right-join-select-star-no-matches        [2m(13.48ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-multiple-rows-expressions [2m(5.09ms)[0m

[1mtests/null/memory.sqltest[0m
  [[32mPASS[0m] null-comparison-desc-le                  [2m(6.33ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] left-hash-join-on-left-in-list-preserves-unmatched-rows [2m(7.17ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-null-values             [2m(5.12ms)[0m

[1mtests/null/memory.sqltest[0m
  [[32mPASS[0m] null-comparison-desc-order-by            [2m(6.24ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-invalid-input                       [2m(512.31µs)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-nested-in-expression              [2m(1.09s)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-julian-day-float                    [2m(448.87µs)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] left-hash-join-on-left-equality-preserves-unmatched-rows [2m(7.29ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-null-expression         [2m(4.91ms)[0m

[1mtests/where/small.sqltest[0m
  [[32mPASS[0m] where-not-equals-a-with-nulls            [2m(3.75ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] right-join-select-star-all-match         [2m(16.15ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-rowid-expression        [2m(4.78ms)[0m

[1mtests/where/small.sqltest[0m
  [[32mPASS[0m] where-is-null-combined                   [2m(3.51ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] left-hash-join-on-left-not-in-list-preserves-unmatched-rows [2m(7.43ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-autoincrement           [2m(6.16ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] time-null-input                          [2m(481.90µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-rowid                   [2m(4.67ms)[0m

[1mtests/where/small.sqltest[0m
  [[32mPASS[0m] where-is-not-null-combined               [2m(3.73ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] upsert-returning-expression-insert       [2m(4.96ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] full-outer-join-non-equijoin-duplicates  [2m(7.07ms)[0m
  [[32mPASS[0m] right-join-select-star-empty-left        [2m(12.87ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-correlated-sub-refs-from-extra [2m(10.15ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-case-expression         [2m(4.92ms)[0m

[1mtests/joins/derived_subquery_on_predicate.sqltest[0m
  [[32mPASS[0m] issue-7362-left-join-or-false            [2m(7.05ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-string-concat           [2m(5.08ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-self-ref-aggregate           [2m(6.63ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-substring               [2m(4.82ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] right-join-select-star-many-columns      [2m(13.23ms)[0m

[1mtests/joins/derived_subquery_on_predicate.sqltest[0m
  [[32mPASS[0m] issue-7363-left-join-not-opposite        [2m(8.34ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-snapshot-where-set-same-col  [2m(6.51ms)[0m

[1mtests/joins/derived_subquery_on_predicate.sqltest[0m
  [[32mPASS[0m] issue-7364-partitioned-comparison        [2m(8.52ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] upsert-returning-insert-path             [2m(11.15ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] right-join-select-star-duplicate-keys    [2m(14.77ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] upsert-returning-update-path             [2m(8.43ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-deep-join-chain              [2m(23.93ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] upsert-returning-do-nothing              [2m(7.65ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] right-join-table-star                    [2m(17.40ms)[0m

[1mtests/joins/derived_subquery_on_predicate.sqltest[0m
  [[32mPASS[0m] issue-7365-null-guarded                  [2m(23.30ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-nested-join-subquery         [2m(10.25ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] upsert-returning-table-qualified         [2m(8.59ms)[0m

[1mtests/join/outer_hash_join.sqltest[0m
  [[32mPASS[0m] right-join-select-star-with-where        [2m(7.79ms)[0m

[1mtests/orderby/memory.sqltest[0m
  [[32mPASS[0m] orderby_rowid_truncation_explicit_rowid  [2m(5.03ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] upsert-returning-multiple-rows           [2m(12.19ms)[0m
  [[32mPASS[0m] upsert-returning-where-no-update         [2m(5.22ms)[0m
  [[32mPASS[0m] upsert-returning-expression-update       [2m(5.50ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] nested-parens-conditionals-and-or-and    [2m(200.74ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] upsert-returning-where-update            [2m(5.45ms)[0m
  [[32mPASS[0m] upsert-returning-complex-expression      [2m(5.56ms)[0m

[1mtests/pragma/foreign_key_list.sqltest[0m
  [[32mPASS[0m] pragma-foreign-key-list-mixed            [2m(5.83ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-function-list-scalar              [2m(2.39ms)[0m

[1mtests/update-from.sqltest[0m
  [[32mPASS[0m] update-from-subquery-with-limit          [2m(8.63ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] upsert-returning-function                [2m(5.42ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-function-list-aggregate           [2m(2.27ms)[0m
  [[32mPASS[0m] test-pragma-journal-mode-unsupported     [2m(342.74µs)[0m
  [[32mPASS[0m] pragma-function-list-omits-unimplemented-window-functions [2m(4.38ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] upsert-returning-literal                 [2m(4.90ms)[0m

[1mtests/pragma/default.sqltest[0m
  [[32mPASS[0m] pragma-function-list-window-row-number   [2m(2.59ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-column-in-order-by                [2m(1.14s)[0m

[1mtests/null/memory.sqltest[0m
  [[32mPASS[0m] composite-index-null-second-col-lt       [2m(6.98ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-delete-cascade-scenario    [2m(11.36ms)[0m
  [[32mPASS[0m] partial-index-update-exit-change-key-reenter [2m(13.06ms)[0m

[1mtests/orderby/memory.sqltest[0m
  [[32mPASS[0m] order-by-plus-2                          [2m(5.17ms)[0m

[1mtests/null/memory.sqltest[0m
  [[32mPASS[0m] composite-index-null-second-col-ge       [2m(6.38ms)[0m

[1mtests/orderby/memory.sqltest[0m
  [[32mPASS[0m] order-by-plus-1                          [2m(5.12ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-delete-null-where          [2m(9.92ms)[0m

[1mtests/null/memory.sqltest[0m
  [[32mPASS[0m] composite-index-null-second-col-le-desc  [2m(6.38ms)[0m

[1mtests/orderby/memory.sqltest[0m
  [[32mPASS[0m] order-by-plus-2-desc                     [2m(5.12ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-binary-one-operand-null-or-eq      [2m(56.85ms)[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-false-3 [2m(428.81µs)[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-identifier-false [2m(402.81µs)[0m

[1mtests/null/memory.sqltest[0m
  [[32mPASS[0m] composite-index-null-second-col-lt-desc  [2m(6.53ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-false-2 [2m(406.39µs)[0m

[1mtests/orderby/memory.sqltest[0m
  [[32mPASS[0m] group-by-plus-1                          [2m(5.07ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-delete-complex-where       [2m(10.08ms)[0m

[1mtests/pragma/foreign_key_list.sqltest[0m
  [[32mPASS[0m] pragma-foreign-key-list-hidden-arg       [2m(5.96ms)[0m

[1mtests/null/memory.sqltest[0m
  [[32mPASS[0m] composite-index-null-second-col-gt       [2m(6.37ms)[0m

[1mtests/orderby/memory.sqltest[0m
  [[32mPASS[0m] order-by-negative-1                      [2m(9.09ms)[0m

[1mtests/pragma/foreign_key_list.sqltest[0m
  [[32mPASS[0m] pragma-foreign-key-list-equals-syntax    [2m(5.61ms)[0m

[1mtests/null/memory.sqltest[0m
  [[32mPASS[0m] composite-index-mixed-null-nonnull       [2m(7.05ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-update-enter-conflict-1    [2m(11.39ms)[0m

[1mtests/pragma/foreign_key_list.sqltest[0m
  [[32mPASS[0m] pragma-foreign-key-list-table-valued-function [2m(6.05ms)[0m

[1mtests/null/memory.sqltest[0m
  [[32mPASS[0m] composite-index-prefix-eq-no-range-no-crash [2m(6.62ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] asc-default-nulls-first                  [2m(5.20ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-update-change-key-conflict [2m(11.47ms)[0m

[1mtests/orderby/memory.sqltest[0m
  [[32mPASS[0m] order-by-negative-100                    [2m(18.00ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-multi-predicate-references-rowid-alias [2m(11.87ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] desc-default-nulls-last                  [2m(5.12ms)[0m
  [[32mPASS[0m] asc-nulls-first                          [2m(5.11ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-update-rowid-no-self-conflict [2m(7.33ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] asc-nulls-last                           [2m(5.19ms)[0m
  [[32mPASS[0m] desc-nulls-first                         [2m(5.16ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-update-exit-then-reenter   [2m(20.28ms)[0m

[1mtests/orderby/memory.sqltest[0m
  [[32mPASS[0m] order-by-out-of-range                    [2m(18.40ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-update-batch-crossing      [2m(11.42ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] desc-nulls-last                          [2m(5.43ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-update-stay-in-predicate-change-to-unique [2m(7.56ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] implicit-asc-nulls-last                  [2m(5.36ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-nested-in-expression              [2m(1.10s)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-update-multirow-conflict   [2m(11.97ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] orderby_asc_verify_rows                  [2m(485.64ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] implicit-asc-nulls-first                 [2m(5.26ms)[0m

[1mtests/orderby/memory.sqltest[0m
  [[32mPASS[0m] order-by-zero                            [2m(18.22ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-update-unrelated-column    [2m(7.79ms)[0m
  [[32mPASS[0m] partial-index-update-only-predicate-col-error [2m(10.81ms)[0m

[1mtests/nulls-first-last.sqltest[0m
  [[32mPASS[0m] text-asc-nulls-last                      [2m(5.26ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-partition-by                      [2m(184.79ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] uspsert-partial-unique                   [2m(7.56ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] nested-parens-conditionals-and-double-or [2m(308.82ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-multi-predicate-references [2m(7.50ms)[0m
  [[32mPASS[0m] upsert-partial-doupdate-basic            [2m(7.03ms)[0m
  [[32mPASS[0m] partial-index-update-null-enters-conflict [2m(10.69ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] from-subquery-self-join                  [2m(6.05ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-unique-basic               [2m(16.94ms)[0m
  [[32mPASS[0m] upsert-partial-target-qualified-where-doupdate [2m(24.82ms)[0m
  [[32mPASS[0m] upsert-partial-doupdate-leave-predicate  [2m(14.40ms)[0m
  [[32mPASS[0m] partial-index-multiple                   [2m(9.93ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] basic-order-by-and-limit-2               [2m(972.26µs)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-update-multicol-enter-conflict [2m(12.17ms)[0m
  [[32mPASS[0m] partial-index-update-multicol-change-second [2m(10.78ms)[0m
  [[32mPASS[0m] upsert-partial-donothing-basic           [2m(5.57ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] from-subquery-with-limit                 [2m(8.42ms)[0m
  [[32mPASS[0m] from-subquery-nested                     [2m(5.15ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-unique-violation           [2m(11.55ms)[0m
  [[32mPASS[0m] upsert-partial-target-where-doupdate     [2m(7.16ms)[0m
  [[32mPASS[0m] partial-index-multiple-violation-priority [2m(13.78ms)[0m
  [[32mPASS[0m] partial-index-update-function-enters     [2m(11.11ms)[0m
  [[32mPASS[0m] partial-index-update-complex             [2m(10.73ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-order-by                          [2m(2.59ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-large-number            [2m(4.92ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-expression-where           [2m(7.71ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-backtick-columns        [2m(4.62ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-delete                     [2m(9.84ms)[0m
  [[32mPASS[0m] partial-index-update-rowid               [2m(8.23ms)[0m
  [[32mPASS[0m] partial-index-multiple-violation-status  [2m(14.67ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-false-4 [2m(494.29µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-case-insensitive        [2m(4.58ms)[0m
  [[32mPASS[0m] insert-returning-nonexistent-column      [2m(3.82ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-delete-function-where      [2m(8.92ms)[0m
  [[32mPASS[0m] partial-index-expensive-violation        [2m(11.60ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] update-returning-nonexistent-column      [2m(4.49ms)[0m

[1mtests/pragma/foreign_key_list.sqltest[0m
  [[32mPASS[0m] pragma-foreign-key-list-prefers-temp-shadowed-table [2m(12.37ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-wrong-table             [2m(5.36ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-delete-all                 [2m(11.48ms)[0m
  [[32mPASS[0m] partial-index-expensive-violation-update [2m(12.14ms)[0m

[1mtests/pragma/foreign_key_list.sqltest[0m
  [[32mPASS[0m] pragma-foreign-key-list-empty-results    [2m(5.54ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-order-by-duplicate-column         [2m(2.28ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-mixed-qualification     [2m(4.81ms)[0m
  [[32mPASS[0m] insert-returning-in-operator-false       [2m(5.00ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-not-in-null-value-not-found     [2m(4.92ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-like-operator           [2m(4.92ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-null-where                 [2m(7.86ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-like-operator-false     [2m(4.79ms)[0m
  [[32mPASS[0m] update-returning-set-clause-reference    [2m(5.98ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-exists-empty-result             [2m(6.37ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-is-null                 [2m(4.78ms)[0m
  [[32mPASS[0m] update-returning-order-dependent         [2m(5.82ms)[0m

[1mtests/partial_idx.sqltest[0m
  [[32mPASS[0m] partial-index-function-where             [2m(10.70ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-not-exists-all-match            [2m(6.74ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-is-not-null             [2m(4.78ms)[0m
  [[32mPASS[0m] update-returning-no-rows                 [2m(5.41ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-scalar-comparison               [2m(4.89ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-between                 [2m(4.77ms)[0m
  [[32mPASS[0m] insert-returning-concat-null             [2m(5.08ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-scalar-max                      [2m(4.88ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-between-false           [2m(4.76ms)[0m
  [[32mPASS[0m] insert-returning-concat-both-null        [2m(5.05ms)[0m
  [[32mPASS[0m] insert-returning-glob                    [2m(4.72ms)[0m
  [[32mPASS[0m] insert-returning-in-operator             [2m(4.94ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] basic-order-by-and-limit-3               [2m(894.02µs)[0m
  [[32mPASS[0m] basic-order-by-and-limit-2-2             [2m(926.78µs)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-in-single-column                [2m(13.12ms)[0m
  [[32mPASS[0m] from-subquery-outer-ref-in-correlated-subquery [2m(7.82ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] basic-order-by-and-limit-2-3             [2m(4.85ms)[0m
  [[32mPASS[0m] basic-order-by-and-limit-3-2             [2m(866.61µs)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-not-in-single-column            [2m(12.05ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] fk-replace-cascade-deletes-children      [2m(19.22ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-in-multiple-columns             [2m(16.63ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] unique-on-conflict-replace               [2m(6.75ms)[0m
  [[32mPASS[0m] unique-on-conflict-ignore                [2m(5.90ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-not-in-multiple-columns         [2m(13.04ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] table-pk-on-conflict-replace             [2m(6.11ms)[0m
  [[32mPASS[0m] table-unique-on-conflict-ignore          [2m(5.53ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-in-null-value-found             [2m(11.05ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] notnull-on-conflict-ignore               [2m(4.71ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] order-by-qualified-2                     [2m(122.85ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] notnull-on-conflict-replace-with-default [2m(4.40ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-in-null-value-not-found         [2m(11.13ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] gencol_insert_or_replace_not_null_no_default [2m(8.66ms)[0m
  [[32mPASS[0m] statement-or-overrides-constraint        [2m(4.73ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-in-null-lhs-nonempty            [2m(11.02ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] statement-or-ignore-overrides-abort      [2m(5.17ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-partition-by-duplicate-column     [2m(182.83ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] orderby_desc_with_offset                 [2m(169.13ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-in-null-lhs-empty               [2m(9.30ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] rowid-pk-on-conflict-replace             [2m(5.44ms)[0m
  [[32mPASS[0m] rowid-pk-on-conflict-ignore              [2m(5.01ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-unicode                 [2m(11.07ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-not-in-null-value-found         [2m(11.37ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-abs-negative            [2m(8.75ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] pk-fail-preserves-partial-changes        [2m(13.79ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-max-min-per-row         [2m(7.53ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] pk-rollback-rolls-back-transaction       [2m(5.42ms)[0m
  [[32mPASS[0m] uniq-fail-preserves-partial-changes      [2m(13.94ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] orderby_desc_regression_verify_order     [2m(897.62µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-replace                 [2m(5.10ms)[0m
  [[32mPASS[0m] insert-returning-lower-upper             [2m(7.51ms)[0m
  [[32mPASS[0m] insert-returning-round                   [2m(4.80ms)[0m
  [[32mPASS[0m] insert-returning-length-empty            [2m(4.91ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-column-in-order-by                [2m(1.12s)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-substr-negative         [2m(8.68ms)[0m
  [[32mPASS[0m] insert-returning-trim                    [2m(4.92ms)[0m

[1mtests/where/default.sqltest[0m
  [[32mPASS[0m] where-clause-no-table-constant-condition-false-5 [2m(454.00µs)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] orderby_desc_regression_verify_order     [2m(797.26µs)[0m
  [[32mPASS[0m] orderby_asc_with_filter_range            [2m(1.17ms)[0m
  [[32mPASS[0m] orderby_desc_with_filter_id_lt           [2m(803.05µs)[0m
  [[32mPASS[0m] orderby_desc_with_filter_id_le           [2m(796.14µs)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-char                    [2m(7.75ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] orderby_desc_regression                  [2m(2.97ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] fk-replace-setnull-nullifies-children    [2m(9.26ms)[0m
  [[32mPASS[0m] constraint-rollback-terminates-transaction [2m(5.19ms)[0m
  [[32mPASS[0m] constraint-rollback-terminates-transaction-unique [2m(5.17ms)[0m

[1mtests/orderby/memory.sqltest[0m
  [[32mPASS[0m] order_by_ambiguous_column                [2m(12.41ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] constraint-fail-in-transaction-keeps-partial [2m(13.30ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] order_by_column_deduplication            [2m(4.48ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] fk-replace-restrict-raises-error         [2m(7.38ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] case-insensitive-alias                   [2m(48.31ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-partition-by-multiple-columns     [2m(229.43ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] order-by-column-number-3                 [2m(184.00ms)[0m

[1mtests/orderby/memory.sqltest[0m
  [[32mPASS[0m] distinct_orderby_regression              [2m(11.30ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] fk-ignore-no-fk-relationship-created     [2m(7.18ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] orderby_desc_verify_rows                 [2m(394.82ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] update-unique-fail                       [2m(6.34ms)[0m

[1mtests/orderby/memory.sqltest[0m
  [[32mPASS[0m] orderby_float_literal                    [2m(2.85ms)[0m
  [[32mPASS[0m] orderby_alias_precedence                 [2m(10.94ms)[0m
  [[32mPASS[0m] orderby_float_literal_with_table         [2m(4.76ms)[0m
  [[32mPASS[0m] orderby_single_row_aggregate_optimization [2m(4.21ms)[0m
  [[32mPASS[0m] orderby_single_row_aggregate_with_data   [2m(4.64ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] update-pk-rollback                       [2m(14.35ms)[0m

[1mtests/orderby/memory.sqltest[0m
  [[32mPASS[0m] orderby_same_rows                        [2m(11.23ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] rollback-clears-deferred-fk-violations   [2m(40.46ms)[0m

[1mtests/orderby/memory.sqltest[0m
  [[32mPASS[0m] orderby_rowid_truncation                 [2m(4.66ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] order-by-column-number-4                 [2m(192.39ms)[0m

[1mtests/orderby/memory.sqltest[0m
  [[32mPASS[0m] orderby_alias_shadows_column             [2m(7.95ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] composite-unique-rollback-terminates-tx  [2m(5.22ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] basic-order-by                           [2m(2.05ms)[0m
  [[32mPASS[0m] basic-order-by-2                         [2m(1.13ms)[0m
  [[32mPASS[0m] order-by-qualified                       [2m(137.36ms)[0m
  [[32mPASS[0m] order_by_column_deduplication            [2m(992.83µs)[0m
  [[32mPASS[0m] basic-order-by-and-limit                 [2m(1.00ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] constraint-fail-keeps-partial-insert     [2m(13.33ms)[0m
  [[32mPASS[0m] composite-pk-fail-keeps-partial          [2m(14.18ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-uncorrelated-in-result-column-in [2m(7.49ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] order-by-agg-not-mentioned-in-select     [2m(411.37ms)[0m

[1mtests/pragma/foreign_key_list.sqltest[0m
  [[32mPASS[0m] pragma-foreign-key-list-attached-database [2m(8.63ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] update-fail-keeps-prior-updates          [2m(6.39ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-correlated-in-result-column     [2m(12.79ms)[0m

[1mtests/on_conflict_constraint_def.sqltest[0m
  [[32mPASS[0m] constraint-fail-keeps-partial-insert-unique [2m(15.45ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-correlated-distinct-aggregate-reset [2m(7.74ms)[0m
  [[32mPASS[0m] subquery-correlated-in-result-column-with-join [2m(9.03ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-expression-index-json-extract-updates-all-matching-rows-alias [2m(10.16ms)[0m
  [[32mPASS[0m] update_index_regression_test             [2m(9.64ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-uncorrelated-in-group-by        [2m(7.31ms)[0m
  [[32mPASS[0m] subquery-not-in-result-column-null-handling [2m(5.21ms)[0m
  [[32mPASS[0m] subquery-not-in-null-lhs-nonempty        [2m(11.31ms)[0m
  [[32mPASS[0m] subquery-not-in-null-lhs-empty           [2m(5.02ms)[0m
  [[32mPASS[0m] subquery-correlated-in-result-column-in  [2m(7.56ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update-expression-index-json-extract-updates-all-matching-rows [2m(10.16ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-time                           [2m(518.78µs)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-in-result-column-null-handling  [2m(5.04ms)[0m

[1mtests/savepoint.sqltest[0m
  [[32mPASS[0m] savepoint-release-root-with-deferred-fk-fails [2m(14.57ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-in-null-handling-multiple-rows  [2m(13.03ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-zeroblob                [2m(4.46ms)[0m
  [[32mPASS[0m] insert-returning-typeof-null             [2m(4.51ms)[0m

[1mtests/savepoint.sqltest[0m
  [[32mPASS[0m] savepoint-not-reusable-after-transaction-boundary [2m(12.87ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-printf                  [2m(4.50ms)[0m
  [[32mPASS[0m] insert-returning-changes                 [2m(4.43ms)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-uncorrelated-in-result-column   [2m(15.59ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-typeof                  [2m(22.88ms)[0m

[1mtests/savepoint.sqltest[0m
  [[32mPASS[0m] savepoint-rollback-to-removes-temp-ddl   [2m(10.53ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-date                    [2m(4.68ms)[0m
  [[32mPASS[0m] insert-returning-strftime                [2m(4.91ms)[0m

[1mtests/subquery/subquery_cte_equivalence_tests.sqltest[0m
  [[32mPASS[0m] banana-activity-streak-cte               [2m(42.35ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-julianday               [2m(7.67ms)[0m
  [[32mPASS[0m] insert-returning-json                    [2m(5.04ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-current-date                        [2m(551.39µs)[0m
  [[32mPASS[0m] date-specific-date                       [2m(466.78µs)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update_where_or_regression_test          [2m(9.01ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-null-input                          [2m(507.51µs)[0m

[1mtests/savepoint.sqltest[0m
  [[32mPASS[0m] savepoint-rollback-to-removes-temp-ddl-with-dml [2m(17.11ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update_rowid_alias_index_regression_test [2m(8.21ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-iso8601                             [2m(489.34µs)[0m
  [[32mPASS[0m] date-julian-day-integer                  [2m(489.90µs)[0m
  [[32mPASS[0m] date-julian-day-float                    [2m(724.98µs)[0m
  [[32mPASS[0m] date-with-milliseconds                   [2m(472.62µs)[0m
  [[32mPASS[0m] date-invalid-input                       [2m(457.77µs)[0m

[1mtests/subquery/memory.sqltest[0m
  [[32mPASS[0m] subquery-correlated-in-group-by          [2m(7.16ms)[0m

[1mtests/returning.sqltest[0m
  [[32mPASS[0m] insert-returning-last-insert-rowid       [2m(6.24ms)[0m

[1mtests/scalar-functions-datetime.sqltest[0m
  [[32mPASS[0m] date-with-timezone-negative              [2m(417.33µs)[0m
  [[32mPASS[0m] date-with-timezone-positive              [2m(411.72µs)[0m
  [[32mPASS[0m] date-with-timezone-day-change-positive   [2m(401.88µs)[0m
  [[32mPASS[0m] date-with-timezone-utc                   [2m(8.75ms)[0m
  [[32mPASS[0m] date-out-of-range                        [2m(585.52µs)[0m
  [[32mPASS[0m] date-time-only                           [2m(478.50µs)[0m

[1mtests/savepoint.sqltest[0m
  [[32mPASS[0m] savepoint-case-insensitive-name-resolution [2m(12.99ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] orderby_desc_with_filter                 [2m(211.22ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-named-in-cte                      [2m(1.66s)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] age_idx_order_desc                       [2m(167.07ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-function-in-order-by              [2m(1.27s)[0m

[1mtests/savepoint.sqltest[0m
  [[32mPASS[0m] savepoint-release-without-active-savepoint-errors [2m(11.00ms)[0m
  [[32mPASS[0m] savepoint-rollback-restores-uncommitted-schema [2m(6.32ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-in-cte                            [2m(1.70s)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] order-by-column-number                   [2m(197.63ms)[0m

[1mtests/savepoint.sqltest[0m
  [[32mPASS[0m] savepoint-release-outer-removes-inner    [2m(12.82ms)[0m
  [[32mPASS[0m] savepoint-rollback-to-top-level-then-release [2m(5.01ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] order-by-column-number-2                 [2m(205.02ms)[0m

[1mtests/savepoint.sqltest[0m
  [[32mPASS[0m] savepoint-cleared-after-rollback         [2m(10.99ms)[0m
  [[32mPASS[0m] savepoint-rollback-undoes-mixed-dml      [2m(7.42ms)[0m
  [[32mPASS[0m] savepoint-interleaved-duplicate-names    [2m(5.74ms)[0m
  [[32mPASS[0m] savepoint-rollback-to-without-active-savepoint-errors [2m(10.59ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] rowid_or_integer_pk_desc                 [2m(804.55µs)[0m

[1mtests/savepoint.sqltest[0m
  [[32mPASS[0m] savepoint-rollback-cache-spill-issue-6351 [2m(127.87ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] orderby_asc_with_filter_range            [2m(59.96ms)[0m

[1mtests/savepoint.sqltest[0m
  [[32mPASS[0m] savepoint-cleared-after-commit           [2m(10.62ms)[0m
  [[32mPASS[0m] savepoint-rollback-to-restores-deferred-fk-counter [2m(6.28ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] orderby_desc_with_filter_id_le           [2m(828.12µs)[0m

[1mtests/savepoint.sqltest[0m
  [[32mPASS[0m] savepoint-duplicate-name-shadowing       [2m(5.41ms)[0m
  [[32mPASS[0m] savepoint-rollback-to-temp-table-unreachable [2m(19.71ms)[0m
  [[32mPASS[0m] savepoint-basic-release                  [2m(4.76ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] orderby_desc_with_filter_id_lt           [2m(17.56ms)[0m

[1mtests/update.sqltest[0m
  [[32mPASS[0m] update_primary_key_constraint_error      [2m(5.95ms)[0m

[1mtests/savepoint.sqltest[0m
  [[32mPASS[0m] savepoint-release-inside-begin-does-not-commit [2m(4.85ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] orderby_desc_regression                  [2m(4.71ms)[0m

[1mtests/savepoint.sqltest[0m
  [[32mPASS[0m] savepoint-release-missing-errors         [2m(2.69ms)[0m
  [[32mPASS[0m] savepoint-nested-release-outer           [2m(4.85ms)[0m
  [[32mPASS[0m] savepoint-rollback-missing-errors        [2m(2.50ms)[0m
  [[32mPASS[0m] savepoint-rollback-reverts-temp-table-changes [2m(11.95ms)[0m
  [[32mPASS[0m] savepoint-rollback-to-can-be-repeated    [2m(5.60ms)[0m
  [[32mPASS[0m] savepoint-rollback-discards-post-savepoint-pages-without-losing-outer-row [2m(6.45ms)[0m
  [[32mPASS[0m] savepoint-rollback-overflow-repeated-issue-6352 [2m(169.84ms)[0m
  [[32mPASS[0m] savepoint-rollback-to-preserves-savepoint [2m(15.55ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] orderby_desc_with_filter                 [2m(134.03ms)[0m

[1mtests/save-all-cursors.sqltest[0m
  [[32mPASS[0m] save-all-cursors-window-triple-selfjoin-via-cte [2m(134.87ms)[0m

[1mtests/savepoint.sqltest[0m
  [[32mPASS[0m] savepoint-rollback-overflow-grow-issue-6352 [2m(97.56ms)[0m
  [[32mPASS[0m] savepoint-rollback-overflow-nested-issue-6352 [2m(202.18ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-in-cte                            [2m(1.50s)[0m

[1mtests/savepoint.sqltest[0m
  [[32mPASS[0m] savepoint-rollback-overflow-issue-6352   [2m(217.56ms)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] order-by-case-insensitive-aggregate      [2m(310.94ms)[0m
  [[32mPASS[0m] orderby_desc_with_offset                 [2m(162.49ms)[0m
  [[32mPASS[0m] orderby_asc_verify_rows                  [2m(365.98ms)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-named-in-cte                      [2m(1.68s)[0m

[1mtests/orderby/default.sqltest[0m
  [[32mPASS[0m] orderby_desc_verify_rows                 [2m(391.47ms)[0m

[1mtests/save-all-cursors.sqltest[0m
  [[32mPASS[0m] save-all-cursors-window-triple-selfjoin-multi-partition [2m(3.42s)[0m

[1mtests/join/default.sqltest[0m
  [[32mPASS[0m] natural-join-and-using-join              [2m(5.04s)[0m
  [[32mPASS[0m] natural-join-and-using-join              [2m(5.12s)[0m

[1mtests/save-all-cursors.sqltest[0m
  [[32mPASS[0m] save-all-cursors-window-triple-selfjoin-delete-scalar-subquery [2m(2.67s)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-multiple-functions                [2m(3.71s)[0m

[1mtests/save-all-cursors.sqltest[0m
  [[32mPASS[0m] save-all-cursors-window-triple-selfjoin-update-in-subquery [2m(2.11s)[0m

[1mtests/window/default.sqltest[0m
  [[32mPASS[0m] window-multiple-functions                [2m(3.73s)[0m
  [[32mPASS[0m] window-in-subquery                       [2m(4.16s)[0m
  [[32mPASS[0m] window-in-subquery                       [2m(4.39s)[0m

[1m[31mFailures:[39m[0m

[31m── rename-column-with-temp-trigger (tests/alter-rename-column-temp-trigger.sqltest) - :memory:[39m
   expected success but got error: Parse error: error in trigger trg after rename column: trigger table not found: t

[31m── rename-column-with-temp-trigger (tests/alter-rename-column-temp-trigger.sqltest) - :memory:[39m
   expected success but got error: Parse error: error in trigger trg after rename column: trigger table not found: t

[1mSummary:[0m
  [32m9532 passed[39m, [31m2 failed[39m, [33m7 skipped[39m
  [2mTotal time: 8.41s[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `testing/sqltests/tests/alter-rename-column-temp-trigger.sqltest`