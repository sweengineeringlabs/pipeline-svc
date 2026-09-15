# Glossary

Alphabetized list of terms used in `pipeline-svc`.

---

**InMemoryPipeline&lt;P&gt;** - `pipeline-svc-core`'s technology-free reference `Pipeline`, generic over its payload type `P`: a fixed, ordered `Vec<Stage<P>>` set once at construction, run in order with no lock (read-only access). No persistence, no distributed coordination.

**PipelineFactory** - Construction facade in `pipeline-svc-saf`: `in_memory::<P>(stages)`, a generic function returning `impl Pipeline<Payload = P>` (zero-cost — `Pipeline` is not object-safe once `Payload` is an associated type, unlike `Scheduler`/`MessageBroker`, and for a different reason than `executor-svc-saf`'s `ExecutorFactory`). Takes `stages` directly, unlike `SchedulerFactory`'s/`TransactionalStoreFactory`'s parameterless `in_memory()`.

[← Docs index](README.md)
