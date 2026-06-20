fn main() {
    #[cfg(feature = "desktop")]
    btclient::runtime::run();

    #[cfg(not(feature = "desktop"))]
    println!("btclient {}", btclient::version());
}
