//! An anti-boilerplate crate for errors that wrap errors.
//!
//! This crate only exports one item: the [`wrapping_error`] macro. See that for documentation.

/// Creates an error that wraps other errors.
///
/// ## Example
///
/// ```
/// use std::{env::VarError, io};
///
/// use wrapping_error::wrapping_error;
///
/// wrapping_error!(pub(crate) AppDataError {
///     Var(VarError) => "could not get $HOME environment variable",
///     Io(io::Error) => "failed to read/write app data",
///     Postcard(postcard::Error) => "failed to serialize/deserialize app data",
/// });
/// ```
#[macro_export]
macro_rules! wrapping_error {
    ($vis:vis $enum:ident { $($variant:ident($error:path)$( => $message:literal)?),+$(,)? }) => {
        use std::{fmt, error};

        #[derive(Debug)]
        $vis enum $enum {
            $($variant($error),)+
        }

        impl fmt::Display for $enum {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    // TODO: find a way to make $error optional and be able to tie (..) to whether it exists
                    $($(Self::$variant(..) => write!(f, $message),)?)+
                    ref err => err.fmt(f),
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
