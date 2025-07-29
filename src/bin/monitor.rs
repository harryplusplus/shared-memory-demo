use anyhow::Context;
use anyhow::Result;
use common::SERVER_STATUS_NAME;
use common::ServerStatus;
use nix::fcntl::OFlag;
use nix::sys::mman::MapFlags;
use nix::sys::mman::ProtFlags;
use nix::sys::mman::mmap;
use nix::sys::mman::munmap;
use nix::sys::mman::shm_open;
use nix::sys::stat::Mode;
use std::ffi::c_void;
use std::mem;
use std::num::NonZero;
use std::ptr::NonNull;
use tokio::pin;
use tokio::select;
use tokio::signal::ctrl_c;
use tokio::time::Duration;
use tokio::time::sleep;

struct ServerStatusReader {
    addr: NonNull<ServerStatus>,
}

impl ServerStatusReader {
    fn try_new() -> Result<Self> {
        let fd = shm_open(SERVER_STATUS_NAME, OFlag::O_RDONLY, Mode::S_IRUSR)?;
        let len: usize = mem::size_of::<ServerStatus>();
        let addr = unsafe {
            mmap(
                None,
                NonZero::new(len).context("Shared memory size must not be zero.")?,
                ProtFlags::PROT_READ,
                MapFlags::MAP_SHARED,
                fd,
                0,
            )
        }?
        .cast::<ServerStatus>();
        Ok(ServerStatusReader { addr })
    }

    fn read(&self) -> ServerStatus {
        unsafe { self.addr.as_ptr().read_volatile() }
    }
}

impl Drop for ServerStatusReader {
    fn drop(&mut self) {
        if let Err(e) =
            unsafe { munmap(self.addr.cast::<c_void>(), mem::size_of::<ServerStatus>()) }
        {
            eprintln!("Failed to munmap(). errno: {}", e);
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, I'm a monitor!");
    let status_reader = ServerStatusReader::try_new()?;
    let ctrl_c = ctrl_c();
    pin!(ctrl_c);
    loop {
        select! {
            _ = &mut ctrl_c => {
                println!("Received Ctrl+C signal.");
                break;
            }
            _ = async {
                let status = status_reader.read();
                println!("Server status: {:?}", status);
                sleep(Duration::from_millis(1000)).await;
            } => {}
        }
    }
    Ok(())
}
