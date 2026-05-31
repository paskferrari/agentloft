<h1 align="center">
  <img src="tauri-app/src-tauri/icons/128x128.png" width="80" alt="AgentLoft"><br>
  AgentLoft
</h1>

<p align="center">
  <strong>A pixel art office where your Claude Code agents come to life.</strong><br>
  Watch them work, type, and collaborate — in real time.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/platform-macOS-black?style=flat&logo=apple" />
  <img src="https://img.shields.io/badge/built%20with-Tauri%20v2-purple?style=flat" />
  <img src="https://img.shields.io/badge/license-MIT-blue?style=flat" />
  <img src="https://img.shields.io/badge/Claude%20Code-native-orange?style=flat" />
</p>

<p align="center">
  <img src="docs/demo.gif" alt="AgentLoft demo" width="700">
</p>

---

## What is this?

AgentLoft hooks into your Claude Code sessions and renders each agent as an animated pixel art character inside a virtual office. When an agent reads a file, it animates. When it runs a command, you see it at the terminal. When it's done, it idles at its desk.

Built on top of [pixel-agents](https://github.com/pablodelucca/pixel-agents) (MIT) — adapted as a native Mac app with no VS Code dependency.

## Features

- **Zero config** — install, open, start Claude Code. Agents appear automatically.
- **Native Mac app** — Tauri v2, not Electron. Tiny 4.8MB DMG.
- **Real-time** — WebSocket connection to Claude Code hooks.
- **Multi-agent** — each Claude session is a separate character.
- **Layout editor** — customize your office: floors, walls, furniture.
- **Works with any project** — hooks into ~/.claude/settings.json.

## Install

### Option A — DMG (recommended)

Download `AgentLoft_0.1.0_aarch64.dmg` from [Releases](https://github.com/paskferrari/agentloft/releases).

> **Requires:** macOS 10.15+ and Node.js 18+

### Option B — run from source

```bash
git clone https://github.com/paskferrari/agentloft
cd agentloft

# Build server + webview
node esbuild.js
cd webview-ui && npx vite build && cd ..

# Build Mac app
cd tauri-app
cargo-tauri build
open src-tauri/target/release/bundle/macos/AgentLoft.app
```

## How it works

```
Claude Code  →  hooks  →  AgentLoft server (port 3200)  →  WebSocket  →  pixel art office
```

1. AgentLoft installs lightweight hooks in `~/.claude/settings.json`
2. Every tool call (Read, Bash, Edit…) fires an event to the local server
3. The server maps events to agent characters
4. Characters animate based on what the agent is doing

## Credits

Built on [pixel-agents](https://github.com/pablodelucca/pixel-agents) by [@pablodelucca](https://github.com/pablodelucca) — MIT License.

## License

MIT
