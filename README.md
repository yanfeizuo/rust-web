# rust-web

## Work3

- run result
![image](https://raw.githubusercontent.com/yanfeizuo/rust-web/main/run_result.png)

## Work4
### No.1
- code segment
```rust
fn main() {
  let red = Traffic::Red(30);
  println!("Red traffic light duration is {}", red.duration());
  let green = Traffic::Green(40);
  println!("Green traffic light duration is {}", green.duration());
  let yellow = Traffic::Yellow(10);
  println!("Yellow traffic light duration is {}", yellow.duration());
}

enum Traffic {
  Red(u8),
  Green(u8),
  Yellow(u8),
}

trait Time {
  fn duration(&self) -> &u8;
}

impl Time for Traffic {
  fn duration(&self) -> &u8 {
    match self {
        Traffic::Red(t) => t,
        Traffic::Green(t) => t,
        Traffic::Yellow(t) => t,
    }
  }
}
```
- run result
![image](https://raw.githubusercontent.com/yanfeizuo/rust-web/main/work4-1.png)

### No.2
- code segment
```rust
use std::u32::MAX;
fn main() {
  let a: [u32; 3] = [1, 3, 8];
  let sum = sum_u32(&a);
  match sum {
      Some(s) => {
          println!("sum is {}", s);
      }
      None => {
          println!("None")
      }
  }
}

fn sum_u32(arr: &[u32]) -> Option<u32> {
  let mut sum: u32 = 0;
  for i in arr.iter() {
      sum += i;
  }
  if sum < MAX {
      Some(sum)
  } else {
      None
  }
}

```
- run result
![image](https://raw.githubusercontent.com/yanfeizuo/rust-web/main/work4-2.png)

### No.3
- code segment
```rust
fn main() {
    let rect = Rectangle {
        width: 5,
        height: 8,
    };
    calc_area(rect)
}

trait Area {
    fn area(&self) -> u32;
}

struct Rectangle<U32> {
    width: U32,
    height: U32,
}

impl Area for Rectangle<u32> {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn calc_area(item: impl Area) {
    let area = item.area();
    println!("Area is {}", area);
}

```
- run result
![image](https://raw.githubusercontent.com/yanfeizuo/rust-web/main/work4-3.png)