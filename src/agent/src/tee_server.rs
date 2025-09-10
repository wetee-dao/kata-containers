use anyhow::{anyhow, Result};
use oci::Spec;
use oci_spec::runtime as oci;
use std::{collections::HashMap, sync::OnceLock};

pub static INIT_DATA: OnceLock<HashMap<String, String>> = OnceLock::new();
pub static IMAGES: OnceLock<Vec<String>> = OnceLock::new();

pub mod binding {
    #![allow(warnings)]
    rust2go::r2g_include_binding!();
}

#[derive(rust2go::R2G, Clone)]
pub struct CrossRequest {
    pub env: Vec<u8>,
    pub data: Vec<u8>,
}

#[derive(rust2go::R2G, Clone)]
pub struct CrossResponse {
    pub code: u8,
    pub images: Vec<String>,
    pub data: Vec<u8>,
}

// Server api
#[rust2go::r2g]
pub trait TEEServer {
    fn start(req: &CrossRequest) -> impl std::future::Future<Output = CrossResponse>;
    fn secret_mount(
        major: i64,
        minor: i64,
        mount_path: String,
    ) -> impl std::future::Future<Output = CrossResponse>;
    fn stop(t: i64) -> impl std::future::Future<Output = CrossResponse>;
}

#[allow(dead_code)]
pub async fn secure_mount(cid: &str, oci: &mut Spec, sl: slog::Logger) -> Result<()> {
    let linux = oci
        .linux()
        .as_ref()
        .ok_or_else(|| anyhow!("Spec didn't contain linux field"))?;
    if let Some(devices) = linux.devices() {
        for specdev in devices.iter() {
            let path = specdev.path().as_path().to_str();
            if path.is_none() {
                continue;
            }
            if let Some(mount_path) = convert_secret_dev_path(path.unwrap()) {
                info!(sl, "WeTEELOG mount device major:min {:?}", specdev,);
                let cresp = unsafe {
                    TEEServerImpl::secret_mount(
                        specdev.major(),
                        specdev.minor(),
                        format!(
                            "/run/kata-containers/{}/rootfs{}",
                            cid,
                            mount_path,
                        )
                        .to_string(),
                    )
                    .await
                };
                if cresp.code != 0 {
                    return Err(anyhow!(
                        "TEE secure mount error: {:?}",
                        String::from_utf8(cresp.data).unwrap().as_str()
                    ));
                }
            }
        }
    }

    Ok(())
}

fn convert_secret_dev_path(input: &str) -> Option<String> {
    // 定义需要移除的前缀
    const PREFIX: &str = "/dev/secret_";

    // 检查输入是否以前缀开头
    if input.starts_with(PREFIX) {
        // 截取前缀后的部分
        let suffix = &input[PREFIX.len()..];

        // 将第一个下划线替换为斜杠（处理 srv☯xxx -> srv/xxx）
        let transformed_suffix = suffix.replace('☯', "/");

        // 拼接为目标路径
        Some(format!("/{}", transformed_suffix))
    } else {
        // 若不匹配前缀，返回 None
        None
    }
}
