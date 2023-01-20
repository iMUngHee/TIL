## 🔥 목표

팀원들과 모각소 목표 설정 및 러스트 기초 학습

## 👨‍💻 활동

<러스트 프로그래밍> 챕터 1~2 학습 및 내용 정리

### 공부내용

<details>
<summary><b>Ch 1. 러스트 소개</b></summary>

</details>

![rust](https://www.rust-lang.org/static/images/rust-logo-blk.svg)

- GC(garbage Collection)가 없다.
- 보안이 중요한 상황에서 신뢰할 수 있다.

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

<details>
<summary><b>Ch 2. 러스트 언어의 기초</b></summary>
 
ㅇㅇ
</details>

## 🏆 소감

평소 다루는 자바스크립트나 파이썬과 달리 로우 레벨의 언어를 학습하는 것은 보다 많은 것을 알아야 했다. 단순히 언어의 문법과 특징을 익히는 것을 넘어 원론적인 컴퓨터 이론에 대해 다시 한 번 깊이 고민하면서 공부할 기회로 삼을 수 있을 것이라 기대한다.
