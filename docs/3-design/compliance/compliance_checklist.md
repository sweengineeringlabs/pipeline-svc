# Architecture Compliance Checklist

**Audience**: Architects, contributors, reviewers.

Derived from [architecture.md](../architecture.md). Every rule here is enforceable —
re-run the listed command after any change and expect the stated result.

## 1. `core` is technology-free

| # | Rule | Verify |
|---|------|--------|
| 1 | `pipeline-svc-core` names no pipeline/workflow technology and has no external dependency | `grep -nE "^\s*(pub )?(struct\|enum\|fn) \w*(Temporal\|Airflow\|Kafka\|Postgres)" main/pipeline/core/src/*.rs` returns nothing; `main/pipeline/core/Cargo.toml`'s `[dependencies]` lists only `pipeline-pattern` |

## 2. `PipelineFactory` returns `Box<dyn Pipeline>` uniformly

| # | Rule | Verify |
|---|------|--------|
| 2 | `PipelineFactory::in_memory` returns `Box<dyn Pipeline>` | `grep -n "Box<dyn Pipeline>" main/pipeline/saf/src/*.rs` shows the constructor's return type |
| 3 | `saf`'s own `lib.rs` never re-exports a concrete backend type (`InMemoryPipeline`) | `grep -n "^pub use" main/pipeline/saf/src/lib.rs` shows only `PipelineFactory` |

## 3. Lint gates

| # | Rule | Verify |
|---|------|--------|
| 4 | `#![deny(unsafe_code)]` enforced across every crate | `cargo build --workspace` fails on any `unsafe` block |
| 5 | `#![warn(missing_docs)]` enforced across every crate | `cargo doc --workspace --no-deps` warns on any undocumented public item |
| 6 | `cargo clippy --workspace --all-targets -- -D warnings` clean | Run before every commit |
| 7 | `cargo fmt --check` clean across every crate | Run before every commit |

[← 3-design index](../README.md)
