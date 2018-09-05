error_chain!{

    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {  
        WsError(::ws::Error);
        SerdeJson(::serde_json::error::Error);
    }

    links{
        Dlc(::spe3d::error::Error, ::spe3d::error::ErrorKind);
    }
}
