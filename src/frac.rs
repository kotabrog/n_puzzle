use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Copy)]
pub struct Frac {
    pub num: i32,
    pub den: u32,
}

impl Frac {
    pub fn new(num: i32, den: u32) -> Result<Frac> {
        if den == 0 {
            return Err(anyhow!("Error: Denominator is zero"));
        }
        Ok(Frac { num, den })
    }

    pub fn throw_num_zero(&self) -> Result<()> {
        if self.num == 0 {
            return Err(anyhow!("Error: Numerator is zero"));
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        if self.den == 1 {
            self.num.to_string()
        } else {
            format!("{}/{}", self.num, self.den)
        }
    }

    #[allow(dead_code)]
    pub fn to_float(&self) -> f64 {
        self.num as f64 / self.den as f64
    }

    pub fn reduction(&self) -> Frac {
        let mut num = self.num;
        let mut den = self.den;
        let mut i: u32 = 2;
        while i <= num.abs() as u32 && i <= den {
            if num % i as i32 == 0 && den % i == 0 {
                num /= i as i32;
                den /= i;
                i = 2;
            } else {
                i += 1;
            }
        }
        Frac { num, den }
    }
}

impl std::ops::Add for Frac {
    type Output = Frac;

    fn add(self, other: Frac) -> Frac {
        let num
            = self.num * other.den as i32 + self.den as i32 * other.num;
        let den = self.den * other.den;
        Frac::new(num, den).unwrap().reduction()
    }
}

impl std::ops::Sub for Frac {
    type Output = Frac;

    fn sub(self, other: Frac) -> Frac {
        let num
            = self.num * other.den as i32 - self.den as i32 * other.num;
        let den = self.den * other.den;
        Frac::new(num, den).unwrap().reduction()
    }
}

impl std::ops::Mul for Frac {
    type Output = Frac;

    fn mul(self, other: Frac) -> Frac {
        let num = self.num * other.num;
        let den = self.den * other.den;
        Frac::new(num, den).unwrap().reduction()
    }
}

impl std::ops::Div for Frac {
    type Output = Frac;

    fn div(self, other: Frac) -> Frac {
        let mut num = self.num * other.den as i32;
        if other.num < 0 {
            num *= -1;
        }
        let den = self.den * other.num.abs() as u32;
        Frac::new(num, den).unwrap().reduction()
    }
}

impl std::cmp::PartialEq for Frac {
    fn eq(&self, other: &Frac) -> bool {
        self.num * other.den as i32 == self.den as i32 * other.num
    }
}

impl std::cmp::Eq for Frac {}

impl std::cmp::PartialOrd for Frac {
    fn partial_cmp(&self, other: &Frac) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Frac {
    fn cmp(&self, other: &Frac) -> std::cmp::Ordering {
        (self.num * other.den as i32).cmp(&(self.den as i32 * other.num))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduction_normal() {
        let frac = Frac::new(2, 4).unwrap();
        assert_eq!(frac.reduction().to_string(), "1/2");
    }

    #[test]
    fn reduction_two_try() {
        let frac = Frac::new(6, 12).unwrap();
        assert_eq!(frac.reduction().to_string(), "1/2");
    }
}
