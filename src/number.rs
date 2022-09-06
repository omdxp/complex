//! complex number
//!
//! This module contains the complex number type and its associated functions.
//!
//! # Examples
//!
//! ```
//! use std::f64::consts::PI;
//!
//! use xcomplex::number::Complex;
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
//! assert_eq!(c.arg(), 1.1071487177940904);
//!
//! assert_eq!(c.conj(), Complex::new(1.0, -2.0));
//! assert_eq!(c.exp(), Complex::new(-1.1312043837568135, 2.4717266720048188));
//! assert_eq!(c.powf(PI), Complex::new(-11.826467250438055, -4.138504280918663));
//! assert_eq!(c.powi(2), Complex::new(-3.0, 4.000000000000002));
//! assert_eq!(c.powc(Complex::new(2.0, 3.0)), Complex::new(-7.041080062171126, -7.259799175444256));
//! assert_eq!(c.ln(), Complex::new(0.8047189562170503, 1.1071487177940904));
//! assert_eq!(c.sqrt(), Complex::new(1.272019649514069, 0.7861513777574233));
//! ```
//!
//! # References
//!
//! * [Complex numbers](https://en.wikipedia.org/wiki/Complex_number)

use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    /// Create a new complex number.
    /// # Examples
    /// ```
    /// use xcomplex::number::Complex;
    /// let c = Complex::new(1.0, 2.0);
    /// ```
    /// # Panics
    /// This function does not panic.
    /// # Safety
    /// This function is safe.
    /// # Aborts
    /// This function does not abort.
    /// # Undefined Behavior
    /// This function does not cause undefined behavior.
    /// # Implementation Details
    /// This function is implemented as a simple wrapper around the `Complex` struct.
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// Return the norm of the complex number.
    /// # Examples
    /// ```
    /// use xcomplex::number::Complex;
    /// let c = Complex::new(1.0, 2.0);
    /// assert_eq!(c.norm(), 2.23606797749979);
    /// ```
    /// # Panics
    /// This function does not panic.
    /// # Safety
    /// This function is safe.
    /// # Aborts
    /// This function does not abort.
    /// # Undefined Behavior
    /// This function does not cause undefined behavior.
    pub fn norm(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    /// Return the argument of the complex number.
    /// # Examples
    /// ```
    /// use xcomplex::number::Complex;
    /// let c = Complex::new(1.0, 2.0);
    /// assert_eq!(c.arg(), 1.1071487177940904);
    /// ```
    /// # Panics
    /// This function does not panic.
    /// # Safety
    /// This function is safe.
    /// # Aborts
    /// This function does not abort.
    /// # Undefined Behavior
    /// This function does not cause undefined behavior.
    pub fn arg(&self) -> f64 {
        self.im.atan2(self.re)
    }

    /// Return the complex conjugate of the complex number.
    /// # Examples
    /// ```
    /// use xcomplex::number::Complex;
    /// let c = Complex::new(1.0, 2.0);
    /// assert_eq!(c.conj(), Complex::new(1.0, -2.0));
    /// ```
    /// # Panics
    /// This function does not panic.
    /// # Safety
    /// This function is safe.
    /// # Aborts
    /// This function does not abort.
    /// # Undefined Behavior
    /// This function does not cause undefined behavior.
    pub fn conj(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    /// Return the exponential of the complex number.
    /// # Examples
    /// ```
    /// use xcomplex::number::Complex;
    /// let c = Complex::new(1.0, 2.0);
    /// assert_eq!(c.exp(), Complex::new(-1.1312043837568135, 2.4717266720048188));
    /// ```
    /// # Panics
    /// This function does not panic.
    /// # Safety
    /// This function is safe.
    /// # Aborts
    /// This function does not abort.
    /// # Undefined Behavior
    /// This function does not cause undefined behavior.
    pub fn exp(&self) -> Self {
        let e = self.re.exp();
        Self {
            re: e * self.im.cos(),
            im: e * self.im.sin(),
        }
    }

    /// Return the complex number raised to the power of a real number.
    /// # Examples
    /// ```
    /// use xcomplex::number::Complex;
    /// use std::f64::consts::PI;
    /// let c = Complex::new(1.0, 2.0);
    /// assert_eq!(c.powf(PI), Complex::new(-11.826467250438055, -4.138504280918663));
    /// ```
    /// # Panics
    /// This function does not panic.
    /// # Safety
    /// This function is safe.
    /// # Aborts
    /// This function does not abort.
    /// # Undefined Behavior
    /// This function does not cause undefined behavior.
    pub fn powf(&self, n: f64) -> Self {
        let r = self.norm();
        let theta = self.arg();
        let e = r.powf(n);
        Self {
            re: e * (theta * n).cos(),
            im: e * (theta * n).sin(),
        }
    }

    /// Return the complex number raised to the power of an integer.
    /// # Examples
    /// ```
    /// use xcomplex::number::Complex;
    /// let c = Complex::new(1.0, 2.0);
    /// assert_eq!(c.powi(2), Complex::new(-3.0, 4.000000000000002));
    /// ```
    /// # Panics
    /// This function does not panic.
    /// # Safety
    /// This function is safe.
    /// # Aborts
    /// This function does not abort.
    /// # Undefined Behavior
    /// This function does not cause undefined behavior.
    pub fn powi(&self, n: i32) -> Self {
        let r = self.norm();
        let theta = self.arg();
        let e = r.powi(n);
        Self {
            re: e * (theta * n as f64).cos(),
            im: e * (theta * n as f64).sin(),
        }
    }

    /// Return the natural logarithm of the complex number.
    /// # Examples
    /// ```
    /// use xcomplex::number::Complex;
    /// let c = Complex::new(1.0, 2.0);
    /// assert_eq!(c.ln(), Complex::new(0.8047189562170503, 1.1071487177940904));
    /// ```
    /// # Panics
    /// This function does not panic.
    /// # Safety
    /// This function is safe.
    /// # Aborts
    /// This function does not abort.
    /// # Undefined Behavior
    /// This function does not cause undefined behavior.
    pub fn ln(&self) -> Self {
        Self {
            re: self.norm().ln(),
            im: self.arg(),
        }
    }

    /// Return the complex number raised to the power of another complex number.
    /// # Examples
    /// ```
    /// use xcomplex::number::Complex;
    /// let c = Complex::new(1.0, 2.0);
    /// assert_eq!(c.powc(Complex::new(2.0, 3.0)), Complex::new(-7.041080062171126, -7.259799175444256));
    /// ```
    /// # Panics
    /// This function does not panic.
    /// # Safety
    /// This function is safe.
    /// # Aborts
    /// This function does not abort.
    /// # Undefined Behavior
    /// This function does not cause undefined behavior.
    pub fn powc(&self, n: Self) -> Self {
        self.ln() * n.exp()
    }

    /// Return the square root of the complex number.
    /// # Examples
    /// ```
    /// use xcomplex::number::Complex;
    /// let c = Complex::new(1.0, 2.0);
    /// assert_eq!(c.sqrt(), Complex::new(1.272019649514069, 0.7861513777574233));
    /// ```
    /// # Panics
    /// This function does not panic.
    /// # Safety
    /// This function is safe.
    /// # Aborts
    /// This function does not abort.
    /// # Undefined Behavior
    /// This function does not cause undefined behavior.
    pub fn sqrt(&self) -> Self {
        let r = self.norm();
        let theta = self.arg();
        Self {
            re: r.sqrt() * (theta / 2.0).cos(),
            im: r.sqrt() * (theta / 2.0).sin(),
        }
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

impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        self.re == other.re && self.im == other.im
    }
    fn ne(&self, other: &Self) -> bool {
        self.re != other.re || self.im != other.im
    }
}
