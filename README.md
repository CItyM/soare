
# Struct of Arrays Derive

`soare` is a Rust procedural macro library that generates a "Struct of Arrays" (SoA) counterpart for your structs. This macro is helpful for optimizing memory layouts in performance-critical applications, such as games or simulations.

## Features

- Automatically creates an SoA type for any struct with named fields.
- Provides methods to:
  - Convert a `Vec<YourStruct>` into a `YourStructSoA`.
  - Access the individual arrays for each field.
  - Push, pop, and remove elements efficiently.
- Optimized memory layout for improved cache performance.

## Installation

Add `soare` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
soare = "0.1.0"
```

## Usage

Annotate your struct with `#[derive(StructOfArrays)]` to generate its SoA counterpart:

```rust
use soare::StructOfArrays;

#[derive(StructOfArrays)]
struct MyStruct {
    x: i32,
    y: f64,
    name: String,
}
```

This will generate the following SoA struct:

```rust
pub struct MyStructSoA {
    len: usize,
    x: Vec<i32>,
    y: Vec<f64>,
    name: Vec<String>,
}
```

### Example

```rust
use soare::StructOfArrays;

#[derive(StructOfArrays)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let points = vec![
        Point { x: 1, y: 2 },
        Point { x: 3, y: 4 },
    ];

    let mut soa = PointSoA::from_instances(points);

    println!("Length: {}", soa.len());
    println!("X coordinates: {:?}", soa.x());
    println!("Y coordinates: {:?}", soa.y());

    soa.push(Point { x: 5, y: 6 });
    println!("After push, length: {}", soa.len());

    let removed = soa.remove(0);
    println!("Removed: x={}, y={}", removed.x, removed.y);
}
```

## Generated API

For a struct `YourStruct`, the macro generates the following:

- A struct named `YourStructSoA` with:
  - A `len` field representing the number of elements.
  - A `Vec` for each field in `YourStruct`.
- Methods:
  - `from_instances(instances: Vec<YourStruct>) -> YourStructSoA`
  - `len(&self) -> usize`
  - `is_empty(&self) -> bool`
  - `push(&mut self, value: YourStruct)`
  - `pop(&mut self) -> Option<YourStruct>`
  - `remove(&mut self, index: usize) -> YourStruct`
  - `swap_remove(&mut self, index: usize) -> YourStruct`
  - Field accessors (e.g., `fn field_name(&self) -> &[FieldType]`).

## Limitations

- Only works with structs that have named fields.
- Does not support enums or tuple structs.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
