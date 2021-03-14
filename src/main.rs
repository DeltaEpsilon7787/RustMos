use tg_atmos_sim::*;

fn main() {
    let seed = (100_000..100_000_000).map(|i| {
        gen_gas_mix_with_temp!(
            with (
                Gas::Pl => i as f64,
                Gas::CO2 => i as f64,
                Gas::O2 => i as f64 / 2.0,
                Gas::H2 => i as f64,
            )
            at (i as f64)
            in (1000.0)
        )
    });

    let output = seed.map(react).fold(0.0, |a, gm| {
        a + gm.gases.0.values().sum::<f64>() + gm.temperature
    });

    println!("{:?}", output);
}
