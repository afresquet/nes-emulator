pub mod adc;
pub mod and;
pub mod asl;
pub mod bcc;
pub mod bcs;
pub mod beq;
pub mod bit;
pub mod bmi;
pub mod bne;
pub mod bpl;
pub mod brk;
pub mod bvc;
pub mod bvs;
pub mod clc;
pub mod cld;
pub mod cli;
pub mod clv;
pub mod cmp;
pub mod cpx;
pub mod cpy;
pub mod dec;
pub mod dex;
pub mod dey;
pub mod eor;
pub mod inc;
pub mod inx;
pub mod iny;
pub mod jmp;
pub mod jsr;
pub mod lda;
pub mod ldx;
pub mod ldy;
pub mod lsr;
pub mod nop;
pub mod ora;
pub mod pha;
pub mod php;
pub mod pla;
pub mod plp;
pub mod rol;
pub mod ror;
pub mod rti;
pub mod rts;
pub mod sbc;
pub mod sec;
pub mod sed;
pub mod sei;
pub mod sta;
pub mod stx;
pub mod sty;
pub mod tax;
pub mod tay;
pub mod tsx;
pub mod txa;
pub mod txs;
pub mod tya;

pub use adc::*;
pub use and::*;
pub use asl::*;
pub use bcc::*;
pub use bcs::*;
pub use beq::*;
pub use bit::*;
pub use bmi::*;
pub use bne::*;
pub use bpl::*;
pub use brk::*;
pub use bvc::*;
pub use bvs::*;
pub use clc::*;
pub use cld::*;
pub use cli::*;
pub use clv::*;
pub use cmp::*;
pub use cpx::*;
pub use cpy::*;
pub use dec::*;
pub use dex::*;
pub use dey::*;
pub use eor::*;
pub use inc::*;
pub use inx::*;
pub use iny::*;
pub use jmp::*;
pub use jsr::*;
pub use lda::*;
pub use ldx::*;
pub use ldy::*;
pub use lsr::*;
pub use nop::*;
pub use ora::*;
pub use pha::*;
pub use php::*;
pub use pla::*;
pub use plp::*;
pub use rol::*;
pub use ror::*;
pub use rti::*;
pub use rts::*;
pub use sbc::*;
pub use sec::*;
pub use sed::*;
pub use sei::*;
pub use sta::*;
pub use stx::*;
pub use sty::*;
pub use tax::*;
pub use tay::*;
pub use tsx::*;
pub use txa::*;
pub use txs::*;
pub use tya::*;

pub enum Address {
    Accumulator(u8),
    Memory { addr: u16, value: u8 },
}

impl Address {
    pub fn value(&self) -> u8 {
        match self {
            Address::Accumulator(value) | Address::Memory { value, .. } => *value,
        }
    }
}
