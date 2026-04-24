
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub mod solution {
    use std::fmt;
    use std::ops::Add;
    use std::ops::AddAssign;
    use std::default::Default;
    use std::cmp::Ordering;
    use std::hash::Hasher;
    use std::hash::Hash;

    #[derive(Debug, PartialEq, Clone, Copy, /* Default, */)]
    pub struct ComplexNumber {
        pub real: f64,
        pub imag: f64,
    }

    #[derive(Debug, PartialEq)]
    pub enum ComplexNumberError {
        ImaginaryNotZero,
    }

    impl ComplexNumber {

        pub fn new(real: f64, imag: f64) -> Self {
            Self{real, imag}
        }

        pub fn real(&self) -> f64 {
            self.real
        }

        pub fn imag(&self) -> f64 {
            self.imag
        }

        pub fn from_real(real: f64) -> Self {
            Self{real, imag: 0.0}
        }

        pub fn to_tuple(&self) -> (f64, f64) {
            (self.real, self.imag)
        }
    }

    impl fmt::Display for ComplexNumber {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.imag < 0.0  {
                write!(f, "{} - {}i", self.real, self.imag.abs())
            } else {
                write!(f, "{} + {}i", self.real, self.imag)
            }
        }
    }

    impl Add for ComplexNumber {
        type Output = Self;
        fn add(self, rhs: ComplexNumber) -> Self {
            ComplexNumber {
                real: self.real + rhs.real,
                imag: self.imag + rhs.imag,
            }
        }
    }

    impl Add<f64> for ComplexNumber {
        type Output = Self;
        fn add(self, rhs: f64) -> Self {
            ComplexNumber {
                real: self.real + rhs,
                imag: self.imag,
            }
        } 
    }

    impl AddAssign for ComplexNumber {
        fn add_assign(&mut self, rhs: ComplexNumber) {
            self.real += rhs.real;
            self.imag += rhs.imag;
        }
    }

    impl Add<&ComplexNumber> for ComplexNumber {
        type Output = Self;
        fn add(self, rhs: &ComplexNumber) -> Self {
            ComplexNumber {
                real: self.real + rhs.real,
                imag: self.imag + rhs.imag,
            }
        }
    }

    impl Add for &ComplexNumber {
        type Output = ComplexNumber;
        fn add(self, rhs: &ComplexNumber) -> ComplexNumber {
            ComplexNumber {
                real: self.real + rhs.real,
                imag: self.imag + rhs.imag,
            }
        }
    }

    impl Default for ComplexNumber {
        fn default() -> Self {
            Self {
                real: 0.0,
                imag: 0.0,
            }
        }
    }
    /* 
    // Commented when implementing TryInto
    impl Into<f64> for ComplexNumber {
        fn into(self) -> f64 {
            if self.imag != 0.0 {
                panic!("Cannot convert complex to real if imaginary part is not equal to zero.")
            }
            self.real
        }
    }
    */

    impl TryInto<f64> for ComplexNumber {
        type Error = ComplexNumberError;

        fn try_into(self) -> Result<f64, ComplexNumberError> {
            if self.imag != 0.0 {
                return Err(ComplexNumberError::ImaginaryNotZero);
            }
            Ok(self.real)
        }
    }
    /*
    impl TryFrom<f64> for ComplexNumber {
        type Error = ComplexNumberError;

        fn try_from(value: f64) -> Result<Self, ComplexNumberError> {
            let result = ComplexNumber {
                real: value,
                imag: 0.0,
            };
            Ok(result)
        }
    }
     */ 
    impl Into<ComplexNumber> for f64 {
        fn into(self) -> ComplexNumber {
            ComplexNumber {
                real: self,
                imag: 0.0,
            }
        }
    } 

    impl Ord for ComplexNumber {
        fn cmp(&self, other: &Self) -> Ordering {
            let a = self.real * self.real + self.imag * self.imag;
            let b = other.real * other.real + other.imag * other.imag;

            a.total_cmp(&b)
        }
    }

    impl PartialOrd for ComplexNumber {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Eq for ComplexNumber {}

    impl AsRef<f64> for ComplexNumber {
        fn as_ref(&self) -> &f64 {
            &self.real
        }
    }

    impl AsMut<f64> for ComplexNumber {
        fn as_mut(&mut self) -> &mut f64 {
            &mut self.real
        }
    }

    impl Hash for ComplexNumber {
        fn hash<H: Hasher>(&self, hasher: &mut H) {
            self.real.to_bits().hash(hasher);
            self.imag.to_bits().hash(hasher);
        }
    }
}
