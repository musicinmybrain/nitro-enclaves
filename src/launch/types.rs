// SPDX-License-Identifier: Apache-2.0

use super::error::*;

use bitflags::bitflags;
use std::time::Duration;

/// The image type of the enclave.
#[derive(Debug)]
pub enum ImageType<'a> {
    /// Enclave Image Format.
    Eif(&'a [u8]),
}

/// Data related to setting enclave memory.
#[derive(Debug)]
pub struct MemoryInfo<'a> {
    /// Enclave image type.
    pub image_type: ImageType<'a>,

    /// Amount of memory (in MiB) to allocate to the enclave.
    pub size_mib: usize,
}

impl<'a> MemoryInfo<'a> {
    pub fn new(image_type: ImageType<'a>, size_mib: usize) -> Self {
        Self {
            image_type,
            size_mib,
        }
    }
}

bitflags! {
    /// Configuration flags for starting an enclave.
    #[repr(transparent)]
    #[derive(Copy, Clone, Default)]
    pub struct StartFlags: u64 {
        /// Start enclave in debug mode.
        const DEBUG = 1;
    }
}

/// Calculate an enclave's poll timeout from its image size and the amount of memory allocated to
/// it.
pub struct PollTimeout(pub Duration);

impl TryFrom<(&[u8], usize)> for PollTimeout {
    type Error = LaunchError;

    fn try_from(args: (&[u8], usize)) -> Result<Self, Self::Error> {
        let mul = 60 * 1000; // One minute in milliseconds.
        let size = args.0.len();

        let file: u64 = ((1 + (size - 1) / (6 << 30)) as u64).saturating_mul(mul);
        let alloc: u64 = ((1 + (args.1 - 1) / (100 << 30)) as u64).saturating_mul(mul);

        Ok(Self(Duration::from_millis(file + alloc)))
    }
}

impl PollTimeout {
    /// Get the underlying std::time::Duration of the poll timeout.
    pub fn duration(&self) -> Duration {
        self.0
    }
}
