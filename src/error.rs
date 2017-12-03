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
    }

    links{
        Dlc(::dlc_decrypter::error::Error, ::dlc_decrypter::error::ErrorKind);
    }
}