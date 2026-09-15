# pipeline-svc-saf

`PipelineFactory`: construction facade for every backend this repo ships
(currently just `in_memory::<P>(stages)`, always available — no feature
gate). See [Architecture](../../../../docs/3-design/architecture.md) for
why this factory returns zero-cost `impl Pipeline<Payload = P>`, not
`Box<dyn Pipeline>`.
