//! # KitsuVM // UNiDOsX's Terminal Emulator
//!
//! Detects the default terminal emulator using multiple detection methods allowing TUI applications work even in GUI environments.
//!
//! It also provides a list of terminals that uses `terminal [command]`, `terminal -- command`, or `terminal -e command` syntax to launch a command in the terminal and a launcher helper.
//!
//! ## Features
//!
//! - `unix`: Enables UNIX (BSD and Linux) detection methods. (enabled by default)
//! - `linux`: Enables Linux-specific detection methods. (enabled by default)
//! - `macos`: Enables macOS-specific detection methods. (enabled by default)
//! - `env-var`: Enables detection using the `TERMINAL_EMULATOR` environment variable. (enabled by linux, macos, and unix)
//! - `terminal-app`: Enables detection for Terminal.app on macOS. (enabled by macos)
//! - `xdg-terminal-exec`: Enables detection using `xdg-terminal-exec`. (enabled by linux and unix)
//! - `x-terminal-emulator`: Enables detection using `x-terminal-emulator`. (enabled by linux)
//! - `gnome-settings`: Enables detection using GNOME settings. (enabled by linux and unix)
//! - `kde-settings`: Enables detection using KDE settings. (enabled by linux and unix)
//! - `hardcoded`: Enables detection using a hardcoded list of known terminal emulators. (enabled by linux and unix)
//! - `hardcoded-traditional`: Enables detection using a traditional hardcoded list of known terminal emulators. (enabled by hardcoded)
//! - `hardcoded-modern`: Enables detection using a modern hardcoded list of known terminal emulators. (enabled by hardcoded)
//! - `hardcoded-desktop-env`: Enables detection using desktop environment-specific hardcoded lists of known terminal emulators. (enabled by hardcoded)
//! - `hardcoded-extended`: Enables detection using an extended hardcoded list of known terminal emulators. (enabled by hardcoded)
//!
//! ## Detection Methods
//!
//! - **Environment Variable**: Checks for the `TERMINAL_EMULATOR` environment variable.
//! - **Terminal.app**: If running on macOS, it checks for Terminal.app as the default terminal.
//! - **xdg-terminal-exec**: Utilizes the `xdg-terminal-exec` command to find the default terminal emulator.
//! - **x-terminal-emulator**: Uses the `x-terminal-emulator` command from Debian-based systems.
//! - **GNOME Settings**: Queries GNOME settings to determine the preferred terminal emulator.
//! - **KDE Settings**: Checks KDE configuration for the default terminal emulator.
//! - **"GIO's Way"**: Uses a hardcoded list of known terminal emulators to find a match.
//!
//! ### Windows Support
//!
//! On Windows, the terminal emulator is part of the Win32 API making the default terminal emulator invocation works through the Win32 call `AllocConsole`, so no detection is necessary.
//!
//! ## Hardcoded Terminal Emulators
//!
//! When searching for terminal emulators using hardcoded lists it will use the PATH environment variable to search for the terminal emulators.
//!
//! ### Traditional
//!
//! - xterm
//! - rxvt
//! - urxvt
//! - aterm
//! - eterm
//! - pterm
//! - mrxvt
//! - st
//! - mlterm
//! - fbterm
//! - kmscon
//!
//! ### Desktop Environment Specific
//!
//! - kgx (GNOME Console)
//! - gnome-terminal
//! - konsole
//! - xfce4-terminal
//! - mate-terminal
//! - lxterminal
//! - qterminal
//! - ptyxis
//! - deepin-terminal
//! - io.elementary.terminal
//!
//! ### Modern
//!
//! - kitty
//! - alacritty
//! - wezterm
//! - ghostty
//! - foot
//! - rio
//! - contour
//! - hyper
//! - tabby
//! - blackbox
//! - warp
//! - extraterm
//!
//! ### Extended
//!
//! - terminator
//! - tilix
//! - guake
//! - yakuake
//! - tilda
//! - terminology
//! - cool-retro-term
//! - sakura
//! - roxterm
//! - edex-ui
//!
//! ## License
//!
//! This project is licensed under the [MIT License](LICENSE).

use std::{
    fmt::{self, Display, Formatter},
    path::PathBuf,
    process::Command,
};

#[cfg(test)]
mod tests;

#[cfg(windows)]
/// Detects the default terminal emulator.
pub fn detect<'a>() -> TerminalEmulator<'a> {
    TerminalEmulator {
        command_line: "",
        execution_syntax: ExecutionSyntax::NativeApi,
        path: PathBuf::new(),
        method: DetectionMethod::Windows,
    }
}

/// Builds a command to run a given command in the specified terminal emulator.
///
/// Returns `None` if the terminal emulator uses a native API for command execution.
pub fn build_command_in_terminal<'a>(terminal: &TerminalEmulator<'a>) -> Option<Command> {
    if terminal.execution_syntax == ExecutionSyntax::NativeApi {
        return None;
    }

    let mut cmd = Command::new(&terminal.path);

    if let Some(arg) = terminal.execution_syntax.as_arg() {
        cmd.arg(arg);
    }

    Some(cmd)
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a terminal emulator.
pub struct TerminalEmulator<'a> {
    /// The command line and name of the terminal emulator.
    command_line: &'a str,
    /// The execution syntax used by the terminal emulator.
    execution_syntax: ExecutionSyntax,
    /// The path to the terminal emulator executable.
    path: PathBuf,
    /// The detection method used to find the terminal emulator.
    method: DetectionMethod,
}

/// Command execution syntax used by terminal emulators.
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExecutionSyntax {
    /// `terminal [command]`
    Command,
    /// `terminal -- command`
    DoubleDash,
    #[default]
    /// `terminal -e command`
    E,
    /// Uses the native API to launch commands in the terminal.
    NativeApi,
}

impl ExecutionSyntax {
    /// Returns the execution syntax as the argument.
    pub fn as_arg(&self) -> Option<&str> {
        match self {
            Self::DoubleDash => Some("--"),
            Self::E => Some("-e"),
            _ => None,
        }
    }
}

impl Display for ExecutionSyntax {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Command => write!(f, "[command]"),
            Self::DoubleDash => write!(f, "-- [command]"),
            Self::E => write!(f, "-e [command]"),
            Self::NativeApi => write!(f, "Native API"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// Methods used to detect the default terminal emulator.
pub enum DetectionMethod {
    /// You are on Windows, uses the Windows API.
    Windows,
    /// Uses the `TERMINAL_EMULATOR` environment variable.
    EnvironmentVariable,
    /// Uses Terminal.app on macOS.
    TerminalApp,
    /// Uses `xdg-terminal-exec`.
    XdgTerminalExec,
    /// Uses `x-terminal-emulator`.
    XTerminalEmulator,
    /// Uses GNOME settings.
    GnomeSettings,
    /// Uses KDE settings.
    KdeSettings,
    /// Uses desktop environment-specific hardcoded lists of known terminal emulators.
    HardcodedDesktopEnv,
    /// Uses a modern hardcoded list of known terminal emulators.
    HardcodedModern,
    /// Uses a traditional hardcoded list of known terminal emulators.
    HardcodedTraditional,
    /// Uses an extended hardcoded list of known terminal emulators.
    HardcodedExtended,
}

impl DetectionMethod {
    /// Returns `true` if the detection method is a hardcoded list.
    pub fn is_hardcoded(&self) -> bool {
        matches!(
            self,
            Self::HardcodedDesktopEnv
                | Self::HardcodedModern
                | Self::HardcodedTraditional
                | Self::HardcodedExtended
        )
    }
}

impl Display for DetectionMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Windows => write!(f, "Windows"),
            Self::EnvironmentVariable => write!(f, "Environment Variable"),
            Self::TerminalApp => write!(f, "Terminal.app"),
            Self::XdgTerminalExec => write!(f, "xdg-terminal-exec"),
            Self::XTerminalEmulator => write!(f, "x-terminal-emulator"),
            Self::GnomeSettings => write!(f, "GNOME Settings"),
            Self::KdeSettings => write!(f, "KDE Settings"),
            Self::HardcodedDesktopEnv => {
                write!(f, "Hardcoded Desktop Environment List")
            }
            Self::HardcodedModern => write!(f, "Hardcoded Modern List"),
            Self::HardcodedTraditional => {
                write!(f, "Hardcoded Traditional List")
            }
            Self::HardcodedExtended => write!(f, "Hardcoded Extended List"),
        }
    }
}
