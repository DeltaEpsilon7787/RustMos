extern crate libc;

use std::{convert::TryInto, slice::ChunksExactMut};

use crate::{gas::GasEnumMap, reactions as R, Gas, GasVec};
use crate::{gas::GAS_AMT, GasMixture};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GasMixtureFFI {
    gases: [f64; GAS_AMT],
    temperature: f64,
    volume: f64,
}

impl From<GasMixture> for GasMixtureFFI {
    fn from(source: GasMixture) -> Self {
        GasMixtureFFI {
            gases: source.gases.0.as_slice().try_into().unwrap(),
            temperature: source.temperature,
            volume: source.volume,
        }
    }
}

impl Into<GasMixture> for GasMixtureFFI {
    fn into(self) -> GasMixture {
        GasMixture {
            gases: GasVec(GasEnumMap::from(|gas: Gas| self.gases[gas as usize])),
            temperature: self.temperature,
            volume: self.volume,
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GasMixtureArrayFFI {
    gas_mixes: *mut GasMixtureFFI,
    len: usize,
}

impl Into<Vec<GasMixture>> for GasMixtureArrayFFI {
    fn into(self) -> Vec<GasMixture> {
        unsafe {
            std::slice::from_raw_parts(self.gas_mixes, self.len)
                .iter()
                .map(|gm_ptr| (*gm_ptr).into())
                .collect()
        }
    }
}

impl GasMixtureArrayFFI {
    pub fn as_slice_mut(self) -> &'static mut [GasMixtureFFI] {
        unsafe { std::slice::from_raw_parts_mut(self.gas_mixes, self.len) }
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GasMixtureManifoldFFI {
    gas_mix_timelines: *mut GasMixtureFFI,
    timelines: usize,
    len: usize,
}

impl GasMixtureManifoldFFI {
    pub fn as_chunks(self) -> ChunksExactMut<'static, GasMixtureFFI> {
        unsafe {
            std::slice::from_raw_parts_mut(self.gas_mix_timelines, self.timelines * self.len)
                .chunks_exact_mut(self.len)
        }
    }
}

/// Take a gas mixture from `in_gas_mix`, react it a single time and write the result into `out_gas_mix`
#[no_mangle]
pub unsafe extern "C" fn react_once(
    in_gas_mix: *const GasMixtureFFI,
    out_gas_mix: *mut GasMixtureFFI,
) {
    *out_gas_mix = R::react_once((*in_gas_mix).into()).into();
}

/// Take a gas mixture from `in_gas_mix`, react it until it stops and write the result into `out_gas_mix`
#[no_mangle]
pub unsafe extern "C" fn react_until_done(
    in_gas_mix: *const GasMixtureFFI,
    out_gas_mix: *mut GasMixtureFFI,
) {
    *out_gas_mix = R::react_until_done((*in_gas_mix).into()).into();
}

/// Take a gas mixture from `in_gas_mix`, react it `out_gas_mix.len` times and write the intermediate states of the mixture on each reaction into `out_gas_mix`.
/// The first element in `out_gas_mix` will the first reaction result.
#[no_mangle]
pub unsafe extern "C" fn react_several(
    in_gas_mix: *const GasMixtureFFI,
    out_gas_mix: *const GasMixtureArrayFFI,
) {
    (*out_gas_mix).as_slice_mut().clone_from_slice(
        R::react_several((*in_gas_mix).into(), (*out_gas_mix).len)
            .iter()
            .map(|gm_ptr| (*gm_ptr).into())
            .collect::<Vec<GasMixtureFFI>>()
            .as_slice(),
    );
}

/// Take an array of gas mixtures from `in_gas_mixes`, react them separately once and write the result to the respective indices in `out_gas_mixes`.
#[no_mangle]
pub unsafe extern "C" fn react_each_once(
    in_gas_mixes: *const GasMixtureArrayFFI,
    out_gas_mixes: *const GasMixtureArrayFFI,
) {
    (*out_gas_mixes).as_slice_mut().clone_from_slice(
        R::react_each_once((*in_gas_mixes).into())
            .iter()
            .map(|gm_ptr| (*gm_ptr).into())
            .collect::<Vec<GasMixtureFFI>>()
            .as_slice(),
    );
}

/// Take an array of gas mixtures in `in_gas_mixes`, react them separately until they stop reacting and write the results to the respective indices in `out_gas_mixes`.
#[no_mangle]
pub unsafe extern "C" fn react_each_until_done(
    in_gas_mixes: *const GasMixtureArrayFFI,
    out_gas_mixes: *const GasMixtureArrayFFI,
) {
    (*out_gas_mixes).as_slice_mut().clone_from_slice(
        R::react_each_until_done((*in_gas_mixes).into())
            .iter()
            .map(|gm_ptr| (*gm_ptr).into())
            .collect::<Vec<GasMixtureFFI>>()
            .as_slice(),
    );
}

// Take an array of gas mixtures in `in_gas_mixes`, react them `out_gas_mixes.len` times and write the intermediate states of each gas mixtures into `out_gas_mixes`.
#[no_mangle]
pub unsafe extern "C" fn react_each_several(
    in_gas_mixes: *const GasMixtureArrayFFI,
    out_gas_mixes: *const GasMixtureManifoldFFI,
) {
    R::react_each_several((*in_gas_mixes).into(), (*out_gas_mixes).len)
        .iter()
        .zip((*out_gas_mixes).as_chunks())
        .for_each(|(timeline1, timeline2)| {
            timeline2.clone_from_slice(
                timeline1
                    .iter()
                    .map(|gm_ptr| (*gm_ptr).into())
                    .collect::<Vec<GasMixtureFFI>>()
                    .as_slice(),
            );
        });
}

/// Take two GasMixtures: `lhs_mix` and `rhs_mix`, merge them and write the resulting mix into `out_mix`
#[no_mangle]
pub unsafe extern "C" fn merge_two(
    lhs_mix: *const GasMixtureFFI,
    rhs_mix: *const GasMixtureFFI,
    out_mix: *mut GasMixtureFFI,
) {
    let lhs: GasMixture = (*lhs_mix).into();
    let rhs: GasMixture = (*rhs_mix).into();
    let out: GasMixtureFFI = (lhs + rhs).into();
    *out_mix = out;
}

/// Take an array of gas mixtures in `mix_array`, merge them all together and write the resulting mix into `out_mix`
#[no_mangle]
pub unsafe extern "C" fn merge_all(
    mix_array: *const GasMixtureArrayFFI,
    out_mix: *mut GasMixtureFFI,
) {
    let gas_vec: Vec<GasMixture> = (*mix_array).into();
    *out_mix = gas_vec
        .iter()
        .fold(GasMixture::zero(), |lhs, rhs| lhs + *rhs)
        .into()
}
