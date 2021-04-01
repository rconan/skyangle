pub enum SkyAngle<T> {
    Degree(T),
    Arcminute(T),
    Arcsecond(T),
    MilliArcsec(T),
}
impl SkyAngle<f64> {
    pub fn to_radians(self) -> f64 {
        match self {
            Self::Degree(val) => val.to_radians(),
            Self::Arcminute(val) => Self::Degree(val/60.0).to_radians(),
            Self::Arcsecond(val) => Self::Arcminute(val/60.0).to_radians(),
            Self::MilliArcsec(val) => Self::Arcsecond(val*1e-3).to_radians(),
        }
    }
}
impl SkyAngle<f32> {
    pub fn to_radians(self) -> f32 {
        match self {
            Self::Degree(val) => val.to_radians(),
            Self::Arcminute(val) => Self::Degree(val/60.0).to_radians(),
            Self::Arcsecond(val) => Self::Arcminute(val/60.0).to_radians(),
            Self::MilliArcsec(val) => Self::Arcsecond(val*1e-3).to_radians(),
        }
    }
}
pub trait Conversion<T> {
    fn from_arcmin(self) -> T;
    fn from_arcsec(self) -> T;
    fn from_mas(self) -> T;
    fn to_arcmin(self) -> T;
    fn to_arcsec(self) -> T;
    fn to_mas(self) -> T;
}
macro_rules! impl_conversion {
    ($($name:ty),+) => {
        $(impl Conversion<$name> for $name {
            /// Converts angle in arcminute to radian
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
            /// Converts angle in radian to arcminute
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
    };
}
impl_conversion!(f64, f32);
