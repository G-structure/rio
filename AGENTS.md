# Agent Instructions

- Use the `.agents/notes/` directory as a private workspace for Markdown notes.
- Do not reference files under `.agents/notes/` in commit messages, PR descriptions, or external documentation.
- When modifying code or documentation outside `.agents/notes`, run `cargo fmt --all` and `cargo test --all` before committing.
- Note-only changes within `.agents/notes` do not require running tests.
