#[doc = r"Register block"]
#[repr(C)]
pub struct MTL_QUEUE {
    #[doc = "0x00 - MTL TxQx Operation Mode register"]
    pub mtl_txqx_op_mode: MTL_TXQX_OP_MODE,
    #[doc = "0x04 - MTL TxQx Underflow register"]
    pub mtl_txqx_undrflw: MTL_TXQX_UNDRFLW,
    #[doc = "0x08 - MTL TxQx Debug register"]
    pub mtl_txqx_dbg: MTL_TXQX_DBG,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - MTL TxQx ETS control register, only TxQ1 support"]
    pub mtl_txqx_ets_ctrl: MTL_TXQX_ETS_CTRL,
    #[doc = "0x14 - MTL TxQx ETS Status register"]
    pub mtl_txqx_ets_stat: MTL_TXQX_ETS_STAT,
    #[doc = "0x18 - no description available"]
    pub mtl_txqx_qntm_wght: MTL_TXQX_QNTM_WGHT,
    #[doc = "0x1c - MTL TxQx SendSlopCredit register, only TxQ1 support"]
    pub mtl_txqx_sndslp_crdt: MTL_TXQX_SNDSLP_CRDT,
    #[doc = "0x20 - MTL TxQx hiCredit register, only TxQ1 support"]
    pub mtl_txqx_hi_crdt: MTL_TXQX_HI_CRDT,
    #[doc = "0x24 - MTL TxQx loCredit register, only TxQ1 support"]
    pub mtl_txqx_lo_crdt: MTL_TXQX_LO_CRDT,
    _reserved9: [u8; 0x04],
    #[doc = "0x2c - no description available"]
    pub mtl_txqx_intctrl_stat: MTL_TXQX_INTCTRL_STAT,
    #[doc = "0x30 - MTL RxQx Operation Mode register"]
    pub mtl_rxqx_op_mode: MTL_RXQX_OP_MODE,
    #[doc = "0x34 - MTL RxQx Missed Packet Overflow Counter register"]
    pub mtl_rxqx_misspkt_ovrflw_cnt: MTL_RXQX_MISSPKT_OVRFLW_CNT,
    #[doc = "0x38 - MTL RxQx Debug register"]
    pub mtl_rxqx_dbg: MTL_RXQX_DBG,
    #[doc = "0x3c - MTL RxQx Control register"]
    pub mtl_rxqx_ctrl: MTL_RXQX_CTRL,
}
#[doc = "MTL_TXQx_OP_MODE (rw) register accessor: an alias for `Reg<MTL_TXQX_OP_MODE_SPEC>`"]
pub type MTL_TXQX_OP_MODE = crate::Reg<mtl_txqx_op_mode::MTL_TXQX_OP_MODE_SPEC>;
#[doc = "MTL TxQx Operation Mode register"]
pub mod mtl_txqx_op_mode;
#[doc = "MTL_TXQx_UNDRFLW (r) register accessor: an alias for `Reg<MTL_TXQX_UNDRFLW_SPEC>`"]
pub type MTL_TXQX_UNDRFLW = crate::Reg<mtl_txqx_undrflw::MTL_TXQX_UNDRFLW_SPEC>;
#[doc = "MTL TxQx Underflow register"]
pub mod mtl_txqx_undrflw;
#[doc = "MTL_TXQx_DBG (r) register accessor: an alias for `Reg<MTL_TXQX_DBG_SPEC>`"]
pub type MTL_TXQX_DBG = crate::Reg<mtl_txqx_dbg::MTL_TXQX_DBG_SPEC>;
#[doc = "MTL TxQx Debug register"]
pub mod mtl_txqx_dbg;
#[doc = "MTL_TXQx_ETS_CTRL (rw) register accessor: an alias for `Reg<MTL_TXQX_ETS_CTRL_SPEC>`"]
pub type MTL_TXQX_ETS_CTRL = crate::Reg<mtl_txqx_ets_ctrl::MTL_TXQX_ETS_CTRL_SPEC>;
#[doc = "MTL TxQx ETS control register, only TxQ1 support"]
pub mod mtl_txqx_ets_ctrl;
#[doc = "MTL_TXQx_ETS_STAT (rw) register accessor: an alias for `Reg<MTL_TXQX_ETS_STAT_SPEC>`"]
pub type MTL_TXQX_ETS_STAT = crate::Reg<mtl_txqx_ets_stat::MTL_TXQX_ETS_STAT_SPEC>;
#[doc = "MTL TxQx ETS Status register"]
pub mod mtl_txqx_ets_stat;
#[doc = "MTL_TXQx_QNTM_WGHT (rw) register accessor: an alias for `Reg<MTL_TXQX_QNTM_WGHT_SPEC>`"]
pub type MTL_TXQX_QNTM_WGHT = crate::Reg<mtl_txqx_qntm_wght::MTL_TXQX_QNTM_WGHT_SPEC>;
#[doc = "no description available"]
pub mod mtl_txqx_qntm_wght;
#[doc = "MTL_TXQx_SNDSLP_CRDT (rw) register accessor: an alias for `Reg<MTL_TXQX_SNDSLP_CRDT_SPEC>`"]
pub type MTL_TXQX_SNDSLP_CRDT = crate::Reg<mtl_txqx_sndslp_crdt::MTL_TXQX_SNDSLP_CRDT_SPEC>;
#[doc = "MTL TxQx SendSlopCredit register, only TxQ1 support"]
pub mod mtl_txqx_sndslp_crdt;
#[doc = "MTL_TXQx_HI_CRDT (rw) register accessor: an alias for `Reg<MTL_TXQX_HI_CRDT_SPEC>`"]
pub type MTL_TXQX_HI_CRDT = crate::Reg<mtl_txqx_hi_crdt::MTL_TXQX_HI_CRDT_SPEC>;
#[doc = "MTL TxQx hiCredit register, only TxQ1 support"]
pub mod mtl_txqx_hi_crdt;
#[doc = "MTL_TXQx_LO_CRDT (rw) register accessor: an alias for `Reg<MTL_TXQX_LO_CRDT_SPEC>`"]
pub type MTL_TXQX_LO_CRDT = crate::Reg<mtl_txqx_lo_crdt::MTL_TXQX_LO_CRDT_SPEC>;
#[doc = "MTL TxQx loCredit register, only TxQ1 support"]
pub mod mtl_txqx_lo_crdt;
#[doc = "MTL_TXQx_INTCTRL_STAT (rw) register accessor: an alias for `Reg<MTL_TXQX_INTCTRL_STAT_SPEC>`"]
pub type MTL_TXQX_INTCTRL_STAT = crate::Reg<mtl_txqx_intctrl_stat::MTL_TXQX_INTCTRL_STAT_SPEC>;
#[doc = "no description available"]
pub mod mtl_txqx_intctrl_stat;
#[doc = "MTL_RXQx_OP_MODE (rw) register accessor: an alias for `Reg<MTL_RXQX_OP_MODE_SPEC>`"]
pub type MTL_RXQX_OP_MODE = crate::Reg<mtl_rxqx_op_mode::MTL_RXQX_OP_MODE_SPEC>;
#[doc = "MTL RxQx Operation Mode register"]
pub mod mtl_rxqx_op_mode;
#[doc = "MTL_RXQx_MISSPKT_OVRFLW_CNT (rw) register accessor: an alias for `Reg<MTL_RXQX_MISSPKT_OVRFLW_CNT_SPEC>`"]
pub type MTL_RXQX_MISSPKT_OVRFLW_CNT =
    crate::Reg<mtl_rxqx_misspkt_ovrflw_cnt::MTL_RXQX_MISSPKT_OVRFLW_CNT_SPEC>;
#[doc = "MTL RxQx Missed Packet Overflow Counter register"]
pub mod mtl_rxqx_misspkt_ovrflw_cnt;
#[doc = "MTL_RXQx_DBG (rw) register accessor: an alias for `Reg<MTL_RXQX_DBG_SPEC>`"]
pub type MTL_RXQX_DBG = crate::Reg<mtl_rxqx_dbg::MTL_RXQX_DBG_SPEC>;
#[doc = "MTL RxQx Debug register"]
pub mod mtl_rxqx_dbg;
#[doc = "MTL_RXQx_CTRL (rw) register accessor: an alias for `Reg<MTL_RXQX_CTRL_SPEC>`"]
pub type MTL_RXQX_CTRL = crate::Reg<mtl_rxqx_ctrl::MTL_RXQX_CTRL_SPEC>;
#[doc = "MTL RxQx Control register"]
pub mod mtl_rxqx_ctrl;
