# KitsuVM // UNiDOsX's Terminal Emulator

Detects the default terminal emulator using multiple detection methods allowing TUI applications work even in GUI environments.

It also provides a list of terminals that uses `terminal [command]`, `terminal -- command`, or `terminal -e command` syntax to launch a command in the terminal and a launcher helper.

## Features

- `unix`: Enables UNIX (BSD and Linux) detection methods. (enabled by default)
- `linux`: Enables Linux-specific detection methods. (enabled by default)
- `macos`: Enables macOS-specific detection methods. (enabled by default)
- `env-var`: Enables detection using the `TERMINAL_EMULATOR` environment variable. (enabled by linux, macos, and unix)
- `terminal-app`: Enables detection for Terminal.app on macOS. (enabled by macos)
- `xdg-terminal-exec`: Enables detection using `xdg-terminal-exec`. (enabled by linux and unix)
- `x-terminal-emulator`: Enables detection using `x-terminal-emulator`. (enabled by linux)
- `gnome-settings`: Enables detection using GNOME settings. (enabled by linux and unix)
- `kde-settings`: Enables detection using KDE settings. (enabled by linux and unix)
- `hardcoded`: Enables detection using a hardcoded list of known terminal emulators. (enabled by linux and unix)
- `hardcoded-traditional`: Enables detection using a traditional hardcoded list of known terminal emulators. (enabled by hardcoded)
- `hardcoded-modern`: Enables detection using a modern hardcoded list of known terminal emulators. (enabled by hardcoded)
- `hardcoded-desktop-env`: Enables detection using desktop environment-specific hardcoded lists of known terminal emulators. (enabled by hardcoded)
- `hardcoded-extended`: Enables detection using an extended hardcoded list of known terminal emulators. (enabled by hardcoded)

## Detection Methods

- **Environment Variable**: Checks for the `TERMINAL_EMULATOR` environment variable.
- **Terminal.app**: If running on macOS, it checks for Terminal.app as the default terminal.
- **xdg-terminal-exec**: Utilizes the `xdg-terminal-exec` command to find the default terminal emulator.
- **x-terminal-emulator**: Uses the `x-terminal-emulator` command from Debian-based systems.
- **GNOME Settings**: Queries GNOME settings to determine the preferred terminal emulator.
- **KDE Settings**: Checks KDE configuration for the default terminal emulator.
- **"GIO's Way"**: Uses a hardcoded list of known terminal emulators to find a match.

### Windows Support

On Windows, the terminal emulator is part of the Win32 API making the default terminal emulator invocation works through the Win32 call `AllocConsole`, so no detection is necessary.

## Hardcoded Terminal Emulators

When searching for terminal emulators using hardcoded lists it will use the PATH environment variable to search for the terminal emulators.

### Traditional

- xterm
- rxvt
- urxvt
- aterm
- eterm
- pterm
- mrxvt
- st
- mlterm
- fbterm
- kmscon

### Desktop Environment Specific

- kgx (GNOME Console)
- gnome-terminal
- konsole
- xfce4-terminal
- mate-terminal
- lxterminal
- qterminal
- ptyxis
- deepin-terminal
- io.elementary.terminal

### Modern

- kitty
- alacritty
- wezterm
- ghostty
- foot
- rio
- contour
- hyper
- tabby
- blackbox
- warp
- extraterm

### Extended

- terminator
- tilix
- guake
- yakuake
- tilda
- terminology
- cool-retro-term
- sakura
- roxterm
- edex-ui

## License

This project is licensed under the [MIT License](LICENSE).
