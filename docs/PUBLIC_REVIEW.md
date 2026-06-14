# Public Review Notes

These notes are written to prevent overclaiming before a public GitHub and Threads release.

## Claim Boundaries

Safe claim:

- "This is an unofficial public alpha reference implementation for a screen-aware Codex CLI resume assistant."
- "It uses the OpenAI Computer Use tool pattern and a local screenshot harness."
- "It does not bypass rate limits; it only checks whether work can resume after legitimate recovery."
- "It separates terminal emulator detection from shell detection."

Claims to avoid:

- "This fully automates every Codex session."
- "This bypasses Codex limits."
- "The prior private watchdog was itself OpenAI Computer Use."
- "It can safely type into any terminal without human review."

## Criticism Risks and Responses

Risk: "This is not Computer Use."

Response: The code uses the Responses API with a `computer` tool and a local `computer_call_output` screenshot loop. The README links the official Computer Use guide and describes the harness boundary honestly.

Risk: "This is dangerous automation."

Response: The public build is observation-first. It blocks model-requested click/type/scroll actions and returns `wait`.

Risk: "It leaks secrets."

Response: API mode is opt-in with `--api` and may send screenshots to OpenAI, so users should run it only on screens they are comfortable transmitting. The repo git-ignores `.env`, logs, screenshots, binaries, and generated image files, and sends `OPENAI_API_KEY` through an HTTP library header rather than a shell command line.

Risk: "It only works on one private environment."

Response: Redis, PM2, `agent-bus`, private hostnames, and personal paths were removed. Windows/WSL, macOS, and Linux capture paths are documented.

Risk: "It confuses terminal app and shell."

Response: The prompt explicitly separates terminal emulators such as WezTerm or Terminal.app from shells such as PowerShell, cmd.exe, bash, zsh, or fish.

## Official Reference

OpenAI Computer Use guide:

<https://developers.openai.com/api/docs/guides/tools-computer-use>
