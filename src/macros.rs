macro_rules! wire_enum {
    ($enum:ident, $wire:ty, $unknown:ident, { $($variant:ident = $value:literal),+ $(,)? }) => {
        impl From<$wire> for $enum {
            fn from(value: $wire) -> Self {
                match value {
                    $($value => Self::$variant,)+
                    other => Self::$unknown(other),
                }
            }
        }

        impl From<$enum> for $wire {
            fn from(value: $enum) -> Self {
                match value {
                    $($enum::$variant => $value,)+
                    $enum::$unknown(value) => value,
                }
            }
        }
    };
}

macro_rules! wire_enum_with_default {
    ($enum:ident, $wire:ty, $fallback:ident, { $($variant:ident = $value:literal),+ $(,)? }) => {
        impl From<$wire> for $enum {
            fn from(value: $wire) -> Self {
                match value {
                    $($value => Self::$variant,)+
                    _ => Self::$fallback,
                }
            }
        }

        impl From<$enum> for $wire {
            fn from(value: $enum) -> Self {
                match value {
                    $($enum::$variant => $value,)+
                }
            }
        }
    };
}
