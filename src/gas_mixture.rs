extern crate enum_map;

use crate::constants as C;
use crate::gas::*;
use std::ops::{Add, Index};

#[derive(Copy, Clone, Debug)]
pub struct GasMixture {
    pub gases: GasVec,
    pub temperature: f64,
    pub volume: f64,
}

impl GasMixture {
    pub fn get_heat_cap(&self) -> f64 {
        self.gases.get_heat_cap()
    }

    pub fn get_fusion_power(&self) -> f64 {
        self.gases.get_fusion_power()
    }

    pub fn get_energy(&self) -> f64 {
        self.get_heat_cap() * self.temperature
    }

    pub fn get_total_amount(&self) -> f64 {
        self.gases.get_total_amount()
    }

    pub fn get_pressure(&self) -> f64 {
        C::R_IDEAL_GAS_EQUATION * self.get_total_amount() * self.temperature / self.volume
    }

    pub fn adjust_thermal_energy(&self, energy: f64) -> Self {
        Self {
            temperature: (self.get_energy() + energy) / self.get_heat_cap(),
            ..*self
        }
    }

    pub fn mix_with(&self, other: &GasMixture) -> Self {
        let lhs_energy = self.get_energy();
        let lhs_cap = self.get_heat_cap();
        let rhs_energy = other.get_energy();
        let rhs_cap = other.get_heat_cap();

        Self {
            gases: self.gases + other.gases,
            temperature: (lhs_energy + rhs_energy) / (lhs_cap + rhs_cap),
            volume: self.volume + other.volume,
        }
    }

    pub fn with_energy(gases: GasVec, energy: f64, volume: f64) -> Self {
        if gases.get_heat_cap() == 0.0 {
            panic!("Null gas mixes may not have energy");
        }
        Self {
            gases,
            temperature: energy / gases.get_heat_cap(),
            volume,
        }
    }
}

impl Add<GasMixture> for GasMixture {
    type Output = Self;

    fn add(self, rhs: GasMixture) -> Self {
        self.mix_with(&rhs)
    }
}

impl Index<Gas> for GasMixture {
    type Output = f64;

    fn index(&self, gas: Gas) -> &f64 {
        &self.gases[gas]
    }
}
