//! `pipeline_svc_core` — the technology-free reference implementation of
//! `pipeline-pattern`'s `Pipeline` trait.
//!
//! [`InMemoryPipeline`] runs a fixed, ordered `Vec<Stage>` in-process — no
//! external pipeline/workflow technology, no persistence, no distributed
//! coordination.

mod in_memory_pipeline;

pub use in_memory_pipeline::InMemoryPipeline;
