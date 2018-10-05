#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PIO Enable Register"]
    pub per: PER,
    #[doc = "0x04 - PIO Disable Register"]
    pub pdr: PDR,
    #[doc = "0x08 - PIO Status Register"]
    pub psr: PSR,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - Output Enable Register"]
    pub oer: OER,
    #[doc = "0x14 - Output Disable Register"]
    pub odr: ODR,
    #[doc = "0x18 - Output Status Register"]
    pub osr: OSR,
    _reserved1: [u8; 4usize],
    #[doc = "0x20 - Glitch Input Filter Enable Register"]
    pub ifer: IFER,
    #[doc = "0x24 - Glitch Input Filter Disable Register"]
    pub ifdr: IFDR,
    #[doc = "0x28 - Glitch Input Filter Status Register"]
    pub ifsr: IFSR,
    _reserved2: [u8; 4usize],
    #[doc = "0x30 - Set Output Data Register"]
    pub sodr: SODR,
    #[doc = "0x34 - Clear Output Data Register"]
    pub codr: CODR,
    #[doc = "0x38 - Output Data Status Register"]
    pub odsr: ODSR,
    #[doc = "0x3c - Pin Data Status Register"]
    pub pdsr: PDSR,
    #[doc = "0x40 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x44 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x48 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x4c - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x50 - Multi-driver Enable Register"]
    pub mder: MDER,
    #[doc = "0x54 - Multi-driver Disable Register"]
    pub mddr: MDDR,
    #[doc = "0x58 - Multi-driver Status Register"]
    pub mdsr: MDSR,
    _reserved3: [u8; 4usize],
    #[doc = "0x60 - Pull-up Disable Register"]
    pub pudr: PUDR,
    #[doc = "0x64 - Pull-up Enable Register"]
    pub puer: PUER,
    #[doc = "0x68 - Pad Pull-up Status Register"]
    pub pusr: PUSR,
    _reserved4: [u8; 4usize],
    #[doc = "0x70 - Peripheral Select Register"]
    pub abcdsr: [ABCDSR; 2],
    _reserved5: [u8; 8usize],
    #[doc = "0x80 - Input Filter Slow Clock Disable Register"]
    pub ifscdr: IFSCDR,
    #[doc = "0x84 - Input Filter Slow Clock Enable Register"]
    pub ifscer: IFSCER,
    #[doc = "0x88 - Input Filter Slow Clock Status Register"]
    pub ifscsr: IFSCSR,
    #[doc = "0x8c - Slow Clock Divider Debouncing Register"]
    pub scdr: SCDR,
    #[doc = "0x90 - Pad Pull-down Disable Register"]
    pub ppddr: PPDDR,
    #[doc = "0x94 - Pad Pull-down Enable Register"]
    pub ppder: PPDER,
    #[doc = "0x98 - Pad Pull-down Status Register"]
    pub ppdsr: PPDSR,
    _reserved6: [u8; 4usize],
    #[doc = "0xa0 - Output Write Enable"]
    pub ower: OWER,
    #[doc = "0xa4 - Output Write Disable"]
    pub owdr: OWDR,
    #[doc = "0xa8 - Output Write Status Register"]
    pub owsr: OWSR,
    _reserved7: [u8; 4usize],
    #[doc = "0xb0 - Additional Interrupt Modes Enable Register"]
    pub aimer: AIMER,
    #[doc = "0xb4 - Additional Interrupt Modes Disables Register"]
    pub aimdr: AIMDR,
    #[doc = "0xb8 - Additional Interrupt Modes Mask Register"]
    pub aimmr: AIMMR,
    _reserved8: [u8; 4usize],
    #[doc = "0xc0 - Edge Select Register"]
    pub esr: ESR,
    #[doc = "0xc4 - Level Select Register"]
    pub lsr: LSR,
    #[doc = "0xc8 - Edge/Level Status Register"]
    pub elsr: ELSR,
    _reserved9: [u8; 4usize],
    #[doc = "0xd0 - Falling Edge/Low Level Select Register"]
    pub fellsr: FELLSR,
    #[doc = "0xd4 - Rising Edge/ High Level Select Register"]
    pub rehlsr: REHLSR,
    #[doc = "0xd8 - Fall/Rise - Low/High Status Register"]
    pub frlhsr: FRLHSR,
    _reserved10: [u8; 4usize],
    #[doc = "0xe0 - Lock Status"]
    pub locksr: LOCKSR,
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protect Status Register"]
    pub wpsr: WPSR,
    _reserved11: [u8; 20usize],
    #[doc = "0x100 - Schmitt Trigger Register"]
    pub schmitt: SCHMITT,
    _reserved12: [u8; 76usize],
    #[doc = "0x150 - Parallel Capture Mode Register"]
    pub pcmr: PCMR,
    #[doc = "0x154 - Parallel Capture Interrupt Enable Register"]
    pub pcier: PCIER,
    #[doc = "0x158 - Parallel Capture Interrupt Disable Register"]
    pub pcidr: PCIDR,
    #[doc = "0x15c - Parallel Capture Interrupt Mask Register"]
    pub pcimr: PCIMR,
    #[doc = "0x160 - Parallel Capture Interrupt Status Register"]
    pub pcisr: PCISR,
    #[doc = "0x164 - Parallel Capture Reception Holding Register"]
    pub pcrhr: PCRHR,
}
#[doc = "PIO Enable Register"]
pub struct PER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PIO Enable Register"]
pub mod per;
#[doc = "PIO Disable Register"]
pub struct PDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PIO Disable Register"]
pub mod pdr;
#[doc = "PIO Status Register"]
pub struct PSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PIO Status Register"]
pub mod psr;
#[doc = "Output Enable Register"]
pub struct OER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Enable Register"]
pub mod oer;
#[doc = "Output Disable Register"]
pub struct ODR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Disable Register"]
pub mod odr;
#[doc = "Output Status Register"]
pub struct OSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Status Register"]
pub mod osr;
#[doc = "Glitch Input Filter Enable Register"]
pub struct IFER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Glitch Input Filter Enable Register"]
pub mod ifer;
#[doc = "Glitch Input Filter Disable Register"]
pub struct IFDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Glitch Input Filter Disable Register"]
pub mod ifdr;
#[doc = "Glitch Input Filter Status Register"]
pub struct IFSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Glitch Input Filter Status Register"]
pub mod ifsr;
#[doc = "Set Output Data Register"]
pub struct SODR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set Output Data Register"]
pub mod sodr;
#[doc = "Clear Output Data Register"]
pub struct CODR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear Output Data Register"]
pub mod codr;
#[doc = "Output Data Status Register"]
pub struct ODSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Data Status Register"]
pub mod odsr;
#[doc = "Pin Data Status Register"]
pub struct PDSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Data Status Register"]
pub mod pdsr;
#[doc = "Interrupt Enable Register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Disable Register"]
pub struct IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "Interrupt Mask Register"]
pub struct IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "Interrupt Status Register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "Multi-driver Enable Register"]
pub struct MDER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Multi-driver Enable Register"]
pub mod mder;
#[doc = "Multi-driver Disable Register"]
pub struct MDDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Multi-driver Disable Register"]
pub mod mddr;
#[doc = "Multi-driver Status Register"]
pub struct MDSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Multi-driver Status Register"]
pub mod mdsr;
#[doc = "Pull-up Disable Register"]
pub struct PUDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pull-up Disable Register"]
pub mod pudr;
#[doc = "Pull-up Enable Register"]
pub struct PUER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pull-up Enable Register"]
pub mod puer;
#[doc = "Pad Pull-up Status Register"]
pub struct PUSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Pull-up Status Register"]
pub mod pusr;
#[doc = "Peripheral Select Register"]
pub struct ABCDSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Select Register"]
pub mod abcdsr;
#[doc = "Input Filter Slow Clock Disable Register"]
pub struct IFSCDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Filter Slow Clock Disable Register"]
pub mod ifscdr;
#[doc = "Input Filter Slow Clock Enable Register"]
pub struct IFSCER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Filter Slow Clock Enable Register"]
pub mod ifscer;
#[doc = "Input Filter Slow Clock Status Register"]
pub struct IFSCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Filter Slow Clock Status Register"]
pub mod ifscsr;
#[doc = "Slow Clock Divider Debouncing Register"]
pub struct SCDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slow Clock Divider Debouncing Register"]
pub mod scdr;
#[doc = "Pad Pull-down Disable Register"]
pub struct PPDDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Pull-down Disable Register"]
pub mod ppddr;
#[doc = "Pad Pull-down Enable Register"]
pub struct PPDER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Pull-down Enable Register"]
pub mod ppder;
#[doc = "Pad Pull-down Status Register"]
pub struct PPDSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Pull-down Status Register"]
pub mod ppdsr;
#[doc = "Output Write Enable"]
pub struct OWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Write Enable"]
pub mod ower;
#[doc = "Output Write Disable"]
pub struct OWDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Write Disable"]
pub mod owdr;
#[doc = "Output Write Status Register"]
pub struct OWSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Write Status Register"]
pub mod owsr;
#[doc = "Additional Interrupt Modes Enable Register"]
pub struct AIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Additional Interrupt Modes Enable Register"]
pub mod aimer;
#[doc = "Additional Interrupt Modes Disables Register"]
pub struct AIMDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Additional Interrupt Modes Disables Register"]
pub mod aimdr;
#[doc = "Additional Interrupt Modes Mask Register"]
pub struct AIMMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Additional Interrupt Modes Mask Register"]
pub mod aimmr;
#[doc = "Edge Select Register"]
pub struct ESR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Edge Select Register"]
pub mod esr;
#[doc = "Level Select Register"]
pub struct LSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Level Select Register"]
pub mod lsr;
#[doc = "Edge/Level Status Register"]
pub struct ELSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Edge/Level Status Register"]
pub mod elsr;
#[doc = "Falling Edge/Low Level Select Register"]
pub struct FELLSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Falling Edge/Low Level Select Register"]
pub mod fellsr;
#[doc = "Rising Edge/ High Level Select Register"]
pub struct REHLSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rising Edge/ High Level Select Register"]
pub mod rehlsr;
#[doc = "Fall/Rise - Low/High Status Register"]
pub struct FRLHSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fall/Rise - Low/High Status Register"]
pub mod frlhsr;
#[doc = "Lock Status"]
pub struct LOCKSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lock Status"]
pub mod locksr;
#[doc = "Write Protect Mode Register"]
pub struct WPMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "Write Protect Status Register"]
pub struct WPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protect Status Register"]
pub mod wpsr;
#[doc = "Schmitt Trigger Register"]
pub struct SCHMITT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Schmitt Trigger Register"]
pub mod schmitt;
#[doc = "Parallel Capture Mode Register"]
pub struct PCMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parallel Capture Mode Register"]
pub mod pcmr;
#[doc = "Parallel Capture Interrupt Enable Register"]
pub struct PCIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parallel Capture Interrupt Enable Register"]
pub mod pcier;
#[doc = "Parallel Capture Interrupt Disable Register"]
pub struct PCIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parallel Capture Interrupt Disable Register"]
pub mod pcidr;
#[doc = "Parallel Capture Interrupt Mask Register"]
pub struct PCIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parallel Capture Interrupt Mask Register"]
pub mod pcimr;
#[doc = "Parallel Capture Interrupt Status Register"]
pub struct PCISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parallel Capture Interrupt Status Register"]
pub mod pcisr;
#[doc = "Parallel Capture Reception Holding Register"]
pub struct PCRHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parallel Capture Reception Holding Register"]
pub mod pcrhr;
