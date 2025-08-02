#![no_std]
extern crate alloc;

pub mod data_structure;
pub mod sample;

#[macro_export]
macro_rules! problem {
    ($dir:ident, $problem:ident) => {
        #[cfg(test)]
        mod $problem {
            #[test]
            #[ignore]
            fn gen_tests() {
                util::gen_tests(stringify!($dir), stringify!($problem));
            }
            #[test]
            #[ignore]
            fn rm_tests() {
                util::rm_tests(stringify!($dir), stringify!($problem));
            }
            #[test]
            #[ignore]
            fn gen_scripts() {
                util::gen_scripts(stringify!($dir), stringify!($problem), stringify!($problem));
            }
            #[test]
            #[ignore]
            fn rm_scripts() {
                util::rm_scripts(stringify!($problem));
            }
            #[test]
            #[ignore]
            fn bundle() {
                util::bundle(stringify!($dir), stringify!($problem), stringify!($problem));
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
            fn gen_scripts() {
                util::gen_scripts(
                    stringify!($dir),
                    stringify!($problem),
                    stringify!($solution),
                );
            }
            #[test]
            #[ignore]
            fn rm_scripts() {
                util::rm_scripts(stringify!($solution));
            }
            #[test]
            #[ignore]
            fn bundle() {
                util::bundle(
                    stringify!($dir),
                    stringify!($problem),
                    stringify!($solution),
                );
            }
        }
    };
}
