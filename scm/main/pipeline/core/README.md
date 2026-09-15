# pipeline-svc-core

`InMemoryPipeline`: the technology-free reference implementation of
`pipeline-pattern`'s `Pipeline` trait — a fixed, ordered `Vec<Stage>` set
once at construction, run with no lock, no persistence, no distributed
coordination.

See [Architecture](../../../../docs/3-design/architecture.md) for the full
explanation.
