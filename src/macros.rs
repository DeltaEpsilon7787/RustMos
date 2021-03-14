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
        with($($g:path => $ma:expr),+)
        at($min_temp:expr)
        with_gm_as ($gm_name:ident) =>
        $code: tt
    } => {
        #[inline]
        fn $name($gm_name: $crate::GasMixture) -> $crate::GasMixture {
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
