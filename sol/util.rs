#[macro_export]
macro_rules! problem {
    ($dir:ident, $problem:ident) => {
        #[cfg(test)]
        mod $problem {
            #[test]
            #[ignore]
            fn gendata() {
                util::gendata(stringify!($dir), stringify!($problem));
            }
            #[test]
            #[ignore]
            fn rmdata() {
                util::rmdata(stringify!($dir), stringify!($problem));
            }
            #[test]
            #[ignore]
            fn gencode() {
                util::gencode(stringify!($dir), stringify!($problem), stringify!($problem));
            }
            #[test]
            #[ignore]
            fn rmcode() {
                util::rmcode(stringify!($problem));
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
            fn gencode() {
                util::gencode(
                    stringify!($dir),
                    stringify!($problem),
                    stringify!($solution),
                );
            }
            #[test]
            #[ignore]
            fn rmcode() {
                util::rmcode(stringify!($solution));
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
