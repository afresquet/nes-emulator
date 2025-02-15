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

use crate::{Mem, OpCode};

use super::CPU;

#[derive(Debug, nes_emulator_macros::Instruction)]
pub enum Instruction {
    #[opcode(ADC_IMMEDIATE | ADC_ZEROPAGE | ADC_ZEROPAGEX | ADC_ABSOLUTE | ADC_ABSOLUTEX | ADC_ABSOLUTEY | ADC_INDIRECTX | ADC_INDIRECTY )]
    ADC(InstructionADC),
    #[opcode(AND_IMMEDIATE | AND_ZEROPAGE | AND_ZEROPAGEX | AND_ABSOLUTE | AND_ABSOLUTEX | AND_ABSOLUTEY | AND_INDIRECTX | AND_INDIRECTY )]
    AND(InstructionAND),
    #[opcode(ASL_ACCUMULATOR | ASL_ZEROPAGE | ASL_ZEROPAGEX | ASL_ABSOLUTE | ASL_ABSOLUTEX )]
    ASL(InstructionASL),
    #[opcode(bcc::BCC)]
    BCC(InstructionBCC),
    #[opcode(bcs::BCS)]
    BCS(InstructionBCS),
    #[opcode(beq::BEQ)]
    BEQ(InstructionBEQ),
    #[opcode(BIT_ZEROPAGE | BIT_ABSOLUTE )]
    BIT(InstructionBIT),
    #[opcode(bmi::BMI)]
    BMI(InstructionBMI),
    #[opcode(bne::BNE)]
    BNE(InstructionBNE),
    #[opcode(bpl::BPL)]
    BPL(InstructionBPL),
    #[opcode(brk::BRK)]
    BRK(InstructionBRK),
    #[opcode(bvc::BVC)]
    BVC(InstructionBVC),
    #[opcode(bvs::BVS)]
    BVS(InstructionBVS),
    #[opcode(clc::CLC)]
    CLC(InstructionCLC),
    #[opcode(cld::CLD)]
    CLD(InstructionCLD),
    #[opcode(cli::CLI)]
    CLI(InstructionCLI),
    #[opcode(clv::CLV)]
    CLV(InstructionCLV),
    #[opcode(CMP_IMMEDIATE | CMP_ZEROPAGE | CMP_ZEROPAGEX | CMP_ABSOLUTE | CMP_ABSOLUTEX | CMP_ABSOLUTEY | CMP_INDIRECTX | CMP_INDIRECTY )]
    CMP(InstructionCMP),
    #[opcode(CPX_IMMEDIATE | CPX_ZEROPAGE | CPX_ABSOLUTE )]
    CPX(InstructionCPX),
    #[opcode(CPY_IMMEDIATE | CPY_ZEROPAGE | CPY_ABSOLUTE )]
    CPY(InstructionCPY),
    #[opcode(DEC_ZEROPAGE | DEC_ZEROPAGEX | DEC_ABSOLUTE | DEC_ABSOLUTEX )]
    DEC(InstructionDEC),
    #[opcode(dex::DEX)]
    DEX(InstructionDEX),
    #[opcode(dey::DEY)]
    DEY(InstructionDEY),
    #[opcode(EOR_IMMEDIATE | EOR_ZEROPAGE | EOR_ZEROPAGEX | EOR_ABSOLUTE | EOR_ABSOLUTEX | EOR_ABSOLUTEY | EOR_INDIRECTX | EOR_INDIRECTY )]
    EOR(InstructionEOR),
    #[opcode(INC_ZEROPAGE | INC_ZEROPAGEX | INC_ABSOLUTE | INC_ABSOLUTEX )]
    INC(InstructionINC),
    #[opcode(inx::INX)]
    INX(InstructionINX),
    #[opcode(iny::INY)]
    INY(InstructionINY),
    #[opcode(JMP_ABSOLUTE | JMP_INDIRECT )]
    JMP(InstructionJMP),
    #[opcode(jsr::JSR)]
    JSR(InstructionJSR),
    #[opcode(LDA_IMMEDIATE | LDA_ZEROPAGE | LDA_ZEROPAGEX | LDA_ABSOLUTE | LDA_ABSOLUTEX | LDA_ABSOLUTEY | LDA_INDIRECTX | LDA_INDIRECTY )]
    LDA(InstructionLDA),
    #[opcode(LDX_IMMEDIATE | LDX_ZEROPAGE | LDX_ZEROPAGEY | LDX_ABSOLUTE | LDX_ABSOLUTEY )]
    LDX(InstructionLDX),
    #[opcode(LDY_IMMEDIATE | LDY_ZEROPAGE | LDY_ZEROPAGEX | LDY_ABSOLUTE | LDY_ABSOLUTEX )]
    LDY(InstructionLDY),
    #[opcode(LSR_ACCUMULATOR | LSR_ZEROPAGE | LSR_ZEROPAGEX | LSR_ABSOLUTE | LSR_ABSOLUTEX )]
    LSR(InstructionLSR),
    #[opcode(nop::NOP)]
    NOP(InstructionNOP),
    #[opcode(ORA_IMMEDIATE | ORA_ZEROPAGE | ORA_ZEROPAGEX | ORA_ABSOLUTE | ORA_ABSOLUTEX | ORA_ABSOLUTEY | ORA_INDIRECTX | ORA_INDIRECTY )]
    ORA(InstructionORA),
    #[opcode(pha::PHA)]
    PHA(InstructionPHA),
    #[opcode(php::PHP)]
    PHP(InstructionPHP),
    #[opcode(pla::PLA)]
    PLA(InstructionPLA),
    #[opcode(plp::PLP)]
    PLP(InstructionPLP),
    #[opcode(ROL_ACCUMULATOR | ROL_ZEROPAGE | ROL_ZEROPAGEX | ROL_ABSOLUTE | ROL_ABSOLUTEX )]
    ROL(InstructionROL),
    #[opcode(ROR_ACCUMULATOR | ROR_ZEROPAGE | ROR_ZEROPAGEX | ROR_ABSOLUTE | ROR_ABSOLUTEX )]
    ROR(InstructionROR),
    #[opcode(rti::RTI)]
    RTI(InstructionRTI),
    #[opcode(rts::RTS)]
    RTS(InstructionRTS),
    #[opcode(SBC_IMMEDIATE | SBC_ZEROPAGE | SBC_ZEROPAGEX | SBC_ABSOLUTE | SBC_ABSOLUTEX | SBC_ABSOLUTEY | SBC_INDIRECTX | SBC_INDIRECTY )]
    SBC(InstructionSBC),
    #[opcode(sec::SEC)]
    SEC(InstructionSEC),
    #[opcode(sed::SED)]
    SED(InstructionSED),
    #[opcode(sei::SEI)]
    SEI(InstructionSEI),
    #[opcode(STA_ZEROPAGE | STA_ZEROPAGEX | STA_ABSOLUTE | STA_ABSOLUTEX | STA_ABSOLUTEY | STA_INDIRECTX | STA_INDIRECTY )]
    STA(InstructionSTA),
    #[opcode(STX_ZEROPAGE | STX_ZEROPAGEY | STX_ABSOLUTE )]
    STX(InstructionSTX),
    #[opcode(STY_ZEROPAGE | STY_ZEROPAGEX | STY_ABSOLUTE )]
    STY(InstructionSTY),
    #[opcode(tax::TAX)]
    TAX(InstructionTAX),
    #[opcode(tay::TAY)]
    TAY(InstructionTAY),
    #[opcode(tsx::TSX)]
    TSX(InstructionTSX),
    #[opcode(txa::TXA)]
    TXA(InstructionTXA),
    #[opcode(txs::TXS)]
    TXS(InstructionTXS),
    #[opcode(tya::TYA)]
    TYA(InstructionTYA),
}
