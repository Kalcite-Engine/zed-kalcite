# Kalcite for Zed

Zed extension for Kalcite: syntax highlighting (including `for`, `break`,
`continue` and `defer`), code outline, brackets, indentation, text objects and
`kalcite-lsp` integration. With the current LSP it provides completion and
hover information for allocation-free `Text` APIs, including
`Text.equals(value, "literal")`, `Text.starts_with(value, "prefix")`, and
`Text.contains(value, "needle")`.

Install Kalcite through the Kallyup `developer` or `full` profile, then install
this repository as a Zed development extension. The extension resolves
`kalcite-lsp` from the active worktree PATH.

```sh
kallyup install developer
```

In Zed, run **zed: install dev extension** and select this repository. For a
published registry release, submit the repository to `zed-industries/extensions`.
