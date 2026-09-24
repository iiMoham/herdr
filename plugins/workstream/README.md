# Workstream plugin (`herdr-plus.workstream`)

Two shortcuts for worktree-based tasks:

- **Start**: asks for a branch name, creates a Git worktree for it, opens it as a workspace grouped
  under the repository, and applies your layout recipe (commands in the first pane and in splits).
- **Finish**: shows the branch's checkout, its uncommitted changes, and any nested repositories with
  unsaved or unpushed work, then removes the checkout and closes its workspace. **The branch is never
  deleted.**

Finish asks before removing anything. A checkout with uncommitted changes needs you to type `force`,
and nested repositories with work that would be lost need you to type `discard`; anything else
cancels. It refuses to run on a repository's main checkout.

## Requirements

- herdr 0.9.1 or newer from this fork (finish uses `worktree removal-check` and `--discard-nested`).
- `python3` (3.9 or newer, standard library only) and `git` on the server's `PATH`.
- Linux or macOS.

## Install

```bash
herdr plugin link /path/to/herdr/plugins/workstream
herdr plugin list          # confirm it is enabled and has no warnings
```

Bind the actions in `~/.config/herdr/config.toml`. These keys are unbound by default:

```toml
[[keys.command]]
key = "prefix+shift+b"
type = "plugin_action"
command = "herdr-plus.workstream.start"
description = "begin workstream"

[[keys.command]]
key = "prefix+shift+f"
type = "plugin_action"
command = "herdr-plus.workstream.finish"
description = "finish workstream"
```

Then reload config (menu → reload config, or `herdr server reload-config`). Both actions are also
available from `herdr plugin action invoke herdr-plus.workstream.start` (or `.finish`).

## Layout recipe (optional)

Create `config.json` in the folder printed by `herdr plugin config-dir herdr-plus.workstream`:

```json
{
  "branch_prefix": "feat/",
  "base": "origin/main",
  "root_command": "claude",
  "splits": [
    { "direction": "right", "command": "npm run dev" },
    { "direction": "down", "command": "" }
  ]
}
```

| Key | Meaning | Default |
| --- | --- | --- |
| `branch_prefix` | Added to the name you type unless it already starts with it | none |
| `base` | Ref new branches start from | `HEAD` of the repository |
| `root_command` | Runs in the new workspace's first pane | none |
| `splits` | Extra panes split from the first one; `direction` is `right` or `down`, `command` is optional | none |

An existing branch is checked out instead of created. Invalid config is reported in the popup.

## Limits

- One checkout per workstream; multi-repository tasks are out of scope.
- Windows is not supported yet (no guaranteed `python3`).
- The popup needs an attached Herdr client, like every plugin popup.

## Development

```bash
just fork-plugins-test     # unit tests for every fork plugin
```
