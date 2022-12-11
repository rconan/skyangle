use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, Copy)]
pub enum SkyAngle<T: Conversion<T>> {
    Radian(T),
    Degree(T),
    Arcminute(T),
    Arcsecond(T),
    MilliArcsec(T),
}
impl<T: Conversion<T> + Display> Display for SkyAngle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkyAngle::Radian(val) => val.fmt(f),
            SkyAngle::Degree(val) => val.fmt(f),
            SkyAngle::Arcminute(val) => val.fmt(f),
            SkyAngle::Arcsecond(val) => val.fmt(f),
            SkyAngle::MilliArcsec(val) => val.fmt(f),
        }
    }
}
impl<T: Conversion<T>> SkyAngle<T> {
    pub fn to_radians(self) -> T {
        match self {
            Self::Radian(val) => val,
            Self::Degree(val) => T::from_degree(val),
            Self::Arcminute(val) => T::from_arcmin(val),
            Self::Arcsecond(val) => T::from_arcsec(val),
            Self::MilliArcsec(val) => T::from_mas(val),
        }
    }
    pub fn into_degree(self) -> Self {
        SkyAngle::Degree(self.to_radians().to_degree())
    }
    pub fn into_arcmin(self) -> Self {
        SkyAngle::Arcminute(self.to_radians().to_arcmin())
    }
    pub fn into_arcsec(self) -> Self {
        SkyAngle::Arcsecond(self.to_radians().to_arcsec())
    }
    pub fn into_mas(self) -> Self {
        SkyAngle::MilliArcsec(self.to_radians().to_mas())
    }
}

impl<T> Add for SkyAngle<T>
where
    T: Conversion<T> + Add<Output = T>,
{
    type Output = T;

    fn add(self, rhs: Self) -> Self::Output {
        self.to_radians() + rhs.to_radians()
    }
}
impl<T> Sub for SkyAngle<T>
where
    T: Conversion<T> + Sub<Output = T>,
{
    type Output = T;

    fn sub(self, rhs: Self) -> Self::Output {
        self.to_radians() - rhs.to_radians()
    }
}
impl<T> Div for SkyAngle<T>
where
    T: Conversion<T> + Div<Output = T>,
{
    type Output = T;

    fn div(self, rhs: Self) -> Self::Output {
        self.to_radians() / rhs.to_radians()
    }
}
impl<T> Div<T> for SkyAngle<T>
where
    T: Conversion<T> + Div<Output = T>,
{
    type Output = T;

    fn div(self, rhs: T) -> Self::Output {
        self.to_radians() / rhs
    }
}
impl<T> Mul<T> for SkyAngle<T>
where
    T: Conversion<T> + Mul<Output = T>,
{
    type Output = T;

    fn mul(self, rhs: T) -> Self::Output {
        self.to_radians() * rhs
    }
}

/// Conversion between angle units
pub trait Conversion<T> {
    fn from_degree(self) -> T;
    fn from_arcmin(self) -> T;
    fn from_arcsec(self) -> T;
    fn from_mas(self) -> T;
    fn to_degree(self) -> T;
    fn to_arcmin(self) -> T;
    fn to_arcsec(self) -> T;
    fn to_mas(self) -> T;
}
macro_rules! impl_conversion {
    ($($name:ty),+) => {
        $(impl Conversion<$name> for $name {
            /// Converts angle in arcminute to radian
            fn from_degree(self) -> $name {
                self.to_radians()
            }            /// Converts angle in arcminute to radian
            fn from_arcmin(self) -> $name {
                self.to_radians() / 60.
            }
            /// Converts angle in arcsecond to radian
            fn from_arcsec(self) -> $name {
                self.from_arcmin() / 60.
            }
            /// Converts angle in milli-arcsecond to radian
            fn from_mas(self) -> $name {
                self.from_arcsec() * 1e-3
            }
            fn to_degree(self) -> $name {
                self.to_degrees()
            }            /// Converts angle in radian to arcminute
            fn to_arcmin(self) -> $name {
                60.0 * self.to_degrees()
            }
            /// Converts angle in radian to arcsecond
            fn to_arcsec(self) -> $name {
                60.0 * self.to_arcmin()
            }
            /// Converts angle in radian to mill-arcsecond
            fn to_mas(self) -> $name {
                1e3 * self.to_arcsec()
            }
        })+
        $(impl Conversion<Vec<$name>> for Vec<$name> {
    fn from_degree(self) -> Vec<$name> {
        self.into_iter().map(|x| x.from_degree()).collect()
    }
    fn from_arcmin(self) -> Vec<$name> {
        self.into_iter().map(|x| x.from_arcmin()).collect()
    }

    fn from_arcsec(self) -> Vec<$name> {
        self.into_iter().map(|x| x.from_arcsec()).collect()
    }

    fn from_mas(self) -> Vec<$name> {
        self.into_iter().map(|x| x.from_mas()).collect()
    }

    fn to_degree(self) -> Vec<$name> {
        self.into_iter().map(|x| x.to_degree()).collect()
    }
    fn to_arcmin(self) -> Vec<$name> {
        self.into_iter().map(|x| x.to_arcmin()).collect()
    }

    fn to_arcsec(self) -> Vec<$name> {
        self.into_iter().map(|x| x.to_arcsec()).collect()
    }

    fn to_mas(self) -> Vec<$name> {
        self.into_iter().map(|x| x.to_mas()).collect()
    }
        })+
        $(impl Conversion<Vec<$name>> for &[$name] {
    fn from_degree(self) -> Vec<$name> {
        self.into_iter().map(|x| x.from_degree()).collect()
    }
    fn from_arcmin(self) -> Vec<$name> {
        self.into_iter().map(|x| x.from_arcmin()).collect()
    }

    fn from_arcsec(self) -> Vec<$name> {
        self.into_iter().map(|x| x.from_arcsec()).collect()
    }

    fn from_mas(self) -> Vec<$name> {
        self.into_iter().map(|x| x.from_mas()).collect()
    }

    fn to_degree(self) -> Vec<$name> {
        self.into_iter().map(|x| x.to_degree()).collect()
    }

    fn to_arcmin(self) -> Vec<$name> {
        self.into_iter().map(|x| x.to_arcmin()).collect()
    }

    fn to_arcsec(self) -> Vec<$name> {
        self.into_iter().map(|x| x.to_arcsec()).collect()
    }

    fn to_mas(self) -> Vec<$name> {
        self.into_iter().map(|x| x.to_mas()).collect()
    }
        })+    };
}
impl_conversion!(f64, f32);
