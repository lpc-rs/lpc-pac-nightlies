#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10..0x30 - Register for reading the AES key."]
    pub aeskey: [AESKEY; 8],
    #[doc = "0x30 - ECRP options."]
    pub ecrp: ECRP,
    _reserved2: [u8; 0x04],
    #[doc = "0x38 - User application specific options."]
    pub user0: USER0,
    #[doc = "0x3c - User application specific options."]
    pub user1: USER1,
}
#[doc = "AESKEY (r) register accessor: an alias for `Reg<AESKEY_SPEC>`"]
pub type AESKEY = crate::Reg<aeskey::AESKEY_SPEC>;
#[doc = "Register for reading the AES key."]
pub mod aeskey;
#[doc = "ECRP (r) register accessor: an alias for `Reg<ECRP_SPEC>`"]
pub type ECRP = crate::Reg<ecrp::ECRP_SPEC>;
#[doc = "ECRP options."]
pub mod ecrp;
#[doc = "USER0 (r) register accessor: an alias for `Reg<USER0_SPEC>`"]
pub type USER0 = crate::Reg<user0::USER0_SPEC>;
#[doc = "User application specific options."]
pub mod user0;
#[doc = "USER1 (r) register accessor: an alias for `Reg<USER1_SPEC>`"]
pub type USER1 = crate::Reg<user1::USER1_SPEC>;
#[doc = "User application specific options."]
pub mod user1;
