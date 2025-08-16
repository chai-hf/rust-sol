#![no_std]
extern crate alloc;

pub mod sample;

#[macro_export]
macro_rules! problem {
    ($dir:ident, $problem:ident) => {
        #[cfg(test)]
        mod $problem {
            #[test]
            #[ignore]
            fn gen_data() {
                util::gen_data(stringify!($dir), stringify!($problem));
            }
            #[test]
            #[ignore]
            fn rm_data() {
                util::rm_data(stringify!($dir), stringify!($problem));
            }
            #[test]
            #[ignore]
            fn gen_code() {
                util::gen_code(stringify!($dir), stringify!($problem), stringify!($problem));
            }
            #[test]
            #[ignore]
            fn rm_code() {
                util::rm_code(stringify!($problem));
            }
        }
    };
}

#[macro_export]
macro_rules! solution {
    ($dir:ident, $problem:ident,$solution:ident) => {
        #[cfg(test)]
        mod $solution {
            #[test]
            #[ignore]
            fn gen_code() {
                util::gen_code(
                    stringify!($dir),
                    stringify!($problem),
                    stringify!($solution),
                );
            }
            #[test]
            #[ignore]
            fn rm_code() {
                util::rm_code(stringify!($solution));
            }
        }
    };
}
