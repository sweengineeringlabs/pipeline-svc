# Architecture Decision Records

**Audience**: Architects, technical leads, contributors.

| ADR | Status | Date | Decision |
|-----|--------|------|----------|
| [ADR-001](ADR-001-in-memory-reference-implementation.md) | Accepted | 2026-09-15 | `InMemoryPipeline` holds a fixed `Vec<Stage>` set once at construction, no lock (read-only access), no dependency on `executor-pattern`/`scheduler-pattern` for running stages |

[← 3-design index](../README.md)
