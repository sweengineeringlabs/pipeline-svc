//! `pipeline_svc_saf` — pipeline construction facade.
//!
//! The `Pipeline` contract (trait + value types + error) lives in
//! `pipeline-pattern`; this crate owns the construction factory that
//! selects among real implementations — currently just
//! `pipeline-svc-core`'s `InMemoryPipeline`.
//!
//! `Pipeline::run` has no generic parameters, so `Pipeline` is fully
//! object-safe: `PipelineFactory` returns `Box<dyn Pipeline>` uniformly,
//! matching `message-broker-svc-saf`'s `MessageBrokerFactory`/
//! `scheduler-svc-saf`'s `SchedulerFactory` shape — unlike
//! `executor-svc-saf`'s `ExecutorFactory`, which must return `impl
//! Executor` because `Executor::run<F: Future>` is generic.

mod pipeline_factory;

pub use pipeline_factory::PipelineFactory;
