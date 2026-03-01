# Repository Guidelines

## Project Structure & Module Organization
- `src/main.rs`: Main CLI for `pd` (crate `peed`). Contains command parsing, USB discovery (`nusb`), serial I/O, stats parsing, screenshot capture/encoding, and mount/eject flows.
- `Cargo.toml` / `Cargo.lock`: Rust 2024 dependencies and lockfile.

## Build, Test, and Development Commands
- `cargo run -- device list`: Enumerate Playdate devices and mount state.
- `cargo run -- device mount`: Send `datadisk` (if serial available) and wait until `/Volumes/PLAYDATE` is readable.
- `cargo run -- device eject`: Eject mounted data disk.
- `cargo run -- device stats --json`: Read runtime/device stats as JSON.
- `cargo run -- device screenshot -f shot.png --open`: Capture screen and open output.
- `cargo test`: Run unit tests.
- `cargo fmt`: Format code before commit.

## Coding Style & Naming Conventions

- Follow Rust 2024 idioms and keep code `rustfmt`-clean.
- Use `snake_case` for functions/variables, `SCREAMING_SNAKE_CASE` for constants, and `CamelCase` for enums/structs.
- Keep CLI errors explicit and actionable (state failure + likely fix).
- Preserve command behavior contracts:
  - `mount` waits for readable mount,
  - `datadisk` is immediate/raw.

## Testing Guidelines
- Tests live in `#[cfg(test)]` blocks in `src/main.rs`.
- Add tests for:
  - CLI parsing (`-d`, `--json`, `--open`, `-f` combinations),
  - serial/mount parsing helpers,
  - stats normalization keys/types (e.g., `cpu_*_percent`, `time_epoch`).
- Use behavior-first names, e.g. `parses_stats_command_with_json`.

## Commit & Pull Request Guidelines
- Use short, imperative commit titles (examples from history: `Add support for serial device commands`, `Eject support`).
- Keep commits focused; avoid unrelated refactors.
- PRs should include:
  - behavior change summary,
  - verification commands and relevant output,
  - hardware assumptions (connected, unlocked, serial vs datadisk mode).

## Safety & Device Notes
- Commands affect physical hardware; be cautious with destructive commands.
- If no serial port appears, device may be in data-disk mode; use `device mount`/`device eject` appropriately.
