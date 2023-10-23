# zellij-cb
Custom compact bar plugin for [Zellij](https://zellij.dev/) based on [Zellij's default plugin](https://github.com/zellij-org/zellij/tree/main/default-plugins/compact-bar).

<div align="center">
  <div>
    <img src="https://github.com/ndavd/zellij-cb/assets/74260683/94c76afa-223c-4fcd-974e-275cb8b1690f" />
  </div>
  <code>{session directory}-{session name} {mode in 1 letter} {...tabs}</code>
</div>

## Features
- Displays session directory name
- Is super compact and minimal
- Configurable

## Available configuration
- Content
  - **DefaultTabName** e.g. `TERM`
- Colors
  - **SessionDirectoryColor** e.g. `blue`
  - **SessionNameColor** e.g. `blue`
  - **TabColor** e.g. `blue`
  - **ActiveTabColor** e.g. `blue`
  - **NormalModeColor** e.g. `blue`
  - **OtherModesColor** e.g. `blue`
  - **OthersColor** e.g. `blue`

## Example usage
Check out my [dotfiles](https://github.com/ndavd/dotfiles/tree/main/.config/zellij).
