//! SiFive vendor support.

use probe_rs_target::Chip;

use crate::{
    Error,
    architecture::riscv::communication_interface::RiscvCommunicationInterface,
    config::{DebugSequence, Registry},
    vendor::Vendor,
};

pub mod sequences;

/// SiFive vendor.
#[derive(docsplay::Display)]
pub struct Sifive;

impl Vendor for Sifive {
    fn try_create_debug_sequence(&self, chip: &Chip) -> Option<DebugSequence> {
        let sequence = if chip.name.starts_with("FU740") {
            sequences::SifiveSequence::create()
        } else {
            return None;
        };

        Some(DebugSequence::Riscv(sequence))
    }

    fn try_detect_riscv_chip(
        &self,
        _registry: &Registry,
        _probe: &mut RiscvCommunicationInterface,
        idcode: u32,
    ) -> Result<Option<String>, Error> {
        // FU740-C000 JTAG IDCODE: 0x20000913
        // Version=2, Part=0, Manufacturer=0x489 (SiFive, JEP106 bank 10 id 0x09)
        if idcode == 0x2000_0913 {
            tracing::info!(
                "SifiveVendor: detected FU740-C000 via IDCODE {:#010x}",
                idcode
            );
            return Ok(Some("FU740-C000".to_string()));
        }

        Ok(None)
    }
}
