//! [`PipelineFactory`] — public pipeline construction surface.

use pipeline_pattern::{Pipeline, Stage};

/// Zero-size factory type for constructing pipeline instances.
pub struct PipelineFactory;

impl PipelineFactory {
    /// Construct the in-process reference pipeline, running `stages` in
    /// order.
    ///
    /// Generic over the payload type `P`, matching `Pipeline::Payload`'s
    /// associated-type shape -- zero-cost, no forced serialization. Returns
    /// `impl Pipeline<Payload = P>`, not `Box<dyn Pipeline>`: `Pipeline` is
    /// not object-safe once `Payload` is an associated type, and this repo
    /// has only ever had one backend with no real caller needing to select
    /// between implementations at runtime, so nothing is lost by that. See
    /// `docs/3-design/architecture.md`.
    ///
    /// Unlike `SchedulerFactory::in_memory()`/`TransactionalStoreFactory::
    /// in_memory()` (both parameterless), this constructor takes `stages`
    /// directly -- a pipeline is meaningless without them; there's no
    /// sensible "empty default" the way an empty in-memory KV store or
    /// scheduler is. See
    /// [`pipeline_svc_core::InMemoryPipeline`]'s own doc comment.
    pub fn in_memory<P: Send + 'static>(stages: Vec<Stage<P>>) -> impl Pipeline<Payload = P> {
        pipeline_svc_core::InMemoryPipeline::new(stages)
    }
}
