use std::{collections::HashMap, sync::LazyLock};

use crate::{instructions::*, CPU};

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
        (0x69, OpCode::new(ADC, Immediate, 2, 2, adc)),
        (0x65, OpCode::new(ADC, ZeroPage, 2, 3, adc)),
        (0x75, OpCode::new(ADC, ZeroPageX, 2, 4, adc)),
        (0x6D, OpCode::new(ADC, Absolute, 3, 4, adc)),
        (0x7D, OpCode::new(ADC, AbsoluteX, 3, 4, adc)),
        (0x79, OpCode::new(ADC, AbsoluteY, 3, 4, adc)),
        (0x61, OpCode::new(ADC, IndirectX, 2, 6, adc)),
        (0x71, OpCode::new(ADC, IndirectY, 2, 5, adc)),
        // AND
        (0x29, OpCode::new(AND, Immediate, 2, 2, and)),
        (0x25, OpCode::new(AND, ZeroPage, 2, 3, and)),
        (0x35, OpCode::new(AND, ZeroPageX, 2, 4, and)),
        (0x2D, OpCode::new(AND, Absolute, 3, 4, and)),
        (0x3D, OpCode::new(AND, AbsoluteX, 3, 4, and)),
        (0x39, OpCode::new(AND, AbsoluteY, 3, 4, and)),
        (0x21, OpCode::new(AND, IndirectX, 2, 6, and)),
        (0x31, OpCode::new(AND, IndirectY, 2, 5, and)),
        // ASL
        (0x0A, OpCode::new(ASL, Accumulator, 1, 2, asl)),
        (0x06, OpCode::new(ASL, ZeroPage, 2, 5, asl)),
        (0x16, OpCode::new(ASL, ZeroPageX, 2, 6, asl)),
        (0x0E, OpCode::new(ASL, Absolute, 3, 6, asl)),
        (0x1E, OpCode::new(ASL, AbsoluteX, 3, 7, asl)),
        // BCC
        (0x90, OpCode::new(BCC, Relative, 2, 2, bcc)),
        // BCS
        (0xB0, OpCode::new(BCS, Relative, 2, 2, bcs)),
        // BEQ
        (0xF0, OpCode::new(BEQ, Relative, 2, 2, beq)),
        // BIT
        (0x24, OpCode::new(BIT, ZeroPage, 2, 3, bit)),
        (0x2C, OpCode::new(BIT, Absolute, 3, 4, bit)),
        // BMI
        (0x30, OpCode::new(BMI, Relative, 2, 2, bmi)),
        // BNE
        (0xD0, OpCode::new(BNE, Relative, 2, 2, bne)),
        // BPL
        (0x10, OpCode::new(BPL, Relative, 2, 2, bpl)),
        // BRK
        (0x00, OpCode::new(BRK, Implied, 1, 7, brk)),
        // BVC
        (0x50, OpCode::new(BVC, Relative, 2, 2, bvc)),
        // BVS
        (0x70, OpCode::new(BVS, Relative, 2, 2, bvs)),
        // CLC
        (0x18, OpCode::new(CLC, Implied, 1, 2, clc)),
        // CLD
        (0xD8, OpCode::new(CLD, Implied, 1, 2, cld)),
        // CLI
        (0x58, OpCode::new(CLI, Implied, 1, 2, cli)),
        // CLV
        (0xB8, OpCode::new(CLV, Implied, 1, 2, clv)),
        // CMP
        (0xC9, OpCode::new(CMP, Immediate, 2, 2, cmp)),
        (0xC5, OpCode::new(CMP, ZeroPage, 2, 3, cmp)),
        (0xD5, OpCode::new(CMP, ZeroPageX, 2, 4, cmp)),
        (0xCD, OpCode::new(CMP, Absolute, 3, 4, cmp)),
        (0xDD, OpCode::new(CMP, AbsoluteX, 3, 4, cmp)),
        (0xD9, OpCode::new(CMP, AbsoluteY, 3, 4, cmp)),
        (0xC1, OpCode::new(CMP, IndirectX, 2, 6, cmp)),
        (0xD1, OpCode::new(CMP, IndirectY, 2, 5, cmp)),
        // CPX
        (0xE0, OpCode::new(CPX, Immediate, 2, 2, cpx)),
        (0xE4, OpCode::new(CPX, ZeroPage, 2, 3, cpx)),
        (0xEC, OpCode::new(CPX, Absolute, 3, 4, cpx)),
        // CPY
        (0xE0, OpCode::new(CPY, Immediate, 2, 2, cpy)),
        (0xE4, OpCode::new(CPY, ZeroPage, 2, 3, cpy)),
        (0xEC, OpCode::new(CPY, Absolute, 3, 4, cpy)),
        // DEC
        (0xC6, OpCode::new(DEC, ZeroPage, 2, 5, dec)),
        (0xD6, OpCode::new(DEC, ZeroPageX, 2, 6, dec)),
        (0xCE, OpCode::new(DEC, Absolute, 3, 6, dec)),
        (0xDE, OpCode::new(DEC, AbsoluteX, 3, 7, dec)),
        // DEX
        (0xCA, OpCode::new(DEX, Implied, 1, 2, dex)),
        // DEY
        (0x88, OpCode::new(DEY, Implied, 1, 2, dey)),
        // EOR
        (0x49, OpCode::new(EOR, Immediate, 2, 2, eor)),
        (0x45, OpCode::new(EOR, ZeroPage, 2, 3, eor)),
        (0x55, OpCode::new(EOR, ZeroPageX, 2, 4, eor)),
        (0x4D, OpCode::new(EOR, Absolute, 3, 4, eor)),
        (0x5D, OpCode::new(EOR, AbsoluteX, 3, 4, eor)),
        (0x59, OpCode::new(EOR, AbsoluteY, 3, 4, eor)),
        (0x41, OpCode::new(EOR, IndirectX, 2, 6, eor)),
        (0x51, OpCode::new(EOR, IndirectY, 2, 5, eor)),
        // INC
        (0xE6, OpCode::new(INC, ZeroPage, 2, 5, inc)),
        (0xF6, OpCode::new(INC, ZeroPageX, 2, 6, inc)),
        (0xEE, OpCode::new(INC, Absolute, 3, 6, inc)),
        (0xFE, OpCode::new(INC, AbsoluteX, 3, 7, inc)),
        // INX
        (0xE8, OpCode::new(INX, Implied, 1, 2, inx)),
        // INY
        (0xC8, OpCode::new(INY, Implied, 1, 2, iny)),
        // JMP
        (0x4C, OpCode::new(JMP, Absolute, 3, 3, jmp)),
        (0x6C, OpCode::new(JMP, Indirect, 3, 5, jmp)),
        // JSR
        (0x20, OpCode::new(JSR, Absolute, 3, 6, jsr)),
        // LDA
        (0xA9, OpCode::new(LDA, Immediate, 2, 2, lda)),
        (0xA5, OpCode::new(LDA, ZeroPage, 2, 3, lda)),
        (0xB5, OpCode::new(LDA, ZeroPageX, 2, 4, lda)),
        (0xAD, OpCode::new(LDA, Absolute, 3, 4, lda)),
        (0xBD, OpCode::new(LDA, AbsoluteX, 3, 4, lda)),
        (0xB9, OpCode::new(LDA, AbsoluteY, 3, 4, lda)),
        (0xA1, OpCode::new(LDA, IndirectX, 2, 6, lda)),
        (0xB1, OpCode::new(LDA, IndirectY, 2, 5, lda)),
        // LDX
        (0xA2, OpCode::new(LDX, Immediate, 2, 2, ldx)),
        (0xA6, OpCode::new(LDX, ZeroPage, 2, 3, ldx)),
        (0xB6, OpCode::new(LDX, ZeroPageY, 2, 4, ldx)),
        (0xAE, OpCode::new(LDX, Absolute, 3, 4, ldx)),
        (0xBE, OpCode::new(LDX, AbsoluteY, 3, 4, ldx)),
        // LDY
        (0xA0, OpCode::new(LDY, Immediate, 2, 2, ldy)),
        (0xA4, OpCode::new(LDY, ZeroPage, 2, 3, ldy)),
        (0xB4, OpCode::new(LDY, ZeroPageX, 2, 4, ldy)),
        (0xAC, OpCode::new(LDY, Absolute, 3, 4, ldy)),
        (0xBC, OpCode::new(LDY, AbsoluteX, 3, 4, ldy)),
        // LSR
        (0x4A, OpCode::new(LSR, Accumulator, 1, 2, lsr)),
        (0x46, OpCode::new(LSR, ZeroPage, 2, 5, lsr)),
        (0x56, OpCode::new(LSR, ZeroPageX, 2, 6, lsr)),
        (0x4E, OpCode::new(LSR, Absolute, 3, 6, lsr)),
        (0x5E, OpCode::new(LSR, AbsoluteX, 3, 7, lsr)),
        // NOP
        (0xEA, OpCode::new(NOP, Implied, 1, 2, nop)),
        // ORA
        (0x09, OpCode::new(ORA, Immediate, 2, 2, ora)),
        (0x05, OpCode::new(ORA, ZeroPage, 2, 3, ora)),
        (0x15, OpCode::new(ORA, ZeroPageX, 2, 4, ora)),
        (0x0D, OpCode::new(ORA, Absolute, 3, 4, ora)),
        (0x1D, OpCode::new(ORA, AbsoluteX, 3, 4, ora)),
        (0x19, OpCode::new(ORA, AbsoluteY, 3, 4, ora)),
        (0x01, OpCode::new(ORA, IndirectX, 2, 6, ora)),
        (0x11, OpCode::new(ORA, IndirectY, 2, 5, ora)),
        // PHA
        (0x48, OpCode::new(PHA, Implied, 1, 3, pha)),
        // PHP
        (0x08, OpCode::new(PHP, Implied, 1, 3, php)),
        // PLA
        (0x68, OpCode::new(PLA, Implied, 1, 4, pla)),
        // PLP
        (0x28, OpCode::new(PLP, Implied, 1, 4, plp)),
        // ROL
        (0x2A, OpCode::new(ROL, Accumulator, 1, 2, rol)),
        (0x26, OpCode::new(ROL, ZeroPage, 2, 5, rol)),
        (0x36, OpCode::new(ROL, ZeroPageX, 2, 6, rol)),
        (0x2E, OpCode::new(ROL, Absolute, 3, 6, rol)),
        (0x3E, OpCode::new(ROL, AbsoluteX, 3, 7, rol)),
        // ROR
        (0x6A, OpCode::new(ROR, Accumulator, 1, 2, ror)),
        (0x66, OpCode::new(ROR, ZeroPage, 2, 5, ror)),
        (0x76, OpCode::new(ROR, ZeroPageX, 2, 6, ror)),
        (0x6E, OpCode::new(ROR, Absolute, 3, 6, ror)),
        (0x7E, OpCode::new(ROR, AbsoluteX, 3, 7, ror)),
        // RTI
        (0x40, OpCode::new(RTI, Implied, 1, 6, rti)),
        // RTS
        (0x60, OpCode::new(RTS, Implied, 1, 6, rts)),
        // SBC
        (0xE9, OpCode::new(SBC, Immediate, 2, 2, sbc)),
        (0xE5, OpCode::new(SBC, ZeroPage, 2, 3, sbc)),
        (0xF5, OpCode::new(SBC, ZeroPageX, 2, 4, sbc)),
        (0xED, OpCode::new(SBC, Absolute, 3, 4, sbc)),
        (0xFD, OpCode::new(SBC, AbsoluteX, 3, 4, sbc)),
        (0xF9, OpCode::new(SBC, AbsoluteY, 3, 4, sbc)),
        (0xE1, OpCode::new(SBC, IndirectX, 2, 6, sbc)),
        (0xF1, OpCode::new(SBC, IndirectY, 2, 5, sbc)),
        // SEC
        (0x38, OpCode::new(SEC, Implied, 1, 2, sec)),
        // SED
        (0xF8, OpCode::new(SED, Implied, 1, 2, sed)),
        // SEI
        (0x78, OpCode::new(SEI, Implied, 1, 2, sei)),
        // STA
        (0x85, OpCode::new(STA, ZeroPage, 2, 3, sta)),
        (0x95, OpCode::new(STA, ZeroPageX, 2, 4, sta)),
        (0x8D, OpCode::new(STA, Absolute, 3, 4, sta)),
        (0x9D, OpCode::new(STA, AbsoluteX, 3, 5, sta)),
        (0x99, OpCode::new(STA, AbsoluteY, 3, 5, sta)),
        (0x81, OpCode::new(STA, IndirectX, 2, 6, sta)),
        (0x91, OpCode::new(STA, IndirectY, 2, 6, sta)),
        // STX
        (0x86, OpCode::new(STX, ZeroPage, 2, 3, stx)),
        (0x96, OpCode::new(STX, ZeroPageY, 2, 4, stx)),
        (0x8E, OpCode::new(STX, Absolute, 3, 4, stx)),
        // STY
        (0x84, OpCode::new(STY, ZeroPage, 2, 3, sty)),
        (0x94, OpCode::new(STY, ZeroPageY, 2, 4, sty)),
        (0x8C, OpCode::new(STY, Absolute, 3, 4, sty)),
        // TAX
        (0xAA, OpCode::new(TAX, Implied, 1, 2, tax)),
        // TAY
        (0xA8, OpCode::new(TAY, Implied, 1, 2, tay)),
        // TSX
        (0xBA, OpCode::new(TSX, Implied, 1, 2, tsx)),
        // TXA
        (0x8A, OpCode::new(TXA, Implied, 1, 2, txa)),
        // TXS
        (0x9A, OpCode::new(TXS, Implied, 1, 2, txs)),
        // TYA
        (0x98, OpCode::new(TYA, Implied, 1, 2, tya)),
    ])
});

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
