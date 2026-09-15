//! Integration tests for [`pipeline_svc_saf::PipelineFactory`].
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use pipeline_pattern::{Payload, Pipeline, PipelineError, PipelineFuture, Stage};
use pipeline_svc_saf::PipelineFactory;

fn append_stage(suffix: &'static str) -> Stage<Payload> {
    Arc::new(move |mut payload: Payload| {
        PipelineFuture::new(async move {
            payload.body.extend_from_slice(suffix.as_bytes());
            Ok(payload)
        })
    })
}

/// @covers: PipelineFactory::in_memory -- constructed with real stages,
/// the returned `impl Pipeline` actually runs them.
#[tokio::test]
async fn test_in_memory_with_stages_runs_them_through_the_returned_pipeline() {
    let pipeline = PipelineFactory::in_memory(vec![append_stage("-a"), append_stage("-b")]);
    let output = pipeline
        .run(Payload::new(b"start".to_vec()))
        .await
        .expect("pipeline must succeed");
    assert_eq!(output.body, b"start-a-b".to_vec());
}

/// @covers: PipelineFactory::in_memory -- constructed with zero stages,
/// the returned pipeline still returns NoStages, not a silently unchanged
/// payload.
#[tokio::test]
async fn test_in_memory_with_no_stages_returns_no_stages_error() {
    let pipeline = PipelineFactory::in_memory(Vec::<Stage<Payload>>::new());
    let result = pipeline.run(Payload::new(b"start".to_vec())).await;
    assert_eq!(result, Err(PipelineError::NoStages));
}
