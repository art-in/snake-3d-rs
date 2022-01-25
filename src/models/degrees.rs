use crate::helpers::ranges::normalize_by_circular_range;

use super::Radians;
use std::ops::{Deref, DerefMut};

pub const HALF_CIRCLE: Degrees = Degrees(180.0);

// create separate type instead of type alias, to avoid accidental use of
// degrees where radians expected and vise versa
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Degrees(pub f64);

impl Deref for Degrees {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Degrees {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Radians> for Degrees {
    fn from(r: Radians) -> Self {
        Degrees((*r * *HALF_CIRCLE) / std::f64::consts::PI)
    }
}

impl Degrees {
    pub fn normalize(&self) -> Self {
        Degrees(normalize_by_circular_range(self.0, -180.0, 181.0))
    }

    pub fn round(&self) -> Self {
        Degrees(self.0.round())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deref() {
        let mut d = Degrees(0.0);
        *d = 20.0;
        assert_eq!(*d, 20.0);
    }

    #[test]
    fn convert_from_radians() {
        let r = Radians(2.0 * std::f64::consts::PI);

        let d1: Degrees = r.into();
        let d2 = Degrees::from(r);

        assert_eq!(*d1, 360.0);
        assert_eq!(*d2, 360.0);
    }
}
