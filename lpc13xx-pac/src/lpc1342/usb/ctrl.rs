#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RD_EN` reader - Read mode control. Enables reading data from the OUT endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBRxData register. This bit is cleared by hardware when the last word of the current packet is read from USBRxData."]
pub type RD_EN_R = crate::BitReader<RD_EN_A>;
#[doc = "Read mode control. Enables reading data from the OUT endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBRxData register. This bit is cleared by hardware when the last word of the current packet is read from USBRxData.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RD_EN_A {
    #[doc = "0: Read mode is disabled."]
    READ_MODE_IS_DISABLE = 0,
    #[doc = "1: Read mode is enabled."]
    READ_MODE_IS_ENABLED = 1,
}
impl From<RD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RD_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RD_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RD_EN_A {
        match self.bits {
            false => RD_EN_A::READ_MODE_IS_DISABLE,
            true => RD_EN_A::READ_MODE_IS_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `READ_MODE_IS_DISABLE`"]
    #[inline(always)]
    pub fn is_read_mode_is_disable(&self) -> bool {
        *self == RD_EN_A::READ_MODE_IS_DISABLE
    }
    #[doc = "Checks if the value of the field is `READ_MODE_IS_ENABLED`"]
    #[inline(always)]
    pub fn is_read_mode_is_enabled(&self) -> bool {
        *self == RD_EN_A::READ_MODE_IS_ENABLED
    }
}
#[doc = "Field `RD_EN` writer - Read mode control. Enables reading data from the OUT endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBRxData register. This bit is cleared by hardware when the last word of the current packet is read from USBRxData."]
pub type RD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, RD_EN_A, O>;
impl<'a, const O: u8> RD_EN_W<'a, O> {
    #[doc = "Read mode is disabled."]
    #[inline(always)]
    pub fn read_mode_is_disable(self) -> &'a mut W {
        self.variant(RD_EN_A::READ_MODE_IS_DISABLE)
    }
    #[doc = "Read mode is enabled."]
    #[inline(always)]
    pub fn read_mode_is_enabled(self) -> &'a mut W {
        self.variant(RD_EN_A::READ_MODE_IS_ENABLED)
    }
}
#[doc = "Field `WR_EN` reader - Write mode control. Enables writing data to the IN endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBTxData register. This bit is cleared by hardware when the number of bytes in USBTxLen have been sent."]
pub type WR_EN_R = crate::BitReader<WR_EN_A>;
#[doc = "Write mode control. Enables writing data to the IN endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBTxData register. This bit is cleared by hardware when the number of bytes in USBTxLen have been sent.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WR_EN_A {
    #[doc = "0: Write mode is disabled."]
    WRITE_MODE_IS_DISABL = 0,
    #[doc = "1: Write mode is enabled."]
    WRITE_MODE_IS_ENABLE = 1,
}
impl From<WR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WR_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl WR_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WR_EN_A {
        match self.bits {
            false => WR_EN_A::WRITE_MODE_IS_DISABL,
            true => WR_EN_A::WRITE_MODE_IS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_MODE_IS_DISABL`"]
    #[inline(always)]
    pub fn is_write_mode_is_disabl(&self) -> bool {
        *self == WR_EN_A::WRITE_MODE_IS_DISABL
    }
    #[doc = "Checks if the value of the field is `WRITE_MODE_IS_ENABLE`"]
    #[inline(always)]
    pub fn is_write_mode_is_enable(&self) -> bool {
        *self == WR_EN_A::WRITE_MODE_IS_ENABLE
    }
}
#[doc = "Field `WR_EN` writer - Write mode control. Enables writing data to the IN endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBTxData register. This bit is cleared by hardware when the number of bytes in USBTxLen have been sent."]
pub type WR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, WR_EN_A, O>;
impl<'a, const O: u8> WR_EN_W<'a, O> {
    #[doc = "Write mode is disabled."]
    #[inline(always)]
    pub fn write_mode_is_disabl(self) -> &'a mut W {
        self.variant(WR_EN_A::WRITE_MODE_IS_DISABL)
    }
    #[doc = "Write mode is enabled."]
    #[inline(always)]
    pub fn write_mode_is_enable(self) -> &'a mut W {
        self.variant(WR_EN_A::WRITE_MODE_IS_ENABLE)
    }
}
#[doc = "Field `LOG_ENDPOINT` reader - Logical Endpoint number."]
pub type LOG_ENDPOINT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOG_ENDPOINT` writer - Logical Endpoint number."]
pub type LOG_ENDPOINT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Read mode control. Enables reading data from the OUT endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBRxData register. This bit is cleared by hardware when the last word of the current packet is read from USBRxData."]
    #[inline(always)]
    pub fn rd_en(&self) -> RD_EN_R {
        RD_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write mode control. Enables writing data to the IN endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBTxData register. This bit is cleared by hardware when the number of bytes in USBTxLen have been sent."]
    #[inline(always)]
    pub fn wr_en(&self) -> WR_EN_R {
        WR_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - Logical Endpoint number."]
    #[inline(always)]
    pub fn log_endpoint(&self) -> LOG_ENDPOINT_R {
        LOG_ENDPOINT_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Read mode control. Enables reading data from the OUT endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBRxData register. This bit is cleared by hardware when the last word of the current packet is read from USBRxData."]
    #[inline(always)]
    pub fn rd_en(&mut self) -> RD_EN_W<0> {
        RD_EN_W::new(self)
    }
    #[doc = "Bit 1 - Write mode control. Enables writing data to the IN endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBTxData register. This bit is cleared by hardware when the number of bytes in USBTxLen have been sent."]
    #[inline(always)]
    pub fn wr_en(&mut self) -> WR_EN_W<1> {
        WR_EN_W::new(self)
    }
    #[doc = "Bits 2:5 - Logical Endpoint number."]
    #[inline(always)]
    pub fn log_endpoint(&mut self) -> LOG_ENDPOINT_W<2> {
        LOG_ENDPOINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
