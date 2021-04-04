#[cfg(test)]
mod tests {
    use crate::reactions as R;
    use crate::{gen_gas_mix_with_temp, temperature, test_reaction, Gas, GasMixture};
    use float_cmp::approx_eq;

    #[test]
    fn energy_merge_test_positive() {
        let mix0 = gen_gas_mix_with_temp!(
            with(
                Gas::Pl => 200.0,
            )
            at(temperature!(1000.0, K))
        );

        let mix1 = gen_gas_mix_with_temp!(
            with(
                Gas::Pl => 200.0,
            )
            at(temperature!(2000.0, K))
        );

        assert!(
            approx_eq!(
                f64,
                (mix0 + mix1).get_energy(),
                mix0.get_energy() + mix1.get_energy()
            ),
            "Energy is not conserved with positive mixes"
        );

        assert!(
            approx_eq!(f64, (mix0 + mix1)[Gas::Pl], 400.0),
            "Matter is not conserved with positive mixes"
        );
    }

    #[test]
    fn energy_merge_test_negative() {
        let mix0 = gen_gas_mix_with_temp!(
            with(
                Gas::Pl => 200.0,
            )
            at(temperature!(1000.0, K))
        );

        let mix1 = gen_gas_mix_with_temp!(
            with(
                Gas::Pl => -50.0,
            )
            at(temperature!(2000.0, K))
        );

        assert!(
            approx_eq!(
                f64,
                (mix0 + mix1).get_energy(),
                mix0.get_energy() + mix1.get_energy()
            ),
            "Energy is not conserved with negative mixes"
        );

        assert!(
            approx_eq!(f64, (mix0 + mix1)[Gas::Pl], 150.0),
            "Matter is not conserved with negative mixes"
        );
    }

    test_reaction!(
        named(n2o_decomp_test)
        testing(R::n2o_decomp)
        init_with(
            Gas::N2O => 20.0
        )
        init_at(temperature!(10000.0, K))
        expect_with(
            Gas::N2 => 3.6,
            Gas::O2 => 1.8,
            Gas::N2O => 16.4
        )
        expect_at(temperature!(11413.612565445026, K))
    );

    test_reaction!(
        named(trit_fire_test_as_oxidizer)
        testing(R::trit_fire)
        init_with(
            Gas::H2 => 100.0,
            Gas::O2 => 50.0
        )
        init_at(temperature!(500., K))
        expect_with(
            Gas::O2 => 50.0,
            Gas::H2O => 0.5,
            Gas::H2 => 99.5
        )
        expect_at(temperature!(565.7568238213399, K))
    );

    test_reaction!(
        named(trit_fire_test_as_fission)
        testing(R::trit_fire)
        init_with(
            Gas::H2 => 100.0,
            Gas::O2 => 500.0
        )
        init_at(temperature!(500., K))
        expect_with(
            Gas::O2 => 410.0,
            Gas::H2O => 100.0,
            Gas::H2 => 90.0
        )
        expect_at(temperature!(21793.893129770993, K))
    );

    test_reaction!(
        named(plasma_fire_test_as_oxidizer)
        testing(R::plasma_fire)
        init_with(
            Gas::Pl => 100.0,
            Gas::O2 => 100.0
        )
        init_at(temperature!(500., K))
        expect_with(
            Gas::O2 => 99.85571305137054,
            Gas::CO2 => 0.1109798775153106,
            Gas::Pl => 99.8890201224847
        )
        expect_at(temperature!(515.6434578678881, K))
    );

    test_reaction!(
        named(plasma_fire_test_as_fusion)
        testing(R::plasma_fire)
        init_with(
            Gas::Pl => 100.0,
            Gas::O2 => 10000.0
        )
        init_at(temperature!(500., K))
        expect_with(
            Gas::O2 => 9998.557130513705,
            Gas::Pl => 98.89020122484689,
            Gas::H2 => 1.109798775153106
        )
        expect_at(temperature!(515.6955382962315, K))
    );

    test_reaction!(
        named(nitryl_formation_test)
        testing(R::nitryl_formation)
        init_with(
            Gas::N2 => 100.0,
            Gas::O2 => 100.0,
            Gas::PlOx => 5.0
        )
        init_at(temperature!(50000.0, K))
        expect_with(
            Gas::N2 => 97.76676046272723,
            Gas::O2 => 97.76676046272723,
            Gas::NO2 => 4.466479074545536,
            Gas::PlOx => 5.0
        )
        expect_at(temperature!(49949.244555971076, K))
    );

    test_reaction!(
        named(bz_react_test_nitrous_reduction)
        testing(R::bz_synth)
        init_with(
            Gas::N2O => 15.0,
            Gas::Pl => 15.0
        )
        init_at(temperature!(2.0, K))
        expect_with(
            Gas::N2O => 7.5
        )
        expect_at(temperature!(5024.0, K))
        in(2500.0)
    );

    test_reaction!(
        named(bz_react_test_bz_formation)
        testing(R::bz_synth)
        init_with(
            Gas::N2O => 15.0,
            Gas::Pl => 45.0
        )
        init_at(temperature!(2.0, K))
        expect_with(
            Gas::O2 => 1.0,
            Gas::Pl => 15.0,
            Gas::BZ => 14.0
        )
        expect_at(temperature!(999.7350993377484, K))
        in(2500.0)
    );

    test_reaction!(
        named(stimulum_test)
        testing(R::stimulum_synth)
        init_with(
            Gas::H2 => 50.0,
            Gas::Pl => 20.0,
            Gas::BZ => 50.0,
            Gas::NO2 => 50.0
        )
        init_at(temperature!(100000., K))
        expect_with(
            Gas::Pl => 19.0,
            Gas::NO2 => 49.0,
            Gas::H2 => 49.0,
            Gas::BZ => 50.0,
            Gas::ST => 0.1
        )
        expect_at(temperature!(104354.42587722163, K))
    );

    test_reaction!(
        named(nob_synth_test)
        testing(R::hnob_synth)
        init_with(
            Gas::N2 => 25.0,
            Gas::H2 => 15.0
        )
        init_at(temperature!(5000000.0, K))
        expect_with(
            Gas::N2 => 17.0,
            Gas::HNb => 0.4,
            Gas::H2 => 11.0
        )
        expect_at(temperature!(1960000.0, K))
    );

    test_reaction!(
        named(fusion_test_suppressed)
        testing(R::fusion)
        init_with(
            Gas::CO2 => 2500.0,
            Gas::Pl => 500.0,
            Gas::BZ => 50.0,
            Gas::H2 => 1.5
        )
        init_at(temperature!(500000.0, K))
        expect_with(
            Gas::CO2 => 576.5539197230712,
            Gas::Pl => 2173.446080276929,
            Gas::H2 => 0.5,
            Gas::BZ => 50.0
        )
        expect_at(temperature!(500000.0, K))
    );

    test_reaction!(
        named(fusion_test_endo)
        testing(R::fusion)
        init_with(
            Gas::CO2 => 2500.0,
            Gas::Pl => 500.0,
            Gas::BZ => 100.0,
            Gas::H2 => 1.5
        )
        init_at(temperature!(500000.0, K))
        expect_with(
            Gas::CO2 => 1551.3402280453292,
            Gas::Pl => 1198.6597719546708,
            Gas::NO2 => 2.7172818684608555,
            Gas::H2 => 0.5,
            Gas::BZ => 102.71728186846086
        )
        expect_at(temperature!(210716.17990979773, K))
    );

    test_reaction!(
        named(fusion_test_exo)
        testing(R::fusion)
        init_with(
            Gas::CO2 => 2500.0,
            Gas::Pl => 500.0,
            Gas::H2 => 1.5
        )
        init_at(temperature!(500000.0, K))
        expect_with(
            Gas::O2 => 9.115832060388129e-06,
            Gas::CO2 => 250.00455791603008,
            Gas::N2O => 9.115832060388129e-06,
            Gas::Pl => 499.9954420839698,
            Gas::H2 => 0.5
        )
        expect_at(temperature!(813992.1067058449, K))
    );

    test_reaction!(
        named(random_react_test)
        testing(R::react)
        init_with(
            Gas::N2 => 522.0,
            Gas::O2 => 970.0,
            Gas::CO2 => 358.0,
            Gas::N2O => 298.0,
            Gas::Pl => 582.0,
            Gas::H2 => 134.0,
            Gas::BZ => 74.0
        )
        init_at(temperature!(6000194.0, K))
        expect_with(
            Gas::N2 => 522.0,
            Gas::O2 => 846.8133333333333,
            Gas::CO2 => 403.0192638737864,
            Gas::N2O => 297.99999990963187,
            Gas::Pl => 2211.447402612144,
            Gas::H2O => 134.0,
            Gas::NO2 => 3.6584809783155587,
            Gas::H2 => 119.6,
            Gas::BZ => 77.65848097831555
        )
        expect_at(temperature!(1969362.373934752, K))
    );
}
