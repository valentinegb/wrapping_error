macro_rules! wrapping_error {
    ($vis:vis $enum:ident { $($variant:ident($error:path)),+$(,)? }) => {
        use std::{fmt, error};

        #[derive(Debug)]
        $vis enum $enum {
            $($variant($error),)+
        }

        impl fmt::Display for $enum {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    err => err.fmt(f),
                }
            }
        }

        impl error::Error for $enum {
            fn source(&self) -> Option<&(dyn error::Error + 'static)> {
                match *self {
                    ref err => Some(err),
                }
            }
        }

        $(
            impl From<$error> for $enum {
                fn from(err: $error) -> Self {
                    Self::$variant(err)
                }
            }
        )+
    };
}
