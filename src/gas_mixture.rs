#![macro_use]
extern crate enum_map;

use crate::gas::*;
use std::ops::{Add, Index};

#[macro_export]
macro_rules! gen_gas_vec (
    ($($t:tt)*) => {
        GasVec(enum_map!{
            $($t)*
            _ => 0.0
        })
    }
);

#[macro_export]
macro_rules! gen_free_gas_mix(
    (
        with($($t:tt)*)
        at($temp:expr)$unit:ident
    ) => {
        gen_gas_mix!(
            with($($t)*)
            at($temp)$unit
            in(0.0) L
        )
    }
);

#[macro_export]
macro_rules! gen_gas_mix(
    (
        with($($t:tt)*)
        at($temp:expr) K
        in($volume:expr) L
    ) => {
        GasMixture {
            gases: gen_gas_vec!($($t)*),
            temperature: $temp,
            volume: $volume
        }
    };
    (
        with($($t:tt)*)
        at($temp:expr) C
        in($volume:expr) L
    ) => {
        gen_gas_mix! {
            with($($t)*)
            at($crate::constants::T0C + $temp) K
            in($volume) L
        }
    };
    (
        with($($t:tt)*)
        at($energy:expr) J
        in($volume:expr) L
    ) => {
        GasMixture::with_energy(
            gen_gas_vec!($($t)*),
            $energy,
            $volume
        )
    };
);

#[derive(Copy, Clone, Debug)]
pub struct GasMixture {
    pub gases: GasVec,
    pub temperature: f64,
    pub volume: f64,
}

impl GasMixture {
    #[inline]
    pub fn get_heat_cap(&self) -> f64 {
        self.gases.get_heat_cap()
    }

    #[inline]
    pub fn get_fusion_power(&self) -> f64 {
        self.gases.get_fusion_power()
    }

    #[inline]
    pub fn get_energy(&self) -> f64 {
        self.get_heat_cap() * self.temperature
    }

    pub fn adjust_thermal_energy(&self, energy: f64) -> Self {
        Self {
            gases: self.gases,
            temperature: (self.get_energy() + energy) / self.get_heat_cap(),
            volume: self.volume,
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
            volume: self.volume + other.volume
        }
    }

    pub fn with_energy(gases: GasVec, energy: f64, volume: f64) -> Self {
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
