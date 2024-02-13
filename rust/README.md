# Learning Rust REST API Project

 Rust is a systems programming language known for its focus on safety, speed, and concurrency. It achieves memory safety without a garbage collector through its innovative ownership system, making it suitable for building fast and reliable software, especially in areas like systems programming, embedded systems, and web development.

## Getting Running

### Installation/Prerequisites

  Please install Rust/Cargo on your local machine directly from [rust](https://www.rust-lang.org/tools/install) 

  Running

  ```bash
  cargo --version
  rustc --version

  ``` 

 In your terminal should return the latest version of rust/cargo.

### Running Project

  Please run the following commands after installing rust


  ```bash
  cd rust
  cargo run
  ```

### Language Project layout resources

  - There is not much on Rust/Axum-specific project layouts. I was able to find an example from the Tokio teams [Axum example page](https://github.com/tokio-rs/axum/tree/main/axum/src)

### Language Specifics
  
  - [Rust](https://doc.rust-lang.org/book/) The book is always a great place to start learning rust.
  - [axum](https://docs.rs/axum/latest/axum/) docs 

#### Summary

  Rust is a modern systems programming language developed by Mozilla, designed to provide high performance and memory safety without sacrificing developer productivity. Its key features include a powerful type system with strong static guarantees, a unique ownership model that prevents common memory-related bugs at compile time, and built-in support for concurrent programming through lightweight threads called "tasks".

  Pros:

- Safety: Rust's ownership system and strict compiler checks ensure memory safety and prevent common bugs like null pointer dereferences and data races, leading to more reliable software.
- Performance: Rust's zero-cost abstractions and control over memory management enable developers to write code that is as fast and efficient as C and C++.
- Concurrency: Rust's lightweight tasks and built-in concurrency primitives make it easy to write scalable and high-performance concurrent applications, with thread safety guaranteed at compile time.
- Community: Rust has a vibrant and growing community, with extensive documentation, libraries, and tools available to support developers.

Cons:

  - Learning Curve: Rust has a steep learning curve, especially for developers coming from higher-level languages, due to its unique ownership and borrowing concepts.
  - Compilation Times: Rust's powerful type system and extensive compiler checks can result in longer compile times, particularly for large projects.
  - Immaturity: While Rust has been stable since version 1.0, its ecosystem and libraries may still be less mature compared to more established languages like C++.
  - Strictness: Rust's strict rules around ownership and borrowing can sometimes feel restrictive, requiring developers to rethink their approach to memory management and data sharing.
  
  Overall, Rust is a promising language that offers a compelling combination of safety, performance, and concurrency, making it well-suited for systems programming, embedded development, and building high-performance applications. However, it may not be the best choice for all projects, especially those with strict time constraints or where familiarity with other languages is preferred.
