# Project Goals

- `liblzma-rs`: `liblzma` C 라이브러리를 대체하는 Rust 구현체
- `liblzma-rs-sys`: `liblzma-sys` Rust 라이브러리를 대체하는 Rust 구현체
- crate 역할 분리: `liblzma-rs`는 `rlib`만, `liblzma-rs-sys`는 `staticlib`(및 Rust 의존을 위한 `rlib`)을 담당한다.
- 외부 API 경계 이동 원칙: 외부 C ABI 경계(`no_mangle extern "C"`)는 `liblzma-rs-sys`로 이동하고, `liblzma-rs`에는 의미 구현(로직)만 남긴다.

# Rules for `liblzma-rs-sys`

- `liblzma-rs-sys`는 `liblzma-sys` 호환성을 맞추기 위한 레이어로 유지한다.
- 기본적으로 `re-export`와 이름/타입 alias 중심으로 유지한다.
- 정책 예외: 외부 API 경계를 `liblzma-rs-sys`로 옮기기 위한 얇은 C ABI wrapper(`#[no_mangle] unsafe extern "C" fn`)는 허용한다.
- 위 wrapper는 `liblzma-rs`의 구현 함수를 그대로 위임 호출하는 얇은 포워더여야 하며, 독자 로직을 넣지 않는다.
- `liblzma-rs-sys`에서 리터럴(`0x..`, 숫자 상수 등)을 사용한 고유 상수 정의를 추가하지 않는다.
- 필요한 상수/심볼은 `liblzma-rs`에서 정의하고 `liblzma-rs-sys`에서 alias/re-export로 노출한다.
- `liblzma-rs-sys`가 expose 하는 API는 `liblzma-sys`와 일점일획도 다르지 않게 유지한다(타입, 함수 시그니처, 상수 타입/값 포함).
- `size_t` 등 C ABI 타입은 반드시 `libc` 타입(`libc::size_t` 등)을 사용한다. 로컬 alias(`type size_t = usize`)는 금지한다.
- 최종 목표는 기존 `liblzma` C header(`lzma.h`)를 그대로 사용해서 `liblzma-rs-sys`를 사용할 수 있는 상태를 유지하는 것이다.
