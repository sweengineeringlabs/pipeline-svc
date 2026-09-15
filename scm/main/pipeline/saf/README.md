# pipeline-svc-saf

`PipelineFactory`: construction facade for every backend this repo ships
(currently just `in_memory(stages)`, always available — no feature gate).
See [Architecture](../../../../docs/3-design/architecture.md) for why this
factory returns `Box<dyn Pipeline>`.
