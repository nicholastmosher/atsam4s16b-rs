#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Configuration Register"]
    pub matrix_mcfg: [MATRIX_MCFG; 4],
    _reserved0: [u8; 48usize],
    #[doc = "0x40 - Slave Configuration Register"]
    pub matrix_scfg: [MATRIX_SCFG; 5],
    _reserved1: [u8; 44usize],
    #[doc = "0x80 - Priority Register A for Slave 0"]
    pub matrix_pras0: MATRIX_PRAS0,
    _reserved2: [u8; 4usize],
    #[doc = "0x88 - Priority Register A for Slave 1"]
    pub matrix_pras1: MATRIX_PRAS1,
    _reserved3: [u8; 4usize],
    #[doc = "0x90 - Priority Register A for Slave 2"]
    pub matrix_pras2: MATRIX_PRAS2,
    _reserved4: [u8; 4usize],
    #[doc = "0x98 - Priority Register A for Slave 3"]
    pub matrix_pras3: MATRIX_PRAS3,
    _reserved5: [u8; 4usize],
    #[doc = "0xa0 - Priority Register A for Slave 4"]
    pub matrix_pras4: MATRIX_PRAS4,
    _reserved6: [u8; 112usize],
    #[doc = "0x114 - System I/O Configuration register"]
    pub ccfg_sysio: CCFG_SYSIO,
    _reserved7: [u8; 4usize],
    #[doc = "0x11c - SMC Chip Select NAND Flash Assignment Register"]
    pub ccfg_smcnfcs: CCFG_SMCNFCS,
    _reserved8: [u8; 196usize],
    #[doc = "0x1e4 - Write Protect Mode Register"]
    pub matrix_wpmr: MATRIX_WPMR,
    #[doc = "0x1e8 - Write Protect Status Register"]
    pub matrix_wpsr: MATRIX_WPSR,
}
#[doc = "Master Configuration Register"]
pub struct MATRIX_MCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Configuration Register"]
pub mod matrix_mcfg;
#[doc = "Slave Configuration Register"]
pub struct MATRIX_SCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slave Configuration Register"]
pub mod matrix_scfg;
#[doc = "Priority Register A for Slave 0"]
pub struct MATRIX_PRAS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priority Register A for Slave 0"]
pub mod matrix_pras0;
#[doc = "Priority Register A for Slave 1"]
pub struct MATRIX_PRAS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priority Register A for Slave 1"]
pub mod matrix_pras1;
#[doc = "Priority Register A for Slave 2"]
pub struct MATRIX_PRAS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priority Register A for Slave 2"]
pub mod matrix_pras2;
#[doc = "Priority Register A for Slave 3"]
pub struct MATRIX_PRAS3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priority Register A for Slave 3"]
pub mod matrix_pras3;
#[doc = "Priority Register A for Slave 4"]
pub struct MATRIX_PRAS4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priority Register A for Slave 4"]
pub mod matrix_pras4;
#[doc = "System I/O Configuration register"]
pub struct CCFG_SYSIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System I/O Configuration register"]
pub mod ccfg_sysio;
#[doc = "SMC Chip Select NAND Flash Assignment Register"]
pub struct CCFG_SMCNFCS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Chip Select NAND Flash Assignment Register"]
pub mod ccfg_smcnfcs;
#[doc = "Write Protect Mode Register"]
pub struct MATRIX_WPMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protect Mode Register"]
pub mod matrix_wpmr;
#[doc = "Write Protect Status Register"]
pub struct MATRIX_WPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protect Status Register"]
pub mod matrix_wpsr;
