#[macro_export]
macro_rules! problem {
    ($dir:ident, $problem:ident) => {
        #[cfg(test)]
        mod $problem {
            #[test]
            #[ignore]
            fn gen_testcases() {
                util::gen_testcases(stringify!($dir), stringify!($problem));
            }
            #[test]
            #[ignore]
            fn rm_testcases() {
                util::rm_testcases(stringify!($dir), stringify!($problem));
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
