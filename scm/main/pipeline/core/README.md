# pipeline-svc-core

`InMemoryPipeline<P>`: the technology-free reference implementation of
`pipeline-pattern`'s `Pipeline` trait, generic over the payload type `P` —
a fixed, ordered `Vec<Stage<P>>` set once at construction, run with no
lock, no persistence, no distributed coordination.

See [Architecture](../../../../docs/3-design/architecture.md) for the full
explanation.
