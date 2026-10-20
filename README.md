# 우사기 깜짝 댄스 — Windows 데스크톱 소스

기존 브라우저 데모를 Windows용 투명 오버레이 앱으로 바꾼 Tauri 프로젝트입니다. 앱 창은 화면 전체에 투명하게 놓이고, 우사기만 보입니다. 창은 마우스 클릭을 통과시키므로 아래에 있는 업무 프로그램을 계속 조작할 수 있습니다.

## Windows에서 실행

### 준비물

- Windows 10/11
- Node.js LTS
- Rust stable MSVC 툴체인
- Microsoft C++ Build Tools의 **Desktop development with C++**
- Microsoft Edge WebView2 Runtime (Windows 10/11에는 보통 설치되어 있음)

### 시작

압축을 푼 프로젝트 폴더에서 PowerShell을 열고:

```powershell
npm install
npm run dev
```

또는:

```powershell
pnpm install
pnpm dev
```

`npm run dev` / `pnpm dev`가 브라우저가 아니라 Windows 데스크톱 오버레이 앱을 엽니다. 설치 파일을 만들려면 `npm run build` 또는 `pnpm build`를 실행합니다. 빌드 산출물은 `src-tauri/target/release/bundle/` 아래에 생성됩니다.

## 앱 사용

- 실행하면 상단 창 제목 없이 우사기만 화면 오른쪽 아래에 표시되며, 우사기는 잠듭니다.
- 6초 뒤 몸을 살짝 꿈틀거리며 말풍선으로 “우나..야하야하..”라는 말풍선을 띄운 다음 고개를 듭니다. 그 뒤에는 랜덤한 간격으로 깨어납니다.
- 우사기가 고개를 든 뒤 5초 안에 마우스를 움직이거나 키보드를 누르면 화면에 우사기 춤 GIF 48개가 무작위 위치와 크기로 빽빽하게 나타납니다.
- 테스트 단축키: `Ctrl + Shift + U` (우사기 깨우기)
- 종료 단축키: `Ctrl + Shift + Q`
- 기존 브라우저형 데모가 필요하면 `npm run browser`로 서버를 시작하고 `http://127.0.0.1:4173`을 엽니다.

## 전역 입력과 개인정보

오버레이 창은 포커스를 받지 않고 클릭을 통과시킵니다. Windows 저수준 키보드·마우스 훅은 입력이 있었는지만 확인합니다. 키 이름이나 입력 문자는 저장·전송하지 않습니다. 훅 콜백은 항상 Windows의 다음 훅으로 이벤트를 전달하므로 키 입력을 막지 않습니다. 전체 화면 플래시, 중앙 배너, 제목 표시줄도 네이티브 앱에서는 표시하지 않습니다. `Ctrl + Shift + U/Q` 조합을 확인하는 동안 키 코드가 메모리에 잠깐 쓰이지만 기록되지는 않습니다.

## 기능 범위

이 데모는 Windows 탐색기 삭제창이나 파일을 가로채지 않습니다. 투명 데스크톱 캐릭터, 전역 입력 감지, 춤 애니메이션을 검증합니다. Windows 실행파일은 Windows 개발 환경에서 위 준비물을 설치한 뒤 빌드해야 합니다.
