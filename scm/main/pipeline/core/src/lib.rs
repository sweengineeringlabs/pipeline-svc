//! `pipeline_svc_core` — the technology-free reference implementation of
//! `pipeline-pattern`'s `Pipeline` trait.
//!
//! [`InMemoryPipeline`] runs a fixed, ordered `Vec<Stage<P>>` in-process —
//! no external pipeline/workflow technology, no persistence, no
//! distributed coordination. Generic over the payload type `P`, matching
//! `Pipeline::Payload`'s associated-type shape (zero-cost, no forced
//! serialization) — see `docs/3-design/architecture.md`.

mod in_memory_pipeline;

pub use in_memory_pipeline::InMemoryPipeline;
