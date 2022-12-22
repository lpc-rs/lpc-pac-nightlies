#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Remap register"]
    pub sysmemremap: SYSMEMREMAP,
    #[doc = "0x04 - Peripheral reset control register"]
    pub presetctrl: PRESETCTRL,
    #[doc = "0x08 - PLL control"]
    pub syspllctrl: SYSPLLCTRL,
    #[doc = "0x0c - PLL status"]
    pub syspllstat: SYSPLLSTAT,
    _reserved4: [u8; 0x10],
    #[doc = "0x20 - system oscillator control"]
    pub sysoscctrl: SYSOSCCTRL,
    #[doc = "0x24 - Watchdog oscillator control"]
    pub wdtoscctrl: WDTOSCCTRL,
    _reserved6: [u8; 0x08],
    #[doc = "0x30 - System reset status register"]
    pub sysrststat: SYSRSTSTAT,
    _reserved7: [u8; 0x0c],
    #[doc = "0x40 - System PLL clock source select register"]
    pub syspllclksel: SYSPLLCLKSEL,
    #[doc = "0x44 - System PLL clock source update enable register"]
    pub syspllclkuen: SYSPLLCLKUEN,
    _reserved9: [u8; 0x28],
    #[doc = "0x70 - Main clock source select"]
    pub mainclksel: MAINCLKSEL,
    #[doc = "0x74 - Main clock source update enable"]
    pub mainclkuen: MAINCLKUEN,
    #[doc = "0x78 - System clock divider"]
    pub sysahbclkdiv: SYSAHBCLKDIV,
    _reserved12: [u8; 0x04],
    #[doc = "0x80 - System clock control"]
    pub sysahbclkctrl: SYSAHBCLKCTRL,
    _reserved13: [u8; 0x10],
    #[doc = "0x94 - USART clock divider"]
    pub uartclkdiv: UARTCLKDIV,
    _reserved14: [u8; 0x48],
    #[doc = "0xe0 - CLKOUT clock source select"]
    pub clkoutsel: CLKOUTSEL,
    #[doc = "0xe4 - CLKOUT clock source update enable"]
    pub clkoutuen: CLKOUTUEN,
    #[doc = "0xe8 - clock out divider"]
    pub clkoutdiv: CLKOUTDIV,
    _reserved17: [u8; 0x04],
    #[doc = "0xf0 - USART common fractional generator divider value"]
    pub uartfrgdiv: UARTFRGDIV,
    #[doc = "0xf4 - USART common fractional generator divider value"]
    pub uartfrgmult: UARTFRGMULT,
    _reserved19: [u8; 0x04],
    #[doc = "0xfc - External trace buffer command register"]
    pub exttracecmd: EXTTRACECMD,
    #[doc = "0x100 - POR captured PIO status 0"]
    pub pioporcap0: PIOPORCAP0,
    _reserved21: [u8; 0x30],
    #[doc = "0x134 - Peripheral clock 6 to the IOCON block for programmable glitch filter"]
    pub ioconclkdiv6: IOCONCLKDIV6,
    #[doc = "0x138 - Peripheral clock 6 to the IOCON block for programmable glitch filter"]
    pub ioconclkdiv5: IOCONCLKDIV5,
    #[doc = "0x13c - Peripheral clock 4 to the IOCON block for programmable glitch filter"]
    pub ioconclkdiv4: IOCONCLKDIV4,
    #[doc = "0x140 - Peripheral clock 3 to the IOCON block for programmable glitch filter"]
    pub ioconclkdiv3: IOCONCLKDIV3,
    #[doc = "0x144 - Peripheral clock 2 to the IOCON block for programmable glitch filter"]
    pub ioconclkdiv2: IOCONCLKDIV2,
    #[doc = "0x148 - Peripheral clock 1 to the IOCON block for programmable glitch filter"]
    pub ioconclkdiv1: IOCONCLKDIV1,
    #[doc = "0x14c - Peripheral clock 0 to the IOCON block for programmable glitch filter"]
    pub ioconclkdiv0: IOCONCLKDIV0,
    #[doc = "0x150 - BOD control register"]
    pub bodctrl: BODCTRL,
    #[doc = "0x154 - System tick timer calibration register"]
    pub systckcal: SYSTCKCAL,
    _reserved30: [u8; 0x18],
    #[doc = "0x170 - IRQ latency register"]
    pub irqlatency: IRQLATENCY,
    #[doc = "0x174 - NMI source selection register"]
    pub nmisrc: NMISRC,
    #[doc = "0x178..0x198 - Pin interrupt select registers N"]
    pub pintsel: [PINTSEL; 8],
    _reserved33: [u8; 0x6c],
    #[doc = "0x204 - Start logic 0 pin wake-up enable register 0"]
    pub starterp0: STARTERP0,
    _reserved34: [u8; 0x0c],
    #[doc = "0x214 - Start logic 0 pin wake-up enable register 1"]
    pub starterp1: STARTERP1,
    _reserved35: [u8; 0x18],
    #[doc = "0x230 - Deep-sleep configuration register"]
    pub pdsleepcfg: PDSLEEPCFG,
    #[doc = "0x234 - Wake-up configuration register"]
    pub pdawakecfg: PDAWAKECFG,
    #[doc = "0x238 - Power configuration register"]
    pub pdruncfg: PDRUNCFG,
    _reserved38: [u8; 0x01bc],
    #[doc = "0x3f8 - Part ID register"]
    pub device_id: DEVICE_ID,
}
#[doc = "SYSMEMREMAP (rw) register accessor: an alias for `Reg<SYSMEMREMAP_SPEC>`"]
pub type SYSMEMREMAP = crate::Reg<sysmemremap::SYSMEMREMAP_SPEC>;
#[doc = "System Remap register"]
pub mod sysmemremap;
#[doc = "PRESETCTRL (rw) register accessor: an alias for `Reg<PRESETCTRL_SPEC>`"]
pub type PRESETCTRL = crate::Reg<presetctrl::PRESETCTRL_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod presetctrl;
#[doc = "SYSPLLCTRL (rw) register accessor: an alias for `Reg<SYSPLLCTRL_SPEC>`"]
pub type SYSPLLCTRL = crate::Reg<syspllctrl::SYSPLLCTRL_SPEC>;
#[doc = "PLL control"]
pub mod syspllctrl;
#[doc = "SYSPLLSTAT (r) register accessor: an alias for `Reg<SYSPLLSTAT_SPEC>`"]
pub type SYSPLLSTAT = crate::Reg<syspllstat::SYSPLLSTAT_SPEC>;
#[doc = "PLL status"]
pub mod syspllstat;
#[doc = "SYSOSCCTRL (rw) register accessor: an alias for `Reg<SYSOSCCTRL_SPEC>`"]
pub type SYSOSCCTRL = crate::Reg<sysoscctrl::SYSOSCCTRL_SPEC>;
#[doc = "system oscillator control"]
pub mod sysoscctrl;
#[doc = "WDTOSCCTRL (rw) register accessor: an alias for `Reg<WDTOSCCTRL_SPEC>`"]
pub type WDTOSCCTRL = crate::Reg<wdtoscctrl::WDTOSCCTRL_SPEC>;
#[doc = "Watchdog oscillator control"]
pub mod wdtoscctrl;
#[doc = "SYSRSTSTAT (rw) register accessor: an alias for `Reg<SYSRSTSTAT_SPEC>`"]
pub type SYSRSTSTAT = crate::Reg<sysrststat::SYSRSTSTAT_SPEC>;
#[doc = "System reset status register"]
pub mod sysrststat;
#[doc = "SYSPLLCLKSEL (rw) register accessor: an alias for `Reg<SYSPLLCLKSEL_SPEC>`"]
pub type SYSPLLCLKSEL = crate::Reg<syspllclksel::SYSPLLCLKSEL_SPEC>;
#[doc = "System PLL clock source select register"]
pub mod syspllclksel;
#[doc = "SYSPLLCLKUEN (rw) register accessor: an alias for `Reg<SYSPLLCLKUEN_SPEC>`"]
pub type SYSPLLCLKUEN = crate::Reg<syspllclkuen::SYSPLLCLKUEN_SPEC>;
#[doc = "System PLL clock source update enable register"]
pub mod syspllclkuen;
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
#[doc = "System clock divider"]
pub mod sysahbclkdiv;
#[doc = "SYSAHBCLKCTRL (rw) register accessor: an alias for `Reg<SYSAHBCLKCTRL_SPEC>`"]
pub type SYSAHBCLKCTRL = crate::Reg<sysahbclkctrl::SYSAHBCLKCTRL_SPEC>;
#[doc = "System clock control"]
pub mod sysahbclkctrl;
#[doc = "UARTCLKDIV (rw) register accessor: an alias for `Reg<UARTCLKDIV_SPEC>`"]
pub type UARTCLKDIV = crate::Reg<uartclkdiv::UARTCLKDIV_SPEC>;
#[doc = "USART clock divider"]
pub mod uartclkdiv;
#[doc = "CLKOUTSEL (rw) register accessor: an alias for `Reg<CLKOUTSEL_SPEC>`"]
pub type CLKOUTSEL = crate::Reg<clkoutsel::CLKOUTSEL_SPEC>;
#[doc = "CLKOUT clock source select"]
pub mod clkoutsel;
#[doc = "CLKOUTUEN (rw) register accessor: an alias for `Reg<CLKOUTUEN_SPEC>`"]
pub type CLKOUTUEN = crate::Reg<clkoutuen::CLKOUTUEN_SPEC>;
#[doc = "CLKOUT clock source update enable"]
pub mod clkoutuen;
#[doc = "CLKOUTDIV (rw) register accessor: an alias for `Reg<CLKOUTDIV_SPEC>`"]
pub type CLKOUTDIV = crate::Reg<clkoutdiv::CLKOUTDIV_SPEC>;
#[doc = "clock out divider"]
pub mod clkoutdiv;
#[doc = "UARTFRGDIV (rw) register accessor: an alias for `Reg<UARTFRGDIV_SPEC>`"]
pub type UARTFRGDIV = crate::Reg<uartfrgdiv::UARTFRGDIV_SPEC>;
#[doc = "USART common fractional generator divider value"]
pub mod uartfrgdiv;
#[doc = "UARTFRGMULT (rw) register accessor: an alias for `Reg<UARTFRGMULT_SPEC>`"]
pub type UARTFRGMULT = crate::Reg<uartfrgmult::UARTFRGMULT_SPEC>;
#[doc = "USART common fractional generator divider value"]
pub mod uartfrgmult;
#[doc = "EXTTRACECMD (rw) register accessor: an alias for `Reg<EXTTRACECMD_SPEC>`"]
pub type EXTTRACECMD = crate::Reg<exttracecmd::EXTTRACECMD_SPEC>;
#[doc = "External trace buffer command register"]
pub mod exttracecmd;
#[doc = "PIOPORCAP0 (rw) register accessor: an alias for `Reg<PIOPORCAP0_SPEC>`"]
pub type PIOPORCAP0 = crate::Reg<pioporcap0::PIOPORCAP0_SPEC>;
#[doc = "POR captured PIO status 0"]
pub mod pioporcap0;
#[doc = "IOCONCLKDIV6 (rw) register accessor: an alias for `Reg<IOCONCLKDIV6_SPEC>`"]
pub type IOCONCLKDIV6 = crate::Reg<ioconclkdiv6::IOCONCLKDIV6_SPEC>;
#[doc = "Peripheral clock 6 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv6;
#[doc = "IOCONCLKDIV5 (rw) register accessor: an alias for `Reg<IOCONCLKDIV5_SPEC>`"]
pub type IOCONCLKDIV5 = crate::Reg<ioconclkdiv5::IOCONCLKDIV5_SPEC>;
#[doc = "Peripheral clock 6 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv5;
#[doc = "IOCONCLKDIV4 (rw) register accessor: an alias for `Reg<IOCONCLKDIV4_SPEC>`"]
pub type IOCONCLKDIV4 = crate::Reg<ioconclkdiv4::IOCONCLKDIV4_SPEC>;
#[doc = "Peripheral clock 4 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv4;
#[doc = "IOCONCLKDIV3 (rw) register accessor: an alias for `Reg<IOCONCLKDIV3_SPEC>`"]
pub type IOCONCLKDIV3 = crate::Reg<ioconclkdiv3::IOCONCLKDIV3_SPEC>;
#[doc = "Peripheral clock 3 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv3;
#[doc = "IOCONCLKDIV2 (rw) register accessor: an alias for `Reg<IOCONCLKDIV2_SPEC>`"]
pub type IOCONCLKDIV2 = crate::Reg<ioconclkdiv2::IOCONCLKDIV2_SPEC>;
#[doc = "Peripheral clock 2 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv2;
#[doc = "IOCONCLKDIV1 (rw) register accessor: an alias for `Reg<IOCONCLKDIV1_SPEC>`"]
pub type IOCONCLKDIV1 = crate::Reg<ioconclkdiv1::IOCONCLKDIV1_SPEC>;
#[doc = "Peripheral clock 1 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv1;
#[doc = "IOCONCLKDIV0 (rw) register accessor: an alias for `Reg<IOCONCLKDIV0_SPEC>`"]
pub type IOCONCLKDIV0 = crate::Reg<ioconclkdiv0::IOCONCLKDIV0_SPEC>;
#[doc = "Peripheral clock 0 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv0;
#[doc = "BODCTRL (rw) register accessor: an alias for `Reg<BODCTRL_SPEC>`"]
pub type BODCTRL = crate::Reg<bodctrl::BODCTRL_SPEC>;
#[doc = "BOD control register"]
pub mod bodctrl;
#[doc = "SYSTCKCAL (rw) register accessor: an alias for `Reg<SYSTCKCAL_SPEC>`"]
pub type SYSTCKCAL = crate::Reg<systckcal::SYSTCKCAL_SPEC>;
#[doc = "System tick timer calibration register"]
pub mod systckcal;
#[doc = "IRQLATENCY (rw) register accessor: an alias for `Reg<IRQLATENCY_SPEC>`"]
pub type IRQLATENCY = crate::Reg<irqlatency::IRQLATENCY_SPEC>;
#[doc = "IRQ latency register"]
pub mod irqlatency;
#[doc = "NMISRC (rw) register accessor: an alias for `Reg<NMISRC_SPEC>`"]
pub type NMISRC = crate::Reg<nmisrc::NMISRC_SPEC>;
#[doc = "NMI source selection register"]
pub mod nmisrc;
#[doc = "PINTSEL (rw) register accessor: an alias for `Reg<PINTSEL_SPEC>`"]
pub type PINTSEL = crate::Reg<pintsel::PINTSEL_SPEC>;
#[doc = "Pin interrupt select registers N"]
pub mod pintsel;
#[doc = "STARTERP0 (rw) register accessor: an alias for `Reg<STARTERP0_SPEC>`"]
pub type STARTERP0 = crate::Reg<starterp0::STARTERP0_SPEC>;
#[doc = "Start logic 0 pin wake-up enable register 0"]
pub mod starterp0;
#[doc = "STARTERP1 (rw) register accessor: an alias for `Reg<STARTERP1_SPEC>`"]
pub type STARTERP1 = crate::Reg<starterp1::STARTERP1_SPEC>;
#[doc = "Start logic 0 pin wake-up enable register 1"]
pub mod starterp1;
#[doc = "PDSLEEPCFG (rw) register accessor: an alias for `Reg<PDSLEEPCFG_SPEC>`"]
pub type PDSLEEPCFG = crate::Reg<pdsleepcfg::PDSLEEPCFG_SPEC>;
#[doc = "Deep-sleep configuration register"]
pub mod pdsleepcfg;
#[doc = "PDAWAKECFG (rw) register accessor: an alias for `Reg<PDAWAKECFG_SPEC>`"]
pub type PDAWAKECFG = crate::Reg<pdawakecfg::PDAWAKECFG_SPEC>;
#[doc = "Wake-up configuration register"]
pub mod pdawakecfg;
#[doc = "PDRUNCFG (rw) register accessor: an alias for `Reg<PDRUNCFG_SPEC>`"]
pub type PDRUNCFG = crate::Reg<pdruncfg::PDRUNCFG_SPEC>;
#[doc = "Power configuration register"]
pub mod pdruncfg;
#[doc = "DEVICE_ID (r) register accessor: an alias for `Reg<DEVICE_ID_SPEC>`"]
pub type DEVICE_ID = crate::Reg<device_id::DEVICE_ID_SPEC>;
#[doc = "Part ID register"]
pub mod device_id;
