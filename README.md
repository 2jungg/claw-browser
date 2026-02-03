# 🌐 Project ClawBrowser 상황판

외부 대형 엔진(Chromium, WebKit 등)을 배제하고 Rust로 바닥부터 직접 구현하는 초경량/고성능 웹 브라우저 프로젝트입니다.

## 🚀 진행 상황
- [x] 0단계: 프로젝트 초기화 (Director 완료)
- [ ] 1단계: **전체 시스템 아키텍처 및 모듈화 기획 (Gemini Pro 진행 중)**
- [ ] 2단계: 핵심 엔진 병렬 개발 (Multi-Opus 가동 예정)
  - [ ] `claw-net`: HTTP/TLS 네트워크 스택
  - [ ] `claw-html`: HTML5 파서 및 DOM 트리 구축
  - [ ] `claw-render`: CSS 엔진 및 GUI 렌더러
- [ ] 3단계: 통합 및 브라우저 UI 개발
- [ ] 4단계: 품질 검수 및 성능 최적화

## 👷 담당 에이전트
- **기획/PM**: Gemini Pro
- **개발**: 오퍼스 군단 (Multiple Claude 4.5 Opus)
- **총괄**: 중미나이 (Flash)

## 📌 주요 특징
- Pure Rust 구현 (Memory Safety 극대화)
- 독자적인 렌더링 파이프라인
- 병렬 개발 모드(Git Worktree) 적용 대상 프로젝트
