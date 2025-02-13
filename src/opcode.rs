use std::{collections::HashMap, sync::LazyLock};

use crate::{instructions::*, CPU};

#[derive(Debug, Clone)]
pub struct OpCode {
    pub ty: OpCodeType,
    pub mode: AddressingMode,
    pub bytes: u8,
    pub cycles: u8,
    pub instruction: fn(&mut CPU, &Self),
}

impl OpCode {
    pub fn new(
        ty: OpCodeType,
        mode: AddressingMode,
        bytes: u8,
        cycles: u8,
        instruction: fn(&mut CPU, &Self),
    ) -> Self {
        Self {
            ty,
            mode,
            bytes,
            cycles,
            instruction,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    NoneAddressing,
    Accumulator,
    Relative,
    Implied,
}

pub static OPCODES: LazyLock<HashMap<u8, OpCode>> = LazyLock::new(|| {
    use AddressingMode::*;
    use OpCodeType::*;

    HashMap::from([
        // ADC
        (ADC_IMMEDIATE, OpCode::new(ADC, Immediate, 2, 2, adc)),
        (ADC_ZEROPAGE, OpCode::new(ADC, ZeroPage, 2, 3, adc)),
        (ADC_ZEROPAGEX, OpCode::new(ADC, ZeroPageX, 2, 4, adc)),
        (ADC_ABSOLUTE, OpCode::new(ADC, Absolute, 3, 4, adc)),
        (ADC_ABSOLUTEX, OpCode::new(ADC, AbsoluteX, 3, 4, adc)),
        (ADC_ABSOLUTEY, OpCode::new(ADC, AbsoluteY, 3, 4, adc)),
        (ADC_INDIRECTX, OpCode::new(ADC, IndirectX, 2, 6, adc)),
        (ADC_INDIRECTY, OpCode::new(ADC, IndirectY, 2, 5, adc)),
        // AND
        (AND_IMMEDIATE, OpCode::new(AND, Immediate, 2, 2, and)),
        (AND_ZEROPAGE, OpCode::new(AND, ZeroPage, 2, 3, and)),
        (AND_ZEROPAGEX, OpCode::new(AND, ZeroPageX, 2, 4, and)),
        (AND_ABSOLUTE, OpCode::new(AND, Absolute, 3, 4, and)),
        (AND_ABSOLUTEX, OpCode::new(AND, AbsoluteX, 3, 4, and)),
        (AND_ABSOLUTEY, OpCode::new(AND, AbsoluteY, 3, 4, and)),
        (AND_INDIRECTX, OpCode::new(AND, IndirectX, 2, 6, and)),
        (AND_INDIRECTY, OpCode::new(AND, IndirectY, 2, 5, and)),
        // ASL
        (ASL_ACCUMULATOR, OpCode::new(ASL, Accumulator, 1, 2, asl)),
        (ASL_ZEROPAGE, OpCode::new(ASL, ZeroPage, 2, 5, asl)),
        (ASL_ZEROPAGEX, OpCode::new(ASL, ZeroPageX, 2, 6, asl)),
        (ASL_ABSOLUTE, OpCode::new(ASL, Absolute, 3, 6, asl)),
        (ASL_ABSOLUTEX, OpCode::new(ASL, AbsoluteX, 3, 7, asl)),
        // BCC
        (bcc::BCC, OpCode::new(BCC, Relative, 2, 2, bcc)),
        // BCS
        (bcs::BCS, OpCode::new(BCS, Relative, 2, 2, bcs)),
        // BEQ
        (beq::BEQ, OpCode::new(BEQ, Relative, 2, 2, beq)),
        // BIT
        (BIT_ZEROPAGE, OpCode::new(BIT, ZeroPage, 2, 3, bit)),
        (BIT_ABSOLUTE, OpCode::new(BIT, Absolute, 3, 4, bit)),
        // BMI
        (bmi::BMI, OpCode::new(BMI, Relative, 2, 2, bmi)),
        // BNE
        (bne::BNE, OpCode::new(BNE, Relative, 2, 2, bne)),
        // BPL
        (bpl::BPL, OpCode::new(BPL, Relative, 2, 2, bpl)),
        // BRK
        (brk::BRK, OpCode::new(BRK, Implied, 1, 7, brk)),
        // BVC
        (bvc::BVC, OpCode::new(BVC, Relative, 2, 2, bvc)),
        // BVS
        (bvs::BVS, OpCode::new(BVS, Relative, 2, 2, bvs)),
        // CLC
        (clc::CLC, OpCode::new(CLC, Implied, 1, 2, clc)),
        // CLD
        (cld::CLD, OpCode::new(CLD, Implied, 1, 2, cld)),
        // CLI
        (cli::CLI, OpCode::new(CLI, Implied, 1, 2, cli)),
        // CLV
        (clv::CLV, OpCode::new(CLV, Implied, 1, 2, clv)),
        // CMP
        (CMP_IMMEDIATE, OpCode::new(CMP, Immediate, 2, 2, cmp)),
        (CMP_ZEROPAGE, OpCode::new(CMP, ZeroPage, 2, 3, cmp)),
        (CMP_ZEROPAGEX, OpCode::new(CMP, ZeroPageX, 2, 4, cmp)),
        (CMP_ABSOLUTE, OpCode::new(CMP, Absolute, 3, 4, cmp)),
        (CMP_ABSOLUTEX, OpCode::new(CMP, AbsoluteX, 3, 4, cmp)),
        (CMP_ABSOLUTEY, OpCode::new(CMP, AbsoluteY, 3, 4, cmp)),
        (CMP_INDIRECTX, OpCode::new(CMP, IndirectX, 2, 6, cmp)),
        (CMP_INDIRECTY, OpCode::new(CMP, IndirectY, 2, 5, cmp)),
        // CPX
        (CPX_IMMEDIATE, OpCode::new(CPX, Immediate, 2, 2, cpx)),
        (CPX_ZEROPAGE, OpCode::new(CPX, ZeroPage, 2, 3, cpx)),
        (CPX_ABSOLUTE, OpCode::new(CPX, Absolute, 3, 4, cpx)),
        // CPY
        (CPY_IMMEDIATE, OpCode::new(CPY, Immediate, 2, 2, cpy)),
        (CPY_ZEROPAGE, OpCode::new(CPY, ZeroPage, 2, 3, cpy)),
        (CPY_ABSOLUTE, OpCode::new(CPY, Absolute, 3, 4, cpy)),
        // DEC
        (DEC_ZEROPAGE, OpCode::new(DEC, ZeroPage, 2, 5, dec)),
        (DEC_ZEROPAGEX, OpCode::new(DEC, ZeroPageX, 2, 6, dec)),
        (DEC_ABSOLUTE, OpCode::new(DEC, Absolute, 3, 6, dec)),
        (DEC_ABSOLUTEX, OpCode::new(DEC, AbsoluteX, 3, 7, dec)),
        // DEX
        (dex::DEX, OpCode::new(DEX, Implied, 1, 2, dex)),
        // DEY
        (dey::DEY, OpCode::new(DEY, Implied, 1, 2, dey)),
        // EOR
        (EOR_IMMEDIATE, OpCode::new(EOR, Immediate, 2, 2, eor)),
        (EOR_ZEROPAGE, OpCode::new(EOR, ZeroPage, 2, 3, eor)),
        (EOR_ZEROPAGEX, OpCode::new(EOR, ZeroPageX, 2, 4, eor)),
        (EOR_ABSOLUTE, OpCode::new(EOR, Absolute, 3, 4, eor)),
        (EOR_ABSOLUTEX, OpCode::new(EOR, AbsoluteX, 3, 4, eor)),
        (EOR_ABSOLUTEY, OpCode::new(EOR, AbsoluteY, 3, 4, eor)),
        (EOR_INDIRECTX, OpCode::new(EOR, IndirectX, 2, 6, eor)),
        (EOR_INDIRECTY, OpCode::new(EOR, IndirectY, 2, 5, eor)),
        // INC
        (INC_ZEROPAGE, OpCode::new(INC, ZeroPage, 2, 5, inc)),
        (INC_ZEROPAGEX, OpCode::new(INC, ZeroPageX, 2, 6, inc)),
        (INC_ABSOLUTE, OpCode::new(INC, Absolute, 3, 6, inc)),
        (INC_ABSOLUTEX, OpCode::new(INC, AbsoluteX, 3, 7, inc)),
        // INX
        (inx::INX, OpCode::new(INX, Implied, 1, 2, inx)),
        // INY
        (iny::INY, OpCode::new(INY, Implied, 1, 2, iny)),
        // JMP
        (
            JMP_ABSOLUTE,
            OpCode::new(JMP, Absolute, /* should be 3? */ 1, 3, jmp),
        ),
        (
            JMP_INDIRECT,
            OpCode::new(JMP, Indirect, /* should be 3? */ 1, 5, jmp),
        ),
        // JSR
        (0x20, OpCode::new(JSR, Absolute, 3, 6, jsr)),
        // LDA
        (LDA_IMMEDIATE, OpCode::new(LDA, Immediate, 2, 2, lda)),
        (LDA_ZEROPAGE, OpCode::new(LDA, ZeroPage, 2, 3, lda)),
        (LDA_ZEROPAGEX, OpCode::new(LDA, ZeroPageX, 2, 4, lda)),
        (LDA_ABSOLUTE, OpCode::new(LDA, Absolute, 3, 4, lda)),
        (LDA_ABSOLUTEX, OpCode::new(LDA, AbsoluteX, 3, 4, lda)),
        (LDA_ABSOLUTEY, OpCode::new(LDA, AbsoluteY, 3, 4, lda)),
        (LDA_INDIRECTX, OpCode::new(LDA, IndirectX, 2, 6, lda)),
        (LDA_INDIRECTY, OpCode::new(LDA, IndirectY, 2, 5, lda)),
        // LDX
        (LDX_IMMEDIATE, OpCode::new(LDX, Immediate, 2, 2, ldx)),
        (LDX_ZEROPAGE, OpCode::new(LDX, ZeroPage, 2, 3, ldx)),
        (LDX_ZEROPAGEY, OpCode::new(LDX, ZeroPageY, 2, 4, ldx)),
        (LDX_ABSOLUTE, OpCode::new(LDX, Absolute, 3, 4, ldx)),
        (LDX_ABSOLUTEY, OpCode::new(LDX, AbsoluteY, 3, 4, ldx)),
        // LDY
        (LDY_IMMEDIATE, OpCode::new(LDY, Immediate, 2, 2, ldy)),
        (LDY_ZEROPAGE, OpCode::new(LDY, ZeroPage, 2, 3, ldy)),
        (LDY_ZEROPAGEX, OpCode::new(LDY, ZeroPageX, 2, 4, ldy)),
        (LDY_ABSOLUTE, OpCode::new(LDY, Absolute, 3, 4, ldy)),
        (LDY_ABSOLUTEX, OpCode::new(LDY, AbsoluteX, 3, 4, ldy)),
        // LSR
        (LSR_ACCUMULATOR, OpCode::new(LSR, Accumulator, 1, 2, lsr)),
        (LSR_ZEROPAGE, OpCode::new(LSR, ZeroPage, 2, 5, lsr)),
        (LSR_ZEROPAGEX, OpCode::new(LSR, ZeroPageX, 2, 6, lsr)),
        (LSR_ABSOLUTE, OpCode::new(LSR, Absolute, 3, 6, lsr)),
        (LSR_ABSOLUTEX, OpCode::new(LSR, AbsoluteX, 3, 7, lsr)),
        // NOP
        (nop::NOP, OpCode::new(NOP, Implied, 1, 2, nop)),
        // ORA
        (ORA_IMMEDIATE, OpCode::new(ORA, Immediate, 2, 2, ora)),
        (ORA_ZEROPAGE, OpCode::new(ORA, ZeroPage, 2, 3, ora)),
        (ORA_ZEROPAGEX, OpCode::new(ORA, ZeroPageX, 2, 4, ora)),
        (ORA_ABSOLUTE, OpCode::new(ORA, Absolute, 3, 4, ora)),
        (ORA_ABSOLUTEX, OpCode::new(ORA, AbsoluteX, 3, 4, ora)),
        (ORA_ABSOLUTEY, OpCode::new(ORA, AbsoluteY, 3, 4, ora)),
        (ORA_INDIRECTX, OpCode::new(ORA, IndirectX, 2, 6, ora)),
        (ORA_INDIRECTY, OpCode::new(ORA, IndirectY, 2, 5, ora)),
        // PHA
        (pha::PHA, OpCode::new(PHA, Implied, 1, 3, pha)),
        // PHP
        (php::PHP, OpCode::new(PHP, Implied, 1, 3, php)),
        // PLA
        (pla::PLA, OpCode::new(PLA, Implied, 1, 4, pla)),
        // PLP
        (plp::PLP, OpCode::new(PLP, Implied, 1, 4, plp)),
        // ROL
        (ROL_ACCUMULATOR, OpCode::new(ROL, Accumulator, 1, 2, rol)),
        (ROL_ZEROPAGE, OpCode::new(ROL, ZeroPage, 2, 5, rol)),
        (ROL_ZEROPAGEX, OpCode::new(ROL, ZeroPageX, 2, 6, rol)),
        (ROL_ABSOLUTE, OpCode::new(ROL, Absolute, 3, 6, rol)),
        (ROL_ABSOLUTEX, OpCode::new(ROL, AbsoluteX, 3, 7, rol)),
        // ROR
        (ROR_ACCUMULATOR, OpCode::new(ROR, Accumulator, 1, 2, ror)),
        (ROR_ZEROPAGE, OpCode::new(ROR, ZeroPage, 2, 5, ror)),
        (ROR_ZEROPAGEX, OpCode::new(ROR, ZeroPageX, 2, 6, ror)),
        (ROR_ABSOLUTE, OpCode::new(ROR, Absolute, 3, 6, ror)),
        (ROR_ABSOLUTEX, OpCode::new(ROR, AbsoluteX, 3, 7, ror)),
        // RTI
        (0x40, OpCode::new(RTI, Implied, 1, 6, rti)),
        // RTS
        (0x60, OpCode::new(RTS, Implied, 1, 6, rts)),
        // SBC
        (SBC_IMMEDIATE, OpCode::new(SBC, Immediate, 2, 2, sbc)),
        (SBC_ZEROPAGE, OpCode::new(SBC, ZeroPage, 2, 3, sbc)),
        (SBC_ZEROPAGEX, OpCode::new(SBC, ZeroPageX, 2, 4, sbc)),
        (SBC_ABSOLUTE, OpCode::new(SBC, Absolute, 3, 4, sbc)),
        (SBC_ABSOLUTEX, OpCode::new(SBC, AbsoluteX, 3, 4, sbc)),
        (SBC_ABSOLUTEY, OpCode::new(SBC, AbsoluteY, 3, 4, sbc)),
        (SBC_INDIRECTX, OpCode::new(SBC, IndirectX, 2, 6, sbc)),
        (SBC_INDIRECTY, OpCode::new(SBC, IndirectY, 2, 5, sbc)),
        // SEC
        (sec::SEC, OpCode::new(SEC, Implied, 1, 2, sec)),
        // SED
        (sed::SED, OpCode::new(SED, Implied, 1, 2, sed)),
        // SEI
        (sei::SEI, OpCode::new(SEI, Implied, 1, 2, sei)),
        // STA
        (STA_ZEROPAGE, OpCode::new(STA, ZeroPage, 2, 3, sta)),
        (STA_ZEROPAGEX, OpCode::new(STA, ZeroPageX, 2, 4, sta)),
        (STA_ABSOLUTE, OpCode::new(STA, Absolute, 3, 4, sta)),
        (STA_ABSOLUTEX, OpCode::new(STA, AbsoluteX, 3, 5, sta)),
        (STA_ABSOLUTEY, OpCode::new(STA, AbsoluteY, 3, 5, sta)),
        (STA_INDIRECTX, OpCode::new(STA, IndirectX, 2, 6, sta)),
        (STA_INDIRECTY, OpCode::new(STA, IndirectY, 2, 6, sta)),
        // STX
        (STX_ZEROPAGE, OpCode::new(STX, ZeroPage, 2, 3, stx)),
        (STX_ZEROPAGEY, OpCode::new(STX, ZeroPageY, 2, 4, stx)),
        (STX_ABSOLUTE, OpCode::new(STX, Absolute, 3, 4, stx)),
        // STY
        (STY_ZEROPAGE, OpCode::new(STY, ZeroPage, 2, 3, sty)),
        (STY_ZEROPAGEX, OpCode::new(STY, ZeroPageX, 2, 4, sty)),
        (STY_ABSOLUTE, OpCode::new(STY, Absolute, 3, 4, sty)),
        // TAX
        (tax::TAX, OpCode::new(TAX, Implied, 1, 2, tax)),
        // TAY
        (tay::TAY, OpCode::new(TAY, Implied, 1, 2, tay)),
        // TSX
        (0xBA, OpCode::new(TSX, Implied, 1, 2, tsx)),
        // TXA
        (txa::TXA, OpCode::new(TXA, Implied, 1, 2, txa)),
        // TXS
        (0x9A, OpCode::new(TXS, Implied, 1, 2, txs)),
        // TYA
        (tya::TYA, OpCode::new(TYA, Implied, 1, 2, tya)),
    ])
});

#[derive(Debug, Clone, Copy)]
pub enum OpCodeType {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}
