fn main() {
    let mut config = prost_build::Config::new();
    config.btree_map(&[".ble_ws.OverviewResponse"]);
    tonic_build::configure()
        .compile_with_config(config, &["proto/ble-ws.proto"], &["proto"])
        .unwrap();
}
