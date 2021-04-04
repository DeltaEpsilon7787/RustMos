#[macro_export]
macro_rules! gen_gas_vec {
    ($($t:tt)*) => {
        $crate::GasVec($crate::enum_map!{
            $($t)*
            _ => 0.0
        })
    }
}

#[macro_export]
macro_rules! gen_gas_mix_with_energy {
    (
        with ($($t:tt)*)
        at ($temp:expr)
    ) => {
        gen_gas_mix_with_energy! {
            with ($($t)*)
            at ($temp)
            in (0.0)
        }
    };
    (
        with ($($t:tt)*)
        at ($energy:expr)
        in ($volume:expr)
    ) => {
        $crate::GasMixture::with_energy(
            $crate::gen_gas_vec!($($t)*),
            $energy,
            $volume
        )
    };
}

#[macro_export]
macro_rules! gen_gas_mix_with_temp {
    {
        with ($($t:tt)*)
        at ($temp:expr)
    } => {
        gen_gas_mix_with_temp! {
            with ($($t)*)
            at ($temp)
            in (0.0)
        }
    };

    {
        with ($($t:tt)*)
        at ($temp:expr)
        in ($volume:expr)
    } => {
        GasMixture {
            gases: $crate::gen_gas_vec!($($t)*),
            temperature: $temp,
            volume: $volume
        }
    };
}

#[macro_export]
macro_rules! temperature {
    ($temp:expr, K) => {
        $temp
    };
    ($temp:expr, C) => {
        temperature!($temp + $crate::constants::T0C, K)
    };
}

#[macro_export]
macro_rules! reaction {
    {
        called ($name:ident)
        with($($g:expr => $ma:expr),+)
        at($min_temp:expr)
        with_gm_as ($gm_name:ident) =>
        $code: tt
    } => {
        #[inline]
        pub fn $name($gm_name: $crate::GasMixture) -> $crate::GasMixture {
            if (
                $gm_name.temperature >= $min_temp &&
                $(
                    $gm_name[$g] >= $ma
                )&&+
            ) {
                $code
            } else {
                $gm_name
            }
        }
    };
}

#[macro_export]
macro_rules! chained_call {
    (
        $final_argument:expr => $last_func:ident
    ) => {
        $last_func($final_argument)
    };
    (
        $starting_value:expr => $target_func:ident => $($rest:ident) => +
    ) => {
        chained_call! {
            $target_func($starting_value) => $($rest) => +
        }
    }
}

#[macro_export]
macro_rules! test_reaction{
    (
        named ($name:ident)
        testing ($func:path)
        init_with ($($gas0:path => $gas_amt0:expr),+)
        init_at ($temp0:expr)
        expect_with ($($t1:tt)*)
        expect_at ($temp1:expr)
    ) => {
        test_reaction!(
            named ($name)
            testing ($func)
            init_with ($($gas0 => $gas_amt0),+)
            init_at ($temp0)
            expect_with ($($t1)*)
            expect_at ($temp1)
            in(1000.0)
        );
    };
    (
        named ($name:ident)
        testing ($func:path)
        init_with ($($gas0:path => $gas_amt0:expr),+)
        init_at ($temp0:expr)
        expect_with ($($gas1:path => $gas_amt1:expr),+)
        expect_at ($temp1:expr)
        in ($vol:expr)
    ) => {
        #[test]
        fn $name() {
            let g0: $crate::GasMixture = $crate::gen_gas_mix_with_temp!(
                with(
                    $($gas0 => $gas_amt0),+,
                )
                at($temp0)
                in($vol)
            );

            let g1: $crate::GasMixture = $crate::gen_gas_mix_with_temp!(
                with(
                    $($gas1 => $gas_amt1),+,
                )
                at($temp1)
                in($vol)
            );

            let result = $func(g0);

            assert!(
                approx_eq!(
                    f64,
                    result.temperature,
                    g1.temperature,
                    epsilon=0.000000001
                ),
                "Wrong temperature: {} != {}",
                result.temperature,
                g1.temperature
            );

            $(
                assert!(
                    approx_eq!(
                        f64,
                        result[$gas1],
                        g1[$gas1],
                        epsilon=0.0000001
                    ),
                    "Wrong amount of {:?}: {:?} != {:?}",
                    $gas1,
                    result[$gas1],
                    g1[$gas1]
                );
            )*
        }
    }
}
