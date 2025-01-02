fn main() {
    #[cfg(feature = "parse")]
    lalrpop::process_src().unwrap();
}
