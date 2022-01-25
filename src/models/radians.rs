use std::ops::{Deref, DerefMut};

use super::{Degrees, HALF_CIRCLE};

#[derive(Default, Clone, Copy)]
pub struct Radians(pub f64);

impl Deref for Radians {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Radians {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Degrees> for Radians {
    fn from(d: Degrees) -> Self {
        Radians((*d * std::f64::consts::PI) / *HALF_CIRCLE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deref() {
        let mut r = Radians(0.0);
        *r = 20.0;
        assert_eq!(*r, 20.0);
    }

    #[test]
    fn convert_from_degrees() {
        let d = Degrees(180.0);

        let r1: Radians = d.into();
        let r2 = Radians::from(d);

        assert_eq!(*r1, std::f64::consts::PI);
        assert_eq!(*r2, std::f64::consts::PI);
    }
}
