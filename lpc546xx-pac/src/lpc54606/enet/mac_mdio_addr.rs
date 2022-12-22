#[doc = "Register `MAC_MDIO_ADDR` reader"]
pub struct R(crate::R<MAC_MDIO_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_MDIO_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_MDIO_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_MDIO_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_MDIO_ADDR` writer"]
pub struct W(crate::W<MAC_MDIO_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_MDIO_ADDR_SPEC>;
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
impl From<crate::W<MAC_MDIO_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_MDIO_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MB` reader - MII busy."]
pub type MB_R = crate::BitReader<bool>;
#[doc = "Field `MB` writer - MII busy."]
pub type MB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_MDIO_ADDR_SPEC, bool, O>;
#[doc = "Field `MOC` reader - MII Operation Command."]
pub type MOC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MOC` writer - MII Operation Command."]
pub type MOC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_MDIO_ADDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CR` reader - CSR Clock Range."]
pub type CR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CR` writer - CSR Clock Range."]
pub type CR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_MDIO_ADDR_SPEC, u8, u8, 4, O>;
#[doc = "Field `NTC` reader - Number of Training Clocks This field controls the number of trailing clock cycles generated on MDC after the end of transmission of MDIO frame."]
pub type NTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NTC` writer - Number of Training Clocks This field controls the number of trailing clock cycles generated on MDC after the end of transmission of MDIO frame."]
pub type NTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_MDIO_ADDR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RDA` reader - Register/Device Address These bits select the PHY register in selected PHY device."]
pub type RDA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDA` writer - Register/Device Address These bits select the PHY register in selected PHY device."]
pub type RDA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_MDIO_ADDR_SPEC, u8, u8, 5, O>;
#[doc = "Field `PA` reader - Physical Layer Address This field indicates which PHY devices (out of 32 devices) the MAC is accessing."]
pub type PA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PA` writer - Physical Layer Address This field indicates which PHY devices (out of 32 devices) the MAC is accessing."]
pub type PA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_MDIO_ADDR_SPEC, u8, u8, 5, O>;
#[doc = "Field `BTB` reader - Back to Back transactions When this bit is set and the NTC has value greater than 0, then the MAC will inform the completion of a read or write command at the end of frame transfer (before the trailing clocks are transmitted)."]
pub type BTB_R = crate::BitReader<bool>;
#[doc = "Field `BTB` writer - Back to Back transactions When this bit is set and the NTC has value greater than 0, then the MAC will inform the completion of a read or write command at the end of frame transfer (before the trailing clocks are transmitted)."]
pub type BTB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_MDIO_ADDR_SPEC, bool, O>;
#[doc = "Field `PSE` reader - Preamble Suppression Enable When this bit is set, the SMA will suppress the 32-bit preamble and transmit MDIO frames with only 1 preamble bit."]
pub type PSE_R = crate::BitReader<bool>;
#[doc = "Field `PSE` writer - Preamble Suppression Enable When this bit is set, the SMA will suppress the 32-bit preamble and transmit MDIO frames with only 1 preamble bit."]
pub type PSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_MDIO_ADDR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - MII busy."]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - MII Operation Command."]
    #[inline(always)]
    pub fn moc(&self) -> MOC_R {
        MOC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:11 - CSR Clock Range."]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Number of Training Clocks This field controls the number of trailing clock cycles generated on MDC after the end of transmission of MDIO frame."]
    #[inline(always)]
    pub fn ntc(&self) -> NTC_R {
        NTC_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:20 - Register/Device Address These bits select the PHY register in selected PHY device."]
    #[inline(always)]
    pub fn rda(&self) -> RDA_R {
        RDA_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Physical Layer Address This field indicates which PHY devices (out of 32 devices) the MAC is accessing."]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - Back to Back transactions When this bit is set and the NTC has value greater than 0, then the MAC will inform the completion of a read or write command at the end of frame transfer (before the trailing clocks are transmitted)."]
    #[inline(always)]
    pub fn btb(&self) -> BTB_R {
        BTB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Preamble Suppression Enable When this bit is set, the SMA will suppress the 32-bit preamble and transmit MDIO frames with only 1 preamble bit."]
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MII busy."]
    #[inline(always)]
    pub fn mb(&mut self) -> MB_W<0> {
        MB_W::new(self)
    }
    #[doc = "Bits 2:3 - MII Operation Command."]
    #[inline(always)]
    pub fn moc(&mut self) -> MOC_W<2> {
        MOC_W::new(self)
    }
    #[doc = "Bits 8:11 - CSR Clock Range."]
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W<8> {
        CR_W::new(self)
    }
    #[doc = "Bits 12:14 - Number of Training Clocks This field controls the number of trailing clock cycles generated on MDC after the end of transmission of MDIO frame."]
    #[inline(always)]
    pub fn ntc(&mut self) -> NTC_W<12> {
        NTC_W::new(self)
    }
    #[doc = "Bits 16:20 - Register/Device Address These bits select the PHY register in selected PHY device."]
    #[inline(always)]
    pub fn rda(&mut self) -> RDA_W<16> {
        RDA_W::new(self)
    }
    #[doc = "Bits 21:25 - Physical Layer Address This field indicates which PHY devices (out of 32 devices) the MAC is accessing."]
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W<21> {
        PA_W::new(self)
    }
    #[doc = "Bit 26 - Back to Back transactions When this bit is set and the NTC has value greater than 0, then the MAC will inform the completion of a read or write command at the end of frame transfer (before the trailing clocks are transmitted)."]
    #[inline(always)]
    pub fn btb(&mut self) -> BTB_W<26> {
        BTB_W::new(self)
    }
    #[doc = "Bit 27 - Preamble Suppression Enable When this bit is set, the SMA will suppress the 32-bit preamble and transmit MDIO frames with only 1 preamble bit."]
    #[inline(always)]
    pub fn pse(&mut self) -> PSE_W<27> {
        PSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIDO address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_mdio_addr](index.html) module"]
pub struct MAC_MDIO_ADDR_SPEC;
impl crate::RegisterSpec for MAC_MDIO_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_mdio_addr::R](R) reader structure"]
impl crate::Readable for MAC_MDIO_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_mdio_addr::W](W) writer structure"]
impl crate::Writable for MAC_MDIO_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_MDIO_ADDR to value 0"]
impl crate::Resettable for MAC_MDIO_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
