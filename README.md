<h1 align="center">dev-serve</h1>
<p align="center">
    <a href="https://github.com/sondr3/dev-serve/actions"><img alt="GitHub Actions Status" src="https://github.com/sondr3/dev-serve/workflows/pipeline/badge.svg" /></a>
    <a href="https://crates.io/crates/dev-serve"><img alt="Crates" src="https://img.shields.io/crates/v/dev-serve.svg" /></a>
</p>

<p align="center">
    <b>Spin up a simple static site server with live reload</b>
</p>

- **Simple**: `dev-serve <dir>` to start a server in `<dir>`.
- **Live reload**: Automatically reloads the page when files change.
- **Customizable**: Change the port, enable/disable live reload, and more.

<details>
<summary>Table of Contents</summary>
<br />

- [What and why](#what-and-why)
- [Usage](#usage)
- [Installation](#installation)
- [License](#license)
</details>

# What and why

Mostly a tool for personal needs where I want to quickly spin up a web server
and reload the page when I make changes. 

# Usage

```sh
$ dev-serve -h

Serve a directory with auto-reload

Usage: dev-serve [OPTIONS] [PATH]

Arguments:
  [PATH]  Directory to serve

Options:
  -p, --port <PORT>                Select port to use [default: 3000]
  -r, --reload                     Auto-reload and watch directory
  -e, --extensions <EXTENSIONS>    File extensions to watch
  -v, --verbose                    Verbose output
  -c, --completions <COMPLETIONS>  [possible values: bash, elvish, fish, powershell, zsh]
  -h, --help                       Print help (see more with '--help')
  -V, --version                    Print version
```

## Help

Finally, help is always available with `dev-serve --help`/`dev-serve -h`.

# Installation

Currently, the package is available a couple of places, including Homebrew, AUR and Nix.

<dl>
  <dt>Cargo</dt>
  <dd><code>cargo install dev-serve</code></dd>

  <dt>Homebrew</dt>
  <dd>
    <ol>
      <li><code>brew tap sondr3/homebrew-taps</code></li>
      <li><code>brew install dev-serve</code></li>
    <ol>
  </dd>
</dl>

## Release pages

You can also download the matching release from the [release
tab](https://github.com/sondr3/dev-serve/releases), extracting the archive and
placing the binary in your `$PATH`. Note that for Linux the
`unknown-linux-musl.tar.gz` is preferred as it is statically linked and thus
should run on any Linux distribution.

# LICENSE

GPLv3+.
