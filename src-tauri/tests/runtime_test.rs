use btclient::runtime::registered_command_names;

#[test]
fn runtime_declares_frontend_command_contract() {
    assert_eq!(
        registered_command_names(),
        &[
            "get_app_snapshot",
            "add_torrent",
            "start_task",
            "pause_task",
            "remove_task",
        ]
    );
}

#[cfg(feature = "desktop")]
#[test]
fn desktop_runtime_uses_rqbit_engine() {
    assert_eq!(btclient::runtime::desktop_engine_adapter_name(), "rqbit");
}
