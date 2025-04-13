# 🦀 Rust Learnings Summary — Translating ORFs

## ✅ Overview

While building `translate()` and related helpers for a TransDecoder-like tool in Rust, you explored core language features hands-on. Here's a breakdown of what you learned — and the key pitfalls you ran into and solved.

---

## 📚 Core Rust Concepts You Now Understand

### 🧵 `String` vs `str`

| Type     | Owned? | Heap? | Growable? | Typical Form | Usage                                  |
|----------|--------|--------|------------|---------------|----------------------------------------|
| `String` | ✅ Yes | ✅ Yes | ✅ Yes     | `String`      | Owns text, mutable, heap-allocated     |
| `str`    | ❌ No  | ❌ No  | ❌ No      | `&str`        | A borrowed slice of a string           |

- Convert `&str` → `String` with `.to_string()` or `.into()`
- Convert `String` → `&str` with `.as_str()` or `&s[..]`
- `String::push(char)` appends a character, `.push_str(&str)` appends a string slice

✅ You now know **when to use each**, and how to pass them between functions safely.

---

### 📦 HashMap Behavior

- Built with `HashMap::new()` or `.collect()` from a literal array
- Created a full codon table: `HashMap<&'static str, char>`

#### Access methods:

| Pattern               | Returns     | Safe? | Notes                           |
|-----------------------|-------------|-------|---------------------------------|
| `map.get("key")`      | `Option<&V>`| ✅    | Safe and preferred              |
| `map["key"]`          | `V`         | ❌    | Panics if key not found         |
| `.unwrap_or(&'X')`    | `&V`        | ✅    | Fallback default                |

✅ You used `.get()` for safety and indexing `[]` for known-good lookups.

---

### 🔁 Iterators: `.iter()`, `.cloned()`, `.into_iter()`

| Method         | Yields         | Ownership | Use Case                            |
|----------------|----------------|-----------|-------------------------------------|
| `.iter()`      | `&T`           | Borrowed  | Safe iteration, preserves original  |
| `.cloned()`    | `T`            | Owned     | When you want copies from `.iter()` |
| `.into_iter()` | `T`            | Owned     | Consumes the container              |

- You used `.into_iter()` + `.collect()` to make a codon table from an array
- You learned that `.iter().cloned()` is useful when you want to **keep the original**

✅ You now get why Rust separates borrowing from moving and cloning.

---

### 📐 Dereferencing with `*`

- `.get()` returns `&T`, so you must `*value` to get the actual value
- Indexing `map["key"]` gives `T` directly — no need to deref

✅ You wrote `.push(*aa)` correctly when working with `&char`, and skipped `*` when not needed

---
### 🔡 Lifetimes: `'static`

- `'static` is the lifetime for string literals and values that live forever
- Used in: `HashMap<&'static str, char>`
- ✅ You now understand when to explicitly declare lifetimes

---

### ✂️ Slicing: `[..]` and `as_str()`

- `&my_string[..]` slices a `String` into a `&str`
- Equivalent to `.as_str()`
- Used to convert codon `String` to `&str` for lookup
- ✅ You used both correctly in your codon lookups

---

### 🔂 `.windows()` vs `.chunks()`

| Method        | Behavior               | Good For             |
|---------------|------------------------|-----------------------|
| `.windows(3)` | Overlapping 3-mers     | k-mer search          |
| `.chunks(3)`  | Non-overlapping groups | Codon translation ✅  |

- `.windows(3)` caused `"M*EKNM*"` output due to overlap
- ✅ Switched to `.chunks(3)` and skipped final chunk if `<3`
- ✅ You debugged this perfectly with a test case

---

### 🛠️ `.collect()` + `.iter()`

- Used `.collect()` to gather values from an iterator
- `.iter().collect::<String>()` turned a `&[char]` into a `String`
- Learned that:
  - `.iter()` = borrow
  - `.collect()` = build owned structure
  - `.cloned()` helps if values are `Copy`/`Clone`
- ✅ You used turbofish (`::<String>`) when Rust needed type hints

---

### 🐟 The Turbofish (`::<T>`)

- Used with `.collect()` or `.parse()` when Rust needs help inferring types
- Syntax: `.collect::<Vec<_>>()` or `.collect::<String>()`
- ✅ You learned to read errors like `cannot infer type for type parameter` as a sign to use the turbofish

---

### 🧪 Writing Unit Tests

- `#[cfg(test)] mod tests` keeps test logic in the same file
- Use `use super::*;` to access parent module’s functions
- Used `assert_eq!`, `assert!`, and `cargo test -- --nocapture`
- ✅ You now write idiomatic inline tests with great coverage

---

### 📊 Debug printing

- Rust doesn't let you `println!("{}", HashMap)` — no `Display` impl
- ✅ Used `println!("{:?}", ...)` and `{:#?}` for pretty output

---

### 🧱 `Vec::new()` and Accumulators

- Used `let mut protein = String::new();` as an accumulator for results
- Pushed to the string with `.push(char)`
- ✅ You understood and used accumulator-style logic with idiomatic syntax

---

### 📚 `use super::*;` and modules

- Inside a `#[cfg(test)] mod tests`, use `super::*` to bring in the parent file's definitions
- `mod mymod;` pulls in another file (like `types.rs`) from the same crate
- ✅ You used this to organize code and tests logically

---

### ✨ Bonus Ergonomics

- `.into()` to coerce a `&'static str` into a `String`
- `.starts_with()` and `.ends_with()` to inspect strings
- `String::new()` vs `vec![]` depending on accumulator type
