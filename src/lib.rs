pub static GAME_SERVER_STATUS_NAME: &'static str = "/game_server_status";

#[repr(C)]
#[derive(Debug, Default)]
pub struct GameServerStatus {
    pub user_count: u32,
}
