use crate::sign::Sign;
use crate::sign::Sign::*;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::Display;
use strum_macros::EnumIter; // 0.17.1
use strum_macros::IntoStaticStr;

#[derive(Debug, Clone, Copy, IntoStaticStr, EnumIter, Display)]
pub enum DecanKind {
    I,
    II,
    III,
}

#[derive(Debug, Clone, Copy, IntoStaticStr, EnumIter, Display, Serialize, Deserialize)]
pub enum SignDecanKind {
    AriesI,
    AriesII,
    AriesIII,
    TaurusI,
    TaurusII,
    TaurusIII,
    GeminiI,
    GeminiII,
    GeminiIII,
    CancerI,
    CancerII,
    CancerIII,
    LeoI,
    LeoII,
    LeoIII,
    VirgoI,
    VirgoII,
    VirgoIII,
    LibraI,
    LibraII,
    LibraIII,
    ScorpioI,
    ScorpioII,
    ScorpioIII,
    SagittariusI,
    SagittariusII,
    SagittariusIII,
    CapricornI,
    CapricornII,
    CapricornIII,
    AquariusI,
    AquariusII,
    AquariusIII,
    PiscesI,
    PiscesII,
    PiscesIII,
}

use crate::body_sign;
use body_sign::SignBody;
impl SignDecanKind {
    /// Returns the (Body, Sign) pair implied by this decan.
    /// Uses your existing `into_sign()` and `decan_ruler()` helpers.
    #[inline]
    pub fn into_body_sign(self) -> body_sign::SignBody {
        let body = self.decan_ruler();
        let sign = self.into_sign();
        SignBody::new(sign, body)
    }

    #[inline]
    pub const fn into_sign(self) -> Sign {
        match self {
            SignDecanKind::AriesI | SignDecanKind::AriesII | SignDecanKind::AriesIII => Sign::Aries,
            SignDecanKind::TaurusI | SignDecanKind::TaurusII | SignDecanKind::TaurusIII => {
                Sign::Taurus
            }
            SignDecanKind::GeminiI | SignDecanKind::GeminiII | SignDecanKind::GeminiIII => {
                Sign::Gemini
            }
            SignDecanKind::CancerI | SignDecanKind::CancerII | SignDecanKind::CancerIII => {
                Sign::Cancer
            }
            SignDecanKind::LeoI | SignDecanKind::LeoII | SignDecanKind::LeoIII => Sign::Leo,
            SignDecanKind::VirgoI | SignDecanKind::VirgoII | SignDecanKind::VirgoIII => Sign::Virgo,
            SignDecanKind::LibraI | SignDecanKind::LibraII | SignDecanKind::LibraIII => Sign::Libra,
            SignDecanKind::ScorpioI | SignDecanKind::ScorpioII | SignDecanKind::ScorpioIII => {
                Sign::Scorpio
            }
            SignDecanKind::SagittariusI
            | SignDecanKind::SagittariusII
            | SignDecanKind::SagittariusIII => Sign::Sagittarius,
            SignDecanKind::CapricornI
            | SignDecanKind::CapricornII
            | SignDecanKind::CapricornIII => Sign::Capricorn,
            SignDecanKind::AquariusI | SignDecanKind::AquariusII | SignDecanKind::AquariusIII => {
                Sign::Aquarius
            }
            SignDecanKind::PiscesI | SignDecanKind::PiscesII | SignDecanKind::PiscesIII => {
                Sign::Pisces
            }
        }
    }

    #[inline]
    pub const fn into_decan(self) -> DecanKind {
        match self {
            // I
            SignDecanKind::AriesI
            | SignDecanKind::TaurusI
            | SignDecanKind::GeminiI
            | SignDecanKind::CancerI
            | SignDecanKind::LeoI
            | SignDecanKind::VirgoI
            | SignDecanKind::LibraI
            | SignDecanKind::ScorpioI
            | SignDecanKind::SagittariusI
            | SignDecanKind::CapricornI
            | SignDecanKind::AquariusI
            | SignDecanKind::PiscesI => DecanKind::I,

            // II
            SignDecanKind::AriesII
            | SignDecanKind::TaurusII
            | SignDecanKind::GeminiII
            | SignDecanKind::CancerII
            | SignDecanKind::LeoII
            | SignDecanKind::VirgoII
            | SignDecanKind::LibraII
            | SignDecanKind::ScorpioII
            | SignDecanKind::SagittariusII
            | SignDecanKind::CapricornII
            | SignDecanKind::AquariusII
            | SignDecanKind::PiscesII => DecanKind::II,

            // III
            SignDecanKind::AriesIII
            | SignDecanKind::TaurusIII
            | SignDecanKind::GeminiIII
            | SignDecanKind::CancerIII
            | SignDecanKind::LeoIII
            | SignDecanKind::VirgoIII
            | SignDecanKind::LibraIII
            | SignDecanKind::ScorpioIII
            | SignDecanKind::SagittariusIII
            | SignDecanKind::CapricornIII
            | SignDecanKind::AquariusIII
            | SignDecanKind::PiscesIII => DecanKind::III,
        }
    }

    #[inline]
    pub const fn decan_ruler(self) -> crate::Body {
        //use PlanetKind::*;
        use crate::Body::*;

        match self {
            // Aries: Mars, Sun, Venus
            SignDecanKind::AriesI => Mars,
            SignDecanKind::AriesII => Sun,
            SignDecanKind::AriesIII => Venus,

            // Taurus: Mercury, Moon, Saturn
            SignDecanKind::TaurusI => Mercury,
            SignDecanKind::TaurusII => Moon,
            SignDecanKind::TaurusIII => Saturn,

            // Gemini: Jupiter, Mars, Sun
            SignDecanKind::GeminiI => Jupiter,
            SignDecanKind::GeminiII => Mars,
            SignDecanKind::GeminiIII => Sun,

            // Cancer: Venus, Mercury, Moon
            SignDecanKind::CancerI => Venus,
            SignDecanKind::CancerII => Mercury,
            SignDecanKind::CancerIII => Moon,

            // Leo: Saturn, Jupiter, Mars
            SignDecanKind::LeoI => Saturn,
            SignDecanKind::LeoII => Jupiter,
            SignDecanKind::LeoIII => Mars,

            // Virgo: Sun, Venus, Mercury
            SignDecanKind::VirgoI => Sun,
            SignDecanKind::VirgoII => Venus,
            SignDecanKind::VirgoIII => Mercury,

            // Libra: Moon, Saturn, Jupiter
            SignDecanKind::LibraI => Moon,
            SignDecanKind::LibraII => Saturn,
            SignDecanKind::LibraIII => Jupiter,

            // Scorpio: Mars, Sun, Venus
            SignDecanKind::ScorpioI => Mars,
            SignDecanKind::ScorpioII => Sun,
            SignDecanKind::ScorpioIII => Venus,

            // Sagittarius: Mercury, Moon, Saturn
            SignDecanKind::SagittariusI => Mercury,
            SignDecanKind::SagittariusII => Moon,
            SignDecanKind::SagittariusIII => Saturn,

            // Capricorn: Jupiter, Mars, Sun
            SignDecanKind::CapricornI => Jupiter,
            SignDecanKind::CapricornII => Mars,
            SignDecanKind::CapricornIII => Sun,

            // Aquarius: Venus, Mercury, Moon
            SignDecanKind::AquariusI => Venus,
            SignDecanKind::AquariusII => Mercury,
            SignDecanKind::AquariusIII => Moon,

            // Pisces: Saturn, Jupiter, Mars
            SignDecanKind::PiscesI => Saturn,
            SignDecanKind::PiscesII => Jupiter,
            SignDecanKind::PiscesIII => Mars,
        }
    }
}
// Optional ergonomic conversion:
impl From<SignDecanKind> for Sign {
    #[inline]
    fn from(d: SignDecanKind) -> Self {
        d.into_sign()
    }
}

// Optional ergonomic conversion
impl From<SignDecanKind> for DecanKind {
    #[inline]
    fn from(v: SignDecanKind) -> Self {
        v.into_decan()
    }
}
