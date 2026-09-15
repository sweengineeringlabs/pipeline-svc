//! `pipeline_svc_saf` — pipeline construction facade.
//!
//! The `Pipeline` contract (trait + value types + error) lives in
//! `pipeline-pattern`; this crate owns the construction factory that
//! selects among real implementations — currently just
//! `pipeline-svc-core`'s `InMemoryPipeline`.
//!
//! `Pipeline::Payload` is an associated type (as of `pipeline-pattern`
//! v0.2.0, a zero-cost abstraction fix — see that crate's own
//! architecture.md), so `Pipeline` is not object-safe: `PipelineFactory::
//! in_memory` returns `impl Pipeline<Payload = P>`, a generic function, not
//! `Box<dyn Pipeline>`. This is a different shape from
//! `message-broker-svc-saf`'s `MessageBrokerFactory`/`scheduler-svc-saf`'s
//! `SchedulerFactory` (both still object-safe, still return one uniform
//! boxed/impl type) for a different reason than `executor-svc-saf`'s
//! `ExecutorFactory` (`Executor::run<F: Future>`'s own generic method) —
//! worth stating explicitly so this isn't read as an inconsistency to
//! "fix" toward matching either sibling.

mod pipeline_factory;

pub use pipeline_factory::PipelineFactory;
