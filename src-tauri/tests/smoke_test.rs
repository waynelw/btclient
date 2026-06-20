#[test]
fn crate_exposes_version() {
    assert_eq!(btclient::version(), env!("CARGO_PKG_VERSION"));
}
