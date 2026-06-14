# Public Review Notes

These notes are written to prevent overclaiming before a public GitHub and Threads release.

## Claim Boundaries

Safe claim:

- "This is an unofficial public alpha reference implementation for a screen-aware Codex CLI resume assistant."
- "It uses the OpenAI Computer Use tool pattern and a local screenshot harness."
- "It does not bypass rate limits; it only checks whether work can resume after legitimate recovery."
- "CUA makes the situational decision; the local helper only enforces narrow mechanical checks such as same foreground window and one configured short message."

Claims to avoid:

- "This fully automates every Codex session."
- "This bypasses Codex limits."
- "The prior private watchdog was itself OpenAI Computer Use."
- "It can safely type into any app or terminal without risk."

## Criticism Risks and Responses

Risk: "This is not Computer Use."

Response: The code uses the Responses API with a `computer` tool and a local `computer_call_output` screenshot loop. The README links the official Computer Use guide and describes the harness boundary honestly.

Risk: "This is dangerous automation."

Response: The public build is observation-first by default. It blocks model-requested click/type/scroll actions and returns `wait` unless the user explicitly enables execution. Execution is opt-in: users must pass `--execute` plus either `--exec-fallback` or `--terminal-send`, and the action runs only after a `resume` decision and confidence threshold. For `--terminal-send`, the user provides the exact short text; the helper rechecks that the foreground window is the same one CUA reviewed before sending text plus Enter. Long fallback prompts are supplied through stdin or a file so shell wrappers do not split a prompt into unintended command-line arguments. Teams that require coordination can add `--coordination-command` so their session bus, artifact claim, or duplicate-work check must pass before execution.

Risk: "It leaks secrets."

Response: API mode is opt-in with `--api` and may send screenshots to OpenAI, so users should run it only on screens they are comfortable transmitting. The repo git-ignores `.env`, logs, screenshots, binaries, and generated image files, and sends `OPENAI_API_KEY` through an HTTP library header rather than a shell command line.

Risk: "It only works on one private environment."

Response: Private orchestration services, private hostnames, and personal paths were removed. Windows/WSL, macOS, and Linux capture paths are documented.

Risk: "It relies on brittle terminal allowlists."

Response: Terminal or shell labels are treated as clues, not final allow/deny rules. CUA is asked to judge the visible situation. For the foreground typing path, the local helper does not decide based on a long list of app names; it only checks that the foreground window handle has not changed since CUA reviewed it.

Risk: "The fallback command could leak logs."

Response: Fallback stdout/stderr bodies are not written into the JSONL event log. The event records only whether the command completed or failed, plus whether fallback, prompt, and advisory lock options were configured.

## Official Reference

OpenAI Computer Use guide:

<https://developers.openai.com/api/docs/guides/tools-computer-use>
