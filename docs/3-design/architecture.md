# pipeline-svc Architecture

**Audience**: Architects, technical leads, contributors.

## Overview

Two crates — one `core` reference implementation, one `saf` facade. No
`spi` crate yet:

- **`pipeline-svc-core`** — the technology-free reference implementation:
  `InMemoryPipeline<P>` (runs a fixed, ordered `Vec<Stage<P>>`, generic
  over the payload type, no persistence, no distributed coordination). See
  [ADR-001](adr/ADR-001-in-memory-reference-implementation.md) for the
  implementation-shape reasoning.
- **`pipeline-svc-saf`** — `PipelineFactory` (`in_memory::<P>(stages)`,
  always available — no feature gate, since there's only one backend). A
  consumer depends on `pipeline-pattern` + `pipeline-svc-saf` alone.

## `Pipeline` is not object-safe — `PipelineFactory` returns `impl Pipeline<Payload = P>`

As of `pipeline-pattern` v0.2.0, `Pipeline::Payload` is an associated type
(a zero-cost abstraction fix — see
[pipeline-pattern#5](https://github.com/sweengineeringlabs/pipeline-pattern/issues/5)
and that crate's own architecture.md), so `Pipeline` is no longer
object-safe: there is no `Box<dyn Pipeline>`. `PipelineFactory::in_memory`
is now a generic function, `pub fn in_memory<P: Send + 'static>(stages:
Vec<Stage<P>>) -> impl Pipeline<Payload = P>` (resolved in
[pipeline-svc#3](https://github.com/sweengineeringlabs/pipeline-svc/issues/3)),
not `Box<dyn Pipeline>`.

This is a different shape from `message-broker-svc-saf`'s
`MessageBrokerFactory`/`scheduler-svc-saf`'s `SchedulerFactory` (both still
object-safe, both still return one uniform boxed/`impl` type) for a
different reason than `executor-svc-saf`'s `ExecutorFactory`
(`Executor::run<F: Future>`'s own generic method, not an associated type on
the trait). Worth stating explicitly so none of these three repos gets
"fixed" toward matching either of the others — each object-safety
consequence has its own real, distinct cause.

## Component Diagram

```mermaid
flowchart TD
    subgraph pattern["pipeline-pattern"]
        contract["Pipeline (generic Payload), Stage&lt;P&gt;, Payload, PipelineError"]
    end

    subgraph svc["pipeline-svc"]
        core["pipeline-svc-core<br/>InMemoryPipeline&lt;P&gt;"]
        saf["pipeline-svc-saf<br/>PipelineFactory::in_memory::&lt;P&gt;"]

        core -->|implements| contract
        saf -->|wires| core
    end
```

## Why `PipelineFactory::in_memory` takes `stages`, unlike `SchedulerFactory`/`TransactionalStoreFactory`'s parameterless `in_memory()`

A scheduler or a key-value store both have a sensible empty starting
state (no jobs scheduled yet, no records written yet) — state accumulates
through later calls (`schedule`, `put`). A pipeline has no equivalent
"empty but usable" state: it exists entirely to run a specific,
pre-configured sequence of stages, so those stages are a construction-time
requirement, not something added afterward one call at a time.

## Why no `spi` crate yet

No real consumer has needed a distributed or external-workflow-engine
pipeline backend (Temporal, Airflow, Kafka Streams). Per this org's own
`<domain>-<technology>-spi` convention, an `spi` crate wraps exactly one
external technology — building one speculatively, with no real technology
chosen and no real consumer validating the choice, would be the same
premature-generalization mistake `pipeline-pattern`'s own ADR-001 declined
to make for branching/DAG support. Add one when a real need does.

## Scope boundary

This repo implements exactly `pipeline-pattern`'s `Pipeline` trait, one
backend. Not covered:

- **Distributed/external-engine backends** — no `spi` crate yet, see
  above.
- **Branching/conditional pipelines (DAG), a short-circuit-without-
  erroring signal, per-stage retries** — all out of scope at the contract
  level too; see `pipeline-pattern`'s own architecture.md Scope boundary
  section. This repo cannot implement what the contract doesn't define.

[← Docs index](../README.md)
