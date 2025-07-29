pub static SERVER_STATUS_NAME: &'static str = "/server_status";

#[repr(C)]
#[derive(Debug, Default)]
pub struct ServerStatus {
    pub user_count: usize,
}
