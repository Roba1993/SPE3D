//! Error structs based on `error_chain` for a simple conversion between the different
//! errors of the crates.
//!
//! When you are leveraging this crate with a own `error_chain` implementation, add this
//! trait over the following command.
//!
//! ```
//! # #[macro_use] extern crate error_chain;
//! # extern crate spe3d;
//! # fn main() {
//! error_chain!{
//!     types {
//!         Error, ErrorKind, ResultExt, Result;
//!     }
//!
//!     links {
//!         Spe3d(::spe3d::error::Error, ::spe3d::error::ErrorKind);
//!     }
//! }
//! # }
//! ```

#![allow(renamed_and_removed_lints)]

error_chain!{

    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
        Utf8(::std::str::Utf8Error);
        FromUtf8(::std::string::FromUtf8Error);
        Reqwest(::reqwest::Error);
        Regex(::regex::Error);
        ParseIntError(::std::num::ParseIntError);
        SerdeJson(::serde_json::error::Error);
        Toml(::toml::de::Error);
        RecvError(::std::sync::mpsc::RecvError);
    }

    links{
        Dlc(::dlc_decrypter::error::Error, ::dlc_decrypter::error::ErrorKind);
    }
}

impl<A> From<::std::sync::PoisonError<A>> for Error {
    fn from(_s: ::std::sync::PoisonError<A>) -> Self {
        Error::from("PoisnLockError")
    }
}

impl<A> From<::std::sync::mpsc::SendError<A>> for Error {
    fn from(_s: ::std::sync::mpsc::SendError<A>) -> Self {
        Error::from("SenderError")
    }
}