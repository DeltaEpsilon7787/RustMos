use tg_atmos_sim::*;

fn main() {
    let i = 6000194;
    let output = gen_gas_mix_with_temp!(
        with (
            Gas::Pl => (3. * i as f64) % 1000.,
            Gas::O2 => (5. * i as f64) % 1000.,
            Gas::CO2 => (7. * i as f64) % 1000.,
            Gas::H2 => (11. * i as f64) % 1000.,
            Gas::N2 => (13. * i as f64) % 1000.,
            Gas::N2O => (17. * i as f64) % 1000.,
            Gas::BZ => (21. * i as f64) % 1000.,
        )
        at (i as f64)
        in (1000.0)
    );

    println!("{:?}", output);
    println!("{:?}", react(output));
}
