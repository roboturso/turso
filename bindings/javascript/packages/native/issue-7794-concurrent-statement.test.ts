import { expect, test } from 'vitest'
import { connect } from './promise.js'

// Regression test for https://github.com/tursodatabase/turso/issues/7794
//
// Executing the same reused prepared statement concurrently (without awaiting
// each call to completion) must return each caller the row for its own bound
// parameters, not the row for the last-bound parameters.
test('issue-7794: concurrent reuse of a prepared statement returns each caller its own row', async () => {
    const db = await connect(":memory:");
    await db.exec("CREATE TABLE t (id INTEGER PRIMARY KEY, name TEXT)");
    for (let i = 1; i <= 10; i++) {
        await db.exec(`INSERT INTO t VALUES (${i}, 'name${i}')`);
    }

    const stmt = await db.prepare("SELECT id, name FROM t WHERE id = ?");
    const ids = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Sanity check: sequential execution is correct.
    const sequential: any[] = [];
    for (const id of ids) {
        sequential.push((await stmt.get(id))?.id);
    }
    expect(sequential).toEqual(ids);

    // Concurrent execution of the same statement object via get().
    const concurrentGet = (await Promise.all(ids.map((id) => stmt.get(id)))).map((r) => r?.id);
    expect(concurrentGet).toEqual(ids);

    // Same via all().
    const concurrentAll = (await Promise.all(ids.map((id) => stmt.all(id)))).map((rows) => rows[0]?.id);
    expect(concurrentAll).toEqual(ids);
})
