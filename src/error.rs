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
        WsError(::ws::Error);
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