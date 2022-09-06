# complex

A complex number library for Rust.

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
xcomplex = "0.1.0"
```

Or run:

```sh
cargo add xcomplex
```

## Examples

```rs
use std::f64::consts::PI;
use xcomplex::number::Complex;
let c = Complex::new(1.0, 2.0);
let d = Complex::new(3.0, 4.0);
assert_eq!(c + d, Complex::new(4.0, 6.0));
assert_eq!(c * d, Complex::new(-5.0, 10.0));
assert_eq!(c - d, Complex::new(-2.0, -2.0));
assert_eq!(c / d, Complex::new(0.44, 0.08));
assert_eq!(c.norm(), 2.23606797749979);
assert_eq!(c.arg(), 1.1071487177940904);
assert_eq!(c.conj(), Complex::new(1.0, -2.0));
assert_eq!(c.exp(), Complex::new(-1.1312043837568135, 2.4717266720048188));
assert_eq!(c.powf(PI), Complex::new(-11.826467250438055, -4.138504280918663));
assert_eq!(c.powi(2), Complex::new(-3.0, 4.000000000000002));
assert_eq!(c.powc(Complex::new(2.0, 3.0)), Complex::new(-7.041080062171126, -7.259799175444256));
assert_eq!(c.ln(), Complex::new(0.8047189562170503, 1.1071487177940904));
assert_eq!(c.sqrt(), Complex::new(1.272019649514069, 0.7861513777574233));
```

## License

MIT License
