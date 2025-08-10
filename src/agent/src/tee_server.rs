pub mod binding {
    #![allow(warnings)]
    rust2go::r2g_include_binding!();
}

#[derive(rust2go::R2G, Clone)]
pub struct CrossRequest {
    pub data: Vec<u8>,
}

#[derive(rust2go::R2G, Clone, Copy)]
pub struct CrossResponse {
    pub pass: bool,
}

// Server api
#[rust2go::r2g]
pub trait TEEServer {
    fn start(
        req: &CrossRequest,
    ) -> impl std::future::Future<Output = CrossResponse>;
}
