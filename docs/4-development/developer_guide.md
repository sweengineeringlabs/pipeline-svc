# pipeline-svc Developer Guide

**Audience**: Developers, contributors.

## Repo Structure

```
pipeline-svc/
├── README.md
├── docs/
│   ├── README.md
│   ├── glossary.md
│   ├── 0-ideation/papers/README.md
│   ├── 3-design/README.md, architecture.md
│   ├── 3-design/compliance/compliance_checklist.md
│   ├── 3-design/adr/README.md, ADR-001-in-memory-reference-implementation.md
│   └── 4-development/README.md, developer_guide.md   # this file
└── scm/
    ├── Cargo.toml          # workspace: [core, saf] -- no spi/ yet
    └── main/pipeline/
        ├── core/              # pipeline-svc-core -- InMemoryPipeline (technology-free)
        └── saf/               # pipeline-svc-saf -- PipelineFactory
```

## Branching and Releases

- `dev` is the default branch; all work lands there first.
- `main` gets fast-forwarded to `dev` after a shipped change, not on every commit.
- Not yet published to crates.io. First publish will be whatever version is
  current at that time (past v0.1.0 already for both crates — see each
  crate's own `Cargo.toml`).
- Depends on [`pipeline-pattern`](https://github.com/sweengineeringlabs/pipeline-pattern)
  by `git`+`tag` (`tag = "v0.2.0"`), this org's standing convention for a
  cross-repo dependency whose target hasn't published to crates.io yet.
  Switch to a version requirement once `pipeline-pattern` publishes.

## Working on Any Crate

Both crates are members of `scm/Cargo.toml`, so from `scm/`:

```
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
```

`PipelineFactory::in_memory::<P>(stages)` is always available, no feature
required — only one backend exists. It's a generic function (returns
`impl Pipeline<Payload = P>`, zero-cost, not `Box<dyn Pipeline>` — see
`docs/3-design/architecture.md`'s own section on why) and, unlike
`SchedulerFactory::in_memory()`/`TransactionalStoreFactory::in_memory()`,
takes `stages` directly rather than being parameterless.

## See Also

- [Architecture](../3-design/architecture.md)
- [ADR-001](../3-design/adr/ADR-001-in-memory-reference-implementation.md)
- [Pattern/Svc Workflow](https://github.com/sweengineeringlabs/template-engine/blob/main/pattern_svc_workflow.md)
