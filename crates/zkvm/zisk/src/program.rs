use serde::{Deserialize, Serialize};

/// Zisk program that contains ELF of compiled guest.
#[derive(Clone, Serialize, Deserialize)]
pub struct ZiskProgram {
    pub(crate) elf: Vec<u8>,
}

impl ZiskProgram {
    pub fn from_elf(elf: Vec<u8>) -> Self {
        Self { elf }
    }

    pub fn elf(&self) -> &[u8] {
        &self.elf
    }
}
