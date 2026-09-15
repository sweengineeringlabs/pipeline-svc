//! [`InMemoryPipeline`] — runs a fixed `Vec<Stage<P>>` in order, no
//! external pipeline/workflow technology.

use pipeline_pattern::{Pipeline, PipelineError, Stage};

/// In-process [`Pipeline`] backed by a fixed, ordered `Vec<Stage<P>>`.
///
/// Generic over the payload type `P` — matches `pipeline-pattern`'s own
/// `Pipeline::Payload` associated type, so this backend forces no
/// serialization on a caller using any `Send + 'static` type of their own
/// choosing, not just `pipeline_pattern::Payload`.
///
/// Each stage's output becomes the next stage's input; the first failing
/// stage stops the pipeline immediately, reporting its exact index. No
/// persistence, no distributed coordination, no retry — that's the
/// reference-impl trade-off; a distributed/external-engine backend is a
/// `spi` crate this repo doesn't have yet (no real consumer has needed
/// one).
pub struct InMemoryPipeline<P> {
    stages: Vec<Stage<P>>,
}

impl<P> InMemoryPipeline<P> {
    /// Construct a pipeline that runs `stages` in order.
    #[must_use]
    pub fn new(stages: Vec<Stage<P>>) -> Self {
        Self { stages }
    }
}

impl<P: Send + 'static> Pipeline for InMemoryPipeline<P> {
    type Payload = P;

    async fn run(&self, input: P) -> Result<P, PipelineError> {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_pipeline_is_send_and_sync() {
        fn _assert_send_sync<T: Send + Sync>() {}
        _assert_send_sync::<InMemoryPipeline<pipeline_pattern::Payload>>();
        assert!(
            std::hint::black_box(true),
            "InMemoryPipeline is Send + Sync (checked above at compile time)"
        );
    }
}
