---
name: herdr-feature-roadmap
description: The fork's 13-feature UX roadmap for herdr - feature order, per-feature specs, classification (TUI / server-runtime / plugin), and status tracking in .local/prd/STATUS.md. Use when picking the next feature, planning one, or updating roadmap status.
---

# herdr-plus feature roadmap

Thirteen user-experience features for the personal fork, each sourced from a real request in
upstream herdr GitHub Discussions (read-only research; never post there). Specs live in
`references/`. They were written against upstream commit `d11c0c34` (2026-09-24). Upstream moves
fast: verify every path and symbol before planning, and trust the code over the spec.

## Order

The order goes from small, isolated TUI changes to features that touch the server API, and ends
with the two that need a wire/protocol decision. Do not reorder without asking; later features
reuse patterns from earlier ones.

| NN | Slug | Class | Size | Source | Notes |
| --- | --- | --- | --- | --- | --- |
| 01 | `confirm-close-tab` | TUI | S | discussion #4533 | reuses the workspace confirm dialog |
| 02 | `tab-bar-padding` | TUI | S | #4398 | client layout geometry |
| 03 | `sidebar-spacing` | TUI | S | #4530, #4534 | Spaces header padding + worktree child gap |
| 04 | `indexed-colors` | config / TUI | S | #4514 | `ansi:N` / 256-color indexes in theme colors |
| 05 | `space-path-token` | TUI (+ maybe snapshot) | M | #4584 | `path` token for Space rows |
| 06 | `update-check-intervals` | server config | S | #4485 | replaces the hard-coded 30 min interval |
| 07 | `config-includes` | config loading | M | #4265 | `include = [...]` layering |
| 08 | `pane-balance` | server API + CLI + key | M | #4527 | new `pane.balance` method |
| 09 | `idle-agent-reminder` | plugin | M | #4431 | notify when an agent sits idle too long |
| 10 | `workstream-shortcuts` | plugin | M | #4526 | start/finish a worktree workstream |
| 11 | `pinned-panes` | plugin | S | #4522 | pin marker via pane metadata tokens |
| 12 | `tab-bar-right-styles` | TUI + snapshot field | M | #4305 | per-entry fg/bg/bold + rules; wire decision |
| 13 | `input-sync` | server runtime + TUI | L | #4423 / #949 | broadcast typed input to a tab's panes; input path |

Features 12 and 13 require a stop-and-ask before implementation (see their specs).

## Workflow per feature

1. **Orient.** Read `references/NN-<slug>.md`. Verify each path and symbol with `rg`. Re-check the
   upstream discussion only if the spec is ambiguous (read-only `gh api graphql`).
2. **Plan.** Write `.local/prd/NN-<slug>.md` using the template below. Show it to the user and wait.
3. **Branch.** `git switch herdr-plus && git switch -c feat/NN-<slug>`.
4. **Implement, test, smoke-test, document, commit, merge** per `herdr-fork-dev`
   (and `herdr-plugin-authoring` for plugin features).
5. **Track.** Update `.local/prd/STATUS.md` at each state change.

### Plan template (`.local/prd/NN-<slug>.md`)

```markdown
# NN slug

- Upstream base: <herdr-plus commit>
- Classification: TUI presentation | server/runtime fact | plugin (why)
- Spec drift: <paths/symbols that moved or changed since the spec>
- Files to change:
- Config keys (name, type, default, reload behavior):
- API methods / fields (name, optional?, advertised how):
- CLI:
- Tests to add (name -> behavior proven):
- Perf: which pane-scaled loop is touched, if any, and how it will be measured
- Docs pages:
- Stop-and-ask items:
```

## Status tracking

`.local/prd/STATUS.md` (ignored by git) holds one row per feature:

```markdown
| NN | slug | state | branch | merged commit | notes |
```

States: `todo` → `planned` (plan written) → `approved` → `in-progress` → `review` (report delivered)
→ `merged` | `deferred` | `dropped`. Record the reason for `deferred`/`dropped`.

## Rules every spec inherits

- Off-by-default or behavior-preserving defaults. Existing configs must render and behave the same.
- New config keys follow the checklist in `herdr-fork-dev` (model, default config, reference JSON,
  reload).
- Client presentation settings belong in the client's local config; server/runtime facts belong in
  server state and the JSON API (AGENTS.md runtime/client boundary).
- No edits to frozen endpoint fixtures; new endpoint capabilities get new method names or
  optional fields.
- Stop and ask before bumping `PROTOCOL_VERSION`, adding a dependency, changing persisted state
  formats, or touching agent detection manifests.
