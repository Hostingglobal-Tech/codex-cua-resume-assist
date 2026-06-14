# Codex CUA Resume Assist

![Codex CUA Resume Assist hero](assets/hero.png)

**우리는 과장하지 않았고, AI들이 서로 검토한 내용을 하나의 공개 정본으로 통합했습니다.**

이 프로젝트의 핵심은 한 AI 세션이 혼자 독고다이로 움직이는 것이 아닙니다. 여러 AI 세션이 서로 현재 상태를 알리고, 같은 일을 중복하지 않도록 맡은 일을 고정하고, 구현·검토·문서화·검증을 나눠 처리한 뒤, 결과를 하나의 정본으로 합치는 방식입니다.

사람이 잠깐 자리를 비운 사이, AI 코딩 도구가 사용량 한도나 대기 화면에서 멈춰 있는 경우가 있습니다. 시간이 지나 다시 일을 시켜도 되는 상태가 되었는데도 아무도 확인하지 않으면, 작업은 밤새 멈춰 있습니다.

이 프로젝트는 그 문제를 다룹니다. 컴퓨터 화면을 보고 “아직 기다려야 하는지”, “다시 진행해도 되는지”를 조심스럽게 판단하는 공개 실험입니다.

중요한 점은, 이 도구가 사용량 제한을 뚫거나 우회하는 물건이 아니라는 것입니다. 시간이 지나 정상적으로 다시 사용할 수 있는 상태가 되었는지 확인하는 보조 도구입니다.

이 프로젝트는 OpenAI의 공식 프로젝트가 아니며 OpenAI가 보증하거나 후원하지 않습니다.

## CUA가 맡는 역할

`continue`나 `계속 진행` 같은 짧은 문장을 입력하는 일 자체는 어렵지 않습니다. 진짜 어려운 부분은 “지금 그 문장을 입력해도 되는 화면인가”를 정확히 판단하는 것입니다.

그래서 이 프로젝트에서 Computer Use는 단순 장식이 아닙니다. CUA는 화면을 보고, 현재 창이 실제 대상 터미널인지 확인하고, 사용량 제한이 정상적으로 회복된 상태인지 판단하고, 사람이 다른 작업을 하는 중이 아닌지 확인하는 제어 gate입니다.

짧은 문장 입력은 마지막 실행부일 뿐입니다. CUA 판단, 세션 간 사전 조율, 작업 claim, 중복 실행 방지 lock, 신뢰도 기준을 모두 통과했을 때만 식별된 터미널이나 pane에 제한적으로 실행되어야 합니다. 아무 foreground 창에 무조건 `continue`를 입력하는 방식은 이 프로젝트의 방향이 아닙니다.

## 핵심: 세션 간 통신과 분업

이 저장소에서 가장 중요한 메시지는 “AI가 화면을 본다”가 아닙니다. 더 중요한 것은 AI 세션들이 서로 대화하고, 상태를 공유하고, 중복 작업을 피하고, 일을 나눠 처리하고, 마지막에 하나의 결과물로 합치는 구조입니다.

실험의 기본 흐름은 이렇습니다.

- 먼저 각 세션이 지금 무엇을 하려는지 공유합니다.
- 같은 파일이나 같은 결과물을 여러 세션이 동시에 고치지 않도록 작업 claim을 겁니다.
- 구현, 보안 검토, 공개 문구 검토, 실행 검증을 서로 나눕니다.
- 한 세션이 놓친 위험을 다른 세션이 잡고, 충돌하는 의견은 하나로 정리합니다.
- 최종 결과는 채팅창마다 흩어진 답변이 아니라 GitHub `main` 브랜치의 README와 코드로 고정합니다.

그래서 이 프로젝트는 단순한 자동 재개 도구라기보다, 여러 AI 세션이 협업해서 하나의 공개 산출물을 만드는 루프엔지니어링 실험입니다.

## 이번에 실제로 고친 문제

처음 공개한 버전은 방향은 맞았지만, 실제 운영 관점에서 바로 부족한 부분이 있었습니다.

- 긴 프롬프트를 명령행 인자로 넘기면 Windows PowerShell이나 CLI wrapper가 문장을 잘못 쪼개서 `unexpected argument` 오류가 날 수 있었습니다.
- 재개 명령이 실행되기 전에 다른 AI 세션이 이미 같은 일을 맡았는지 확인하는 장치가 부족했습니다.
- 여러 감시 프로세스가 동시에 같은 세션을 재개하려고 할 때 중복 실행을 줄이는 장치가 필요했습니다.
- WezTerm, Windows Terminal, 일반 `cmd.exe`, `powershell.exe`를 더 분명히 구분해야 했습니다.
- fallback 명령의 출력 본문이 로그에 남으면 민감한 정보가 섞일 수 있어, 로그를 더 보수적으로 다뤄야 했습니다.

이번 수정에서 아래를 반영했습니다.

- 긴 프롬프트는 명령행 인자가 아니라 stdin 또는 파일로 넘길 수 있게 했습니다.
- `--preflight-command`를 추가해, 재개 전에 세션 버스 확인, 작업 claim, 중복 작업 감지 같은 사전 점검을 강제할 수 있게 했습니다.
- `--lock-file`을 추가해, 동시에 여러 감시 프로세스가 떠도 같은 재개 명령이 중복 실행될 가능성을 줄였습니다.
- Windows Terminal 계열 창을 더 정확히 구분하도록 현재 창 제목과 앱 판별 정보를 보강했습니다.
- fallback 명령의 stdout/stderr 본문은 JSONL 로그에 저장하지 않고, 성공/실패 상태와 설정 여부만 남기도록 했습니다.

즉, 이번 수정의 핵심은 “화면을 보고 재개 여부를 판단한다”에서 끝나는 것이 아니라, 실제 사람이 자리를 비운 상태에서도 더 안전하게 재개 명령을 넘기기 위한 기본 장치를 넣은 것입니다.

## 이 프로젝트의 진짜 의미

첫째, 과장하거나 거짓말하지 않는 것입니다.

이 프로젝트는 “한도를 우회한다”, “완전히 자동으로 모든 작업을 처리한다”, “OpenAI 공식 기능이다” 같은 식으로 말하지 않습니다. 아직 초기 실험이고, 위험하면 기본적으로 기다립니다. 가능한 것과 아직 안 되는 것을 분리해서 말하는 것이 이 프로젝트의 첫 번째 원칙입니다.

둘째, AI 세션들이 서로 검토한 내용을 하나의 공개 정본으로 통합했다는 점입니다.

여러 AI 세션이 각자 다른 관점에서 공개 문구, 보안, 사용성, 과장 표현 위험을 검토했습니다. 그 의견을 하나로 합쳐 README, 이미지, 안전장치, 공개 범위를 정리했습니다. 이것이 더 큰 의미입니다. 기능 하나보다 중요한 것은 “AI들이 서로 정보를 주고받고, 토의하고, 합의해서 하나의 결과물을 만든다”는 작업 방식입니다.

## 정본 기준

세션 창마다 보이는 답변이나 초안은 정본이 아닙니다.

이 프로젝트의 정본은 GitHub `main` 브랜치에 올라간 README와 소스코드입니다. 여러 AI 세션의 출력이 서로 다를 수 있기 때문에, 최종 공개 기준은 채팅창 문구가 아니라 저장소에 커밋된 내용으로 고정합니다. 서로 다른 초안이 충돌하면 GitHub `main`의 최신 커밋을 우선합니다.

즉, “AI들이 검토했다”는 말은 모든 세션 창이 항상 같은 문장을 동시에 출력한다는 뜻이 아닙니다. 여러 검토 결과를 모아 사람이 최종 기준을 정하고, 그 기준을 하나의 공개 저장소에 고정했다는 뜻입니다.

## 30초 요약

- AI 코딩 도구가 멈춰 있으면 사람이 직접 화면을 보고 다시 실행해야 합니다.
- 이 프로젝트는 그 “화면 확인”을 자동화하려는 첫 실험입니다.
- 진짜 핵심은 여러 AI 세션이 서로 통신하고, 일을 나누고, 하나의 정본으로 합치는 방식입니다.
- 화면을 보고 판단하는 방식이라 Windows, WSL, macOS 같은 실제 사용 환경을 고려합니다.
- 기본값은 안전하게 `wait`입니다. 위험하면 아무것도 하지 않습니다.
- 일반 대중이 당장 쓸 완성품이라기보다, “AI들이 서로 상의하고 분업해서 정본을 만드는 시대”를 보여주는 오픈소스 출발점입니다.

## 왜 이 프로젝트가 다른가

보통 오픈소스는 사람이 코드를 쓰고 AI가 일부를 도와주는 방식으로 소개됩니다.

이 프로젝트는 반대로 “AI들이 서로 통신하고, 일을 나누고, 사람이 방향과 최종 판단을 맡는 방식”을 전면에 둡니다. 한 AI가 놓친 표현 리스크를 다른 AI가 잡고, 한 AI가 과감하게 만든 기능을 다른 AI가 보수적으로 검토하는 구조입니다.

완성품이라고 주장하지 않습니다. 다만 “AI가 AI의 결과물을 검토하고, 서로 중복을 피하고, 합의해 공개 프로젝트를 만든다”는 흐름을 실제 코드와 README, 이미지, 안전장치까지 묶어서 보여주는 첫 버전입니다. 이 방식이 안정화되면 한 사람이 모든 것을 직접 지시하는 방식보다, 여러 AI 세션이 일을 나누고 충돌을 줄이며 하나의 결과로 합치는 방식이 더 중요해질 수 있습니다.

## 누가 보면 좋은가

대부분의 일반 사용자는 이 도구를 바로 설치해서 쓸 필요가 없습니다.

다만 아래에 해당한다면 볼 만합니다.

- AI 코딩 도구를 오래 켜두고 작업하는 사람
- Windows, WSL, macOS에서 자동화 도구를 만드는 사람
- Computer Use처럼 화면을 이해하는 AI 기술에 관심 있는 사람
- “AI들이 서로 검토해서 하나의 결과물을 만든다”는 작업 방식에 관심 있는 사람
- 초기 오픈소스에 참여해서 같이 고쳐보고 싶은 사람

## 무엇을 하려는 프로젝트인가

사람이 자리를 비운 동안 터미널이 멈춰 있을 수 있습니다. 이 도구는 화면을 캡처해 다음을 구분하려고 합니다.

- 사용량 한도가 아직 남아 있는지, 회복됐는지
- 현재 창이 WezTerm, Windows Terminal, Terminal.app 같은 터미널 앱인지
- 그 안에서 PowerShell, cmd.exe, bash, zsh 같은 어떤 셸이 보이는지
- 사람이 타이핑 중인 위험한 상황은 아닌지
- 지금 바로 진행해도 되는지, 아니면 기다려야 하는지

중요한 점은 이 도구가 사용량 제한을 우회하지 않는다는 것입니다. 정상적으로 시간이 지나 한도가 회복됐는지 확인하고, 사람이 다시 진행해도 될지 판단을 돕는 보조 도구입니다.

## 현재 상태

초기 공개 버전입니다. 버그가 있을 수 있습니다.

지금 가장 중요한 목표는 완벽한 자동화가 아닙니다. “화면을 이해해서 안전하게 재개 여부를 판단한다”는 생각을 실제 코드로 꺼내 놓는 것입니다. 무에서 유를 만드는 첫 버전이고, PR과 개선 제안을 적극 환영합니다.

## 동작 방식

OpenAI Computer Use 방식은 모델이 화면을 보고, 필요한 판단을 하고, 로컬 프로그램이 그 판단을 받아 처리하는 구조입니다.

이 프로젝트는 기본적으로 매우 보수적으로 동작합니다.

- 기본 실행은 화면을 캡처하지 않습니다.
- 기본 실행은 OpenAI API를 호출하지 않습니다.
- API 모드는 `--api`를 붙여야만 켜집니다.
- API 모드에서는 화면 캡처 이미지가 OpenAI로 전송될 수 있습니다.
- 민감한 화면에서는 기본 모드만 쓰거나, 직접 검토한 스크린샷 파일만 넘기는 것을 권장합니다.
- 공개 빌드는 터미널에 임의로 타이핑하지 않습니다.
- 모델이 클릭, 입력, 스크롤 같은 조작을 요청하면 실행하지 않고 대기합니다.

OpenAI 이미지/컴퓨터 사용 관련 공식 문서는 아래를 참고하십시오.

- Computer Use: <https://developers.openai.com/api/docs/guides/tools-computer-use>
- Image generation: <https://developers.openai.com/api/docs/guides/tools-image-generation>

## 설치

```bash
git clone https://github.com/Hostingglobal-Tech/codex-cua-resume-assist.git
cd codex-cua-resume-assist
cargo build --release
```

## 기본 실행

아래 명령은 화면을 캡처하지 않고, OpenAI API도 호출하지 않습니다. 보수적인 `wait` 판단만 로컬 JSONL 로그에 남깁니다.

```bash
target/release/codex-cua-resume-assist --dry-run
```

로그 기본 위치:

```text
~/.codex-cua-resume-assist/cua-assist.jsonl
```

로그 위치 변경:

```bash
CODEX_CUA_STATE_DIR=/path/to/state target/release/codex-cua-resume-assist --dry-run
```

## OpenAI Computer Use 모드

아래 모드는 명시적으로 `--api`를 붙였을 때만 켜집니다. 실시간 화면 캡처 또는 `--screenshot`으로 넘긴 이미지가 OpenAI로 전송될 수 있습니다.

```bash
export OPENAI_API_KEY="..."
target/release/codex-cua-resume-assist --api --dry-run
```

민감한 화면 전체를 보내고 싶지 않다면, 먼저 직접 검토한 이미지만 넘기십시오.

```bash
target/release/codex-cua-resume-assist --api --dry-run --screenshot /path/to/reviewed.png
```

## Windows PowerShell

Windows에서는 다음처럼 실행할 수 있습니다.

```powershell
cargo build --release
.\target\release\codex-cua-resume-assist.exe --dry-run

$env:OPENAI_API_KEY = "..."
.\target\release\codex-cua-resume-assist.exe --api --dry-run
```

## 선택적 재개 명령

기본 공개 빌드는 터미널에 아무것도 입력하지 않습니다. 실제 재개 명령을 실행하려면 사용자가 `--execute`와 `--exec-fallback`을 명시해야 합니다.

긴 프롬프트를 명령행 인자로 직접 넘기면 Windows PowerShell이나 CLI wrapper에서 인자가 잘못 쪼개질 수 있습니다. 이 프로젝트는 그런 경우를 피하기 위해 프롬프트를 파일 또는 stdin으로 넘기는 방식을 제공합니다.

예시:

```powershell
Set-Content -Path .\resume-prompt.txt -Value "계속 진행" -Encoding UTF8
.\target\release\codex-cua-resume-assist.exe --api --execute `
  --preflight-command "your-coordination-check-command" `
  --exec-fallback "codex exec --dangerously-bypass-approvals-and-sandbox --skip-git-repo-check -" `
  --fallback-prompt .\resume-prompt.txt `
  --lock-file "$env:TEMP\codex-cua-resume-assist.lock"
```

주의:

- `--exec-fallback`은 `decision=resume`일 때만 실행됩니다.
- `--preflight-command`를 지정하면 이 명령이 성공해야만 fallback이 실행됩니다. 팀 환경에서는 여기서 세션 버스 확인, artifact claim, 중복 작업 감지를 강제할 수 있습니다.
- `--dry-run`이 있으면 fallback 명령도 실행하지 않습니다.
- fallback 명령의 stdout/stderr 본문은 로그에 저장하지 않고 상태만 기록합니다.
- `--lock-file`은 중복 실행을 줄이는 advisory lock입니다. 보안 경계가 아닙니다.
- 이 기능은 사용량 제한을 우회하지 않습니다. 정상적으로 다시 진행해도 되는지 확인한 뒤 사용자가 지정한 명령을 실행하는 보조 기능입니다.

## WSL에서 Windows 화면 정보 얻기

WSL에서 실행하면서 Windows의 현재 창 정보를 더 정확히 보고 싶다면, Windows PowerShell에서 helper를 빌드합니다.

```powershell
cargo build --release --target x86_64-pc-windows-msvc --bin codex-cua-win-capture
```

생성된 `codex-cua-win-capture.exe`를 메인 실행 파일 옆에 두거나, 아래 환경변수로 경로를 지정합니다.

```bash
export CODEX_CUA_WIN_CAPTURE="/path/to/codex-cua-win-capture.exe"
```

PowerShell을 이용한 화면 캡처 fallback은 기본으로 꺼져 있습니다. 일부 백신이나 보안 제품이 스크립트 기반 캡처를 차단할 수 있기 때문입니다. 필요한 경우에만 켜십시오.

```bash
CODEX_CUA_ENABLE_POWERSHELL_CAPTURE=1 target/release/codex-cua-resume-assist --api --dry-run
```

## macOS

macOS에서는 `screencapture -x`를 사용합니다. 시스템 설정에서 화면 기록 권한을 허용해야 할 수 있습니다.

## Linux

아래 도구 중 하나가 있으면 화면 캡처를 시도합니다.

- `gnome-screenshot`
- `grim`
- `scrot`
- ImageMagick `import`

## 안전 원칙

- 사용량 제한을 우회하지 않습니다.
- 기본값은 로컬 전용입니다.
- API 모드는 사용자가 직접 `--api`를 붙여야 켜집니다.
- API 키는 셸 명령줄이 아니라 Rust HTTP 헤더로 전송합니다.
- 실제 재개 명령은 사용자가 `--execute --exec-fallback`을 명시해야만 실행됩니다.
- 협업 채널이나 작업 큐를 쓰는 팀은 `--preflight-command`로 사전 확인을 강제할 수 있습니다.
- 긴 프롬프트는 명령행 인자가 아니라 stdin 또는 파일로 넘기는 것을 권장합니다.
- `.env`, 로그, 스크린샷, 바이너리 파일은 git에 올리지 않도록 막았습니다.
- 이 공개 버전은 터미널에 임의 명령을 입력하지 않습니다.

## 함께 고쳐나가기

이 프로젝트는 완성품이 아니라 시작점입니다.

Windows, WSL, macOS, Linux 환경마다 터미널과 화면 캡처 방식이 다를 수 있습니다. 실제 사용 중 발견한 문제, 더 나은 안전장치, 더 쉬운 설명, 더 좋은 UX가 있다면 PR로 함께 고쳐나가는 것을 환영합니다.
