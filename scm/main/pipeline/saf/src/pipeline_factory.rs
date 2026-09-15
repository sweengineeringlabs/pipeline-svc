//! [`PipelineFactory`] — public pipeline construction surface.

use pipeline_pattern::{Pipeline, Stage};

/// Zero-size factory type for constructing pipeline instances.
pub struct PipelineFactory;

impl PipelineFactory {
    /// Construct the in-process reference pipeline, running `stages` in
    /// order.
    ///
    /// Unlike `SchedulerFactory::in_memory()`/`TransactionalStoreFactory::
    /// in_memory()` (both parameterless), this constructor takes `stages`
    /// directly -- a pipeline is meaningless without them; there's no
    /// sensible "empty default" the way an empty in-memory KV store or
    /// scheduler is. See
    /// [`pipeline_svc_core::InMemoryPipeline`]'s own doc comment.
    pub fn in_memory(stages: Vec<Stage>) -> Box<dyn Pipeline> {
        Box::new(pipeline_svc_core::InMemoryPipeline::new(stages))
    }
}
