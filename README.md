# pipeline-svc

> **TLDR:** The in-process `InMemoryPipeline`, on top of
> [`pipeline-pattern`](https://github.com/sweengineeringlabs/pipeline-pattern)'s
> contract. See [Architecture](docs/3-design/architecture.md) for the full design.

Companion implementation repo to
[`pipeline-pattern`](https://github.com/sweengineeringlabs/pipeline-pattern) — see
that repo's own ADR-001 for why this domain was designed contract-first,
and its amendment on the "no existing pilot" premise that turned out wrong.

## Quick Start

```rust
use std::sync::Arc;

use pipeline_pattern::{Payload, Pipeline, PipelineFuture, Stage};
use pipeline_svc_saf::PipelineFactory;

fn append_stage(suffix: &'static str) -> Stage<Payload> {
    Arc::new(move |mut payload: Payload| {
        PipelineFuture::new(async move {
            payload.body.extend_from_slice(suffix.as_bytes());
            Ok(payload)
        })
    })
}

async fn run_it() {
    let pipeline = PipelineFactory::in_memory(vec![append_stage("-a"), append_stage("-b")]);
    let output = pipeline.run(Payload::new(b"start".to_vec())).await.unwrap();
    assert_eq!(output.body, b"start-a-b".to_vec());
}
```

`Payload` is one ready-made choice here — `PipelineFactory::in_memory` is
generic, so a caller can thread any `Send + 'static` type through instead.

## Crates

| Crate | What it is |
|-------|------------|
| [`pipeline-svc-core`](scm/main/pipeline/core) | The technology-free reference implementation: `InMemoryPipeline<P>` (fixed `Vec<Stage<P>>`, no lock, no persistence) |
| [`pipeline-svc-saf`](scm/main/pipeline/saf) | `PipelineFactory` — construction facade consumers depend on |

No `spi` crate yet — no real consumer has needed a distributed or
external-workflow-engine backend. See [Architecture](docs/3-design/architecture.md).

## Documentation

| Document | Description |
|----------|--------------|
| [Docs index](docs/README.md) | Full documentation index |
| [Architecture](docs/3-design/architecture.md) | Component diagram, why `PipelineFactory` returns zero-cost `impl Pipeline<Payload = P>`, not `Box<dyn Pipeline>` |
| [ADR-001](docs/3-design/adr/ADR-001-in-memory-reference-implementation.md) | Why `InMemoryPipeline` uses no lock at all |
| [Developer Guide](docs/4-development/developer_guide.md) | Repo layout, working on this crate |

## License

MIT OR Apache-2.0
