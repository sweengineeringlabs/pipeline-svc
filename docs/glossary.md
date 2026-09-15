# Glossary

Alphabetized list of terms used in `pipeline-svc`.

---

**InMemoryPipeline** - `pipeline-svc-core`'s technology-free reference `Pipeline`: a fixed, ordered `Vec<Stage>` set once at construction, run in order with no lock (read-only access). No persistence, no distributed coordination.

**PipelineFactory** - Construction facade in `pipeline-svc-saf`: `in_memory(stages)`, returning `Box<dyn Pipeline>` (`Pipeline` is object-safe, like `Scheduler`, unlike `executor-svc-saf`'s `ExecutorFactory`). Takes `stages` directly, unlike `SchedulerFactory`'s/`TransactionalStoreFactory`'s parameterless `in_memory()`.

[← Docs index](README.md)
