---
marp: true
theme: uncover
---

# Performance in Rust and Beyond

-- Swarnim Arun
-- Rust Engineeer!?

---

# Who am I?

- Rust Engineer
- Compilers, Systems and Games Developer
- Write useless things in Rust, Zig, C++, Typescript, Haskell, Swift, Dart and similar languages
- Every so often I write libraries to help me write better software or take part in GameJams

---

# Slides

Link: [swarnimarun.github.com/rust-perf-101](https://swarnimarun.github.io/rust-perf-101)

![qr-code-to-the-above-url](./qrcode.png)

---

# Small things can bring big performance wins

Not all small changes are small in terms of performance.

---

# Know thy code and tools

Often people don't really know or understand what the cost of the code they are writing is, and this leads to badly written code that could have been far more optimized.

---

# Abstractions are never really zero cost,

Further they may also make understanding the performance cost harder because of assumptions one might make about a programmming language compiler optimizations.

---

# Benchmarks are worse than profiling. 

---

# Benchmarks are better than assumptions.

---

# Performance needs to be tailored as per usage, and environment

For example a web app may not care about compute performance of the code but rather the latency or in case of serverless the cost of cold starts.

---

# Think about performance, while writing code.

Consicious thinking about performance and understanding are the most impactful things you can do to increase the performance of your code in general.

--- 

# Let's write some code.

```rust
fn main() {
    let mut edit_value_in_place = vec![1, 3, 5];
    let move_value = vec![1, 3, 5];

    // so which one is faster?
    edit_in_place(&mut edit_value_in_place);
    let move_value = use_move(move_value);
}
```

---

# What about some abstraction.

```rust
fn main() {
    let s1 = String::from("this is a string");
    let s2 = String::from("smallstring");

    // so which one is faster?
    // or moving
    let s3: String = [s1, s2].concat();
    // mutating the root
    let s3: String = {
        let mut s1 = s1;
        s1.push_str(&s2);
        s1
    };
}
```

---

# Thanks

- twitter.com/swarnimarun
- github.com/swarnimarun
