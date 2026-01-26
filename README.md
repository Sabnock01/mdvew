# mdvew

Render markdown files as images in the terminal.

## Install

```bash
cargo install mdvew
```

Requires Chrome or Chromium installed on your system.

## Usage

```bash
mdvew README.md              # display in terminal
mdvew README.md -t dark      # dark mode
mdvew README.md -o out.png   # save to file
mdvew README.md -b           # open in browser
```

## Options

| Flag | Description |
|------|-------------|
| `-o, --output <FILE>` | Save PNG instead of displaying |
| `-t, --theme <light\|dark>` | Color theme (default: light) |
| `-b, --browser` | Open in system browser |
| `-w, --viewport-width <PX>` | Render width in pixels (default: auto) |

## Acknowledgements

- [viuer](https://github.com/atanunq/viuer) - Terminal image display
- [viu](https://github.com/atanunq/viu) - Inspiration for terminal image viewing
- [github-markdown-css](https://github.com/sindresorhus/github-markdown-css) - GitHub-flavored markdown styling
- [glow](https://github.com/charmbracelet/glow) - Terminal markdown rendering
- [inlyne](https://github.com/Inlyne-Project/inlyne) - GPU-native markdown rendering
