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
    pub fn new(instruction: u8) -> Option<Self> {
        use AddressingMode as AM;

        let addressing_mode = match instruction {
            ADC_IMMEDIATE | AND_IMMEDIATE | CMP_IMMEDIATE | CPX_IMMEDIATE | CPY_IMMEDIATE
            | EOR_IMMEDIATE | LDA_IMMEDIATE | LDX_IMMEDIATE | LDY_IMMEDIATE | ORA_IMMEDIATE
            | SBC_IMMEDIATE => AM::Immediate,

            ADC_ZEROPAGE | AND_ZEROPAGE | ASL_ZEROPAGE | BIT_ZEROPAGE | CMP_ZEROPAGE
            | CPX_ZEROPAGE | CPY_ZEROPAGE | DEC_ZEROPAGE | EOR_ZEROPAGE | INC_ZEROPAGE
            | LDA_ZEROPAGE | LDX_ZEROPAGE | LDY_ZEROPAGE | LSR_ZEROPAGE | ORA_ZEROPAGE
            | ROL_ZEROPAGE | ROR_ZEROPAGE | SBC_ZEROPAGE | STA_ZEROPAGE | STX_ZEROPAGE
            | STY_ZEROPAGE => AM::ZeroPage,

            ADC_ZEROPAGEX | AND_ZEROPAGEX | ASL_ZEROPAGEX | CMP_ZEROPAGEX | DEC_ZEROPAGEX
            | EOR_ZEROPAGEX | INC_ZEROPAGEX | LDA_ZEROPAGEX | LDY_ZEROPAGEX | LSR_ZEROPAGEX
            | ORA_ZEROPAGEX | ROL_ZEROPAGEX | ROR_ZEROPAGEX | SBC_ZEROPAGEX | STA_ZEROPAGEX
            | STY_ZEROPAGEX => AM::ZeroPageX,

            LDX_ZEROPAGEY | STX_ZEROPAGEY => AM::ZeroPageY,

            ADC_ABSOLUTE
            | AND_ABSOLUTE
            | ASL_ABSOLUTE
            | BIT_ABSOLUTE
            | CMP_ABSOLUTE
            | CPX_ABSOLUTE
            | CPY_ABSOLUTE
            | DEC_ABSOLUTE
            | EOR_ABSOLUTE
            | INC_ABSOLUTE
            | JMP_ABSOLUTE
            | jsr::JSR
            | LDA_ABSOLUTE
            | LDX_ABSOLUTE
            | LDY_ABSOLUTE
            | LSR_ABSOLUTE
            | ORA_ABSOLUTE
            | ROL_ABSOLUTE
            | ROR_ABSOLUTE
            | SBC_ABSOLUTE
            | STA_ABSOLUTE
            | STX_ABSOLUTE
            | STY_ABSOLUTE => AM::Absolute,

            ADC_ABSOLUTEX | AND_ABSOLUTEX | ASL_ABSOLUTEX | CMP_ABSOLUTEX | DEC_ABSOLUTEX
            | EOR_ABSOLUTEX | INC_ABSOLUTEX | LDA_ABSOLUTEX | LDY_ABSOLUTEX | LSR_ABSOLUTEX
            | ORA_ABSOLUTEX | ROL_ABSOLUTEX | ROR_ABSOLUTEX | SBC_ABSOLUTEX | STA_ABSOLUTEX => {
                AM::AbsoluteX
            }

            ADC_ABSOLUTEY | AND_ABSOLUTEY | CMP_ABSOLUTEY | EOR_ABSOLUTEY | LDA_ABSOLUTEY
            | LDX_ABSOLUTEY | ORA_ABSOLUTEY | SBC_ABSOLUTEY | STA_ABSOLUTEY => AM::AbsoluteY,

            JMP_INDIRECT => AM::Indirect,

            ADC_INDIRECTX | AND_INDIRECTX | CMP_INDIRECTX | EOR_INDIRECTX | LDA_INDIRECTX
            | ORA_INDIRECTX | SBC_INDIRECTX | STA_INDIRECTX => AM::IndirectX,

            ADC_INDIRECTY | AND_INDIRECTY | CMP_INDIRECTY | EOR_INDIRECTY | LDA_INDIRECTY
            | ORA_INDIRECTY | SBC_INDIRECTY | STA_INDIRECTY => AM::IndirectY,

            ASL_ACCUMULATOR | LSR_ACCUMULATOR | ROL_ACCUMULATOR | ROR_ACCUMULATOR => {
                AM::Accumulator
            }

            bcc::BCC
            | bcs::BCS
            | beq::BEQ
            | bmi::BMI
            | bne::BNE
            | bpl::BPL
            | bvc::BVC
            | bvs::BVS => AM::Relative,

            brk::BRK
            | clc::CLC
            | cld::CLD
            | cli::CLI
            | clv::CLV
            | dex::DEX
            | dey::DEY
            | inx::INX
            | iny::INY
            | nop::NOP
            | pha::PHA
            | php::PHP
            | pla::PLA
            | plp::PLP
            | rti::RTI
            | rts::RTS
            | sec::SEC
            | sed::SED
            | sei::SEI
            | tax::TAX
            | tay::TAY
            | tsx::TSX
            | txa::TXA
            | txs::TXS
            | tya::TYA => AM::Implied,

            _ => return None,
        };

        Some(addressing_mode)
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
