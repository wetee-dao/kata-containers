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
    fn start(
        req: &CrossRequest,
    ) -> impl std::future::Future<Output = CrossResponse>;
}
