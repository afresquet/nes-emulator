use crate::{instructions::*, CPU};

pub trait OpCode {
    /// Construct instruction
    fn fetch(cpu: &mut CPU) -> Instruction;
    /// Perform instruction, returning the number of cycles
    fn execute(self, cpu: &mut CPU);
    /// Number of cycles
    fn cycles(&self) -> u8;
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
    Accumulator,
    Relative,
    Implied,
}

impl AddressingMode {
    pub fn new(instruction: u8) -> Self {
        use AddressingMode as AM;

        let addressing_mode = match instruction {
            AAC_IMMEDIATE1 | AAC_IMMEDIATE2 | ADC_IMMEDIATE | AND_IMMEDIATE | ARR_IMMEDIATE
            | ASR_IMMEDIATE | ATX_IMMEDIATE | AXS_IMMEDIATE | CMP_IMMEDIATE | CPX_IMMEDIATE
            | CPY_IMMEDIATE | DOP_IMMEDIATE1 | DOP_IMMEDIATE2 | DOP_IMMEDIATE3 | DOP_IMMEDIATE4
            | DOP_IMMEDIATE5 | EOR_IMMEDIATE | LDA_IMMEDIATE | LDX_IMMEDIATE | LDY_IMMEDIATE
            | ORA_IMMEDIATE | SBC_IMMEDIATE | SBC_IMMEDIATE2 | XAA_IMMEDIATE => AM::Immediate,

            ADC_ZEROPAGE | AAX_ZEROPAGE | AND_ZEROPAGE | ASL_ZEROPAGE | BIT_ZEROPAGE
            | CMP_ZEROPAGE | CPX_ZEROPAGE | CPY_ZEROPAGE | DCP_ZEROPAGE | DEC_ZEROPAGE
            | DOP_ZEROPAGE1 | DOP_ZEROPAGE2 | DOP_ZEROPAGE3 | EOR_ZEROPAGE | INC_ZEROPAGE
            | ISC_ZEROPAGE | LAX_ZEROPAGE | LDA_ZEROPAGE | LDX_ZEROPAGE | LDY_ZEROPAGE
            | LSR_ZEROPAGE | ORA_ZEROPAGE | RLA_ZEROPAGE | ROL_ZEROPAGE | ROR_ZEROPAGE
            | RRA_ZEROPAGE | SBC_ZEROPAGE | SLO_ZEROPAGE | SRE_ZEROPAGE | STA_ZEROPAGE
            | STX_ZEROPAGE | STY_ZEROPAGE => AM::ZeroPage,

            ADC_ZEROPAGEX | AND_ZEROPAGEX | ASL_ZEROPAGEX | CMP_ZEROPAGEX | DCP_ZEROPAGEX
            | DEC_ZEROPAGEX | DOP_ZEROPAGEX1 | DOP_ZEROPAGEX2 | DOP_ZEROPAGEX3 | DOP_ZEROPAGEX4
            | DOP_ZEROPAGEX5 | DOP_ZEROPAGEX6 | EOR_ZEROPAGEX | INC_ZEROPAGEX | ISC_ZEROPAGEX
            | LDA_ZEROPAGEX | LDY_ZEROPAGEX | LSR_ZEROPAGEX | ORA_ZEROPAGEX | RLA_ZEROPAGEX
            | ROL_ZEROPAGEX | ROR_ZEROPAGEX | RRA_ZEROPAGEX | SBC_ZEROPAGEX | SLO_ZEROPAGEX
            | SRE_ZEROPAGEX | STA_ZEROPAGEX | STY_ZEROPAGEX => AM::ZeroPageX,

            AAX_ZEROPAGEY | LAX_ZEROPAGEY | LDX_ZEROPAGEY | STX_ZEROPAGEY => AM::ZeroPageY,

            #[rustfmt::skip]
            ADC_ABSOLUTE | AAX_ABSOLUTE | AND_ABSOLUTE | ASL_ABSOLUTE | BIT_ABSOLUTE | CMP_ABSOLUTE
            | CPX_ABSOLUTE | CPY_ABSOLUTE | DCP_ABSOLUTE | DEC_ABSOLUTE | EOR_ABSOLUTE | INC_ABSOLUTE
            | ISC_ABSOLUTE | JMP_ABSOLUTE | jsr::JSR | LAX_ABSOLUTE| LDA_ABSOLUTE | LDX_ABSOLUTE
            | LDY_ABSOLUTE | LSR_ABSOLUTE | ORA_ABSOLUTE | RLA_ABSOLUTE | ROL_ABSOLUTE | ROR_ABSOLUTE
            | RRA_ABSOLUTE | SBC_ABSOLUTE | SLO_ABSOLUTE|SRE_ABSOLUTE| STA_ABSOLUTE | STX_ABSOLUTE
            | STY_ABSOLUTE | TOP_ABSOLUTE => AM::Absolute,

            ADC_ABSOLUTEX | AND_ABSOLUTEX | ASL_ABSOLUTEX | CMP_ABSOLUTEX | DCP_ABSOLUTEX
            | DEC_ABSOLUTEX | EOR_ABSOLUTEX | INC_ABSOLUTEX | ISC_ABSOLUTEX | LDA_ABSOLUTEX
            | LDY_ABSOLUTEX | LSR_ABSOLUTEX | ORA_ABSOLUTEX | RLA_ABSOLUTEX | ROL_ABSOLUTEX
            | ROR_ABSOLUTEX | RRA_ABSOLUTEX | SBC_ABSOLUTEX | SLO_ABSOLUTEX | SRE_ABSOLUTEX
            | STA_ABSOLUTEX | SYA_ABSOLUTEX | TOP_ABSOLUTEX1 | TOP_ABSOLUTEX2 | TOP_ABSOLUTEX3
            | TOP_ABSOLUTEX4 | TOP_ABSOLUTEX5 | TOP_ABSOLUTEX6 => AM::AbsoluteX,

            ADC_ABSOLUTEY | AND_ABSOLUTEY | AXA_ABSOLUTEY | CMP_ABSOLUTEY | DCP_ABSOLUTEY
            | EOR_ABSOLUTEY | ISC_ABSOLUTEY | LAR_ABSOLUTEY | LAX_ABSOLUTEY | LDA_ABSOLUTEY
            | LDX_ABSOLUTEY | ORA_ABSOLUTEY | RLA_ABSOLUTEY | RRA_ABSOLUTEY | SBC_ABSOLUTEY
            | SLO_ABSOLUTEY | SRE_ABSOLUTEY | STA_ABSOLUTEY | SXA_ABSOLUTEY | XAS_ABSOLUTEY => {
                AM::AbsoluteY
            }

            JMP_INDIRECT => AM::Indirect,

            AAX_INDIRECTX | ADC_INDIRECTX | AND_INDIRECTX | CMP_INDIRECTX | DCP_INDIRECTX
            | EOR_INDIRECTX | ISC_INDIRECTX | LAX_INDIRECTX | LDA_INDIRECTX | ORA_INDIRECTX
            | RLA_INDIRECTX | RRA_INDIRECTX | SBC_INDIRECTX | SLO_INDIRECTX | SRE_INDIRECTX
            | STA_INDIRECTX => AM::IndirectX,

            ADC_INDIRECTY | AND_INDIRECTY | AXA_INDIRECTY | CMP_INDIRECTY | DCP_INDIRECTY
            | EOR_INDIRECTY | ISC_INDIRECTY | LAX_INDIRECTY | LDA_INDIRECTY | ORA_INDIRECTY
            | RLA_INDIRECTY | RRA_INDIRECTY | SBC_INDIRECTY | SLO_INDIRECTY | SRE_INDIRECTY
            | STA_INDIRECTY => AM::IndirectY,

            ASL_ACCUMULATOR | LSR_ACCUMULATOR | ROL_ACCUMULATOR | ROR_ACCUMULATOR => {
                AM::Accumulator
            }

            #[rustfmt::skip]
            bcc::BCC | bcs::BCS | beq::BEQ | bmi::BMI | bne::BNE | bpl::BPL | bvc::BVC
            | bvs::BVS => AM::Relative,

            #[rustfmt::skip]
            brk::BRK | clc::CLC | cld::CLD | cli::CLI | clv::CLV | dex::DEX | dey::DEY | inx::INX
            | iny::INY | KIL_IMPLIED1 | KIL_IMPLIED2 | KIL_IMPLIED3 | KIL_IMPLIED4 | KIL_IMPLIED5
            | KIL_IMPLIED6 | KIL_IMPLIED7 | KIL_IMPLIED8 | KIL_IMPLIED9 | KIL_IMPLIED10 | KIL_IMPLIED11
            | KIL_IMPLIED12 | nop::NOP | NOP_IMPLIED1 | NOP_IMPLIED2 | NOP_IMPLIED3 | NOP_IMPLIED4
            | NOP_IMPLIED5 | NOP_IMPLIED6 | pha::PHA | php::PHP | pla::PLA | plp::PLP | rti::RTI
            | rts::RTS | sec::SEC | sed::SED | sei::SEI | tax::TAX | tay::TAY | tsx::TSX | txa::TXA
            | txs::TXS | tya::TYA => AM::Implied,
        };

        addressing_mode
    }

    pub fn bytes(&self) -> u16 {
        match self {
            AddressingMode::Accumulator | AddressingMode::Implied => 1,

            AddressingMode::Immediate
            | AddressingMode::ZeroPage
            | AddressingMode::ZeroPageX
            | AddressingMode::ZeroPageY
            | AddressingMode::IndirectX
            | AddressingMode::IndirectY
            | AddressingMode::Relative => 2,

            AddressingMode::Absolute
            | AddressingMode::AbsoluteX
            | AddressingMode::AbsoluteY
            | AddressingMode::Indirect => 3,
        }
    }
}
