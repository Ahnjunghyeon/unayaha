# Usagi 0.4.0 Windows compile fix

Windows `npm run dev` 로그에서 확인된 두 Rust 컴파일 오류를 고치는 적용 스크립트입니다.

- `BaseDirectory` import를 `path::BaseDirectory`로 변경
- `window.set_position(position)?`를 `window.set_position(*position)?`로 변경

## 적용 방법

1. ZIP을 프로젝트 루트에 풉니다. 루트에는 `src-tauri` 폴더가 있어야 합니다.
2. PowerShell에서 해당 프로젝트 폴더로 이동해 실행합니다.

```powershell
powershell -ExecutionPolicy Bypass -File .pply-tauri-compile-fix.ps1
npm run dev
```

스크립트는 두 위치를 각각 정확히 하나씩 찾을 때만 파일을 저장합니다. 소스 모양이 다르면 변경 없이 오류 메시지를 내고 멈춥니다.

참고: 이 ZIP은 전체 0.4.0 소스가 아니라 수정 적용용 스크립트입니다. 현재 확인 가능한 GitHub `main` 및 작업 폴더 소스는 0.3.0이며, 사용자가 실행한 0.4.0 프로젝트 파일은 이 환경에 없습니다.
