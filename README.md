# fixed-fast

**fixed-fast** is a deterministic, high-precision, fixed-point arithmetic library for Rust using `i128`. It is designed for applications that require consistent numerical behavior across platforms — such as financial systems, smart contracts, or simulation engines.

---

## ✨ Features

- ✅ Arbitrary decimal precision via `FixedDecimal<const DECIMALS: u32>`
- ✅ Fully deterministic operations (no `f64`, no floating-point errors)
- ✅ Fast, accurate fixed-point implementations of:
  - Natural logarithm `ln(x)`
  - Exponentiation `exp(x)`, `x^y`, `x^n`
  - Square root `sqrt(x)`
- ✅ Runtime lookup table - blazing fast
- ✅ Pure `i128` math for safe, efficient operations

---

## 🚀 Example

```rust
use fixed_decimal::FixedDecimal;

type D10 = FixedDecimal<10>; // 10 decimal places

let x = D10::from_str("1.5");          // 1.5