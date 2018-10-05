#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Frame Number Register"]
    pub frm_num: FRM_NUM,
    #[doc = "0x04 - Global State Register"]
    pub glb_stat: GLB_STAT,
    #[doc = "0x08 - Function Address Register"]
    pub faddr: FADDR,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x14 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x1c - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x20 - Interrupt Clear Register"]
    pub icr: ICR,
    _reserved1: [u8; 4usize],
    #[doc = "0x28 - Reset Endpoint Register"]
    pub rst_ep: RST_EP,
    _reserved2: [u8; 4usize],
    #[doc = "0x30 - Endpoint Control and Status Register"]
    pub csr: [CSR; 8],
    #[doc = "0x50 - Endpoint FIFO Data Register"]
    pub fdr: [FDR; 8],
    _reserved3: [u8; 4usize],
    #[doc = "0x74 - Transceiver Control Register"]
    pub txvc: TXVC,
}
#[doc = "Frame Number Register"]
pub struct FRM_NUM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frame Number Register"]
pub mod frm_num;
#[doc = "Global State Register"]
pub struct GLB_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global State Register"]
pub mod glb_stat;
#[doc = "Function Address Register"]
pub struct FADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Function Address Register"]
pub mod faddr;
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
#[doc = "Interrupt Clear Register"]
pub struct ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "Reset Endpoint Register"]
pub struct RST_EP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Endpoint Register"]
pub mod rst_ep;
#[doc = "Endpoint Control and Status Register"]
pub struct CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control and Status Register"]
pub mod csr;
#[doc = "Endpoint Control and Status Register"]
pub struct CSR0_ISOENDPT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control and Status Register"]
pub mod csr0_isoendpt;
#[doc = "Endpoint FIFO Data Register"]
pub struct FDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint FIFO Data Register"]
pub mod fdr;
#[doc = "Transceiver Control Register"]
pub struct TXVC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transceiver Control Register"]
pub mod txvc;
