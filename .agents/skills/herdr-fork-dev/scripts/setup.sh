#!/usr/bin/env bash
# One-time macOS setup for the personal herdr fork. Idempotent: safe to re-run.
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

if ! command -v brew >/dev/null 2>&1; then
  echo "error: Homebrew is required (https://brew.sh)" >&2
  exit 1
fi

# rustup, zig (libghostty-vt needs exactly 0.16.0), just, nextest, bun (docs tests), gh
brew install rustup zig just cargo-nextest gh
brew list oven-sh/bun/bun >/dev/null 2>&1 || brew install oven-sh/bun/bun

export PATH=/opt/homebrew/opt/rustup/bin:$HOME/.cargo/bin:/opt/homebrew/bin:$PATH

zig_version="$(zig version)"
if [[ "$zig_version" != 0.16.* ]]; then
  echo "error: Zig 0.16.x required, found $zig_version (set ZIG to a 0.16 binary)" >&2
  exit 1
fi

# Maintenance tests need tomllib (Python 3.11+); Apple's /usr/bin/python3 is 3.9.
if ! python3 -c 'import sys; sys.exit(sys.version_info < (3, 11))'; then
  echo "error: python3 on PATH is $(python3 --version 2>&1); need 3.11+ (brew install python)" >&2
  exit 1
fi

# Installs the channel and components pinned in rust-toolchain.toml.
rustup toolchain install

# Remotes: origin = fork, upstream = herdrdev/herdr with pushes disabled.
if git remote get-url upstream >/dev/null 2>&1; then
  git remote set-url --push upstream DISABLED-never-push-to-upstream
fi
push_url="$(git remote get-url --push upstream 2>/dev/null || true)"
if [[ -n "$push_url" && "$push_url" != "DISABLED-never-push-to-upstream" ]]; then
  echo "error: upstream push URL is not disabled" >&2
  exit 1
fi

just install-hooks

# Expose the project skills to Claude Code (.claude/skills -> .agents/skills).
mkdir -p .claude/skills
for skill in .agents/skills/*/; do
  name="$(basename "$skill")"
  [[ -e ".claude/skills/$name" ]] || ln -s "../../.agents/skills/$name" ".claude/skills/$name"
done

mkdir -p .local/prd

cargo build
echo "setup complete: $(target/debug/herdr --version)"
echo "remember: export PATH=/opt/homebrew/opt/rustup/bin:\$HOME/.cargo/bin:/opt/homebrew/bin:\$PATH"
