pub struct Interrupt {
    pub ty: InterruptType,
    pub handler_addr: u16,
    pub b_flag_mask: u8,
    pub cpu_cycles: u8,
}

impl Interrupt {
    pub const NMI: Self = Self {
        ty: InterruptType::NMI,
        handler_addr: 0xFFFA,
        b_flag_mask: 0b0010_0000,
        cpu_cycles: 2,
    };
}

pub enum InterruptType {
    NMI,
}
