pub mod aac;
pub mod aax;
pub mod adc;
pub mod and;
pub mod arr;
pub mod asl;
pub mod asr;
pub mod atx;
pub mod axa;
pub mod axs;
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
pub mod dcp;
pub mod dec;
pub mod dex;
pub mod dey;
pub mod eor;
pub mod inc;
pub mod inx;
pub mod iny;
pub mod isc;
pub mod jmp;
pub mod jsr;
pub mod kil;
pub mod lar;
pub mod lax;
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
pub mod rla;
pub mod rol;
pub mod ror;
pub mod rra;
pub mod rti;
pub mod rts;
pub mod sbc;
pub mod sec;
pub mod sed;
pub mod sei;
pub mod slo;
pub mod sre;
pub mod sta;
pub mod stx;
pub mod sty;
pub mod sxa;
pub mod sya;
pub mod tax;
pub mod tay;
pub mod tsx;
pub mod txa;
pub mod txs;
pub mod tya;
pub mod xaa;
pub mod xas;

pub use aac::*;
pub use aax::*;
pub use adc::*;
pub use and::*;
pub use arr::*;
pub use asl::*;
pub use asr::*;
pub use atx::*;
pub use axa::*;
pub use axs::*;
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
pub use dcp::*;
pub use dec::*;
pub use dex::*;
pub use dey::*;
pub use eor::*;
pub use inc::*;
pub use inx::*;
pub use iny::*;
pub use isc::*;
pub use jmp::*;
pub use jsr::*;
pub use kil::*;
pub use lar::*;
pub use lax::*;
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
pub use rla::*;
pub use rol::*;
pub use ror::*;
pub use rra::*;
pub use rti::*;
pub use rts::*;
pub use sbc::*;
pub use sec::*;
pub use sed::*;
pub use sei::*;
pub use slo::*;
pub use sre::*;
pub use sta::*;
pub use stx::*;
pub use sty::*;
pub use sxa::*;
pub use sya::*;
pub use tax::*;
pub use tay::*;
pub use tsx::*;
pub use txa::*;
pub use txs::*;
pub use tya::*;
pub use xaa::*;
pub use xas::*;

use crate::{Mem, OpCode};

use super::CPU;

#[derive(Debug, nes_emulator_macros::Instruction)]
pub enum Instruction {
    #[opcode(AAC_IMMEDIATE1 | AAC_IMMEDIATE2)]
    ANC(InstructionAAC),
    #[opcode(AAX_ZEROPAGE | AAX_ZEROPAGEY | AAX_INDIRECTX | AAX_ABSOLUTE)]
    SAX(InstructionAAX),
    #[opcode(ADC_IMMEDIATE | ADC_ZEROPAGE | ADC_ZEROPAGEX | ADC_ABSOLUTE | ADC_ABSOLUTEX | ADC_ABSOLUTEY | ADC_INDIRECTX | ADC_INDIRECTY )]
    ADC(InstructionADC),
    #[opcode(AND_IMMEDIATE | AND_ZEROPAGE | AND_ZEROPAGEX | AND_ABSOLUTE | AND_ABSOLUTEX | AND_ABSOLUTEY | AND_INDIRECTX | AND_INDIRECTY )]
    AND(InstructionAND),
    #[opcode(ARR_IMMEDIATE)]
    ARR(InstructionARR),
    #[opcode(ASL_ACCUMULATOR | ASL_ZEROPAGE | ASL_ZEROPAGEX | ASL_ABSOLUTE | ASL_ABSOLUTEX )]
    ASL(InstructionASL),
    #[opcode(ASR_IMMEDIATE)]
    ASR(InstructionASR),
    #[opcode(ATX_IMMEDIATE)]
    LXA(InstructionATX),
    #[opcode(AXA_ABSOLUTEY | AXA_INDIRECTY)]
    SHA(InstructionAXA),
    #[opcode(AXS_IMMEDIATE)]
    SBX(InstructionAXS),
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
    #[opcode(DCP_ZEROPAGE | DCP_ZEROPAGEX | DCP_ABSOLUTE | DCP_ABSOLUTEX | DCP_ABSOLUTEY | DCP_INDIRECTX | DCP_INDIRECTY)]
    DCP(InstructionDCP),
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
    #[opcode(ISC_ZEROPAGE | ISC_ZEROPAGEX | ISC_ABSOLUTE | ISC_ABSOLUTEX | ISC_ABSOLUTEY | ISC_INDIRECTX | ISC_INDIRECTY)]
    ISB(InstructionISC),
    #[opcode(JMP_ABSOLUTE | JMP_INDIRECT )]
    JMP(InstructionJMP),
    #[opcode(jsr::JSR)]
    JSR(InstructionJSR),
    #[opcode(KIL_IMPLIED1 | KIL_IMPLIED2 | KIL_IMPLIED3 | KIL_IMPLIED4 | KIL_IMPLIED5 | KIL_IMPLIED6 | KIL_IMPLIED7 | KIL_IMPLIED8 | KIL_IMPLIED9 | KIL_IMPLIED10 | KIL_IMPLIED11 | KIL_IMPLIED12)]
    JAM(InstructionKIL),
    #[opcode(LAR_ABSOLUTEY)]
    LAE(InstructionLAR),
    #[opcode(LAX_ZEROPAGE | LAX_ZEROPAGEY | LAX_ABSOLUTE | LAX_ABSOLUTEY | LAX_INDIRECTX | LAX_INDIRECTY)]
    LAX(InstructionLAX),
    #[opcode(LDA_IMMEDIATE | LDA_ZEROPAGE | LDA_ZEROPAGEX | LDA_ABSOLUTE | LDA_ABSOLUTEX | LDA_ABSOLUTEY | LDA_INDIRECTX | LDA_INDIRECTY )]
    LDA(InstructionLDA),
    #[opcode(LDX_IMMEDIATE | LDX_ZEROPAGE | LDX_ZEROPAGEY | LDX_ABSOLUTE | LDX_ABSOLUTEY )]
    LDX(InstructionLDX),
    #[opcode(LDY_IMMEDIATE | LDY_ZEROPAGE | LDY_ZEROPAGEX | LDY_ABSOLUTE | LDY_ABSOLUTEX )]
    LDY(InstructionLDY),
    #[opcode(LSR_ACCUMULATOR | LSR_ZEROPAGE | LSR_ZEROPAGEX | LSR_ABSOLUTE | LSR_ABSOLUTEX )]
    LSR(InstructionLSR),
    #[opcode(nop::NOP | DOP_IMMEDIATE1 | DOP_IMMEDIATE2 | DOP_IMMEDIATE3 | DOP_IMMEDIATE4 | DOP_IMMEDIATE5 | DOP_ZEROPAGE1 | DOP_ZEROPAGE2 | DOP_ZEROPAGE3 | DOP_ZEROPAGEX1 | DOP_ZEROPAGEX2 | DOP_ZEROPAGEX3 | DOP_ZEROPAGEX4 | DOP_ZEROPAGEX5 | DOP_ZEROPAGEX6 | NOP_IMPLIED1 | NOP_IMPLIED2 | NOP_IMPLIED3 | NOP_IMPLIED4 | NOP_IMPLIED5 | NOP_IMPLIED6 | TOP_ABSOLUTE | TOP_ABSOLUTEX1 | TOP_ABSOLUTEX2 | TOP_ABSOLUTEX3 | TOP_ABSOLUTEX4 | TOP_ABSOLUTEX5 | TOP_ABSOLUTEX6)]
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
    #[opcode(RLA_ZEROPAGE | RLA_ZEROPAGEX | RLA_ABSOLUTE | RLA_ABSOLUTEX | RLA_ABSOLUTEY | RLA_INDIRECTX | RLA_INDIRECTY)]
    RLA(InstructionRLA),
    #[opcode(ROL_ACCUMULATOR | ROL_ZEROPAGE | ROL_ZEROPAGEX | ROL_ABSOLUTE | ROL_ABSOLUTEX )]
    ROL(InstructionROL),
    #[opcode(ROR_ACCUMULATOR | ROR_ZEROPAGE | ROR_ZEROPAGEX | ROR_ABSOLUTE | ROR_ABSOLUTEX )]
    ROR(InstructionROR),
    #[opcode(RRA_ZEROPAGE | RRA_ZEROPAGEX | RRA_ABSOLUTE | RRA_ABSOLUTEX | RRA_ABSOLUTEY | RRA_INDIRECTX | RRA_INDIRECTY)]
    RRA(InstructionRRA),
    #[opcode(rti::RTI)]
    RTI(InstructionRTI),
    #[opcode(rts::RTS)]
    RTS(InstructionRTS),
    #[opcode(SBC_IMMEDIATE | SBC_IMMEDIATE2 | SBC_ZEROPAGE | SBC_ZEROPAGEX | SBC_ABSOLUTE | SBC_ABSOLUTEX | SBC_ABSOLUTEY | SBC_INDIRECTX | SBC_INDIRECTY )]
    SBC(InstructionSBC),
    #[opcode(sec::SEC)]
    SEC(InstructionSEC),
    #[opcode(sed::SED)]
    SED(InstructionSED),
    #[opcode(sei::SEI)]
    SEI(InstructionSEI),
    #[opcode(SLO_ZEROPAGE | SLO_ZEROPAGEX | SLO_ABSOLUTE | SLO_ABSOLUTEX | SLO_ABSOLUTEY | SLO_INDIRECTX | SLO_INDIRECTY)]
    SLO(InstructionSLO),
    #[opcode(SRE_ZEROPAGE | SRE_ZEROPAGEX | SRE_ABSOLUTE | SRE_ABSOLUTEX | SRE_ABSOLUTEY | SRE_INDIRECTX | SRE_INDIRECTY)]
    SRE(InstructionSRE),
    #[opcode(STA_ZEROPAGE | STA_ZEROPAGEX | STA_ABSOLUTE | STA_ABSOLUTEX | STA_ABSOLUTEY | STA_INDIRECTX | STA_INDIRECTY )]
    STA(InstructionSTA),
    #[opcode(STX_ZEROPAGE | STX_ZEROPAGEY | STX_ABSOLUTE )]
    STX(InstructionSTX),
    #[opcode(STY_ZEROPAGE | STY_ZEROPAGEX | STY_ABSOLUTE )]
    STY(InstructionSTY),
    #[opcode(SXA_ABSOLUTEY)]
    SHX(InstructionSXA),
    #[opcode(SYA_ABSOLUTEX)]
    SHY(InstructionSYA),
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
    #[opcode(XAA_IMMEDIATE)]
    ANE(InstructionXAA),
    #[opcode(XAS_ABSOLUTEY)]
    SHS(InstructionXAS),
}

#[rustfmt::skip]
pub fn is_unofficial_opcode(opcode: u8) -> bool {
    matches!(
        opcode,
        AAC_IMMEDIATE1 | AAC_IMMEDIATE2 | AAX_ZEROPAGE | AAX_ZEROPAGEY | AAX_INDIRECTX
        | AAX_ABSOLUTE | ARR_IMMEDIATE | ASR_IMMEDIATE | ATX_IMMEDIATE | AXA_ABSOLUTEY
        | AXA_INDIRECTY | AXS_IMMEDIATE | DCP_ZEROPAGE | DCP_ZEROPAGEX | DCP_ABSOLUTE
        | DCP_ABSOLUTEX | DCP_ABSOLUTEY | DCP_INDIRECTX | DCP_INDIRECTY | DOP_IMMEDIATE1
        | DOP_IMMEDIATE2 | DOP_IMMEDIATE3 | DOP_IMMEDIATE4 | DOP_IMMEDIATE5 | DOP_ZEROPAGE1
        | DOP_ZEROPAGE2 | DOP_ZEROPAGE3 | DOP_ZEROPAGEX1 | DOP_ZEROPAGEX2 | DOP_ZEROPAGEX3
        | DOP_ZEROPAGEX4 | DOP_ZEROPAGEX5 | DOP_ZEROPAGEX6 | TOP_ABSOLUTE | TOP_ABSOLUTEX1
        | TOP_ABSOLUTEX2 | TOP_ABSOLUTEX3 | TOP_ABSOLUTEX4 | TOP_ABSOLUTEX5 | TOP_ABSOLUTEX6
        | NOP_IMPLIED1 | NOP_IMPLIED2 | NOP_IMPLIED3 | NOP_IMPLIED4 | NOP_IMPLIED5
        | NOP_IMPLIED6 | ISC_ZEROPAGE | ISC_ZEROPAGEX | ISC_ABSOLUTE | ISC_ABSOLUTEX
        | ISC_ABSOLUTEY | ISC_INDIRECTX | ISC_INDIRECTY | KIL_IMPLIED1 | KIL_IMPLIED2
        | KIL_IMPLIED3 | KIL_IMPLIED4 | KIL_IMPLIED5 | KIL_IMPLIED6 | KIL_IMPLIED7
        | KIL_IMPLIED8 | KIL_IMPLIED9 | KIL_IMPLIED10 | KIL_IMPLIED11 | KIL_IMPLIED12
        | LAR_ABSOLUTEY | LAX_ZEROPAGE | LAX_ZEROPAGEY | LAX_ABSOLUTE | LAX_ABSOLUTEY
        | LAX_INDIRECTX | LAX_INDIRECTY | RLA_ZEROPAGE | RLA_ZEROPAGEX | RLA_ABSOLUTE
        | RLA_ABSOLUTEX | RLA_ABSOLUTEY | RLA_INDIRECTX | RLA_INDIRECTY | RRA_ZEROPAGE
        | RRA_ZEROPAGEX | RRA_ABSOLUTE | RRA_ABSOLUTEX | RRA_ABSOLUTEY | RRA_INDIRECTX
        | RRA_INDIRECTY | SBC_IMMEDIATE2 | SLO_ZEROPAGE | SLO_ZEROPAGEX | SLO_ABSOLUTE
        | SLO_ABSOLUTEX | SLO_ABSOLUTEY | SLO_INDIRECTX | SLO_INDIRECTY | SRE_ZEROPAGE
        | SRE_ZEROPAGEX | SRE_ABSOLUTE | SRE_ABSOLUTEX | SRE_ABSOLUTEY | SRE_INDIRECTX
        | SRE_INDIRECTY | SXA_ABSOLUTEY | SYA_ABSOLUTEX | XAA_IMMEDIATE | XAS_ABSOLUTEY
    )
}
