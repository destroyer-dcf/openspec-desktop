---
type: "query"
date: "2026-05-31T08:59:55.117733+00:00"
question: "Why does commands connect so many communities?"
contributor: "graphify"
source_nodes: ["commands", "permissions", "allow", "deny", "acl-manifests.json"]
---

# Q: Why does commands connect so many communities?

## Answer

Expanded from original query via vocab tokens: [commands, permissions, identifier, bootstrap, project, state, core, acl]. The graph shows that many 'commands' nodes come from generated ACL schema entries in src-tauri/gen/schemas/acl-manifests.json. These entries are repeated per permission/capability, so 'commands' acts as a structural container token that appears across many permission subgraphs (allow/deny variants), which inflates cross-community bridging in clustering.

## Source Nodes

- commands
- permissions
- allow
- deny
- acl-manifests.json