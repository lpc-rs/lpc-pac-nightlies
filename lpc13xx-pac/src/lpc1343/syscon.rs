#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System memory remap"]
    pub sysmemremap: SYSMEMREMAP,
    #[doc = "0x04 - Peripheral reset control"]
    pub presetctrl: PRESETCTRL,
    #[doc = "0x08 - System PLL control"]
    pub syspllctrl: SYSPLLCTRL,
    #[doc = "0x0c - System PLL status"]
    pub syspllstat: SYSPLLSTAT,
    #[doc = "0x10 - USB PLL control"]
    pub usbpllctrl: USBPLLCTRL,
    #[doc = "0x14 - USB PLL status"]
    pub usbpllstat: USBPLLSTAT,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - System oscillator control"]
    pub sysoscctrl: SYSOSCCTRL,
    #[doc = "0x24 - Watchdog oscillator control"]
    pub wdtoscctrl: WDTOSCCTRL,
    #[doc = "0x28 - IRC control"]
    pub ircctrl: IRCCTRL,
    _reserved9: [u8; 0x04],
    #[doc = "0x30 - System reset status register"]
    pub sysresstat: SYSRESSTAT,
    _reserved10: [u8; 0x0c],
    #[doc = "0x40 - System PLL clock source select"]
    pub syspllclksel: SYSPLLCLKSEL,
    #[doc = "0x44 - System PLL clock source update enable"]
    pub syspllclkuen: SYSPLLCLKUEN,
    #[doc = "0x48 - USB PLL clock source select"]
    pub usbpllclksel: USBPLLCLKSEL,
    #[doc = "0x4c - USB PLL clock source update enable"]
    pub usbpllclkuen: USBPLLCLKUEN,
    _reserved14: [u8; 0x20],
    #[doc = "0x70 - Main clock source select"]
    pub mainclksel: MAINCLKSEL,
    #[doc = "0x74 - Main clock source update enable"]
    pub mainclkuen: MAINCLKUEN,
    #[doc = "0x78 - System AHB clock divider"]
    pub sysahbclkdiv: SYSAHBCLKDIV,
    _reserved17: [u8; 0x04],
    #[doc = "0x80 - System AHB clock control"]
    pub sysahbclkctrl: SYSAHBCLKCTRL,
    _reserved18: [u8; 0x10],
    #[doc = "0x94 - SSP clock divder"]
    pub ssp0clkdiv: SSP0CLKDIV,
    #[doc = "0x98 - UART clock divder"]
    pub uartclkdiv: UARTCLKDIV,
    #[doc = "0x9c - SPISP1 clock divder"]
    pub ssp1clkdiv: SSP1CLKDIV,
    _reserved21: [u8; 0x0c],
    #[doc = "0xac - ARM trace clock divider"]
    pub traceclkdiv: TRACECLKDIV,
    #[doc = "0xb0 - SYSTICK clock divder"]
    pub systickclkdiv: SYSTICKCLKDIV,
    _reserved23: [u8; 0x0c],
    #[doc = "0xc0 - USB clock source select"]
    pub usbclksel: USBCLKSEL,
    #[doc = "0xc4 - USB clock source update enable"]
    pub usbclkuen: USBCLKUEN,
    #[doc = "0xc8 - USB clock source divider"]
    pub usbclkdiv: USBCLKDIV,
    _reserved26: [u8; 0x04],
    #[doc = "0xd0 - WDT clock source select"]
    pub wdtclksel: WDTCLKSEL,
    #[doc = "0xd4 - WDT clock source update enable"]
    pub wdtclkuen: WDTCLKUEN,
    #[doc = "0xd8 - WDT clock divider"]
    pub wdtclkdiv: WDTCLKDIV,
    _reserved29: [u8; 0x04],
    #[doc = "0xe0 - CLKOUT clock source select"]
    pub clkoutclksel: CLKOUTCLKSEL,
    #[doc = "0xe4 - CLKOUT clock source update enable"]
    pub clkoutuen: CLKOUTUEN,
    #[doc = "0xe8 - CLKOUT clock divider"]
    pub clkoutdiv: CLKOUTDIV,
    _reserved32: [u8; 0x14],
    #[doc = "0x100 - POR captured PIO status 0"]
    pub pioporcap0: PIOPORCAP0,
    #[doc = "0x104 - POR captured PIO status 1"]
    pub pioporcap1: PIOPORCAP1,
    _reserved34: [u8; 0x48],
    #[doc = "0x150 - BOD control"]
    pub bodctrl: BODCTRL,
    #[doc = "0x154 - System tick counter calibration"]
    pub systckcal: SYSTCKCAL,
    _reserved36: [u8; 0xa8],
    #[doc = "0x200 - Start logic edge control register 0; bottom 32 interrupts"]
    pub startaprp0: STARTAPRP0,
    #[doc = "0x204 - Start logic signal enable register 0; bottom 32 interrupts"]
    pub starterp0: STARTERP0,
    #[doc = "0x208 - Start logic reset register 0; bottom 32 interrupts"]
    pub startrsrp0clr: STARTRSRP0CLR,
    #[doc = "0x20c - Start logic status register 0; bottom 32 interrupts"]
    pub startsrp0: STARTSRP0,
    #[doc = "0x210 - Start logic edge control register 1; top 8 interrupts"]
    pub startaprp1: STARTAPRP1,
    #[doc = "0x214 - Start logic signal enable register 1; top 8 interrupts"]
    pub starterp1: STARTERP1,
    #[doc = "0x218 - Start logic reset register 1; top 8 interrupts"]
    pub startrsrp1clr: STARTRSRP1CLR,
    #[doc = "0x21c - Start logic status register 1; top 8 interrupts"]
    pub startsrp1: STARTSRP1,
    _reserved44: [u8; 0x10],
    #[doc = "0x230 - Power-down states in Deep-sleep mode"]
    pub pdsleepcfg: PDSLEEPCFG,
    #[doc = "0x234 - Power-down states after wake-up from Deep-sleep mode"]
    pub pdawakecfg: PDAWAKECFG,
    #[doc = "0x238 - Power-down configuration register"]
    pub pdruncfg: PDRUNCFG,
    _reserved47: [u8; 0x01b8],
    #[doc = "0x3f4 - Device ID"]
    pub device_id: DEVICE_ID,
}
#[doc = "SYSMEMREMAP (rw) register accessor: an alias for `Reg<SYSMEMREMAP_SPEC>`"]
pub type SYSMEMREMAP = crate::Reg<sysmemremap::SYSMEMREMAP_SPEC>;
#[doc = "System memory remap"]
pub mod sysmemremap;
#[doc = "PRESETCTRL (rw) register accessor: an alias for `Reg<PRESETCTRL_SPEC>`"]
pub type PRESETCTRL = crate::Reg<presetctrl::PRESETCTRL_SPEC>;
#[doc = "Peripheral reset control"]
pub mod presetctrl;
#[doc = "SYSPLLCTRL (rw) register accessor: an alias for `Reg<SYSPLLCTRL_SPEC>`"]
pub type SYSPLLCTRL = crate::Reg<syspllctrl::SYSPLLCTRL_SPEC>;
#[doc = "System PLL control"]
pub mod syspllctrl;
#[doc = "SYSPLLSTAT (r) register accessor: an alias for `Reg<SYSPLLSTAT_SPEC>`"]
pub type SYSPLLSTAT = crate::Reg<syspllstat::SYSPLLSTAT_SPEC>;
#[doc = "System PLL status"]
pub mod syspllstat;
#[doc = "USBPLLCTRL (rw) register accessor: an alias for `Reg<USBPLLCTRL_SPEC>`"]
pub type USBPLLCTRL = crate::Reg<usbpllctrl::USBPLLCTRL_SPEC>;
#[doc = "USB PLL control"]
pub mod usbpllctrl;
#[doc = "USBPLLSTAT (r) register accessor: an alias for `Reg<USBPLLSTAT_SPEC>`"]
pub type USBPLLSTAT = crate::Reg<usbpllstat::USBPLLSTAT_SPEC>;
#[doc = "USB PLL status"]
pub mod usbpllstat;
#[doc = "SYSOSCCTRL (rw) register accessor: an alias for `Reg<SYSOSCCTRL_SPEC>`"]
pub type SYSOSCCTRL = crate::Reg<sysoscctrl::SYSOSCCTRL_SPEC>;
#[doc = "System oscillator control"]
pub mod sysoscctrl;
#[doc = "WDTOSCCTRL (rw) register accessor: an alias for `Reg<WDTOSCCTRL_SPEC>`"]
pub type WDTOSCCTRL = crate::Reg<wdtoscctrl::WDTOSCCTRL_SPEC>;
#[doc = "Watchdog oscillator control"]
pub mod wdtoscctrl;
#[doc = "IRCCTRL (rw) register accessor: an alias for `Reg<IRCCTRL_SPEC>`"]
pub type IRCCTRL = crate::Reg<ircctrl::IRCCTRL_SPEC>;
#[doc = "IRC control"]
pub mod ircctrl;
#[doc = "SYSRESSTAT (r) register accessor: an alias for `Reg<SYSRESSTAT_SPEC>`"]
pub type SYSRESSTAT = crate::Reg<sysresstat::SYSRESSTAT_SPEC>;
#[doc = "System reset status register"]
pub mod sysresstat;
#[doc = "SYSPLLCLKSEL (rw) register accessor: an alias for `Reg<SYSPLLCLKSEL_SPEC>`"]
pub type SYSPLLCLKSEL = crate::Reg<syspllclksel::SYSPLLCLKSEL_SPEC>;
#[doc = "System PLL clock source select"]
pub mod syspllclksel;
#[doc = "SYSPLLCLKUEN (rw) register accessor: an alias for `Reg<SYSPLLCLKUEN_SPEC>`"]
pub type SYSPLLCLKUEN = crate::Reg<syspllclkuen::SYSPLLCLKUEN_SPEC>;
#[doc = "System PLL clock source update enable"]
pub mod syspllclkuen;
#[doc = "USBPLLCLKSEL (rw) register accessor: an alias for `Reg<USBPLLCLKSEL_SPEC>`"]
pub type USBPLLCLKSEL = crate::Reg<usbpllclksel::USBPLLCLKSEL_SPEC>;
#[doc = "USB PLL clock source select"]
pub mod usbpllclksel;
#[doc = "USBPLLCLKUEN (rw) register accessor: an alias for `Reg<USBPLLCLKUEN_SPEC>`"]
pub type USBPLLCLKUEN = crate::Reg<usbpllclkuen::USBPLLCLKUEN_SPEC>;
#[doc = "USB PLL clock source update enable"]
pub mod usbpllclkuen;
#[doc = "MAINCLKSEL (rw) register accessor: an alias for `Reg<MAINCLKSEL_SPEC>`"]
pub type MAINCLKSEL = crate::Reg<mainclksel::MAINCLKSEL_SPEC>;
#[doc = "Main clock source select"]
pub mod mainclksel;
#[doc = "MAINCLKUEN (rw) register accessor: an alias for `Reg<MAINCLKUEN_SPEC>`"]
pub type MAINCLKUEN = crate::Reg<mainclkuen::MAINCLKUEN_SPEC>;
#[doc = "Main clock source update enable"]
pub mod mainclkuen;
#[doc = "SYSAHBCLKDIV (rw) register accessor: an alias for `Reg<SYSAHBCLKDIV_SPEC>`"]
pub type SYSAHBCLKDIV = crate::Reg<sysahbclkdiv::SYSAHBCLKDIV_SPEC>;
#[doc = "System AHB clock divider"]
pub mod sysahbclkdiv;
#[doc = "SYSAHBCLKCTRL (rw) register accessor: an alias for `Reg<SYSAHBCLKCTRL_SPEC>`"]
pub type SYSAHBCLKCTRL = crate::Reg<sysahbclkctrl::SYSAHBCLKCTRL_SPEC>;
#[doc = "System AHB clock control"]
pub mod sysahbclkctrl;
#[doc = "SSP0CLKDIV (rw) register accessor: an alias for `Reg<SSP0CLKDIV_SPEC>`"]
pub type SSP0CLKDIV = crate::Reg<ssp0clkdiv::SSP0CLKDIV_SPEC>;
#[doc = "SSP clock divder"]
pub mod ssp0clkdiv;
#[doc = "UARTCLKDIV (rw) register accessor: an alias for `Reg<UARTCLKDIV_SPEC>`"]
pub type UARTCLKDIV = crate::Reg<uartclkdiv::UARTCLKDIV_SPEC>;
#[doc = "UART clock divder"]
pub mod uartclkdiv;
#[doc = "SSP1CLKDIV (rw) register accessor: an alias for `Reg<SSP1CLKDIV_SPEC>`"]
pub type SSP1CLKDIV = crate::Reg<ssp1clkdiv::SSP1CLKDIV_SPEC>;
#[doc = "SPISP1 clock divder"]
pub mod ssp1clkdiv;
#[doc = "TRACECLKDIV (rw) register accessor: an alias for `Reg<TRACECLKDIV_SPEC>`"]
pub type TRACECLKDIV = crate::Reg<traceclkdiv::TRACECLKDIV_SPEC>;
#[doc = "ARM trace clock divider"]
pub mod traceclkdiv;
#[doc = "SYSTICKCLKDIV (rw) register accessor: an alias for `Reg<SYSTICKCLKDIV_SPEC>`"]
pub type SYSTICKCLKDIV = crate::Reg<systickclkdiv::SYSTICKCLKDIV_SPEC>;
#[doc = "SYSTICK clock divder"]
pub mod systickclkdiv;
#[doc = "USBCLKSEL (rw) register accessor: an alias for `Reg<USBCLKSEL_SPEC>`"]
pub type USBCLKSEL = crate::Reg<usbclksel::USBCLKSEL_SPEC>;
#[doc = "USB clock source select"]
pub mod usbclksel;
#[doc = "USBCLKUEN (rw) register accessor: an alias for `Reg<USBCLKUEN_SPEC>`"]
pub type USBCLKUEN = crate::Reg<usbclkuen::USBCLKUEN_SPEC>;
#[doc = "USB clock source update enable"]
pub mod usbclkuen;
#[doc = "USBCLKDIV (rw) register accessor: an alias for `Reg<USBCLKDIV_SPEC>`"]
pub type USBCLKDIV = crate::Reg<usbclkdiv::USBCLKDIV_SPEC>;
#[doc = "USB clock source divider"]
pub mod usbclkdiv;
#[doc = "WDTCLKSEL (rw) register accessor: an alias for `Reg<WDTCLKSEL_SPEC>`"]
pub type WDTCLKSEL = crate::Reg<wdtclksel::WDTCLKSEL_SPEC>;
#[doc = "WDT clock source select"]
pub mod wdtclksel;
#[doc = "WDTCLKUEN (rw) register accessor: an alias for `Reg<WDTCLKUEN_SPEC>`"]
pub type WDTCLKUEN = crate::Reg<wdtclkuen::WDTCLKUEN_SPEC>;
#[doc = "WDT clock source update enable"]
pub mod wdtclkuen;
#[doc = "WDTCLKDIV (rw) register accessor: an alias for `Reg<WDTCLKDIV_SPEC>`"]
pub type WDTCLKDIV = crate::Reg<wdtclkdiv::WDTCLKDIV_SPEC>;
#[doc = "WDT clock divider"]
pub mod wdtclkdiv;
#[doc = "CLKOUTCLKSEL (rw) register accessor: an alias for `Reg<CLKOUTCLKSEL_SPEC>`"]
pub type CLKOUTCLKSEL = crate::Reg<clkoutclksel::CLKOUTCLKSEL_SPEC>;
#[doc = "CLKOUT clock source select"]
pub mod clkoutclksel;
#[doc = "CLKOUTUEN (rw) register accessor: an alias for `Reg<CLKOUTUEN_SPEC>`"]
pub type CLKOUTUEN = crate::Reg<clkoutuen::CLKOUTUEN_SPEC>;
#[doc = "CLKOUT clock source update enable"]
pub mod clkoutuen;
#[doc = "CLKOUTDIV (rw) register accessor: an alias for `Reg<CLKOUTDIV_SPEC>`"]
pub type CLKOUTDIV = crate::Reg<clkoutdiv::CLKOUTDIV_SPEC>;
#[doc = "CLKOUT clock divider"]
pub mod clkoutdiv;
#[doc = "PIOPORCAP0 (r) register accessor: an alias for `Reg<PIOPORCAP0_SPEC>`"]
pub type PIOPORCAP0 = crate::Reg<pioporcap0::PIOPORCAP0_SPEC>;
#[doc = "POR captured PIO status 0"]
pub mod pioporcap0;
#[doc = "PIOPORCAP1 (r) register accessor: an alias for `Reg<PIOPORCAP1_SPEC>`"]
pub type PIOPORCAP1 = crate::Reg<pioporcap1::PIOPORCAP1_SPEC>;
#[doc = "POR captured PIO status 1"]
pub mod pioporcap1;
#[doc = "BODCTRL (rw) register accessor: an alias for `Reg<BODCTRL_SPEC>`"]
pub type BODCTRL = crate::Reg<bodctrl::BODCTRL_SPEC>;
#[doc = "BOD control"]
pub mod bodctrl;
#[doc = "SYSTCKCAL (rw) register accessor: an alias for `Reg<SYSTCKCAL_SPEC>`"]
pub type SYSTCKCAL = crate::Reg<systckcal::SYSTCKCAL_SPEC>;
#[doc = "System tick counter calibration"]
pub mod systckcal;
#[doc = "STARTAPRP0 (rw) register accessor: an alias for `Reg<STARTAPRP0_SPEC>`"]
pub type STARTAPRP0 = crate::Reg<startaprp0::STARTAPRP0_SPEC>;
#[doc = "Start logic edge control register 0; bottom 32 interrupts"]
pub mod startaprp0;
#[doc = "STARTERP0 (rw) register accessor: an alias for `Reg<STARTERP0_SPEC>`"]
pub type STARTERP0 = crate::Reg<starterp0::STARTERP0_SPEC>;
#[doc = "Start logic signal enable register 0; bottom 32 interrupts"]
pub mod starterp0;
#[doc = "STARTRSRP0CLR (w) register accessor: an alias for `Reg<STARTRSRP0CLR_SPEC>`"]
pub type STARTRSRP0CLR = crate::Reg<startrsrp0clr::STARTRSRP0CLR_SPEC>;
#[doc = "Start logic reset register 0; bottom 32 interrupts"]
pub mod startrsrp0clr;
#[doc = "STARTSRP0 (r) register accessor: an alias for `Reg<STARTSRP0_SPEC>`"]
pub type STARTSRP0 = crate::Reg<startsrp0::STARTSRP0_SPEC>;
#[doc = "Start logic status register 0; bottom 32 interrupts"]
pub mod startsrp0;
#[doc = "STARTAPRP1 (rw) register accessor: an alias for `Reg<STARTAPRP1_SPEC>`"]
pub type STARTAPRP1 = crate::Reg<startaprp1::STARTAPRP1_SPEC>;
#[doc = "Start logic edge control register 1; top 8 interrupts"]
pub mod startaprp1;
#[doc = "STARTERP1 (rw) register accessor: an alias for `Reg<STARTERP1_SPEC>`"]
pub type STARTERP1 = crate::Reg<starterp1::STARTERP1_SPEC>;
#[doc = "Start logic signal enable register 1; top 8 interrupts"]
pub mod starterp1;
#[doc = "STARTRSRP1CLR (w) register accessor: an alias for `Reg<STARTRSRP1CLR_SPEC>`"]
pub type STARTRSRP1CLR = crate::Reg<startrsrp1clr::STARTRSRP1CLR_SPEC>;
#[doc = "Start logic reset register 1; top 8 interrupts"]
pub mod startrsrp1clr;
#[doc = "STARTSRP1 (r) register accessor: an alias for `Reg<STARTSRP1_SPEC>`"]
pub type STARTSRP1 = crate::Reg<startsrp1::STARTSRP1_SPEC>;
#[doc = "Start logic status register 1; top 8 interrupts"]
pub mod startsrp1;
#[doc = "PDSLEEPCFG (rw) register accessor: an alias for `Reg<PDSLEEPCFG_SPEC>`"]
pub type PDSLEEPCFG = crate::Reg<pdsleepcfg::PDSLEEPCFG_SPEC>;
#[doc = "Power-down states in Deep-sleep mode"]
pub mod pdsleepcfg;
#[doc = "PDAWAKECFG (rw) register accessor: an alias for `Reg<PDAWAKECFG_SPEC>`"]
pub type PDAWAKECFG = crate::Reg<pdawakecfg::PDAWAKECFG_SPEC>;
#[doc = "Power-down states after wake-up from Deep-sleep mode"]
pub mod pdawakecfg;
#[doc = "PDRUNCFG (rw) register accessor: an alias for `Reg<PDRUNCFG_SPEC>`"]
pub type PDRUNCFG = crate::Reg<pdruncfg::PDRUNCFG_SPEC>;
#[doc = "Power-down configuration register"]
pub mod pdruncfg;
#[doc = "DEVICE_ID (r) register accessor: an alias for `Reg<DEVICE_ID_SPEC>`"]
pub type DEVICE_ID = crate::Reg<device_id::DEVICE_ID_SPEC>;
#[doc = "Device ID"]
pub mod device_id;
