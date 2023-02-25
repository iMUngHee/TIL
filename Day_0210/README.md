# Rust Tutorial

## 🔥 목표

팀원들과 모각소 목표 설정 및 러스트 기초 학습

## 👨‍💻 활동

<러스트 프로그래밍> 챕터 3 학습 및 내용 정리

### 공부내용

<details>
<summary><b>Ch 3. 복합 데이터 타입</b></summary>

![rust](https://www.rust-lang.org/static/images/rust-logo-blk.svg)

### `struct`

```rust
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}
```

- `struct`를 사용하면 다른 타입들로 구성된 복합 타입 생성 가능
- 정의에는 필드와 필드의 관련 타입이 포함
- 일반적으로 편의 메서드를 사용해 구조체를 생성

### `impl`

```rust
impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        // ...
    }
}

fn main() {
    let mut f5 = File::new("5.txt");

    // ...
}
```

- 메서드를 정의하기 위해 `impl` 블록을 사용
- `new()` 메서드로 적절한 기본값을 가지는 객체를 생성

### `enum`

```rust
#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}
```

- 열거형은 러스트의 패턴 일치 기능과 함께 사용하면 읽기 쉬운 코드를 만드는 데 도움이 됨
- 구조체처럼 열거형은 `impl` 블록을 통해 메서드를 정의할 수 있음
- 열거형은 열것값에 데이터를 포함시켜 구조체 같은 성격을 띄게 하는 것도 가능

  ```rust
  enum Suit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
  }

  enum Card {
    King(Suit),
    Queen(Suit),
    Jack(Suit),
    Ace(Suit),
    Pip(Suit),
  }
  ```

### `trait`

```rust
#[derive(Debug)]
struct File;

trait Read {
    fn read(
      self: &Self,
      save_to: &mut Vec<u8>,
    ) -> Result<usize, String>;
}

impl Read for File {
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        Ok(0)
    }
}

```

- 다른 언어의 인터페이스, 프로토콜, 타입 클래스, 추상 기본 클래스 또는 계약 등과 비슷함
- 코드 재사용과 무비용 추상화에 도움
- 특정 타입의 트레이트에 붙는 `impl` 키워드와 대조

</details>

## 🏆 소감

객체에 관한 러스트의 특징은 다른 언어보다 많은 생각을 하게 한다. 많은 것을 할 수 있는 만큼 많은 것을 생각해야 했다. 그 중에 트레이트는 Swift의 프로토콜을 떠오르게 했다. 클래스가 없거나 비중이 적은 두 언어의 공통성은 흥미로웠다.
