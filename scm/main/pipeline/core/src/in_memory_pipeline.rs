//! [`InMemoryPipeline`] — runs a fixed `Vec<Stage>` in order, no external
//! pipeline/workflow technology.

use pipeline_pattern::{Payload, Pipeline, PipelineError, PipelineFuture, Stage};

/// In-process [`Pipeline`] backed by a fixed, ordered `Vec<Stage>`.
///
/// Each stage's output becomes the next stage's input; the first failing
/// stage stops the pipeline immediately, reporting its exact index. No
/// persistence, no distributed coordination, no retry — that's the
/// reference-impl trade-off; a distributed/external-engine backend is a
/// `spi` crate this repo doesn't have yet (no real consumer has needed
/// one).
pub struct InMemoryPipeline {
    stages: Vec<Stage>,
}

impl InMemoryPipeline {
    /// Construct a pipeline that runs `stages` in order.
    #[must_use]
    pub fn new(stages: Vec<Stage>) -> Self {
        Self { stages }
    }
}

impl Pipeline for InMemoryPipeline {
    fn run(&self, input: Payload) -> PipelineFuture<'_, Result<Payload, PipelineError>> {
        PipelineFuture::new(async move {
            if self.stages.is_empty() {
                return Err(PipelineError::NoStages);
            }
            let mut payload = input;
            for (index, stage) in self.stages.iter().enumerate() {
                payload = stage(payload)
                    .await
                    .map_err(|message| PipelineError::StageFailed { index, message })?;
            }
            Ok(payload)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_pipeline_is_send_and_sync() {
        fn _assert_send_sync<T: Send + Sync>() {}
        _assert_send_sync::<InMemoryPipeline>();
        assert!(
            std::hint::black_box(true),
            "InMemoryPipeline is Send + Sync (checked above at compile time)"
        );
    }
}
