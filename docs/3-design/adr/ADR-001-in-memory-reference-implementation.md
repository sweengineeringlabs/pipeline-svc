# ADR-001: `InMemoryPipeline`'s implementation shape

**Status**: Accepted
**Date**: 2026-09-15

## Context

`pipeline-pattern` was believed to have no existing pilot when this crate
was first built (see that repo's own ADR-001 and its amendment — that
belief turned out wrong, `edge-pipeline` already existed, though its own
design didn't change what this crate's implementation choices below
needed to be). The implementation question is simpler than
`scheduler-svc`'s (no timing, concurrency, or cancellation semantics to get
right) — `InMemoryPipeline` is fundamentally: hold an ordered
`Vec<Stage<P>>`, run each in turn, feed one stage's output into the next
stage's input, stop at the first failure.

## Decision

- **A fixed `Vec<Stage<P>>`, set once at construction** (`InMemoryPipeline::new(stages)`),
  not a mutable pipeline that stages can be added to or removed from after
  construction. Simpler, and matches how a pipeline is actually used: a
  caller assembles its stages once, then runs many inputs through the same
  fixed sequence. Adding stage mutation later would be additive if a real
  consumer needs it.
- **No lock of any kind.** `run(&self, ...)` only ever reads `self.stages`
  -- there is no mutable state to protect, unlike
  `oltp-svc-core::InMemoryTransactionalStore`'s `RwLock<HashMap<..>>` or
  `scheduler-svc-core::InMemoryScheduler`'s `Mutex`-protected cancellation
  flags. This is the simplest of this org's reference implementations for
  exactly this reason.
- **`PipelineFactory::in_memory::<P>(stages: Vec<Stage<P>>)`**, unlike
  `SchedulerFactory::in_memory()`/`TransactionalStoreFactory::in_memory()`'s
  parameterless constructors -- see `docs/3-design/architecture.md`'s own
  section on why a pipeline has no sensible "empty but usable" starting
  state the way a scheduler or a store does. Generic over `P`, matching
  `Pipeline::Payload`'s associated-type shape (`pipeline-pattern` v0.2.0) --
  see that crate's own architecture.md for why.

## Consequences

- `pipeline-svc-core` depends on `pipeline-pattern` only -- no `futures`
  crate dependency, since `PipelineFuture` (used directly by `Stage<P>`,
  not re-wrapped) is `std`-only.
- `InMemoryPipeline<P>` is `Send + Sync` with no interior mutability at
  all, so it can be shared across threads (e.g. behind an `Arc`) with zero
  synchronization overhead per call -- verified by a real compile-time
  assertion test (`test_in_memory_pipeline_is_send_and_sync`).
- A future distributed/external-engine backend (tracked, not forgotten --
  see the open backlog issue on this repo) will need real
  synchronization/network-error handling this reference implementation
  doesn't have; that's the expected shape of a reference implementation,
  not a defect in this one.
