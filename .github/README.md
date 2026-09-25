# MoMo

**A terminal runtime for coding agents, based on [herdr](https://github.com/herdrdev/herdr).**

MoMo keeps everything herdr does (workspaces, tabs, and panes that survive disconnects; agent
detection for Claude Code, Codex, and friends; SSH machines; plugins) and adds features aimed at
running many agents at once: an animated herd that shows what every agent is doing, typing into all
panes of a tab at once, a live output stream for scripts, safer worktree cleanup, and notifications
that take you to the right pane.

> MoMo is an unofficial fork. It is not affiliated with or endorsed by the herdr project.
> It installs next to herdr and never changes a herdr install.

## Contents

[Install](#install) · [Terminal setup](#terminal-setup) · [What MoMo adds](#what-momo-adds) ·
[Keyboard shortcuts](#keyboard-shortcuts) · [Command line](#command-line) ·
[Shell completions](#shell-completions) · [Everything else works like herdr](#everything-else-works-like-herdr)

## Install

MoMo runs on **macOS** (Apple Silicon and Intel) and **Linux** (x86_64 and arm64). On **Windows**,
run it inside WSL (see below). The same one-line command works in every terminal app (Terminal,
iTerm2, Ghostty, WezTerm, kitty, Alacritty, Warp, GNOME Terminal, Konsole, Windows Terminal, …) and
every shell (zsh, bash, fish, …), because it hands the script to `sh`:

```bash
curl -fsSL https://github.com/iiMoham/momo/releases/latest/download/install.sh | sh
```

The installer picks the binary for your machine, checks its SHA-256 checksum, and installs `momo` to
`~/.local/bin`. Then start it:

```bash
momo
```

On first start MoMo copies your herdr config (`~/.config/herdr/config.toml`), if you have one, to
`~/.config/momo/config.toml`. After that the two configs are separate.

### macOS

Open any terminal app and run the command above. `curl` ships with macOS. If `momo` is not found
afterwards, add `~/.local/bin` to your `PATH` (see [Add momo to your PATH](#add-momo-to-your-path)).

### Linux

The installer needs `curl` (it already uses `sh`, `awk`, and `sha256sum`, which every distribution
has). Install `curl` if it is missing, then run the command above:

| Distribution | Install curl |
| --- | --- |
| Ubuntu, Debian, Mint, Pop!_OS | `sudo apt install -y curl` |
| Fedora, RHEL, CentOS Stream | `sudo dnf install -y curl` |
| Arch, Manjaro | `sudo pacman -S --needed curl` |
| openSUSE | `sudo zypper install -y curl` |
| Alpine | `sudo apk add curl` |

Linux binaries are fully static, so they run on any distribution and C library (glibc or musl).

### Windows (WSL)

MoMo has no native Windows build. Run it in WSL 2, which gives you a real Linux shell inside Windows
Terminal:

```powershell
wsl --install          # in PowerShell as Administrator, once; then restart
```

Open **Ubuntu** from the Start menu (or a WSL tab in Windows Terminal) and run the Linux command
above. WSL support is not tested by us yet; please report problems.

### Add momo to your PATH

The installer tells you when `~/.local/bin` is not on your `PATH`. Add it for your shell, then open a
new terminal:

| Shell | Command |
| --- | --- |
| zsh (macOS default) | `echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc` |
| bash on Linux / WSL | `echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc` |
| bash on macOS | `echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bash_profile` |
| fish | `fish_add_path ~/.local/bin` |

### Install options

```bash
# Install somewhere else (for example, system-wide):
curl -fsSL https://github.com/iiMoham/momo/releases/latest/download/install.sh | sudo env MOMO_INSTALL_DIR=/usr/local/bin sh

# Install a specific release:
curl -fsSL https://github.com/iiMoham/momo/releases/latest/download/install.sh | MOMO_VERSION=0.9.1-momo.2 sh
```

| | |
| --- | --- |
| Update | `momo update` (from MoMo's releases, never from herdr) |
| Version | `momo --version`, e.g. `momo 0.9.1-momo.2` (herdr 0.9.1 plus MoMo release 2) |
| Uninstall | `rm ~/.local/bin/momo`, and `rm -r ~/.config/momo ~/.local/state/momo` to remove config and sessions |

## Terminal setup

MoMo works in any modern terminal, with the mouse and without extra setup. Three optional tweaks:

- **Background color.** The default theme looks best on a `#0b1020` background.
- **Alt/Option key on macOS.** Some shortcuts use Alt (for example Alt+B / Alt+F to move by word in
  MoMo's text fields, or your own `alt+…` bindings). macOS terminals type special characters with
  Option instead, until you change this:

  | Terminal | Setting |
  | --- | --- |
  | Terminal.app | Settings → Profiles → Keyboard → **Use Option as Meta key** |
  | iTerm2 | Settings → Profiles → Keys → General → **Left Option key: Esc+** |
  | Ghostty | `macos-option-as-alt = true` in the Ghostty config |
  | kitty | `macos_option_as_alt left` in `kitty.conf` |
  | Alacritty | `option_as_alt = "OnlyLeft"` under `[window]` in `alacritty.toml` |
  | WezTerm | works by default with the left Option key |

  Linux and Windows terminals send Alt as-is.
- **Right-click menus.** Tabs, panes, and sidebar rows have right-click menus. In iTerm2, turn on
  Settings → Profiles → Terminal → **Enable mouse reporting**; if right-click still opens iTerm's own
  menu, remove the right-click entry in Settings → Pointer → Bindings.

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

## Keyboard shortcuts

MoMo is mouse-first: you can click panes, tabs, workspaces, and agents, drag split borders, and use
right-click menus without any shortcut. The keyboard is optional.

Shortcuts start with the **prefix**, `ctrl+b` by default. `prefix+c` means: press `ctrl+b`, let go,
then press `c`. Press `prefix+?` inside MoMo to see every active shortcut (type `/` to filter).
The keys are the same in every terminal and on every OS.

### Start with these

| Action | Keys |
| --- | --- |
| New tab | `prefix+c` |
| Split side by side / stacked | `prefix+v` / `prefix+minus` |
| Move between panes | `prefix+h` `j` `k` `l` (left, down, up, right) |
| Workspace navigation | `prefix+w` |
| Detach and leave everything running | `prefix+q` |

### Panes

| Action | Keys |
| --- | --- |
| Focus pane left / down / up / right | `prefix+h` / `prefix+j` / `prefix+k` / `prefix+l` |
| Swap pane left / down / up / right | `prefix+shift+h` / `prefix+shift+j` / `prefix+shift+k` / `prefix+shift+l` |
| Next / previous pane | `prefix+tab` / `prefix+shift+tab` |
| Split side by side | `prefix+v` |
| Split stacked | `prefix+minus` |
| Zoom (full size) on / off | `prefix+z` |
| Close pane | `prefix+x` |
| Rename pane | `prefix+shift+p` |
| Resize mode | `prefix+r` |
| Copy mode | `prefix+[` |
| Open scrollback in `$EDITOR` | `prefix+e` |

### Tabs

| Action | Keys |
| --- | --- |
| New tab | `prefix+c` |
| Next / previous tab | `prefix+n` / `prefix+p` |
| Go to tab 1–9 | `prefix+1` … `prefix+9` |
| Rename tab | `prefix+shift+t` |
| Close tab | `prefix+shift+x` |

### Workspaces, agents, and the app

| Action | Keys |
| --- | --- |
| Workspace navigation | `prefix+w` |
| Goto picker (jump to any agent or terminal) | `prefix+g` |
| New workspace | `prefix+shift+n` |
| New Git worktree workspace | `prefix+shift+g` |
| Rename workspace | `prefix+shift+w` |
| Close workspace | `prefix+shift+d` |
| Jump to the notification's pane | `prefix+o` |
| Show / hide the sidebar | `prefix+b` |
| Settings | `prefix+s` |
| All shortcuts (help) | `prefix+?` |
| Reload config | `prefix+shift+r` |
| Detach | `prefix+q` |
| Paste a clipboard image into a remote session | `ctrl+v` |

### Inside a mode

| Mode | Keys |
| --- | --- |
| Workspace navigation (`prefix+w`) | `↑`/`↓` choose workspace · `1`–`9` jump to workspace · `h` `j` `k` `l` or `←`/`→` move focus · `tab`/`shift+tab` cycle panes · `enter` open · `esc` back |
| Resize (`prefix+r`) | `h`/`l` width · `j`/`k` height · `esc` done |
| Copy (`prefix+[`) | `h` `j` `k` `l`, `w`/`b`/`e`, `{`/`}` move · `/` or `?` search, `n`/`N` next/previous · `v` or `space` select · `y` or `enter` copy · `q` or `esc` exit |
| Goto picker (`prefix+g`) | `↑`/`↓` or `j`/`k` move · `/` search · `b`/`w`/`i`/`d` show blocked/working/idle/done agents · `a` all · `enter` jump |

In MoMo's text fields (names, filters, search): `ctrl+a`/`ctrl+e` start/end, `alt+b`/`alt+f` by word,
`ctrl+u`/`ctrl+k` cut to start/end, `ctrl+w` cut previous word, `ctrl+y` paste the cut text.

### Not bound by default

These actions exist but have no key until you give them one:
`toggle_input_sync` (type into every pane of the tab), `last_pane`, `previous_workspace`,
`next_workspace`, `switch_workspace` (1–9), `previous_agent`, `next_agent`, `focus_agent` (1–9),
`move_tab_previous`, `move_tab_next`, `open_worktree`, `remove_worktree`, `clear_pane`, and
`resize_pane_left` / `_down` / `_up` / `_right`.

### Change shortcuts

Edit `~/.config/momo/config.toml`, then run `momo server reload-config` (or press `prefix+shift+r`):

```toml
[keys]
prefix = "ctrl+a"                               # tmux-style prefix
toggle_input_sync = "prefix+i"                  # bind an unbound action
next_tab = ["prefix+n", "ctrl+alt+]"]           # several keys for one action
switch_workspace = "prefix+shift+1..9"          # numbered shortcuts
focus_agent = "prefix+alt+1..9"
```

`ctrl+alt+…` chords are the safest prefix-free choice: terminals, shells, and desktops rarely use
them. Avoid `ctrl+alt+arrows` (GNOME, Ghostty, Konsole), `ctrl+alt+t` (opens a terminal on Ubuntu
and Fedora), `ctrl+alt+l` (KDE lock screen), and `ctrl+alt+f1`…`f12` (Linux consoles). If a shortcut
does nothing, your terminal or desktop took it first. `momo config reset-keys` restores the defaults
(it backs up your config first).

## Command line

Every command works from any shell, including from inside a MoMo pane, where it targets the current
session. Add `--session <name>` to target another session, or `--machine <name>` for a saved SSH
machine. `momo <group> --help` shows the options.

| Command | What it does |
| --- | --- |
| `momo` | Start MoMo or attach to the running session |
| `momo --session <name>` | Use or create a named session |
| `momo --remote <ssh-target>` | Attach to MoMo on another machine over SSH |
| `momo status` | Show client and server status |
| `momo update` | Update to the latest MoMo release |
| `momo --version` / `momo --help` | Version / help |
| `momo --skill` | Print the guide that tells coding agents how to drive MoMo |
| `momo session list \| attach \| stop \| delete` | Manage named sessions |
| `momo server stop \| reload-config` | Stop the server / reload `config.toml` |
| `momo config check \| reset-keys` | Validate the config / restore default shortcuts |
| `momo workspace list \| create \| get \| focus \| rename \| close` | Workspaces |
| `momo tab list \| create \| get \| focus \| rename \| close` | Tabs |
| `momo tab sync on \| off \| toggle` | Type into every pane of a tab |
| `momo pane list \| get \| split \| focus \| zoom \| resize \| swap \| move \| rename \| close` | Panes |
| `momo pane read \| send-text \| send-keys \| run` | Read a pane or type into it |
| `momo pane wait-output \| watch` | Wait for text / stream every change as JSON |
| `momo agent list \| get \| read \| prompt \| send-keys \| wait \| focus \| start \| rename` | Work with coding agents |
| `momo worktree list \| create \| open \| remove \| removal-check` | Git worktrees as workspaces |
| `momo machine list \| add \| status \| reconnect \| rename \| remove` | Saved SSH machines |
| `momo integration install \| uninstall \| status <agent>` | Agent hooks (claude, codex, …) |
| `momo plugin install \| link \| list \| enable \| disable \| action \| log` | Plugins |
| `momo notification show` | Show a notification |
| `momo completion <shell>` | Shell completions (below) |

## Shell completions

Tab completion for every `momo` command and option. Run the line for your shell once, then open a new
terminal:

| Shell | Setup |
| --- | --- |
| zsh | `mkdir -p ~/.zfunc && momo completion zsh > ~/.zfunc/_momo`, then add `fpath=(~/.zfunc $fpath); autoload -Uz compinit && compinit` to `~/.zshrc` |
| bash | `mkdir -p ~/.local/share/bash-completion/completions && momo completion bash > ~/.local/share/bash-completion/completions/momo` (needs the `bash-completion` package) |
| fish | `momo completion fish > ~/.config/fish/completions/momo.fish` |
| PowerShell (`pwsh`) | `momo completion powershell >> $PROFILE` |
| Elvish | `momo completion elvish >> ~/.config/elvish/rc.elv` |

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
