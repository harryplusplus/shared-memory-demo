use anyhow::Context;
use anyhow::Result;
use core::GAME_SERVER_STATUS_NAME;
use core::GameServerStatus;
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
use std::ptr::write_volatile;

struct GameServerStatusWriter {
    addr: NonNull<GameServerStatus>,
}

impl GameServerStatusWriter {
    fn try_new() -> Result<Self> {
        let fd = shm_open(
            GAME_SERVER_STATUS_NAME,
            OFlag::O_CREAT | OFlag::O_RDWR,
            Mode::S_IWUSR,
        )?;
        let len: usize = mem::size_of::<GameServerStatus>();
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
        .cast::<GameServerStatus>();
        Ok(GameServerStatusWriter { addr })
    }

    fn write(&self, status: GameServerStatus) {
        unsafe { write_volatile(self.addr.as_ptr(), status) };
    }
}

impl Drop for GameServerStatusWriter {
    fn drop(&mut self) {
        if let Err(e) = unsafe {
            munmap(
                self.addr.cast::<c_void>(),
                mem::size_of::<GameServerStatus>(),
            )
        } {
            eprintln!("Failed to munmap. errno: {}", e);
        }

        if let Err(e) = shm_unlink(GAME_SERVER_STATUS_NAME) {
            eprintln!("Failed to shm_unlink. errno: {}", e);
        }
    }
}

fn main() -> Result<()> {
    println!("Hello, I'm a game server!");
    let status_writer = GameServerStatusWriter::try_new()?;
    status_writer.write(GameServerStatus { user_count: 1 });
    Ok(())
}
