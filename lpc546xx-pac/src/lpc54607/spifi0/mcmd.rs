#[doc = "Register `MCMD` reader"]
pub struct R(crate::R<MCMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCMD` writer"]
pub struct W(crate::W<MCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POLL` reader - This bit should be written as 0."]
pub type POLL_R = crate::BitReader<bool>;
#[doc = "Field `POLL` writer - This bit should be written as 0."]
pub type POLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCMD_SPEC, bool, O>;
#[doc = "Field `DOUT` reader - This bit should be written as 0."]
pub type DOUT_R = crate::BitReader<bool>;
#[doc = "Field `DOUT` writer - This bit should be written as 0."]
pub type DOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCMD_SPEC, bool, O>;
#[doc = "Field `INTLEN` reader - This field controls how many intermediate bytes precede the data. (Each such byte may require 8 or 2 SCK cycles, depending on whether the intermediate field is in serial, 2-bit, or 4-bit format.) Intermediate bytes are output by the SPIFI, and include post-address control information, dummy and delay bytes. See the description of the Intermediate Data register for the contents of such bytes."]
pub type INTLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTLEN` writer - This field controls how many intermediate bytes precede the data. (Each such byte may require 8 or 2 SCK cycles, depending on whether the intermediate field is in serial, 2-bit, or 4-bit format.) Intermediate bytes are output by the SPIFI, and include post-address control information, dummy and delay bytes. See the description of the Intermediate Data register for the contents of such bytes."]
pub type INTLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCMD_SPEC, u8, u8, 3, O>;
#[doc = "Field `FIELDFORM` reader - This field controls how the fields of the command are sent."]
pub type FIELDFORM_R = crate::FieldReader<u8, FIELDFORM_A>;
#[doc = "This field controls how the fields of the command are sent.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FIELDFORM_A {
    #[doc = "0: All serial. All fields of the command are serial."]
    ALL_SERIAL = 0,
    #[doc = "1: Quad/dual data. Data field is quad/dual, other fields are serial."]
    QUAD_DUAL_DATA = 1,
    #[doc = "2: Serial opcode. Opcode field is serial. Other fields are quad/dual."]
    SERIAL_OPCODE = 2,
    #[doc = "3: All quad/dual. All fields of the command are in quad/dual format."]
    ALL_QUAD_DUAL = 3,
}
impl From<FIELDFORM_A> for u8 {
    #[inline(always)]
    fn from(variant: FIELDFORM_A) -> Self {
        variant as _
    }
}
impl FIELDFORM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIELDFORM_A {
        match self.bits {
            0 => FIELDFORM_A::ALL_SERIAL,
            1 => FIELDFORM_A::QUAD_DUAL_DATA,
            2 => FIELDFORM_A::SERIAL_OPCODE,
            3 => FIELDFORM_A::ALL_QUAD_DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ALL_SERIAL`"]
    #[inline(always)]
    pub fn is_all_serial(&self) -> bool {
        *self == FIELDFORM_A::ALL_SERIAL
    }
    #[doc = "Checks if the value of the field is `QUAD_DUAL_DATA`"]
    #[inline(always)]
    pub fn is_quad_dual_data(&self) -> bool {
        *self == FIELDFORM_A::QUAD_DUAL_DATA
    }
    #[doc = "Checks if the value of the field is `SERIAL_OPCODE`"]
    #[inline(always)]
    pub fn is_serial_opcode(&self) -> bool {
        *self == FIELDFORM_A::SERIAL_OPCODE
    }
    #[doc = "Checks if the value of the field is `ALL_QUAD_DUAL`"]
    #[inline(always)]
    pub fn is_all_quad_dual(&self) -> bool {
        *self == FIELDFORM_A::ALL_QUAD_DUAL
    }
}
#[doc = "Field `FIELDFORM` writer - This field controls how the fields of the command are sent."]
pub type FIELDFORM_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MCMD_SPEC, u8, FIELDFORM_A, 2, O>;
impl<'a, const O: u8> FIELDFORM_W<'a, O> {
    #[doc = "All serial. All fields of the command are serial."]
    #[inline(always)]
    pub fn all_serial(self) -> &'a mut W {
        self.variant(FIELDFORM_A::ALL_SERIAL)
    }
    #[doc = "Quad/dual data. Data field is quad/dual, other fields are serial."]
    #[inline(always)]
    pub fn quad_dual_data(self) -> &'a mut W {
        self.variant(FIELDFORM_A::QUAD_DUAL_DATA)
    }
    #[doc = "Serial opcode. Opcode field is serial. Other fields are quad/dual."]
    #[inline(always)]
    pub fn serial_opcode(self) -> &'a mut W {
        self.variant(FIELDFORM_A::SERIAL_OPCODE)
    }
    #[doc = "All quad/dual. All fields of the command are in quad/dual format."]
    #[inline(always)]
    pub fn all_quad_dual(self) -> &'a mut W {
        self.variant(FIELDFORM_A::ALL_QUAD_DUAL)
    }
}
#[doc = "Field `FRAMEFORM` reader - This field controls the opcode and address fields."]
pub type FRAMEFORM_R = crate::FieldReader<u8, FRAMEFORM_A>;
#[doc = "This field controls the opcode and address fields.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FRAMEFORM_A {
    #[doc = "1: Opcode. Opcode only, no address."]
    OPCODE = 1,
    #[doc = "2: Opcode one byte. Opcode, least-significant byte of address."]
    OPCODE_1_BYTE = 2,
    #[doc = "3: Opcode two bytes. Opcode, 2 least-significant bytes of address."]
    OPCODE_2_BYTES = 3,
    #[doc = "4: Opcode three bytes. Opcode, 3 least-significant bytes of address."]
    OPCODE_3_BYTES = 4,
    #[doc = "5: Opcode four bytes. Opcode, 4 bytes of address."]
    OPCODE_4_BYTES = 5,
    #[doc = "6: No opcode three bytes. No opcode, 3 least-significant bytes of address."]
    NO_OPCODE_3_BYTES = 6,
    #[doc = "7: No opcode, 4 bytes of address."]
    NO_OPCODE_4_BYTES = 7,
}
impl From<FRAMEFORM_A> for u8 {
    #[inline(always)]
    fn from(variant: FRAMEFORM_A) -> Self {
        variant as _
    }
}
impl FRAMEFORM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FRAMEFORM_A> {
        match self.bits {
            1 => Some(FRAMEFORM_A::OPCODE),
            2 => Some(FRAMEFORM_A::OPCODE_1_BYTE),
            3 => Some(FRAMEFORM_A::OPCODE_2_BYTES),
            4 => Some(FRAMEFORM_A::OPCODE_3_BYTES),
            5 => Some(FRAMEFORM_A::OPCODE_4_BYTES),
            6 => Some(FRAMEFORM_A::NO_OPCODE_3_BYTES),
            7 => Some(FRAMEFORM_A::NO_OPCODE_4_BYTES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OPCODE`"]
    #[inline(always)]
    pub fn is_opcode(&self) -> bool {
        *self == FRAMEFORM_A::OPCODE
    }
    #[doc = "Checks if the value of the field is `OPCODE_1_BYTE`"]
    #[inline(always)]
    pub fn is_opcode_1_byte(&self) -> bool {
        *self == FRAMEFORM_A::OPCODE_1_BYTE
    }
    #[doc = "Checks if the value of the field is `OPCODE_2_BYTES`"]
    #[inline(always)]
    pub fn is_opcode_2_bytes(&self) -> bool {
        *self == FRAMEFORM_A::OPCODE_2_BYTES
    }
    #[doc = "Checks if the value of the field is `OPCODE_3_BYTES`"]
    #[inline(always)]
    pub fn is_opcode_3_bytes(&self) -> bool {
        *self == FRAMEFORM_A::OPCODE_3_BYTES
    }
    #[doc = "Checks if the value of the field is `OPCODE_4_BYTES`"]
    #[inline(always)]
    pub fn is_opcode_4_bytes(&self) -> bool {
        *self == FRAMEFORM_A::OPCODE_4_BYTES
    }
    #[doc = "Checks if the value of the field is `NO_OPCODE_3_BYTES`"]
    #[inline(always)]
    pub fn is_no_opcode_3_bytes(&self) -> bool {
        *self == FRAMEFORM_A::NO_OPCODE_3_BYTES
    }
    #[doc = "Checks if the value of the field is `NO_OPCODE_4_BYTES`"]
    #[inline(always)]
    pub fn is_no_opcode_4_bytes(&self) -> bool {
        *self == FRAMEFORM_A::NO_OPCODE_4_BYTES
    }
}
#[doc = "Field `FRAMEFORM` writer - This field controls the opcode and address fields."]
pub type FRAMEFORM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MCMD_SPEC, u8, FRAMEFORM_A, 3, O>;
impl<'a, const O: u8> FRAMEFORM_W<'a, O> {
    #[doc = "Opcode. Opcode only, no address."]
    #[inline(always)]
    pub fn opcode(self) -> &'a mut W {
        self.variant(FRAMEFORM_A::OPCODE)
    }
    #[doc = "Opcode one byte. Opcode, least-significant byte of address."]
    #[inline(always)]
    pub fn opcode_1_byte(self) -> &'a mut W {
        self.variant(FRAMEFORM_A::OPCODE_1_BYTE)
    }
    #[doc = "Opcode two bytes. Opcode, 2 least-significant bytes of address."]
    #[inline(always)]
    pub fn opcode_2_bytes(self) -> &'a mut W {
        self.variant(FRAMEFORM_A::OPCODE_2_BYTES)
    }
    #[doc = "Opcode three bytes. Opcode, 3 least-significant bytes of address."]
    #[inline(always)]
    pub fn opcode_3_bytes(self) -> &'a mut W {
        self.variant(FRAMEFORM_A::OPCODE_3_BYTES)
    }
    #[doc = "Opcode four bytes. Opcode, 4 bytes of address."]
    #[inline(always)]
    pub fn opcode_4_bytes(self) -> &'a mut W {
        self.variant(FRAMEFORM_A::OPCODE_4_BYTES)
    }
    #[doc = "No opcode three bytes. No opcode, 3 least-significant bytes of address."]
    #[inline(always)]
    pub fn no_opcode_3_bytes(self) -> &'a mut W {
        self.variant(FRAMEFORM_A::NO_OPCODE_3_BYTES)
    }
    #[doc = "No opcode, 4 bytes of address."]
    #[inline(always)]
    pub fn no_opcode_4_bytes(self) -> &'a mut W {
        self.variant(FRAMEFORM_A::NO_OPCODE_4_BYTES)
    }
}
#[doc = "Field `OPCODE` reader - The opcode of the command (not used for some FRAMEFORM values)."]
pub type OPCODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPCODE` writer - The opcode of the command (not used for some FRAMEFORM values)."]
pub type OPCODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCMD_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 14 - This bit should be written as 0."]
    #[inline(always)]
    pub fn poll(&self) -> POLL_R {
        POLL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This bit should be written as 0."]
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - This field controls how many intermediate bytes precede the data. (Each such byte may require 8 or 2 SCK cycles, depending on whether the intermediate field is in serial, 2-bit, or 4-bit format.) Intermediate bytes are output by the SPIFI, and include post-address control information, dummy and delay bytes. See the description of the Intermediate Data register for the contents of such bytes."]
    #[inline(always)]
    pub fn intlen(&self) -> INTLEN_R {
        INTLEN_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:20 - This field controls how the fields of the command are sent."]
    #[inline(always)]
    pub fn fieldform(&self) -> FIELDFORM_R {
        FIELDFORM_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:23 - This field controls the opcode and address fields."]
    #[inline(always)]
    pub fn frameform(&self) -> FRAMEFORM_R {
        FRAMEFORM_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:31 - The opcode of the command (not used for some FRAMEFORM values)."]
    #[inline(always)]
    pub fn opcode(&self) -> OPCODE_R {
        OPCODE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 14 - This bit should be written as 0."]
    #[inline(always)]
    pub fn poll(&mut self) -> POLL_W<14> {
        POLL_W::new(self)
    }
    #[doc = "Bit 15 - This bit should be written as 0."]
    #[inline(always)]
    pub fn dout(&mut self) -> DOUT_W<15> {
        DOUT_W::new(self)
    }
    #[doc = "Bits 16:18 - This field controls how many intermediate bytes precede the data. (Each such byte may require 8 or 2 SCK cycles, depending on whether the intermediate field is in serial, 2-bit, or 4-bit format.) Intermediate bytes are output by the SPIFI, and include post-address control information, dummy and delay bytes. See the description of the Intermediate Data register for the contents of such bytes."]
    #[inline(always)]
    pub fn intlen(&mut self) -> INTLEN_W<16> {
        INTLEN_W::new(self)
    }
    #[doc = "Bits 19:20 - This field controls how the fields of the command are sent."]
    #[inline(always)]
    pub fn fieldform(&mut self) -> FIELDFORM_W<19> {
        FIELDFORM_W::new(self)
    }
    #[doc = "Bits 21:23 - This field controls the opcode and address fields."]
    #[inline(always)]
    pub fn frameform(&mut self) -> FRAMEFORM_W<21> {
        FRAMEFORM_W::new(self)
    }
    #[doc = "Bits 24:31 - The opcode of the command (not used for some FRAMEFORM values)."]
    #[inline(always)]
    pub fn opcode(&mut self) -> OPCODE_W<24> {
        OPCODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPIFI memory command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcmd](index.html) module"]
pub struct MCMD_SPEC;
impl crate::RegisterSpec for MCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcmd::R](R) reader structure"]
impl crate::Readable for MCMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcmd::W](W) writer structure"]
impl crate::Writable for MCMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCMD to value 0"]
impl crate::Resettable for MCMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
