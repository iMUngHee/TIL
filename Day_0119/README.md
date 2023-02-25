# Rust Tutorial

## 🔥 목표

팀원들과 모각소 목표 설정 및 러스트 기초 학습

## 👨‍💻 활동

<러스트 프로그래밍> 챕터 1 학습 및 내용 정리

### 공부내용

<details>
<summary><b>Ch 1. 러스트 소개</b></summary>

![rust](https://www.rust-lang.org/static/images/rust-logo-blk.svg)

- GC(garbage Collection)가 없다.
- 보안이 중요한 상황에서 신뢰할 수 있다.
- 안정성이 좋으며 다음으로부터 자유롭다.
  - Dangling pointer
  - Data race
  - Buffer overflow
  - Iterator invalidation
- 고차 프로그래밍, 타입 애너테이션, 조건부 컴파일 그리고 암묵적 반환 등이 가능하다.

```text
# 러스트의 기본적인 프로젝트 구조

chapter_1/hello
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
└── target
```

```rust
// Hello, World!
fn greet_world() {
    println!("Hello, world!");
    let southern_germany = "GrüB Gott!";
    let korean = "안녕, 세상아!";
    let regions = [southern_germany, korean];

    for region in regions.iter() {
        println!("{}", &region);
    }

}

fn main() {
    greet_world();
}
```

### Rust 도구

- cargo: 전체 크레이트(패키지) 관리
  - `cargo new`
  - `cargo build`
  - `cargo run`
  - `cargo doc`
- rustup: 툴체인 및 러스트 관리
- rustc: 러서트 소스 코드의 컴파일을 관리

### Rust 경구

- **Empowering everyone(모두에게 권한을 부여한다)**
  - 능력이나 배경에 관계없이 모든 프로그래머의 참여를 환영한다.
- **Blazingly fast(엄청나게 빠르다)**
  - 러스트는 빠른 프로그래밍 언어다.
- **Fearless concurrency(두려움 없는 동시성)**
  - 러스트는 프로그래머를 괴롭혔던 동종 언어의 모든 오류로부터 여러분을 해방시킨다.
- **No Rust 2.0(러스트 2.0은 없다)**
  - 오늘 작성된 러스트 코드는 항상 미래의 러스트 컴파일러에서도 정상적으로 컴파일된다.
- **Zero-cost abstractions(무비용 추상화)**
  - 러스트를 통해 얻을 수 있는 기능들은 런타임 비용이 발생하지 않는다.

</details>

## 🏆 소감

평소 다루는 자바스크립트나 파이썬과 달리 로우 레벨의 언어를 학습하는 것은 보다 많은 것을 알아야 했다. 단순히 언어의 문법과 특징을 익히는 것을 넘어 원론적인 컴퓨터 이론에 대해 다시 한 번 깊이 고민하면서 공부할 기회로 삼을 수 있을 것이라 기대한다.
