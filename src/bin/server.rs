use ::common::SERVER_STATUS_NAME;
use ::common::ServerStatus;
use anyhow::Context;
use anyhow::Result;
use nix::fcntl::OFlag;
use nix::sys::mman::MapFlags;
use nix::sys::mman::ProtFlags;
use nix::sys::mman::mmap;
use nix::sys::mman::munmap;
use nix::sys::mman::shm_open;
use nix::sys::mman::shm_unlink;
use nix::sys::stat::Mode;
use nix::unistd::ftruncate;
use std::ffi::c_void;
use std::mem;
use std::num::NonZero;
use std::ptr::NonNull;
use tokio::pin;
use tokio::select;
use tokio::signal::ctrl_c;
use tokio::time::Duration;
use tokio::time::sleep;

struct ServerStatusWriter {
    addr: NonNull<ServerStatus>,
}

impl ServerStatusWriter {
    fn try_new() -> Result<Self> {
        let fd = shm_open(
            SERVER_STATUS_NAME,
            OFlag::O_CREAT | OFlag::O_RDWR,
            Mode::S_IRUSR | Mode::S_IWUSR,
        )?;
        let len: usize = mem::size_of::<ServerStatus>();
        ftruncate(&fd, len.try_into()?)?;
        let addr = unsafe {
            mmap(
                None,
                NonZero::new(len).context("Shared memory size must not be zero.")?,
                ProtFlags::PROT_WRITE,
                MapFlags::MAP_SHARED,
                fd,
                0,
            )
        }?
        .cast::<ServerStatus>();
        Ok(ServerStatusWriter { addr })
    }

    fn write(&self, status: ServerStatus) {
        unsafe { self.addr.as_ptr().write_volatile(status) };
    }
}

impl Drop for ServerStatusWriter {
    fn drop(&mut self) {
        if let Err(e) =
            unsafe { munmap(self.addr.cast::<c_void>(), mem::size_of::<ServerStatus>()) }
        {
            eprintln!("Failed to munmap(). errno: {}", e);
        }

        if let Err(e) = shm_unlink(SERVER_STATUS_NAME) {
            eprintln!("Failed to shm_unlink(). errno: {}", e);
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, I'm a server!");
    let status_writer = ServerStatusWriter::try_new()?;
    let ctrl_c = ctrl_c();
    pin!(ctrl_c);
    let mut user_count: usize = 0;
    loop {
        select! {
            _ = &mut ctrl_c => {
                println!("Received Ctrl+C signal.");
                break;
            }
            _ = async {
                user_count = user_count.wrapping_add(1);
                status_writer.write(ServerStatus { user_count });
                sleep(Duration::from_millis(500)).await;
            } => {}
        }
    }
    Ok(())
}
