#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_rbr_thr_dll: [u8; 0x04],
    _reserved_1_dlm_ier: [u8; 0x04],
    _reserved_2_iir_fcr: [u8; 0x04],
    #[doc = "0x0c - Line Control Register"]
    pub lcr: LCR,
    _reserved4: [u8; 0x04],
    #[doc = "0x14 - Line Status Register"]
    pub lsr: LSR,
    _reserved5: [u8; 0x04],
    #[doc = "0x1c - Scratch Pad Register"]
    pub scr: SCR,
    _reserved6: [u8; 0x0c],
    #[doc = "0x2c - Oversampling register"]
    pub osr: OSR,
    _reserved7: [u8; 0x18],
    #[doc = "0x48 - Smart Card Interface control register"]
    pub scictrl: SCICTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - Transmit Holding Register"]
    #[inline(always)]
    pub fn rbr_thr_dll_thr(&self) -> &RBR_THR_DLL_THR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const RBR_THR_DLL_THR) }
    }
    #[doc = "0x00 - Receiver Buffer Register"]
    #[inline(always)]
    pub fn rbr_thr_dll_rbr(&self) -> &RBR_THR_DLL_RBR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const RBR_THR_DLL_RBR) }
    }
    #[doc = "0x00 - Divisor Latch LSB"]
    #[inline(always)]
    pub fn rbr_thr_dll_dll(&self) -> &RBR_THR_DLL_DLL {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const RBR_THR_DLL_DLL) }
    }
    #[doc = "0x04 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn dlm_ier_ier(&self) -> &DLM_IER_IER {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const DLM_IER_IER) }
    }
    #[doc = "0x04 - Divisor Latch MSB"]
    #[inline(always)]
    pub fn dlm_ier_dlm(&self) -> &DLM_IER_DLM {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const DLM_IER_DLM) }
    }
    #[doc = "0x08 - Interrupt ID Register"]
    #[inline(always)]
    pub fn iir_fcr_iir(&self) -> &IIR_FCR_IIR {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const IIR_FCR_IIR) }
    }
    #[doc = "0x08 - FIFO Control Register"]
    #[inline(always)]
    pub fn iir_fcr_fcr(&self) -> &IIR_FCR_FCR {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const IIR_FCR_FCR) }
    }
}
#[doc = "RBR_THR_DLL_DLL (rw) register accessor: an alias for `Reg<RBR_THR_DLL_DLL_SPEC>`"]
pub type RBR_THR_DLL_DLL = crate::Reg<rbr_thr_dll_dll::RBR_THR_DLL_DLL_SPEC>;
#[doc = "Divisor Latch LSB"]
pub mod rbr_thr_dll_dll;
#[doc = "RBR_THR_DLL_RBR (r) register accessor: an alias for `Reg<RBR_THR_DLL_RBR_SPEC>`"]
pub type RBR_THR_DLL_RBR = crate::Reg<rbr_thr_dll_rbr::RBR_THR_DLL_RBR_SPEC>;
#[doc = "Receiver Buffer Register"]
pub mod rbr_thr_dll_rbr;
#[doc = "RBR_THR_DLL_THR (w) register accessor: an alias for `Reg<RBR_THR_DLL_THR_SPEC>`"]
pub type RBR_THR_DLL_THR = crate::Reg<rbr_thr_dll_thr::RBR_THR_DLL_THR_SPEC>;
#[doc = "Transmit Holding Register"]
pub mod rbr_thr_dll_thr;
#[doc = "DLM_IER_DLM (rw) register accessor: an alias for `Reg<DLM_IER_DLM_SPEC>`"]
pub type DLM_IER_DLM = crate::Reg<dlm_ier_dlm::DLM_IER_DLM_SPEC>;
#[doc = "Divisor Latch MSB"]
pub mod dlm_ier_dlm;
#[doc = "DLM_IER_IER (rw) register accessor: an alias for `Reg<DLM_IER_IER_SPEC>`"]
pub type DLM_IER_IER = crate::Reg<dlm_ier_ier::DLM_IER_IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod dlm_ier_ier;
#[doc = "IIR_FCR_FCR (w) register accessor: an alias for `Reg<IIR_FCR_FCR_SPEC>`"]
pub type IIR_FCR_FCR = crate::Reg<iir_fcr_fcr::IIR_FCR_FCR_SPEC>;
#[doc = "FIFO Control Register"]
pub mod iir_fcr_fcr;
#[doc = "IIR_FCR_IIR (r) register accessor: an alias for `Reg<IIR_FCR_IIR_SPEC>`"]
pub type IIR_FCR_IIR = crate::Reg<iir_fcr_iir::IIR_FCR_IIR_SPEC>;
#[doc = "Interrupt ID Register"]
pub mod iir_fcr_iir;
#[doc = "LCR (rw) register accessor: an alias for `Reg<LCR_SPEC>`"]
pub type LCR = crate::Reg<lcr::LCR_SPEC>;
#[doc = "Line Control Register"]
pub mod lcr;
#[doc = "LSR (r) register accessor: an alias for `Reg<LSR_SPEC>`"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "Line Status Register"]
pub mod lsr;
#[doc = "SCR (rw) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Scratch Pad Register"]
pub mod scr;
#[doc = "OSR (rw) register accessor: an alias for `Reg<OSR_SPEC>`"]
pub type OSR = crate::Reg<osr::OSR_SPEC>;
#[doc = "Oversampling register"]
pub mod osr;
#[doc = "SCICTRL (rw) register accessor: an alias for `Reg<SCICTRL_SPEC>`"]
pub type SCICTRL = crate::Reg<scictrl::SCICTRL_SPEC>;
#[doc = "Smart Card Interface control register"]
pub mod scictrl;
