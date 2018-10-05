#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRCCU Descriptor Base Register"]
    pub dscr: DSCR,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - CRCCU DMA Enable Register"]
    pub dma_en: DMA_EN,
    #[doc = "0x0c - CRCCU DMA Disable Register"]
    pub dma_dis: DMA_DIS,
    #[doc = "0x10 - CRCCU DMA Status Register"]
    pub dma_sr: DMA_SR,
    #[doc = "0x14 - CRCCU DMA Interrupt Enable Register"]
    pub dma_ier: DMA_IER,
    #[doc = "0x18 - CRCCU DMA Interrupt Disable Register"]
    pub dma_idr: DMA_IDR,
    #[doc = "0x1c - CRCCU DMA Interrupt Mask Register"]
    pub dma_imr: DMA_IMR,
    #[doc = "0x20 - CRCCU DMA Interrupt Status Register"]
    pub dma_isr: DMA_ISR,
    _reserved1: [u8; 16usize],
    #[doc = "0x34 - CRCCU Control Register"]
    pub cr: CR,
    #[doc = "0x38 - CRCCU Mode Register"]
    pub mr: MR,
    #[doc = "0x3c - CRCCU Status Register"]
    pub sr: SR,
    #[doc = "0x40 - CRCCU Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x44 - CRCCU Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x48 - CRCCU Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x4c - CRCCU Interrupt Status Register"]
    pub isr: ISR,
}
#[doc = "CRCCU Descriptor Base Register"]
pub struct DSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRCCU Descriptor Base Register"]
pub mod dscr;
#[doc = "CRCCU DMA Enable Register"]
pub struct DMA_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRCCU DMA Enable Register"]
pub mod dma_en;
#[doc = "CRCCU DMA Disable Register"]
pub struct DMA_DIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRCCU DMA Disable Register"]
pub mod dma_dis;
#[doc = "CRCCU DMA Status Register"]
pub struct DMA_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRCCU DMA Status Register"]
pub mod dma_sr;
#[doc = "CRCCU DMA Interrupt Enable Register"]
pub struct DMA_IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRCCU DMA Interrupt Enable Register"]
pub mod dma_ier;
#[doc = "CRCCU DMA Interrupt Disable Register"]
pub struct DMA_IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRCCU DMA Interrupt Disable Register"]
pub mod dma_idr;
#[doc = "CRCCU DMA Interrupt Mask Register"]
pub struct DMA_IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRCCU DMA Interrupt Mask Register"]
pub mod dma_imr;
#[doc = "CRCCU DMA Interrupt Status Register"]
pub struct DMA_ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRCCU DMA Interrupt Status Register"]
pub mod dma_isr;
#[doc = "CRCCU Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRCCU Control Register"]
pub mod cr;
#[doc = "CRCCU Mode Register"]
pub struct MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRCCU Mode Register"]
pub mod mr;
#[doc = "CRCCU Status Register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRCCU Status Register"]
pub mod sr;
#[doc = "CRCCU Interrupt Enable Register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRCCU Interrupt Enable Register"]
pub mod ier;
#[doc = "CRCCU Interrupt Disable Register"]
pub struct IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRCCU Interrupt Disable Register"]
pub mod idr;
#[doc = "CRCCU Interrupt Mask Register"]
pub struct IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRCCU Interrupt Mask Register"]
pub mod imr;
#[doc = "CRCCU Interrupt Status Register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRCCU Interrupt Status Register"]
pub mod isr;
