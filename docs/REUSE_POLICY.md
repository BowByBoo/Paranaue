# Forge Reuse Before Reinvent Policy

Forge is built as a long-lived product. We do not reinvent mature infrastructure merely to increase the amount of code we own.

Before implementing a substantial subsystem, the engineering loop must ask:

1. Does a mature implementation or library already solve this problem?
2. Is it actively maintained enough for the Forge lifetime we expect?
3. Is its license compatible with Forge's intended distribution?
4. Does its API fit our architecture without forcing unrelated design decisions?
5. What are its security, portability, performance, dependency, and binary-size costs?
6. Can we replace or remove it later without rewriting the product?
7. Does adopting it provide more reliability than implementing the subsystem ourselves?

Preferred outcomes, in order of engineering value rather than ideology:

- Reuse a mature dependency when it is clearly the safest and simplest choice.
- Adapt or wrap a mature implementation when its concepts are useful but its public API does not fit Forge.
- Study existing implementations and build a small Forge-specific component when the domain is small or the existing solutions impose unnecessary complexity.
- Implement a subsystem from scratch only when there is a defensible technical reason.

## Source and license discipline

Code is never copied merely because it compiles.

For third-party source code, record its origin and applicable license before incorporating it. Preserve required copyright and license notices. When only concepts, algorithms, or public APIs are studied, implement independently rather than copying protected expression.

Dependencies should be pinned through Cargo's lockfile for reproducible application builds and reviewed before upgrades.

## Research record

When an important subsystem decision is influenced by existing projects, the roadmap should record:

- projects/crates reviewed;
- why they were considered;
- license status;
- maintenance/health observations;
- important trade-offs;
- final Forge decision.

This policy exists to save engineering effort without sacrificing ownership, security, or architectural coherence.
