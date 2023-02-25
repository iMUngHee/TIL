# Rust Tutorial

## 🔥 목표

팀원들과 모각소 목표 설정 및 러스트 기초 학습

## 👨‍💻 활동

<러스트 프로그래밍> 활용: Rocket.rs를 활용한 백 엔드 구현

### 공부내용

<details>
<summary><b>Rocket를 활용한 백 엔드 구현</b></summary>

![rust](https://www.rust-lang.org/static/images/rust-logo-blk.svg)

### Rocket

<img width="128" alt="image" src="https://user-images.githubusercontent.com/88721306/221363455-274b1490-0417-4817-b80a-8e4cb18a21e3.png">

- Rust의 Back-end 프레임워크
- 다른 언어의 웹 프레임워크와 사용이 비슷하여 입문에 용이
- Rust의 장점을 적극 반영하여 안전하고 확장성 높은 개발 가능

<img width="1177" alt="rocket_example" src="https://user-images.githubusercontent.com/88721306/221363494-94ed8f3a-d6a6-4ea5-af62-3c755ff93fe0.png">

### 구현 API

<img width="1021" alt="API" src="https://user-images.githubusercontent.com/88721306/221363469-a662857d-a611-451d-bc60-39205970f8c8.png">

```text
프로젝트 구조
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── Rocket.toml
├── db
│   ├── db.sqlite
│   └── migrations
│       └── 20230223230641_initial-tables.sql
├── sqlx-data.json
└── src
    └── main.rs
```

</details>

## 🏆 소감

파이썬의 FastAPI와 비슷한 느낌을 받았다. DI 등이 명확하게 기능으로 있지는 않는 것 같았으나, 공식 문서에서 싱글톤 패턴을 지원한다는 것으로 보아 생각보다 많은 기능을 지원하는 듯 하다. 가장 어려웠던 점은 데이터베이스와 연결하는 과정이었으며, 타입을 알아내는데 많은 시간이 소요되었다.
