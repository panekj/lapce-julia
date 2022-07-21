use lapce_plugin::{register_plugin, start_lsp, LapcePlugin};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default)]
struct State {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    arch: String,
    os: String,
    configuration: Configuration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    system_lsp: bool,
    enabled: bool,
    options: Option<Value>,
}

register_plugin!(State);

impl LapcePlugin for State {
    fn initialize(&mut self, info: serde_json::Value) {
        eprintln!("starting plugin");
        let info = serde_json::from_value::<PluginInfo>(info).unwrap();
        
        if info.configuration.enabled {
            let exec_path = if info.configuration.system_lsp {
                "julia".to_string()
            } else {
                // TODO: implement LSP installation
                "".to_string()
            };

            eprintln!("exec path: {}", &exec_path);

            start_lsp(
                &exec_path,
                "julia",
                info.configuration.options,
                info.configuration.system_lsp,
            );
        }
    }
}
