# Repository Guidelines

## Project Structure & Module Organization

- `src/main.rs`: Primary Rust CLI entrypoint for `pd` (crate `peed`), including command parsing, USB discovery (`nusb`), mount/eject logic, and serial command dispatch.
- `Cargo.toml` / `Cargo.lock`: Rust dependencies and build metadata.

## Build, Test, and Development Commands

- `cargo test`: Run unit tests in `src/main.rs`.
- `cargo fmt`: Format Rust code; run before committing.

## Coding Style & Naming Conventions

- Follow Rust 2024 idioms and keep code `rustfmt`-clean.
- Use `snake_case` for functions/variables, `SCREAMING_SNAKE_CASE` for constants, and `CamelCase` for types/enums.
- Keep CLI errors actionable (`what failed` + `how to recover`).

## Testing Guidelines

- Add tests for:
  - CLI argument parsing (new commands/flags),
  - USB/mount parsing behavior,
  - Device selection and normalization logic.
- Test names should describe behavior, e.g. `parses_device_mount_alias_command`.

## Commit & Pull Request Guidelines

- Follow current history style: short, imperative commit subjects (examples: `Eject support`, `Add mount discovery`).
- Keep commits focused (one logical change per commit when possible).
- PRs should include:
  - What changed and why,
  - Exact commands used for verification,
  - Any hardware assumptions (Playdate mode, mounted/unmounted state),
  - Sample CLI output for new commands.

## Security & Device Safety Notes

- Serial commands execute on physical hardware; avoid dangerous commands in automated scripts.
- Validate target serial before destructive operations (`eject`, `factoryreset`, formatting-related commands).
