//! Integration tests for [`pipeline_svc_core::InMemoryPipeline`].
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use pipeline_pattern::{Payload, Pipeline, PipelineError, PipelineFuture, Stage};
use pipeline_svc_core::InMemoryPipeline;

fn append_stage(suffix: &'static str) -> Stage {
    Arc::new(move |mut payload: Payload| {
        PipelineFuture::new(async move {
            payload.body.extend_from_slice(suffix.as_bytes());
            Ok(payload)
        })
    })
}

fn failing_stage() -> Stage {
    Arc::new(|_payload: Payload| PipelineFuture::new(async { Err("boom".to_string()) }))
}

fn add_header_stage(key: &'static str, value: &'static str) -> Stage {
    Arc::new(move |mut payload: Payload| {
        PipelineFuture::new(async move {
            payload.headers.insert(key.to_string(), value.to_string());
            Ok(payload)
        })
    })
}

/// @covers: InMemoryPipeline::run -- zero stages returns NoStages, not a
/// silently unchanged payload.
#[tokio::test]
async fn test_run_with_no_stages_returns_no_stages_error() {
    let pipeline = InMemoryPipeline::new(vec![]);
    let result = pipeline.run(Payload::new(b"input".to_vec())).await;
    assert_eq!(result, Err(PipelineError::NoStages));
}

/// @covers: InMemoryPipeline::run -- a single stage's transformation is
/// applied to the input.
#[tokio::test]
async fn test_run_with_one_stage_applies_it() {
    let pipeline = InMemoryPipeline::new(vec![append_stage("-a")]);
    let output = pipeline
        .run(Payload::new(b"start".to_vec()))
        .await
        .expect("pipeline must succeed");
    assert_eq!(output.body, b"start-a".to_vec());
}

/// @covers: InMemoryPipeline::run -- three chained stages compose in
/// order: each stage's output becomes the next stage's input.
#[tokio::test]
async fn test_run_with_three_stages_chains_output_to_input_in_order() {
    let pipeline = InMemoryPipeline::new(vec![
        append_stage("-a"),
        append_stage("-b"),
        append_stage("-c"),
    ]);
    let output = pipeline
        .run(Payload::new(b"start".to_vec()))
        .await
        .expect("pipeline must succeed");
    assert_eq!(output.body, b"start-a-b-c".to_vec());
}

/// @covers: InMemoryPipeline::run -- a failing stage at index 1 of 3 stops
/// the pipeline immediately, reporting the exact index, and the third
/// stage provably never runs (proven via an AtomicBool, not inferred from
/// the return value alone).
#[tokio::test]
async fn test_run_stops_at_failing_stage_and_third_stage_never_runs() {
    let stage_three_ran = Arc::new(AtomicBool::new(false));
    let marker = Arc::clone(&stage_three_ran);
    let marking_stage: Stage = Arc::new(move |payload: Payload| {
        let marker = Arc::clone(&marker);
        PipelineFuture::new(async move {
            marker.store(true, Ordering::SeqCst);
            Ok(payload)
        })
    });

    let pipeline = InMemoryPipeline::new(vec![append_stage("-a"), failing_stage(), marking_stage]);
    let result = pipeline.run(Payload::new(b"start".to_vec())).await;

    assert_eq!(
        result,
        Err(PipelineError::StageFailed {
            index: 1,
            message: "boom".to_string(),
        })
    );
    assert!(
        !stage_three_ran.load(Ordering::SeqCst),
        "stage 3 must never run once stage 2 fails"
    );
}

/// @covers: InMemoryPipeline::run -- headers survive unchanged through a
/// stage that doesn't touch them, and are visibly changed by one that
/// does.
#[tokio::test]
async fn test_run_headers_survive_and_can_be_modified_by_stages() {
    let pipeline = InMemoryPipeline::new(vec![
        append_stage("-a"),
        add_header_stage("content-type", "text/plain"),
    ]);
    let mut input = Payload::new(b"start".to_vec());
    input
        .headers
        .insert("correlation-id".to_string(), "abc-123".to_string());

    let output = pipeline.run(input).await.expect("pipeline must succeed");

    assert_eq!(
        output.headers.get("correlation-id").map(String::as_str),
        Some("abc-123"),
        "header untouched by any stage must survive unchanged"
    );
    assert_eq!(
        output.headers.get("content-type").map(String::as_str),
        Some("text/plain"),
        "header set by a later stage must be visible in the final output"
    );
}
