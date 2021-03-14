use crate::constants as C;
use crate::enum_map;
use crate::gas::*;
use crate::{chained_call, gas_mixture::*, gen_gas_mix_with_energy, reaction, temperature};

fn verify_hnob(gm: &GasMixture) -> bool {
    gm[Gas::HNb] < 5.0
}

reaction! (
    called(n2o_decomp)
    with(
        Gas::N2O => C::MINIMUM_MOLE_COUNT
    )
    at(temperature!(C::N2O_DECOMPOSITION_MIN_ENERGY, K))
    with_gm_as(gm) => {
        let n2o = gm[Gas::N2O];
        let t = gm.temperature;
        let burned_fuel = (2e-5 * (t - (1e-5 * t.powi(2)))).max(0.) * n2o;

        gm + gen_gas_mix_with_energy!(
            with (
                Gas::N2O => -burned_fuel,
                Gas::O2 => burned_fuel / 2.,
                Gas::N2 => burned_fuel,
            )
            at (C::N2O_DECOMPOSITION_ENERGY_RELEASED * burned_fuel)
        )
    }
);

reaction! (
    called(plasma_fire)
    with(
        Gas::Pl => C::MINIMUM_MOLE_COUNT,
        Gas::O2 => C::MINIMUM_MOLE_COUNT
    )
    at(temperature!(C::PLASMA_MINIMUM_BURN_TEMPERATURE, K))
    with_gm_as(gm) => {
        let pl = gm[Gas::Pl];
        let o2 = gm[Gas::O2];
        let t = gm.temperature;

        let temp_scale = ((t - C::PLASMA_MINIMUM_BURN_TEMPERATURE) / C::PLASMA_TEMP_SCALE).min(1.);

        let plasma_burn_rate = pl * temp_scale / C::PLASMA_BURN_RATE_DELTA;
        let plasma_burn_rate = if o2 > pl * C::PLASMA_OXYGEN_FULLBURN {
            plasma_burn_rate
        } else {
            plasma_burn_rate / C::PLASMA_OXYGEN_FULLBURN
        };

        let oxygen_burn_rate = C::OXYGEN_BURN_RATE_BASE - temp_scale;
        let plasma_burn_rate = {
            pl
                .min(plasma_burn_rate)
                .min(o2 / oxygen_burn_rate)
        };

        let is_satured = o2 / pl > C::SUPER_SATURATION_THRESHOLD;
        let energy_release = plasma_burn_rate * C::FIRE_PLASMA_ENERGY_RELEASED;

        gm + gen_gas_mix_with_energy!(
            with (
                Gas::Pl => -plasma_burn_rate,
                Gas::O2 => -plasma_burn_rate * oxygen_burn_rate,
                Gas::H2 if is_satured => plasma_burn_rate,
                Gas::CO2 if !is_satured => plasma_burn_rate,
            )
            at (energy_release)
        )
    }
);

reaction! (
    called(trit_fire)
    with(
        Gas::H2 => C::MINIMUM_MOLE_COUNT,
        Gas::O2 => C::MINIMUM_MOLE_COUNT
    )
    at(temperature!(100.0, C))
    with_gm_as(gm) => {
        let e = gm.get_energy();
        let h2 = gm[Gas::H2];
        let o2 = gm[Gas::O2];

        let o2_no_combust = o2 < h2 || e < C::MINIMUM_HEAT_CAPACITY;
        let burned_fuel = if o2_no_combust {o2 / C::TRITIUM_BURN_OXY_FACTOR} else {h2};
        let primary_energy_release = C::FIRE_HYDROGEN_ENERGY_RELEASED * burned_fuel;
        let extra_energy_release = if !o2_no_combust {primary_energy_release * (C::TRITIUM_BURN_TRIT_FACTOR - 1.)} else {0.};
        let energy_release = extra_energy_release + primary_energy_release;

        gm + gen_gas_mix_with_energy!(
            with(
                Gas::H2O => burned_fuel,
                Gas::H2 if o2_no_combust => -burned_fuel,
                Gas::H2 if !o2_no_combust => -burned_fuel / C::TRITIUM_BURN_TRIT_FACTOR,
                Gas::O2 if !o2_no_combust => -h2 * (1. - 1. / C::TRITIUM_BURN_TRIT_FACTOR),
            )
            at (energy_release)
        )
    }
);

reaction! (
    called(fusion)
    with(
        Gas::H2 => C::FUSION_TRITIUM_MOLES_USED,
        Gas::Pl => C::FUSION_MOLE_THRESHOLD,
        Gas::CO2 => C::FUSION_MOLE_THRESHOLD
    )
    at(temperature!(C::FUSION_TEMPERATURE_THRESHOLD, K))
    with_gm_as(gm) => {
        let pl = gm[Gas::Pl];
        let co2 = gm[Gas::CO2];
        let e = gm.get_energy();
        let v = gm.volume;

        let scale_factor = gm.volume / C::ATMOS_PI;
        let toroidal_size = 2. * C::ATMOS_PI + ((v - C::TOROID_VOLUME_BREAKEVEN) / C::TOROID_VOLUME_BREAKEVEN).atan();
        let instability = (gm.get_fusion_power() * C::INSTABILITY_GAS_POWER_FACTOR).powi(2) % toroidal_size;

        let plasma_mod = (pl - C::FUSION_MOLE_THRESHOLD) / scale_factor;
        let carbon_mod = (co2 - C::FUSION_MOLE_THRESHOLD) / scale_factor;

        let plasma_mod = (plasma_mod - instability * carbon_mod.sin()) % toroidal_size;
        let carbon_mod = (carbon_mod - plasma_mod) % toroidal_size;

        let delta_plasma = plasma_mod * scale_factor + C::FUSION_MOLE_THRESHOLD - pl;
        let delta_carbon = carbon_mod * scale_factor + C::FUSION_MOLE_THRESHOLD - co2;

        let reaction_energy = -delta_plasma * C::PLASMA_BINDING_ENERGY;
        let is_suppressed_endo = instability < C::FUSION_INSTABILITY_ENDOTHERMALITY && reaction_energy < 0.;

        let reaction_energy =
        if reaction_energy < 0. { reaction_energy * (instability - C::FUSION_INSTABILITY_ENDOTHERMALITY).sqrt() }
        else { reaction_energy };

        let product_release = C::FUSION_TRITIUM_MOLES_USED * reaction_energy * C::FUSION_TRITIUM_CONVERSION_COEFFICIENT;
        let is_exothermic = reaction_energy > 0.;

        let gas_vec_out = GasVec (enum_map! {
            Gas::O2 if is_exothermic => product_release,
            Gas::N2O if is_exothermic => product_release,
            Gas::BZ if !is_exothermic => product_release,
            Gas::NO2 if !is_exothermic => product_release,
            Gas::Pl => delta_plasma,
            Gas::CO2 => delta_carbon,
            Gas::H2 => -C::FUSION_TRITIUM_MOLES_USED,
            _ => 0.0
        });

        let zero_mix = GasMixture {
            gases: gas_vec_out,
            temperature: gm.temperature,
            volume: 0.
        };

        let delta_mix = GasMixture::with_energy(gas_vec_out, reaction_energy, 0.);

        if is_suppressed_endo {
            gm + zero_mix
        } else if e + reaction_energy < 0. {
            gm
        } else {
            gm + delta_mix
        }
    }
);

reaction! (
    called(nitryl_formation)
    with(
        Gas::N2 => 20.,
        Gas::O2 => 20.,
        Gas::PlOx => 5.
    )
    at(temperature!(C::FIRE_MINIMUM_TEMPERATURE_TO_EXIST * 60., K))
    with_gm_as(gm) => {
        let n2 = gm[Gas::N2];
        let o2 = gm[Gas::O2];
        let t = gm.temperature;

        let heat_eff = (t / C::FIRE_MINIMUM_TEMPERATURE_TO_EXIST / 60.).min(n2).min(o2);
        let energy_use = heat_eff * C::NITRYL_FORMATION_ENERGY;

        gm + gen_gas_mix_with_energy!(
            with(
                Gas::N2 => -heat_eff,
                Gas::O2 => -heat_eff,
                Gas::N2O => 2. * heat_eff,
            )
            at(-energy_use)
        )
    }
);

reaction! (
    called(bz_synth)
    with(
        Gas::N2O => 10.,
        Gas::Pl => 10.
    )
    at(f64::NEG_INFINITY)
    with_gm_as(gm) => {
        let p = gm.get_pressure();
        let pl = gm[Gas::Pl];
        let n2o = gm[Gas::N2O];

        let half_atm_pressure = 2. * p / C::ONE_ATMOSPHERE;
        let efficiency = (half_atm_pressure * (pl / n2o).max(1.)).powi(-1);
        let usage = efficiency
            .min(n2o)
            .min(pl / 2.);

        let is_balanced = usage == n2o;

        let energy_release = 2. * usage * C::FIRE_CARBON_ENERGY_RELEASED;

        let bz_prod = usage - p.max(1.);

        gm + gen_gas_mix_with_energy!(
            with(
                Gas::N2O => -usage,
                Gas::Pl => -2. * usage,
                Gas::BZ if is_balanced => bz_prod,
                Gas::O2 if is_balanced => p.max(1.),
            )
            at (energy_release)
        )
    }
);

pub fn react(gm: GasMixture) -> GasMixture {
    if verify_hnob(&gm) {
        chained_call! (
            gm =>
            n2o_decomp =>
            trit_fire =>
            plasma_fire =>
            fusion =>
            nitryl_formation =>
            bz_synth
        )
    } else {
        gm
    }
}
