error_chain! {
    foreign_links {
        Io(std::io::Error);
        StripPrefix(std::path::StripPrefixError);
        WalkDir(walkdir::Error);
        BorrowError(std::cell::BorrowError);
        BorrowMutError(std::cell::BorrowMutError);
        GlobsetError(globset::Error);
    }
}
