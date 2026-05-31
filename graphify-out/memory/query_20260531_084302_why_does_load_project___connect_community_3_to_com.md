---
type: "query"
date: "2026-05-31T08:43:02.698204+00:00"
question: "Why does load_project() connect Community 3 to Community 1 and Community 12?"
contributor: "graphify"
source_nodes: ["load_project()", "open_project()", "bootstrap_state()", "run()", "Dashboard.svelte"]
---

# Q: Why does load_project() connect Community 3 to Community 1 and Community 12?

## Answer

Expanded from original query via vocab tokens: [load, project, loader, bootstrap, state, commands, config, change, active, archived, init, open]. The graph shows load_project() as a bridge from loader logic (Community 3) into command/state orchestration (Community 1) through open_project(), and into app startup (Community 12) through bootstrap_state() and run(). Key paths: load_project() <-calls- open_project(); load_project() <-calls- bootstrap_state() <-calls- run().

## Source Nodes

- load_project()
- open_project()
- bootstrap_state()
- run()
- Dashboard.svelte