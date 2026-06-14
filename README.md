# Codex CUA Resume Assist

![Codex CUA Resume Assist hero](assets/hero.png)

**우리는 과장하지 않았고, AI들이 서로 검토해 하나의 정본을 만들었습니다.**

사람이 잠깐 자리를 비운 사이, AI 코딩 도구가 사용량 한도나 대기 화면에서 멈춰 있는 경우가 있습니다. 시간이 지나 다시 일을 시켜도 되는 상태가 되었는데도 아무도 확인하지 않으면, 작업은 밤새 멈춰 있습니다.

이 프로젝트는 그 문제를 다룹니다. 컴퓨터 화면을 보고 “아직 기다려야 하는지”, “다시 진행해도 되는지”를 조심스럽게 판단하는 공개 실험입니다.

중요한 점은, 이 도구가 사용량 제한을 뚫거나 우회하는 물건이 아니라는 것입니다. 시간이 지나 정상적으로 다시 사용할 수 있는 상태가 되었는지 확인하는 보조 도구입니다.

이 프로젝트는 OpenAI의 공식 프로젝트가 아니며 OpenAI가 보증하거나 후원하지 않습니다.

## 이 프로젝트의 진짜 의미

첫째, 과장하거나 거짓말하지 않는 것입니다.

이 프로젝트는 “한도를 우회한다”, “완전히 자동으로 모든 작업을 처리한다”, “OpenAI 공식 기능이다” 같은 식으로 말하지 않습니다. 아직 초기 실험이고, 위험하면 기본적으로 기다립니다. 가능한 것과 아직 안 되는 것을 분리해서 말하는 것이 이 프로젝트의 첫 번째 원칙입니다.

둘째, AI 세션들이 서로 검토하고 의견을 맞춰 정본을 만들었다는 점입니다.

여러 AI 세션이 각자 다른 관점에서 공개 문구, 보안, 사용성, 과장 표현 위험을 검토했습니다. 그 의견을 하나로 합쳐 README, 이미지, 안전장치, 공개 범위를 정리했습니다. 이것이 더 큰 의미입니다. 기능 하나보다 중요한 것은 “AI들이 서로 정보를 주고받고, 토의하고, 합의해서 하나의 결과물을 만든다”는 작업 방식입니다.

## 30초 요약

- AI 코딩 도구가 멈춰 있으면 사람이 직접 화면을 보고 다시 실행해야 합니다.
- 이 프로젝트는 그 “화면 확인”을 자동화하려는 첫 실험입니다.
- 화면을 보고 판단하는 방식이라 Windows, WSL, macOS 같은 실제 사용 환경을 고려합니다.
- 기본값은 안전하게 `wait`입니다. 위험하면 아무것도 하지 않습니다.
- 일반 대중이 당장 쓸 완성품이라기보다, “AI들이 서로 상의하고 정본을 만드는 시대”를 보여주는 오픈소스 출발점입니다.

## 왜 이 프로젝트가 다른가

보통 오픈소스는 사람이 코드를 쓰고 AI가 일부를 도와주는 방식으로 소개됩니다.

이 프로젝트는 반대로 “AI들이 서로 검토하고, 사람이 방향과 최종 판단을 맡는 방식”을 전면에 둡니다. 한 AI가 놓친 표현 리스크를 다른 AI가 잡고, 한 AI가 과감하게 만든 기능을 다른 AI가 보수적으로 검토하는 구조입니다.

완성품이라고 주장하지 않습니다. 다만 “AI가 AI의 결과물을 검토하고 합의해 공개 프로젝트를 만든다”는 흐름을 실제 코드와 README, 이미지, 안전장치까지 묶어서 보여주는 첫 버전입니다. 이 방식이 안정화되면 한 사람이 모든 것을 직접 지시하는 방식보다, 여러 AI 세션이 일을 나누고 충돌을 줄이며 하나의 결과로 합치는 방식이 더 중요해질 수 있습니다.

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
- `.env`, 로그, 스크린샷, 바이너리 파일은 git에 올리지 않도록 막았습니다.
- 이 공개 버전은 터미널에 임의 명령을 입력하지 않습니다.

## 함께 고쳐나가기

이 프로젝트는 완성품이 아니라 시작점입니다.

Windows, WSL, macOS, Linux 환경마다 터미널과 화면 캡처 방식이 다를 수 있습니다. 실제 사용 중 발견한 문제, 더 나은 안전장치, 더 쉬운 설명, 더 좋은 UX가 있다면 PR로 함께 고쳐나가는 것을 환영합니다.
