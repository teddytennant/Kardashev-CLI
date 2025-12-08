<p align="center"><code>npm i -g @kardashev/cli</code><br />or <code>brew install --cask kardashev</code></p>

<p align="center"><strong>Kardashev CLI</strong> is a multi-agent AI collaboration platform that runs locally on your computer.
</br>
</br>Kardashev enables parallel model orchestration with intelligent synthesis for enhanced AI-human collaboration.</p>

<p align="center">
  <img src="./.github/kardashev-cli-splash.png" alt="Kardashev CLI splash" width="80%" />
  </p>

---

## Quickstart

### Installing and running Kardashev CLI

Install globally with your preferred package manager. If you use npm:

```shell
npm install -g @kardashev/cli
```

Alternatively, if you use Homebrew:

```shell
brew install --cask kardashev
```

Then simply run `kardashev` to get started:

```shell
kardashev
```

If you're running into upgrade issues with Homebrew, see the [FAQ entry on brew upgrade kardashev](./docs/faq.md#brew-upgrade-kardashev-isnt-upgrading-me).

<details>
<summary>You can also go to the <a href="https://github.com/teddytennant/kardashev-cli/releases/latest">latest GitHub Release</a> and download the appropriate binary for your platform.</summary>

Each GitHub Release contains many executables, but in practice, you likely want one of these:

- macOS
  - Apple Silicon/arm64: `kardashev-aarch64-apple-darwin.tar.gz`
  - x86_64 (older Mac hardware): `kardashev-x86_64-apple-darwin.tar.gz`
- Linux
  - x86_64: `kardashev-x86_64-unknown-linux-musl.tar.gz`
  - arm64: `kardashev-aarch64-unknown-linux-musl.tar.gz`

Each archive contains a single entry with the platform baked into the name (e.g., `kardashev-x86_64-unknown-linux-musl`), so you likely want to rename it to `kardashev` after extracting it.

</details>

### Authentication

<p align="center">
  <img src="./.github/kardashev-cli-login.png" alt="Kardashev CLI login" width="80%" />
  </p>

Kardashev uses its own API for AI model access. To get started:

1. Get an API key from [kardashev.ai](https://kardashev.ai/)
2. Run `kardashev` and enter your API key when prompted
3. Your API key will be securely stored in `~/.kardashev/config.toml`

Alternatively, you can set the `KARDASHEV_API_KEY` environment variable.

### Model Context Protocol (MCP)

Kardashev can access MCP servers. To configure them, refer to the [config docs](./docs/config.md#mcp_servers).

### Configuration

Kardashev CLI supports a rich set of configuration options, with preferences stored in `~/.kardashev/config.toml`. For full configuration options, see [Configuration](./docs/config.md).

### Execpolicy

See the [Execpolicy quickstart](./docs/execpolicy.md) to set up rules that govern what commands Kardashev can execute.

### Docs & FAQ

- [**Getting started**](./docs/getting-started.md)
  - [CLI usage](./docs/getting-started.md#cli-usage)
  - [Slash Commands](./docs/slash_commands.md)
  - [Running with a prompt as input](./docs/getting-started.md#running-with-a-prompt-as-input)
  - [Example prompts](./docs/getting-started.md#example-prompts)
  - [Custom prompts](./docs/prompts.md)
  - [Memory with AGENTS.md](./docs/getting-started.md#memory-with-agentsmd)
- [**Configuration**](./docs/config.md)
  - [Example config](./docs/example-config.md)
- [**Sandbox & approvals**](./docs/sandbox.md)
- [**Execpolicy quickstart**](./docs/execpolicy.md)
- [**Authentication**](./docs/authentication.md)
  - [Auth methods](./docs/authentication.md#forcing-a-specific-auth-method-advanced)
  - [Login on a "Headless" machine](./docs/authentication.md#connecting-on-a-headless-machine)
- **Automating Kardashev**
  - [GitHub Action](https://github.com/teddytennant/kardashev-action)
  - [TypeScript SDK](./sdk/typescript/README.md)
  - [Non-interactive mode (`kardashev exec`)](./docs/exec.md)
- [**Advanced**](./docs/advanced.md)
  - [Tracing / verbose logging](./docs/advanced.md#tracing--verbose-logging)
  - [Model Context Protocol (MCP)](./docs/advanced.md#model-context-protocol-mcp)
- [**Zero data retention (ZDR)**](./docs/zdr.md)
- [**Contributing**](./docs/contributing.md)
- [**Install & build**](./docs/install.md)
  - [System Requirements](./docs/install.md#system-requirements)
  - [DotSlash](./docs/install.md#dotslash)
  - [Build from source](./docs/install.md#build-from-source)
- [**FAQ**](./docs/faq.md)
- [**Open source fund**](./docs/open-source-fund.md)

---

## License

This repository is licensed under the [Apache-2.0 License](LICENSE).
