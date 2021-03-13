#[macro_export]
macro_rules! gen_gas_vec (
    ($($t:tt)*) => {
        $crate::GasVec($crate::enum_map!{
            $($t)*
            _ => 0.0
        })
    }
);

#[macro_export]
macro_rules! gen_gas_mix(
    (
        with($($t:tt)*)
        at($temp:expr)$unit:ident
    ) => {
        $crate::gen_gas_mix!(
            with($($t)*)
            at($temp)$unit
            in(0.0) L
        )
    };    
    (
        with($($t:tt)*)
        at($temp:expr) K
        in($volume:expr) L
    ) => {
        GasMixture {
            gases: $crate::gen_gas_vec!($($t)*),
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
        $crate::GasMixture::with_energy(
            $crate::gen_gas_vec!($($t)*),
            $energy,
            $volume
        )
    };
);

#[macro_export]
macro_rules! temperature {
    ($temp:expr, Kelvin) => {
        $temp
    };
    ($temp:expr, Celcius) => {
        temperature!($temp + $crate::constants::T0C, Kelvin)
    };
}

#[macro_export]
macro_rules! reaction {
    (
        called ($name:ident)
        with ( $($g:path => $ma:expr),+ )
        at ($min_temp:expr)
        with_gm_as ($gm_name:ident) =>
        $code: tt
    ) => {
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
