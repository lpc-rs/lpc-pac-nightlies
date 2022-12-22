#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Control register. Contains the clock divide value, resolution selection, sampling time selection, and mode controls."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Input Select. Allows selection of the temperature sensor as an alternate input to ADC channel 0."]
    pub insel: INSEL,
    #[doc = "0x08..0x10 - ADC Conversion Sequence-n control register: Controls triggering and channel selection for conversion sequence-n. Also specifies interrupt mode for sequence-n."]
    pub seq_ctrl: [SEQ_CTRL; 2],
    #[doc = "0x10..0x18 - ADC Sequence-n Global Data register. This register contains the result of the most recent ADC conversion performed under sequence-n."]
    pub seq_gdat: [SEQ_GDAT; 2],
    _reserved4: [u8; 0x08],
    #[doc = "0x20..0x50 - ADC Channel 0 Data register. This register contains the result of the most recent conversion completed on channel 0."]
    pub dat: [DAT; 12],
    #[doc = "0x50 - ADC Low Compare Threshold register 0: Contains the lower threshold level for automatic threshold comparison for any channels linked to threshold pair 0."]
    pub thr0_low: THR0_LOW,
    #[doc = "0x54 - ADC Low Compare Threshold register 1: Contains the lower threshold level for automatic threshold comparison for any channels linked to threshold pair 1."]
    pub thr1_low: THR1_LOW,
    #[doc = "0x58 - ADC High Compare Threshold register 0: Contains the upper threshold level for automatic threshold comparison for any channels linked to threshold pair 0."]
    pub thr0_high: THR0_HIGH,
    #[doc = "0x5c - ADC High Compare Threshold register 1: Contains the upper threshold level for automatic threshold comparison for any channels linked to threshold pair 1."]
    pub thr1_high: THR1_HIGH,
    #[doc = "0x60 - ADC Channel-Threshold Select register. Specifies which set of threshold compare registers are to be used for each channel"]
    pub chan_thrsel: CHAN_THRSEL,
    #[doc = "0x64 - ADC Interrupt Enable register. This register contains enable bits that enable the sequence-A, sequence-B, threshold compare and data overrun interrupts to be generated."]
    pub inten: INTEN,
    #[doc = "0x68 - ADC Flags register. Contains the four interrupt/DMA trigger flags and the individual component overrun and threshold-compare flags. (The overrun bits replicate information stored in the result registers)."]
    pub flags: FLAGS,
    #[doc = "0x6c - ADC Startup register."]
    pub startup: STARTUP,
    #[doc = "0x70 - ADC Calibration register."]
    pub calib: CALIB,
}
impl RegisterBlock {
    #[doc = "0x08 - ADC Conversion Sequence-n control register: Controls triggering and channel selection for conversion sequence-n. Also specifies interrupt mode for sequence-n."]
    #[inline(always)]
    pub fn seq_ctrla(&self) -> &SEQ_CTRL {
        &self.seq_ctrl[0]
    }
    #[doc = "0x0c - ADC Conversion Sequence-n control register: Controls triggering and channel selection for conversion sequence-n. Also specifies interrupt mode for sequence-n."]
    #[inline(always)]
    pub fn seq_ctrlb(&self) -> &SEQ_CTRL {
        &self.seq_ctrl[1]
    }
    #[doc = "0x10 - ADC Sequence-n Global Data register. This register contains the result of the most recent ADC conversion performed under sequence-n."]
    #[inline(always)]
    pub fn seq_gdata(&self) -> &SEQ_GDAT {
        &self.seq_gdat[0]
    }
    #[doc = "0x14 - ADC Sequence-n Global Data register. This register contains the result of the most recent ADC conversion performed under sequence-n."]
    #[inline(always)]
    pub fn seq_gdatb(&self) -> &SEQ_GDAT {
        &self.seq_gdat[1]
    }
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "ADC Control register. Contains the clock divide value, resolution selection, sampling time selection, and mode controls."]
pub mod ctrl;
#[doc = "INSEL (rw) register accessor: an alias for `Reg<INSEL_SPEC>`"]
pub type INSEL = crate::Reg<insel::INSEL_SPEC>;
#[doc = "Input Select. Allows selection of the temperature sensor as an alternate input to ADC channel 0."]
pub mod insel;
#[doc = "SEQ_CTRL (rw) register accessor: an alias for `Reg<SEQ_CTRL_SPEC>`"]
pub type SEQ_CTRL = crate::Reg<seq_ctrl::SEQ_CTRL_SPEC>;
#[doc = "ADC Conversion Sequence-n control register: Controls triggering and channel selection for conversion sequence-n. Also specifies interrupt mode for sequence-n."]
pub mod seq_ctrl;
#[doc = "SEQ_GDAT (r) register accessor: an alias for `Reg<SEQ_GDAT_SPEC>`"]
pub type SEQ_GDAT = crate::Reg<seq_gdat::SEQ_GDAT_SPEC>;
#[doc = "ADC Sequence-n Global Data register. This register contains the result of the most recent ADC conversion performed under sequence-n."]
pub mod seq_gdat;
#[doc = "DAT (r) register accessor: an alias for `Reg<DAT_SPEC>`"]
pub type DAT = crate::Reg<dat::DAT_SPEC>;
#[doc = "ADC Channel 0 Data register. This register contains the result of the most recent conversion completed on channel 0."]
pub mod dat;
#[doc = "THR0_LOW (rw) register accessor: an alias for `Reg<THR0_LOW_SPEC>`"]
pub type THR0_LOW = crate::Reg<thr0_low::THR0_LOW_SPEC>;
#[doc = "ADC Low Compare Threshold register 0: Contains the lower threshold level for automatic threshold comparison for any channels linked to threshold pair 0."]
pub mod thr0_low;
#[doc = "THR1_LOW (rw) register accessor: an alias for `Reg<THR1_LOW_SPEC>`"]
pub type THR1_LOW = crate::Reg<thr1_low::THR1_LOW_SPEC>;
#[doc = "ADC Low Compare Threshold register 1: Contains the lower threshold level for automatic threshold comparison for any channels linked to threshold pair 1."]
pub mod thr1_low;
#[doc = "THR0_HIGH (rw) register accessor: an alias for `Reg<THR0_HIGH_SPEC>`"]
pub type THR0_HIGH = crate::Reg<thr0_high::THR0_HIGH_SPEC>;
#[doc = "ADC High Compare Threshold register 0: Contains the upper threshold level for automatic threshold comparison for any channels linked to threshold pair 0."]
pub mod thr0_high;
#[doc = "THR1_HIGH (rw) register accessor: an alias for `Reg<THR1_HIGH_SPEC>`"]
pub type THR1_HIGH = crate::Reg<thr1_high::THR1_HIGH_SPEC>;
#[doc = "ADC High Compare Threshold register 1: Contains the upper threshold level for automatic threshold comparison for any channels linked to threshold pair 1."]
pub mod thr1_high;
#[doc = "CHAN_THRSEL (rw) register accessor: an alias for `Reg<CHAN_THRSEL_SPEC>`"]
pub type CHAN_THRSEL = crate::Reg<chan_thrsel::CHAN_THRSEL_SPEC>;
#[doc = "ADC Channel-Threshold Select register. Specifies which set of threshold compare registers are to be used for each channel"]
pub mod chan_thrsel;
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "ADC Interrupt Enable register. This register contains enable bits that enable the sequence-A, sequence-B, threshold compare and data overrun interrupts to be generated."]
pub mod inten;
#[doc = "FLAGS (rw) register accessor: an alias for `Reg<FLAGS_SPEC>`"]
pub type FLAGS = crate::Reg<flags::FLAGS_SPEC>;
#[doc = "ADC Flags register. Contains the four interrupt/DMA trigger flags and the individual component overrun and threshold-compare flags. (The overrun bits replicate information stored in the result registers)."]
pub mod flags;
#[doc = "STARTUP (rw) register accessor: an alias for `Reg<STARTUP_SPEC>`"]
pub type STARTUP = crate::Reg<startup::STARTUP_SPEC>;
#[doc = "ADC Startup register."]
pub mod startup;
#[doc = "CALIB (rw) register accessor: an alias for `Reg<CALIB_SPEC>`"]
pub type CALIB = crate::Reg<calib::CALIB_SPEC>;
#[doc = "ADC Calibration register."]
pub mod calib;
