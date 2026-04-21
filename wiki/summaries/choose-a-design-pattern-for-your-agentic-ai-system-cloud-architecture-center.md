# Choose a design pattern for your agentic AI system

- source: `raw/Choose a design pattern for your agentic AI system    Cloud Architecture Center.md`
- source link: https://docs.cloud.google.com/architecture/choose-design-pattern-agentic-ai-system
- original title: Choose a design pattern for your agentic AI system
- author: Google Cloud
- published: 2026-04-22
- type: Cloud Architecture Center article

## Summary

This article provides a structured approach to selecting an agentic AI system design pattern. It starts by defining key requirements—task complexity, latency, cost, and human involvement—then reviews common patterns (single‑agent, multi‑agent sequential, parallel, loop, review‑critique, iterative refinement, coordinator, hierarchical, swarm, human‑in‑the‑loop, custom logic). Finally, it offers a comparison matrix to help decide which pattern best fits a given workload.

## Key Claims

1. **Requirements‑driven selection** – The choice of pattern should be guided by workload characteristics rather than a one‑size‑fits‑all mindset.
2. **Pattern taxonomy** – Patterns can be grouped into single‑agent, multi‑agent orchestration, and custom‑logic families, each with distinct trade‑offs in control, latency, and cost.
3. **Trade‑off matrix** – The article presents a concise table mapping patterns to control, latency, cost, and suitability for deterministic vs. dynamic workflows.
4. **Iterative refinement** – Even within a chosen pattern, iterative loops (e.g., review‑critique, loop, or ReAct) can be added to improve quality.
5. **Human‑in‑the‑loop** – For high‑stakes or safety‑critical tasks, a human checkpoint should be integrated into the workflow.

## Related Concepts

- [agent-design-patterns](../concepts/agent-design-patterns.md)
- [harness-engineering](../concepts/harness-engineering.md)

## Alignment With Current Wiki

The summary aligns with the existing `agent-design-patterns` concept page, extending it with a practical decision matrix and a clearer mapping of patterns to workload characteristics. It also reinforces the importance of human oversight and iterative loops, which are covered in the `harness-engineering` concept.
