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
#[doc = "Field `TIMEOUT` reader - This field contains the number of serial clock periods without the processor reading data in memory mode, which will cause the SPIFI hardware to terminate the command by driving the CS pin high and negating the CMD bit in the Status register. (This allows the flash memory to enter a lower-power state.) If the processor reads data from the flash region after a time-out, the command in the Memory Command Register is issued again."]
pub type TIMEOUT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIMEOUT` writer - This field contains the number of serial clock periods without the processor reading data in memory mode, which will cause the SPIFI hardware to terminate the command by driving the CS pin high and negating the CMD bit in the Status register. (This allows the flash memory to enter a lower-power state.) If the processor reads data from the flash region after a time-out, the command in the Memory Command Register is issued again."]
pub type TIMEOUT_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u16, u16, 16, 0>;
#[doc = "Field `CSHIGH` reader - This field controls the minimum CS high time, expressed as a number of serial clock periods minus one."]
pub type CSHIGH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSHIGH` writer - This field controls the minimum CS high time, expressed as a number of serial clock periods minus one."]
pub type CSHIGH_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, 16>;
#[doc = "Field `D_PRFTCH_DIS` reader - This bit allows conditioning of memory mode prefetches based on the AHB HPROT (instruction/data) access information. A 1 in this register means that the SPIFI will not attempt a speculative prefetch when it encounters data accesses."]
pub type D_PRFTCH_DIS_R = crate::BitReader<bool>;
#[doc = "Field `D_PRFTCH_DIS` writer - This bit allows conditioning of memory mode prefetches based on the AHB HPROT (instruction/data) access information. A 1 in this register means that the SPIFI will not attempt a speculative prefetch when it encounters data accesses."]
pub type D_PRFTCH_DIS_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 21>;
#[doc = "Field `INTEN` reader - If this bit is 1 when a command ends, the SPIFI will assert its interrupt request output. See INTRQ in the status register for further details."]
pub type INTEN_R = crate::BitReader<bool>;
#[doc = "Field `INTEN` writer - If this bit is 1 when a command ends, the SPIFI will assert its interrupt request output. See INTRQ in the status register for further details."]
pub type INTEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 22>;
#[doc = "SPI Mode 3 select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE3_A {
    #[doc = "0: SCK LOW. The SPIFI drives SCK low after the rising edge at which the last bit of each command is captured, and keeps it low while CS is HIGH."]
    SCK_LOW = 0,
    #[doc = "1: SCK HIGH. the SPIFI keeps SCK high after the rising edge for the last bit of each command and while CS is HIGH, and drives it low after it drives CS LOW. (Known serial flash devices can handle either mode, but some devices may require a particular mode for proper operation.) MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final falling edge on SCK on which to sample the last data bit of the frame."]
    SCK_HIGH = 1,
}
impl From<MODE3_A> for bool {
    #[inline(always)]
    fn from(variant: MODE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE3` reader - SPI Mode 3 select."]
pub type MODE3_R = crate::BitReader<MODE3_A>;
impl MODE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE3_A {
        match self.bits {
            false => MODE3_A::SCK_LOW,
            true => MODE3_A::SCK_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `SCK_LOW`"]
    #[inline(always)]
    pub fn is_sck_low(&self) -> bool {
        *self == MODE3_A::SCK_LOW
    }
    #[doc = "Checks if the value of the field is `SCK_HIGH`"]
    #[inline(always)]
    pub fn is_sck_high(&self) -> bool {
        *self == MODE3_A::SCK_HIGH
    }
}
#[doc = "Field `MODE3` writer - SPI Mode 3 select."]
pub type MODE3_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, MODE3_A, 23>;
impl<'a> MODE3_W<'a> {
    #[doc = "SCK LOW. The SPIFI drives SCK low after the rising edge at which the last bit of each command is captured, and keeps it low while CS is HIGH."]
    #[inline(always)]
    pub fn sck_low(self) -> &'a mut W {
        self.variant(MODE3_A::SCK_LOW)
    }
    #[doc = "SCK HIGH. the SPIFI keeps SCK high after the rising edge for the last bit of each command and while CS is HIGH, and drives it low after it drives CS LOW. (Known serial flash devices can handle either mode, but some devices may require a particular mode for proper operation.) MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final falling edge on SCK on which to sample the last data bit of the frame."]
    #[inline(always)]
    pub fn sck_high(self) -> &'a mut W {
        self.variant(MODE3_A::SCK_HIGH)
    }
}
#[doc = "Cache prefetching enable. The SPIFI includes an internal cache. A 1 in this bit disables prefetching of cache lines.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRFTCH_DIS_A {
    #[doc = "0: Enable. Cache prefetching enabled."]
    ENABLE = 0,
    #[doc = "1: Disable. Disables prefetching of cache lines."]
    DISABLE = 1,
}
impl From<PRFTCH_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: PRFTCH_DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRFTCH_DIS` reader - Cache prefetching enable. The SPIFI includes an internal cache. A 1 in this bit disables prefetching of cache lines."]
pub type PRFTCH_DIS_R = crate::BitReader<PRFTCH_DIS_A>;
impl PRFTCH_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRFTCH_DIS_A {
        match self.bits {
            false => PRFTCH_DIS_A::ENABLE,
            true => PRFTCH_DIS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PRFTCH_DIS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PRFTCH_DIS_A::DISABLE
    }
}
#[doc = "Field `PRFTCH_DIS` writer - Cache prefetching enable. The SPIFI includes an internal cache. A 1 in this bit disables prefetching of cache lines."]
pub type PRFTCH_DIS_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, PRFTCH_DIS_A, 27>;
impl<'a> PRFTCH_DIS_W<'a> {
    #[doc = "Enable. Cache prefetching enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PRFTCH_DIS_A::ENABLE)
    }
    #[doc = "Disable. Disables prefetching of cache lines."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PRFTCH_DIS_A::DISABLE)
    }
}
#[doc = "Select dual protocol.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DUAL_A {
    #[doc = "0: Quad protocol. This protocol uses IO3:0."]
    QUAD = 0,
    #[doc = "1: Dual protocol. This protocol uses IO1:0."]
    DUAL = 1,
}
impl From<DUAL_A> for bool {
    #[inline(always)]
    fn from(variant: DUAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DUAL` reader - Select dual protocol."]
pub type DUAL_R = crate::BitReader<DUAL_A>;
impl DUAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DUAL_A {
        match self.bits {
            false => DUAL_A::QUAD,
            true => DUAL_A::DUAL,
        }
    }
    #[doc = "Checks if the value of the field is `QUAD`"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == DUAL_A::QUAD
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == DUAL_A::DUAL
    }
}
#[doc = "Field `DUAL` writer - Select dual protocol."]
pub type DUAL_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, DUAL_A, 28>;
impl<'a> DUAL_W<'a> {
    #[doc = "Quad protocol. This protocol uses IO3:0."]
    #[inline(always)]
    pub fn quad(self) -> &'a mut W {
        self.variant(DUAL_A::QUAD)
    }
    #[doc = "Dual protocol. This protocol uses IO1:0."]
    #[inline(always)]
    pub fn dual(self) -> &'a mut W {
        self.variant(DUAL_A::DUAL)
    }
}
#[doc = "Select active clock edge for input data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFCLK_A {
    #[doc = "0: Rising edge. Read data is sampled on rising edges on the clock, as in classic SPI operation."]
    RISING_EDGE = 0,
    #[doc = "1: Falling edge. Read data is sampled on falling edges of the clock, allowing a full serial clock of of time in order to maximize the serial clock frequency. MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final falling edge on SCK on which to sample the last data bit of the frame."]
    FALLING_EDGE = 1,
}
impl From<RFCLK_A> for bool {
    #[inline(always)]
    fn from(variant: RFCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFCLK` reader - Select active clock edge for input data."]
pub type RFCLK_R = crate::BitReader<RFCLK_A>;
impl RFCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFCLK_A {
        match self.bits {
            false => RFCLK_A::RISING_EDGE,
            true => RFCLK_A::FALLING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == RFCLK_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == RFCLK_A::FALLING_EDGE
    }
}
#[doc = "Field `RFCLK` writer - Select active clock edge for input data."]
pub type RFCLK_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, RFCLK_A, 29>;
impl<'a> RFCLK_W<'a> {
    #[doc = "Rising edge. Read data is sampled on rising edges on the clock, as in classic SPI operation."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(RFCLK_A::RISING_EDGE)
    }
    #[doc = "Falling edge. Read data is sampled on falling edges of the clock, allowing a full serial clock of of time in order to maximize the serial clock frequency. MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final falling edge on SCK on which to sample the last data bit of the frame."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(RFCLK_A::FALLING_EDGE)
    }
}
#[doc = "Feedback clock select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FBCLK_A {
    #[doc = "0: Internal clock. The SPIFI samples read data using an internal clock."]
    INTERNAL_CLOCK = 0,
    #[doc = "1: Feedback clock. Read data is sampled using a feedback clock from the SCK pin. This allows slightly more time for each received bit. MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final falling edge on SCK on which to sample the last data bit of the frame."]
    FEEDBACK_CLOCK = 1,
}
impl From<FBCLK_A> for bool {
    #[inline(always)]
    fn from(variant: FBCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FBCLK` reader - Feedback clock select."]
pub type FBCLK_R = crate::BitReader<FBCLK_A>;
impl FBCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FBCLK_A {
        match self.bits {
            false => FBCLK_A::INTERNAL_CLOCK,
            true => FBCLK_A::FEEDBACK_CLOCK,
        }
    }
    #[doc = "Checks if the value of the field is `INTERNAL_CLOCK`"]
    #[inline(always)]
    pub fn is_internal_clock(&self) -> bool {
        *self == FBCLK_A::INTERNAL_CLOCK
    }
    #[doc = "Checks if the value of the field is `FEEDBACK_CLOCK`"]
    #[inline(always)]
    pub fn is_feedback_clock(&self) -> bool {
        *self == FBCLK_A::FEEDBACK_CLOCK
    }
}
#[doc = "Field `FBCLK` writer - Feedback clock select."]
pub type FBCLK_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, FBCLK_A, 30>;
impl<'a> FBCLK_W<'a> {
    #[doc = "Internal clock. The SPIFI samples read data using an internal clock."]
    #[inline(always)]
    pub fn internal_clock(self) -> &'a mut W {
        self.variant(FBCLK_A::INTERNAL_CLOCK)
    }
    #[doc = "Feedback clock. Read data is sampled using a feedback clock from the SCK pin. This allows slightly more time for each received bit. MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final falling edge on SCK on which to sample the last data bit of the frame."]
    #[inline(always)]
    pub fn feedback_clock(self) -> &'a mut W {
        self.variant(FBCLK_A::FEEDBACK_CLOCK)
    }
}
#[doc = "Field `DMAEN` reader - A 1 in this bit enables the DMA Request output from the SPIFI. Set this bit only when a DMA channel is used to transfer data in peripheral mode. Do not set this bit when a DMA channel is used for memory-to-memory transfers from the SPIFI memory area. DMAEN should only be used in Command mode."]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - A 1 in this bit enables the DMA Request output from the SPIFI. Set this bit only when a DMA channel is used to transfer data in peripheral mode. Do not set this bit when a DMA channel is used for memory-to-memory transfers from the SPIFI memory area. DMAEN should only be used in Command mode."]
pub type DMAEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:15 - This field contains the number of serial clock periods without the processor reading data in memory mode, which will cause the SPIFI hardware to terminate the command by driving the CS pin high and negating the CMD bit in the Status register. (This allows the flash memory to enter a lower-power state.) If the processor reads data from the flash region after a time-out, the command in the Memory Command Register is issued again."]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - This field controls the minimum CS high time, expressed as a number of serial clock periods minus one."]
    #[inline(always)]
    pub fn cshigh(&self) -> CSHIGH_R {
        CSHIGH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - This bit allows conditioning of memory mode prefetches based on the AHB HPROT (instruction/data) access information. A 1 in this register means that the SPIFI will not attempt a speculative prefetch when it encounters data accesses."]
    #[inline(always)]
    pub fn d_prftch_dis(&self) -> D_PRFTCH_DIS_R {
        D_PRFTCH_DIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - If this bit is 1 when a command ends, the SPIFI will assert its interrupt request output. See INTRQ in the status register for further details."]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SPI Mode 3 select."]
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 27 - Cache prefetching enable. The SPIFI includes an internal cache. A 1 in this bit disables prefetching of cache lines."]
    #[inline(always)]
    pub fn prftch_dis(&self) -> PRFTCH_DIS_R {
        PRFTCH_DIS_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Select dual protocol."]
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Select active clock edge for input data."]
    #[inline(always)]
    pub fn rfclk(&self) -> RFCLK_R {
        RFCLK_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Feedback clock select."]
    #[inline(always)]
    pub fn fbclk(&self) -> FBCLK_R {
        FBCLK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - A 1 in this bit enables the DMA Request output from the SPIFI. Set this bit only when a DMA channel is used to transfer data in peripheral mode. Do not set this bit when a DMA channel is used for memory-to-memory transfers from the SPIFI memory area. DMAEN should only be used in Command mode."]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field contains the number of serial clock periods without the processor reading data in memory mode, which will cause the SPIFI hardware to terminate the command by driving the CS pin high and negating the CMD bit in the Status register. (This allows the flash memory to enter a lower-power state.) If the processor reads data from the flash region after a time-out, the command in the Memory Command Register is issued again."]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bits 16:19 - This field controls the minimum CS high time, expressed as a number of serial clock periods minus one."]
    #[inline(always)]
    pub fn cshigh(&mut self) -> CSHIGH_W {
        CSHIGH_W::new(self)
    }
    #[doc = "Bit 21 - This bit allows conditioning of memory mode prefetches based on the AHB HPROT (instruction/data) access information. A 1 in this register means that the SPIFI will not attempt a speculative prefetch when it encounters data accesses."]
    #[inline(always)]
    pub fn d_prftch_dis(&mut self) -> D_PRFTCH_DIS_W {
        D_PRFTCH_DIS_W::new(self)
    }
    #[doc = "Bit 22 - If this bit is 1 when a command ends, the SPIFI will assert its interrupt request output. See INTRQ in the status register for further details."]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W {
        INTEN_W::new(self)
    }
    #[doc = "Bit 23 - SPI Mode 3 select."]
    #[inline(always)]
    pub fn mode3(&mut self) -> MODE3_W {
        MODE3_W::new(self)
    }
    #[doc = "Bit 27 - Cache prefetching enable. The SPIFI includes an internal cache. A 1 in this bit disables prefetching of cache lines."]
    #[inline(always)]
    pub fn prftch_dis(&mut self) -> PRFTCH_DIS_W {
        PRFTCH_DIS_W::new(self)
    }
    #[doc = "Bit 28 - Select dual protocol."]
    #[inline(always)]
    pub fn dual(&mut self) -> DUAL_W {
        DUAL_W::new(self)
    }
    #[doc = "Bit 29 - Select active clock edge for input data."]
    #[inline(always)]
    pub fn rfclk(&mut self) -> RFCLK_W {
        RFCLK_W::new(self)
    }
    #[doc = "Bit 30 - Feedback clock select."]
    #[inline(always)]
    pub fn fbclk(&mut self) -> FBCLK_W {
        FBCLK_W::new(self)
    }
    #[doc = "Bit 31 - A 1 in this bit enables the DMA Request output from the SPIFI. Set this bit only when a DMA channel is used to transfer data in peripheral mode. Do not set this bit when a DMA channel is used for memory-to-memory transfers from the SPIFI memory area. DMAEN should only be used in Command mode."]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPIFI control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
#[doc = "`reset()` method sets CTRL to value 0x400f_ffff"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x400f_ffff
    }
}
