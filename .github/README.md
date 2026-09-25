# MoMo

**A terminal runtime for coding agents, based on [herdr](https://github.com/herdrdev/herdr).**

MoMo keeps everything herdr does (workspaces, tabs, and panes that survive disconnects; agent
detection for Claude Code, Codex, and friends; SSH machines; plugins) and adds features aimed at
running many agents at once: an animated herd that shows what every agent is doing, typing into all
panes of a tab at once, a live output stream for scripts, safer worktree cleanup, and notifications
that take you to the right pane.

> MoMo is an unofficial fork. It is not affiliated with or endorsed by the herdr project.
> It installs next to herdr and never changes a herdr install.

## Install

macOS (Apple Silicon or Intel) and Linux (x86_64 or arm64):

```bash
curl -fsSL https://github.com/iiMoham/momo/releases/latest/download/install.sh | sh
```

The installer downloads the binary for your machine, checks its SHA-256 checksum, and puts `momo`
in `~/.local/bin` (set `MOMO_INSTALL_DIR` to change that, or `MOMO_VERSION=0.9.1-momo.1` to pick a
release). Then run:

```bash
momo
```

On first start MoMo copies your herdr config (`~/.config/herdr/config.toml`) to
`~/.config/momo/config.toml`, so your keys and settings carry over. After that the two configs are
separate.

| | |
| --- | --- |
| Update | `momo update` (from MoMo's releases, never from herdr) |
| Version | `momo --version`, e.g. `momo 0.9.1-momo.1` (herdr 0.9.1 plus MoMo release 1) |
| Uninstall | `rm ~/.local/bin/momo`, and `rm -r ~/.config/momo ~/.local/state/momo` for config and sessions |
| Windows | not supported; use MoMo on a Mac or Linux machine, or over SSH |

## What MoMo adds

### A herd you can read at a glance

The sidebar opens with an animated pixel herd: one sheep per agent, colored by what it is doing.
Sheep walk while agents work (faster when more work), a magenta sheep with a blinking `!` needs you,
and everyone naps when all is quiet. The line underneath counts it for you:
`MoMo  ● 2 working  ▲ 1 needs you`.

The default **midnight-neon** theme uses the same language everywhere: cyan for focus and working
agents, magenta for "needs you", green for done. For the full effect set your terminal's background
to `#0b1020`.

```toml
[ui]
animation = true      # false turns the herd off
animation_fps = 24    # 1-30

[theme]
name = "midnight-neon"   # any herdr theme still works; "neon-day" is the light sibling
```

### Type into every pane of a tab

Input sync sends your typing and pastes to all panes in the current tab, like tmux's
`synchronize-panes`. A synced tab shows an `S` after its name, in the warning color, so a live
broadcast is hard to miss.

```bash
momo tab sync on        # or off / toggle; add a tab id to target another tab
```

Or bind a key (`[keys] toggle_input_sync = "prefix+i"`) or right-click the tab.

### Stream pane output to scripts

`pane watch` prints one JSON line per screen change (at most every 100 ms), so scripts and bots can
follow an agent without polling:

```bash
momo pane watch w1:p2 --text --lines 40
```

The socket API has the same stream as the `pane.output_changed` event.

### Worktree removal that can't lose nested work

Before removing a Git worktree, MoMo looks inside it for nested repositories (ignored sub-repos,
submodules, other worktrees) with uncommitted or unpushed work that Git itself cannot see, and
refuses to delete them unless you say so.

```bash
momo worktree removal-check --workspace w3      # show what would be lost
momo worktree remove --workspace w3             # refuses if nested work would be lost
momo worktree remove --workspace w3 --discard-nested
```

### Workstream shortcuts (plugin)

`plugins/workstream` starts a task in one key press (branch → worktree → workspace → your pane
layout) and finishes it safely (reviews changes and nested work, removes the checkout, keeps the
branch). See [its README](../plugins/workstream/README.md).

### Notifications that open the right pane

On macOS with [`terminal-notifier`](https://github.com/julienXX/terminal-notifier)
(`brew install terminal-notifier`) and `[ui.toast] delivery = "system"`, clicking an agent's
notification brings your terminal forward and focuses the agent's pane.

### Smaller touches

- `[ui] confirm_close_tab = true` asks before closing a tab from a key or the tab menu.
- `[ui] tab_bar_padding = 1` (or `2`) leaves blank rows between the tab bar and the panes.

## Everything else works like herdr

MoMo is herdr underneath, so [herdr's documentation](https://herdr.dev/docs/) applies. Read `herdr`
as `momo` and `~/.config/herdr` as `~/.config/momo`. A few things stay shared on purpose:

- **Agent hooks.** `momo integration install claude` (or codex, …) installs the same hooks as herdr;
  they work with both apps.
- **Environment variables** keep their `HERDR_` names (`HERDR_PANE_ID`, `HERDR_SOCKET_PATH`, …), so
  scripts and plugins written for herdr work in MoMo.
- **SSH machines.** `momo --remote host` uses `momo` on the remote if it's there, otherwise an
  existing herdr, and installs MoMo into `~/.local/bin/momo` if neither is usable.
- **Worktrees** default to `~/.herdr/worktrees`, so herdr and MoMo see the same checkouts.
- **Agent detection** rules still update from herdr.dev.

MoMo has no preview channel; `momo channel set preview` is refused.

## Roadmap

Planned next, roughly in order:

- Agent-first sidebar (agents on top, "needs you" first)
- Bottom status bar and a "needs you" spotlight
- Cleaner pane borders with the agent's state inline
- Shared "done" acknowledgements across clients, idle-agent reminders, quick session switching,
  config includes, pane balancing, and a scripting API for the running window

## Build from source

Needs Rust 1.96+ and Zig 0.16:

```bash
git clone https://github.com/iiMoham/momo && cd momo
cargo build --release
./target/release/momo --version
```

Maintainers: see [`packaging/momo/README.md`](../packaging/momo/README.md) for cutting a release.

## Credits and license

MoMo is built on [herdr](https://github.com/herdrdev/herdr) by its authors and contributors, under
the [Apache License 2.0](../LICENSE). MoMo's changes are under the same license; see
[`NOTICE`](../NOTICE).
