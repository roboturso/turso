//! Regression test for https://github.com/tursodatabase/turso/issues/8195
//!
//! A WAL page lookup with a watermark below the checkpoint backfill floor is a
//! normal reader state: the reader pinned its snapshot before a passive
//! checkpoint copied those frames into the main database file. The lookup must
//! answer "not in the WAL" (or a clean error), never abort the process.
//! Before the fix, `find_frame` hit
//! `turso_assert!(frame_watermark >= nbackfills)` and panicked.

use turso_core::CheckpointMode;

use crate::common::TempDatabase;

#[turso_macros::test()]
fn wal_watermark_below_backfill_floor_does_not_panic(db: TempDatabase) {
    let conn = db.connect_limbo();
    conn.execute("create table t(x integer primary key, y)")
        .unwrap();
    for i in 0..50 {
        conn.execute(format!("insert into t values ({i}, {i})").as_str())
            .unwrap();
    }
    let max_frame = conn.wal_state().unwrap().max_frame;
    assert!(
        max_frame > 30,
        "expected more than 30 WAL frames, got {max_frame}"
    );

    // A passive checkpoint moves the backfill floor up to the WAL tip.
    conn.checkpoint(CheckpointMode::Passive {
        upper_bound_inclusive: None,
    })
    .unwrap();

    // Probe page 1 with a watermark below the floor, like a reader whose
    // snapshot predates the checkpoint. The lookup must complete: found,
    // not found, or a clean error are all acceptable; a panic is not.
    let mut page = vec![0u8; 4096];
    let _ = conn.try_wal_watermark_read_page(1, &mut page, Some(30));
}
