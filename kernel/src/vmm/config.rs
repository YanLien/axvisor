use std::string::ToString;

use alloc::vec::Vec;
use anyhow::bail;
use axvm::config::AxVMCrateConfig;

pub fn init_guest_vms() -> anyhow::Result<()> {
    let vm_configs = get_guest_prelude_vmconfig()?;

    for config in vm_configs.iter() {
        init_guest_vm(config)?;
    }

    Ok(())
}

fn get_guest_prelude_vmconfig() -> anyhow::Result<Vec<AxVMCrateConfig>> {
    let mut vm_configs = Vec::new();
    // First try to get configs from filesystem if fs feature is enabled
    let mut gvm_raw_configs = config::filesystem_vm_configs();

    // If no filesystem configs found, fallback to static configs
    if gvm_raw_configs.is_empty() {
        let static_configs = config::static_vm_configs();
        if static_configs.is_empty() {
            info!("Static VM configs are empty.");
            info!("Now axvisor will entry the shell...");
        } else {
            info!("Using static VM configs.");
        }

        gvm_raw_configs.extend(static_configs.into_iter().map(|s| s.to_string()));
    }
    for raw in gvm_raw_configs {
        let vm_config: AxVMCrateConfig = toml::from_str(&raw)?;
        vm_configs.push(vm_config);
    }

    Ok(vm_configs)
}

fn init_guest_vm(config: &AxVMCrateConfig) -> anyhow::Result<()> {
    debug!("Initializing guest VM `{}`", config.base.name);

    Ok(())
}

#[allow(clippy::module_inception, dead_code)]
pub mod config {
    use alloc::string::String;
    use alloc::vec::Vec;

    /// Default static VM configs. Used when no VM config is provided.
    pub fn default_static_vm_configs() -> Vec<&'static str> {
        vec![]
    }

    /// Read VM configs from filesystem
    #[cfg(feature = "fs")]
    pub fn filesystem_vm_configs() -> Vec<String> {
        use axstd::fs;
        use axstd::io::{BufReader, Read};

        let config_dir = "/guest/vm_default";

        let mut configs = Vec::new();

        debug!("Read VM config files from filesystem.");

        let entries = match fs::read_dir(config_dir) {
            Ok(entries) => {
                info!("Find dir: {config_dir}");
                entries
            }
            Err(_e) => {
                info!("NOT find dir: {config_dir} in filesystem");
                return configs;
            }
        };

        for entry in entries.flatten() {
            let path = entry.path();
            // Check if the file has a .toml extension
            let path_str = path.as_str();
            debug!("Considering file: {path_str}");
            if path_str.ends_with(".toml") {
                let toml_file = fs::File::open(path_str).expect("Failed to open file");
                let file_size = toml_file
                    .metadata()
                    .expect("Failed to get file metadata")
                    .len() as usize;

                info!("File {path_str} size: {file_size}");

                if file_size == 0 {
                    warn!("File {path_str} is empty");
                    continue;
                }

                let mut file = BufReader::new(toml_file);
                let mut buffer = vec![0u8; file_size];
                match file.read_exact(&mut buffer) {
                    Ok(()) => {
                        debug!(
                            "Successfully read config file {} as bytes, size: {}",
                            path_str,
                            buffer.len()
                        );
                        // Convert to string
                        let content = alloc::string::String::from_utf8(buffer)
                            .expect("Failed to convert bytes to UTF-8 string");

                        if content.contains("[base]")
                            && content.contains("[kernel]")
                            && content.contains("[devices]")
                        {
                            configs.push(content);
                            info!(
                                "TOML config: {path_str} is valid, start the virtual machine directly now. "
                            );
                        } else {
                            warn!(
                                "File {path_str} does not appear to contain valid VM config structure"
                            );
                        }
                    }
                    Err(e) => {
                        error!("Failed to read file {path_str}: {e:?}");
                    }
                }
            }
        }

        configs
    }

    /// Fallback function for when "fs" feature is not enabled
    #[cfg(not(feature = "fs"))]
    pub fn filesystem_vm_configs() -> Vec<String> {
        Vec::new()
    }

    include!(concat!(env!("OUT_DIR"), "/vm_configs.rs"));
}
