//! complex number
//!
//! This module contains the complex number type and its associated functions.
//!
//! # Examples
//!
//! ```
//! use std::f64::consts::PI;
//! use std::f64::consts::FRAC_PI_2;
//!
//! use complex::number::Complex;
//!
//! let c = Complex::new(1.0, 2.0);
//! let d = Complex::new(3.0, 4.0);
//!
//! assert_eq!(c + d, Complex::new(4.0, 6.0));
//! assert_eq!(c * d, Complex::new(-5.0, 10.0));
//! assert_eq!(c - d, Complex::new(-2.0, -2.0));
//! assert_eq!(c / d, Complex::new(0.44, 0.08));
//!
//! assert_eq!(c.norm(), 2.23606797749979);
//! assert_eq!(c.arg(), FRAC_PI_2);
//!
//! assert_eq!(c.conj(), Complex::new(1.0, -2.0));
//! assert_eq!(c.exp(), Complex::new(2.718281828459045, 5.43656365691809));
//! assert_eq!(c.powf(PI), Complex::new(-4.810477380965351, -9.621336707931702));
//! assert_eq!(c.powi(2), Complex::new(-3.0, 4.0));
//! assert_eq!(c.sqrt(), Complex::new(1.272019649514069, 0.7861513777574233));
//! ```
//!
//! # References
//!
//! * [Complex numbers](https://en.wikipedia.org/wiki/Complex_number)

use std::ops::{Add, Div, Mul, Sub};
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }
}

impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let d = rhs.re * rhs.re + rhs.im * rhs.im;
        Self {
            re: (self.re * rhs.re + self.im * rhs.im) / d,
            im: (self.im * rhs.re - self.re * rhs.im) / d,
        }
    }
}
