#[macro_use]
mod unique_resource;

pub use unique_resource::{ResourceDeleter, UniqueResource};

mod scope_guard;
pub use scope_guard::ScopeGuard;

#[cfg(windows)]
mod window_win32;

#[cfg(windows)]
pub use self::window_win32::{FrameContext, SimpleWindow};

#[cfg(unix)]
mod window_x11;
#[cfg(unix)]
pub use self::window_x11::SimpleWindow;

mod events;
mod keysyms;

pub mod input {
    pub use super::events::*;
    pub use super::keysyms::*;
}

mod memory_mapped_file;
pub use self::memory_mapped_file::MemoryMappedFile;
