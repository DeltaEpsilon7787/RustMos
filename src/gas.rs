extern crate enum_map;

use enum_map as EM;
use std::ops::{Add, Index, Mul};

#[derive(Copy, Clone, Debug, EM::Enum)]
pub enum Gas {
    N2,
    O2,
    CO2,
    N2O,
    Pl,
    H2O,
    HNb,
    NO2,
    H2,
    BZ,
    ST,
    PlOx,
}
impl Gas {
    fn heat_cap_of(self) -> f64 {
        match self {
            Gas::N2 => 20.,
            Gas::O2 => 20.,
            Gas::CO2 => 30.,
            Gas::N2O => 40.,
            Gas::Pl => 200.,
            Gas::H2O => 40.,
            Gas::HNb => 2000.,
            Gas::NO2 => 20.,
            Gas::H2 => 10.,
            Gas::BZ => 0.,
            Gas::ST => 5.,
            Gas::PlOx => 80.,
        }
    }

    fn fusion_power_of(self) -> f64 {
        match self {
            Gas::N2O => 10.,
            Gas::H2O => 8.,
            Gas::NO2 => 16.,
            Gas::H2 => 1.,
            Gas::BZ => 8.,
            Gas::ST => 7.,
            Gas::PlOx => -10.,
            _ => 0.,
        }
    }
}
pub type GasEnumMap = EM::EnumMap<Gas, f64>;

#[derive(Copy, Clone, Debug)]
pub struct GasVec(pub GasEnumMap);

impl GasVec {
    pub fn get_heat_cap(&self) -> f64 {
        self.0
            .iter()
            .map(|(g, a)| a * Gas::heat_cap_of(g))
            .sum::<f64>()
    }

    pub fn get_fusion_power(&self) -> f64 {
        self.0
            .iter()
            .map(|(g, a)| a * Gas::fusion_power_of(g))
            .sum::<f64>()
    }

    pub fn get_total_amount(&self) -> f64 {
        self.0
            .values()
            .sum()
    }
}

impl Add<GasVec> for GasVec {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        GasVec(GasEnumMap::from(|g| self.0[g] + rhs.0[g]))
    }
}

impl Mul<f64> for GasVec {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        GasVec(GasEnumMap::from(|g| self.0[g] * rhs))
    }
}

impl Index<Gas> for GasVec {
    type Output = f64;

    fn index(&self, gas: Gas) -> &f64 {
        &self.0[gas]
    }
}
