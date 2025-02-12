use std::{collections::HashMap, sync::LazyLock};

pub struct OpCode {
    pub ty: OpCodeType,
    pub mode: AddressingMode,
    pub bytes: u8,
    pub cycles: u8,
}

impl OpCode {
    pub fn new(ty: OpCodeType, mode: AddressingMode, bytes: u8, cycles: u8) -> Self {
        Self {
            ty,
            mode,
            bytes,
            cycles,
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
        (0x69, OpCode::new(ADC, Immediate, 2, 2)),
        (0x65, OpCode::new(ADC, ZeroPage, 2, 3)),
        (0x75, OpCode::new(ADC, ZeroPageX, 2, 4)),
        (0x6D, OpCode::new(ADC, Absolute, 3, 4)),
        (0x7D, OpCode::new(ADC, AbsoluteX, 3, 4)),
        (0x79, OpCode::new(ADC, AbsoluteY, 3, 4)),
        (0x61, OpCode::new(ADC, IndirectX, 2, 6)),
        (0x71, OpCode::new(ADC, IndirectY, 2, 5)),
        // AND
        (0x29, OpCode::new(AND, Immediate, 2, 2)),
        (0x25, OpCode::new(AND, ZeroPage, 2, 3)),
        (0x35, OpCode::new(AND, ZeroPageX, 2, 4)),
        (0x2D, OpCode::new(AND, Absolute, 3, 4)),
        (0x3D, OpCode::new(AND, AbsoluteX, 3, 4)),
        (0x39, OpCode::new(AND, AbsoluteY, 3, 4)),
        (0x21, OpCode::new(AND, IndirectX, 2, 6)),
        (0x31, OpCode::new(AND, IndirectY, 2, 5)),
        // ASL
        (0x0A, OpCode::new(ASL, Accumulator, 1, 2)),
        (0x06, OpCode::new(ASL, ZeroPage, 2, 5)),
        (0x16, OpCode::new(ASL, ZeroPageX, 2, 6)),
        (0x0E, OpCode::new(ASL, Absolute, 3, 6)),
        (0x1E, OpCode::new(ASL, AbsoluteX, 3, 7)),
        // BCC
        (0x90, OpCode::new(BCC, Relative, 2, 2)),
        // BCS
        (0xB0, OpCode::new(BCS, Relative, 2, 2)),
        // BEQ
        (0xF0, OpCode::new(BEQ, Relative, 2, 2)),
        // BIT
        (0x24, OpCode::new(BIT, ZeroPage, 2, 3)),
        (0x2C, OpCode::new(BIT, Absolute, 3, 4)),
        // BMI
        (0x30, OpCode::new(BMI, Relative, 2, 2)),
        // BNE
        (0xD0, OpCode::new(BNE, Relative, 2, 2)),
        // BPL
        (0x10, OpCode::new(BPL, Relative, 2, 2)),
        // BRK
        (0x00, OpCode::new(BRK, Implied, 1, 7)),
        // BVC
        (0x50, OpCode::new(BVC, Relative, 2, 2)),
        // BVS
        (0x70, OpCode::new(BVS, Relative, 2, 2)),
        // CLC
        (0x18, OpCode::new(CLC, Implied, 1, 2)),
        // CLD
        (0xD8, OpCode::new(CLD, Implied, 1, 2)),
        // CLI
        (0x58, OpCode::new(CLI, Implied, 1, 2)),
        // CLV
        (0xB8, OpCode::new(CLV, Implied, 1, 2)),
        // CMP
        (0xC9, OpCode::new(CMP, Immediate, 2, 2)),
        (0xC5, OpCode::new(CMP, ZeroPage, 2, 3)),
        (0xD5, OpCode::new(CMP, ZeroPageX, 2, 4)),
        (0xCD, OpCode::new(CMP, Absolute, 3, 4)),
        (0xDD, OpCode::new(CMP, AbsoluteX, 3, 4)),
        (0xD9, OpCode::new(CMP, AbsoluteY, 3, 4)),
        (0xC1, OpCode::new(CMP, IndirectX, 2, 6)),
        (0xD1, OpCode::new(CMP, IndirectY, 2, 5)),
        // CPX
        (0xE0, OpCode::new(CPX, Immediate, 2, 2)),
        (0xE4, OpCode::new(CPX, ZeroPage, 2, 3)),
        (0xEC, OpCode::new(CPX, Absolute, 3, 4)),
        // CPY
        (0xE0, OpCode::new(CPY, Immediate, 2, 2)),
        (0xE4, OpCode::new(CPY, ZeroPage, 2, 3)),
        (0xEC, OpCode::new(CPY, Absolute, 3, 4)),
        // DEC
        (0xC6, OpCode::new(DEC, ZeroPage, 2, 5)),
        (0xD6, OpCode::new(DEC, ZeroPageX, 2, 6)),
        (0xCE, OpCode::new(DEC, Absolute, 3, 6)),
        (0xDE, OpCode::new(DEC, AbsoluteX, 3, 7)),
        // DEX
        (0xCA, OpCode::new(DEX, Implied, 1, 2)),
        // DEY
        (0x88, OpCode::new(DEY, Implied, 1, 2)),
        // EOR
        (0x49, OpCode::new(EOR, Immediate, 2, 2)),
        (0x45, OpCode::new(EOR, ZeroPage, 2, 3)),
        (0x55, OpCode::new(EOR, ZeroPageX, 2, 4)),
        (0x4D, OpCode::new(EOR, Absolute, 3, 4)),
        (0x5D, OpCode::new(EOR, AbsoluteX, 3, 4)),
        (0x59, OpCode::new(EOR, AbsoluteY, 3, 4)),
        (0x41, OpCode::new(EOR, IndirectX, 2, 6)),
        (0x51, OpCode::new(EOR, IndirectY, 2, 5)),
        // INC
        (0xE6, OpCode::new(INC, ZeroPage, 2, 5)),
        (0xF6, OpCode::new(INC, ZeroPageX, 2, 6)),
        (0xEE, OpCode::new(INC, Absolute, 3, 6)),
        (0xFE, OpCode::new(INC, AbsoluteX, 3, 7)),
        // INX
        (0xE8, OpCode::new(INX, Implied, 1, 2)),
        // INY
        (0xC8, OpCode::new(INY, Implied, 1, 2)),
        // JMP
        (0x4C, OpCode::new(JMP, Absolute, 3, 3)),
        (0x6C, OpCode::new(JMP, Indirect, 3, 5)),
        // JSR
        (0x20, OpCode::new(JSR, Absolute, 3, 6)),
        // LDA
        (0xA9, OpCode::new(LDA, Immediate, 2, 2)),
        (0xA5, OpCode::new(LDA, ZeroPage, 2, 3)),
        (0xB5, OpCode::new(LDA, ZeroPageX, 2, 4)),
        (0xAD, OpCode::new(LDA, Absolute, 3, 4)),
        (0xBD, OpCode::new(LDA, AbsoluteX, 3, 4)),
        (0xB9, OpCode::new(LDA, AbsoluteY, 3, 4)),
        (0xA1, OpCode::new(LDA, IndirectX, 2, 6)),
        (0xB1, OpCode::new(LDA, IndirectY, 2, 5)),
        // LDX
        (0xA2, OpCode::new(LDX, Immediate, 2, 2)),
        (0xA6, OpCode::new(LDX, ZeroPage, 2, 3)),
        (0xB6, OpCode::new(LDX, ZeroPageY, 2, 4)),
        (0xAE, OpCode::new(LDX, Absolute, 3, 4)),
        (0xBE, OpCode::new(LDX, AbsoluteY, 3, 4)),
        // LDY
        (0xA0, OpCode::new(LDY, Immediate, 2, 2)),
        (0xA4, OpCode::new(LDY, ZeroPage, 2, 3)),
        (0xB4, OpCode::new(LDY, ZeroPageX, 2, 4)),
        (0xAC, OpCode::new(LDY, Absolute, 3, 4)),
        (0xBC, OpCode::new(LDY, AbsoluteX, 3, 4)),
        // LSR
        (0x4A, OpCode::new(LSR, Accumulator, 1, 2)),
        (0x46, OpCode::new(LSR, ZeroPage, 2, 5)),
        (0x56, OpCode::new(LSR, ZeroPageX, 2, 6)),
        (0x4E, OpCode::new(LSR, Absolute, 3, 6)),
        (0x5E, OpCode::new(LSR, AbsoluteX, 3, 7)),
        // NOP
        (0xEA, OpCode::new(NOP, Implied, 1, 2)),
        // ORA
        (0x09, OpCode::new(ORA, Immediate, 2, 2)),
        (0x05, OpCode::new(ORA, ZeroPage, 2, 3)),
        (0x15, OpCode::new(ORA, ZeroPageX, 2, 4)),
        (0x0D, OpCode::new(ORA, Absolute, 3, 4)),
        (0x1D, OpCode::new(ORA, AbsoluteX, 3, 4)),
        (0x19, OpCode::new(ORA, AbsoluteY, 3, 4)),
        (0x01, OpCode::new(ORA, IndirectX, 2, 6)),
        (0x11, OpCode::new(ORA, IndirectY, 2, 5)),
        // PHA
        (0x48, OpCode::new(PHA, Implied, 1, 3)),
        // PHP
        (0x08, OpCode::new(PHP, Implied, 1, 3)),
        // PLA
        (0x68, OpCode::new(PLA, Implied, 1, 4)),
        // PLP
        (0x28, OpCode::new(PLP, Implied, 1, 4)),
        // ROL
        (0x2A, OpCode::new(ROL, Accumulator, 1, 2)),
        (0x26, OpCode::new(ROL, ZeroPage, 2, 5)),
        (0x36, OpCode::new(ROL, ZeroPageX, 2, 6)),
        (0x2E, OpCode::new(ROL, Absolute, 3, 6)),
        (0x3E, OpCode::new(ROL, AbsoluteX, 3, 7)),
        // ROR
        (0x6A, OpCode::new(ROR, Accumulator, 1, 2)),
        (0x66, OpCode::new(ROR, ZeroPage, 2, 5)),
        (0x76, OpCode::new(ROR, ZeroPageX, 2, 6)),
        (0x6E, OpCode::new(ROR, Absolute, 3, 6)),
        (0x7E, OpCode::new(ROR, AbsoluteX, 3, 7)),
        // RTI
        (0x40, OpCode::new(RTI, Implied, 1, 6)),
        // RTS
        (0x60, OpCode::new(RTS, Implied, 1, 6)),
        // SBC
        (0xE9, OpCode::new(SBC, Immediate, 2, 2)),
        (0xE5, OpCode::new(SBC, ZeroPage, 2, 3)),
        (0xF5, OpCode::new(SBC, ZeroPageX, 2, 4)),
        (0xED, OpCode::new(SBC, Absolute, 3, 4)),
        (0xFD, OpCode::new(SBC, AbsoluteX, 3, 4)),
        (0xF9, OpCode::new(SBC, AbsoluteY, 3, 4)),
        (0xE1, OpCode::new(SBC, IndirectX, 2, 6)),
        (0xF1, OpCode::new(SBC, IndirectY, 2, 5)),
        // SEC
        (0x38, OpCode::new(SEC, Implied, 1, 2)),
        // SED
        (0xF8, OpCode::new(SED, Implied, 1, 2)),
        // SEI
        (0x78, OpCode::new(SEI, Implied, 1, 2)),
        // STA
        (0x85, OpCode::new(STA, ZeroPage, 2, 3)),
        (0x95, OpCode::new(STA, ZeroPageX, 2, 4)),
        (0x8D, OpCode::new(STA, Absolute, 3, 4)),
        (0x9D, OpCode::new(STA, AbsoluteX, 3, 5)),
        (0x99, OpCode::new(STA, AbsoluteY, 3, 5)),
        (0x81, OpCode::new(STA, IndirectX, 2, 6)),
        (0x91, OpCode::new(STA, IndirectY, 2, 6)),
        // STX
        (0x86, OpCode::new(STX, ZeroPage, 2, 3)),
        (0x96, OpCode::new(STX, ZeroPageY, 2, 4)),
        (0x8E, OpCode::new(STX, Absolute, 3, 4)),
        // STY
        (0x84, OpCode::new(STY, ZeroPage, 2, 3)),
        (0x94, OpCode::new(STY, ZeroPageY, 2, 4)),
        (0x8C, OpCode::new(STY, Absolute, 3, 4)),
        // TAX
        (0xAA, OpCode::new(TAX, Implied, 1, 2)),
        // TAY
        (0xA8, OpCode::new(TAY, Implied, 1, 2)),
        // TSX
        (0xBA, OpCode::new(TSX, Implied, 1, 2)),
        // TXA
        (0x8A, OpCode::new(TXA, Implied, 1, 2)),
        // TXS
        (0x9A, OpCode::new(TXS, Implied, 1, 2)),
        // TYA
        (0x98, OpCode::new(TYA, Implied, 1, 2)),
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
