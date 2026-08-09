//! Regression test for issue #8291.
//!
//! Running the whopper in btree-rebalance mode with seed 16095637479200742728
//! panics in `core/storage/wal.rs`:
//!
//!   end_write_tx called while write lock not held according to connection state
//!
//! The panic fires during a statement's best-effort reset, which wraps the
//! reset in `catch_unwind` (core/statement.rs). In debug builds the panic is
//! therefore swallowed and the process exits cleanly; only release builds
//! (panic = "abort") die. To detect the bug regardless of build profile we
//! install a panic hook and assert it never fired.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use rand::Rng;
use turso_whopper::{
    StepResult, Whopper, WhopperOpts,
    chaotic_btree::BtreeRebalanceProfile,
    chaotic_elle::ChaoticWorkloadProfile,
    properties::{IntegrityCheckProperty, Property},
    workloads::{IntegrityCheckWorkload, WalCheckpointWorkload, Workload},
};

#[test]
fn btree_rebalance_seed_16095637479200742728_does_not_panic() -> anyhow::Result<()> {
    let panicked = Arc::new(AtomicBool::new(false));
    let panicked_in_hook = panicked.clone();
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        panicked_in_hook.store(true, Ordering::SeqCst);
        previous_hook(info);
    }));

    // Same setup the CLI uses for `--mode btree-rebalance
    // --allocation-fault-probability 0` (see main.rs).
    let workloads: Vec<(u32, Box<dyn Workload>)> = vec![
        (20, Box::new(IntegrityCheckWorkload)),
        (
            5,
            Box::new(WalCheckpointWorkload {
                allow_passive: false,
            }),
        ),
    ];
    let properties: Vec<Box<dyn Property>> = vec![Box::new(IntegrityCheckProperty)];
    let chaotic_profiles: Vec<(f64, &'static str, Box<dyn ChaoticWorkloadProfile>)> = vec![(
        1.0,
        "btree-rebalance",
        Box::new(BtreeRebalanceProfile::default()),
    )];

    let opts = WhopperOpts::btree_rebalance()
        .with_seed(16095637479200742728)
        .with_max_connections(4)
        .with_workloads(workloads)
        .with_properties(properties)
        .with_chaotic_profiles(chaotic_profiles)
        .with_allocation_fault_probability(0.0);

    let mut whopper = Whopper::new(opts)?;

    // Mirror the CLI main loop exactly. The reopen dice roll consumes RNG
    // state each step, so it must stay in place (with probability 0.0) for
    // the seed to replay the reported run.
    while !whopper.is_done() {
        if whopper.rng.random_bool(0.0) {
            whopper.reopen()?;
        }
        match whopper.step()? {
            StepResult::Ok => {}
            StepResult::WalSizeLimitExceeded => break,
        }
    }
    whopper.finalize_properties()?;
    // The panic fires while connections shut down, so drop before checking.
    drop(whopper);

    assert!(
        !panicked.load(Ordering::SeqCst),
        "a panic fired during the run (see output above): issue #8291"
    );
    Ok(())
}
