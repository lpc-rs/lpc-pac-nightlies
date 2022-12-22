#[doc = "Register `STARTER1` reader"]
pub struct R(crate::R<STARTER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STARTER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STARTER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STARTER1` writer"]
pub struct W(crate::W<STARTER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTER1_SPEC>;
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
impl From<crate::W<STARTER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STARTER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PINT4` reader - GPIO pin interrupt 4 wake-up."]
pub type PINT4_R = crate::BitReader<bool>;
#[doc = "Field `PINT4` writer - GPIO pin interrupt 4 wake-up."]
pub type PINT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER1_SPEC, bool, O>;
#[doc = "Field `PINT5` reader - GPIO pin interrupt 5 wake-up."]
pub type PINT5_R = crate::BitReader<bool>;
#[doc = "Field `PINT5` writer - GPIO pin interrupt 5 wake-up."]
pub type PINT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER1_SPEC, bool, O>;
#[doc = "Field `PINT6` reader - GPIO pin interrupt 6 wake-up."]
pub type PINT6_R = crate::BitReader<bool>;
#[doc = "Field `PINT6` writer - GPIO pin interrupt 6 wake-up."]
pub type PINT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER1_SPEC, bool, O>;
#[doc = "Field `PINT7` reader - GPIO pin interrupt 7 wake-up."]
pub type PINT7_R = crate::BitReader<bool>;
#[doc = "Field `PINT7` writer - GPIO pin interrupt 7 wake-up."]
pub type PINT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER1_SPEC, bool, O>;
#[doc = "Field `CTIMER2` reader - Standard counter/timer CTIMER2 wake-up."]
pub type CTIMER2_R = crate::BitReader<bool>;
#[doc = "Field `CTIMER2` writer - Standard counter/timer CTIMER2 wake-up."]
pub type CTIMER2_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER1_SPEC, bool, O>;
#[doc = "Field `CTIMER4` reader - Standard counter/timer CTIMER4 wake-up."]
pub type CTIMER4_R = crate::BitReader<bool>;
#[doc = "Field `CTIMER4` writer - Standard counter/timer CTIMER4 wake-up."]
pub type CTIMER4_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER1_SPEC, bool, O>;
#[doc = "Field `SPIFI` reader - SPIFI interrupt wake-up. 0 = Wake-up disabled. 1 = Wake-up enabled."]
pub type SPIFI_R = crate::BitReader<bool>;
#[doc = "Field `SPIFI` writer - SPIFI interrupt wake-up. 0 = Wake-up disabled. 1 = Wake-up enabled."]
pub type SPIFI_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER1_SPEC, bool, O>;
#[doc = "Field `FLEXCOMM8` reader - Flexcomm Interface 8 wake-up."]
pub type FLEXCOMM8_R = crate::BitReader<bool>;
#[doc = "Field `FLEXCOMM8` writer - Flexcomm Interface 8 wake-up."]
pub type FLEXCOMM8_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER1_SPEC, bool, O>;
#[doc = "Field `FLEXCOMM9` reader - Flexcomm Interface 9 wake-up."]
pub type FLEXCOMM9_R = crate::BitReader<bool>;
#[doc = "Field `FLEXCOMM9` writer - Flexcomm Interface 9 wake-up."]
pub type FLEXCOMM9_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER1_SPEC, bool, O>;
#[doc = "Field `USB1` reader - USB 1 wake-up."]
pub type USB1_R = crate::BitReader<bool>;
#[doc = "Field `USB1` writer - USB 1 wake-up."]
pub type USB1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER1_SPEC, bool, O>;
#[doc = "Field `USB1_ACT` reader - USB 1 activity wake-up."]
pub type USB1_ACT_R = crate::BitReader<bool>;
#[doc = "Field `USB1_ACT` writer - USB 1 activity wake-up."]
pub type USB1_ACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER1_SPEC, bool, O>;
#[doc = "Field `ENET_INT1` reader - Ethernet."]
pub type ENET_INT1_R = crate::BitReader<bool>;
#[doc = "Field `ENET_INT1` writer - Ethernet."]
pub type ENET_INT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER1_SPEC, bool, O>;
#[doc = "Field `ENET_INT2` reader - Ethernet."]
pub type ENET_INT2_R = crate::BitReader<bool>;
#[doc = "Field `ENET_INT2` writer - Ethernet."]
pub type ENET_INT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER1_SPEC, bool, O>;
#[doc = "Field `ENET_INT0` reader - Ethernet."]
pub type ENET_INT0_R = crate::BitReader<bool>;
#[doc = "Field `ENET_INT0` writer - Ethernet."]
pub type ENET_INT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER1_SPEC, bool, O>;
#[doc = "Field `SMARTCARD0` reader - Smart card 0 wake-up."]
pub type SMARTCARD0_R = crate::BitReader<bool>;
#[doc = "Field `SMARTCARD0` writer - Smart card 0 wake-up."]
pub type SMARTCARD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER1_SPEC, bool, O>;
#[doc = "Field `SMARTCARD1` reader - Smart card 1 wake-up."]
pub type SMARTCARD1_R = crate::BitReader<bool>;
#[doc = "Field `SMARTCARD1` writer - Smart card 1 wake-up."]
pub type SMARTCARD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GPIO pin interrupt 4 wake-up."]
    #[inline(always)]
    pub fn pint4(&self) -> PINT4_R {
        PINT4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO pin interrupt 5 wake-up."]
    #[inline(always)]
    pub fn pint5(&self) -> PINT5_R {
        PINT5_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO pin interrupt 6 wake-up."]
    #[inline(always)]
    pub fn pint6(&self) -> PINT6_R {
        PINT6_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO pin interrupt 7 wake-up."]
    #[inline(always)]
    pub fn pint7(&self) -> PINT7_R {
        PINT7_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Standard counter/timer CTIMER2 wake-up."]
    #[inline(always)]
    pub fn ctimer2(&self) -> CTIMER2_R {
        CTIMER2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Standard counter/timer CTIMER4 wake-up."]
    #[inline(always)]
    pub fn ctimer4(&self) -> CTIMER4_R {
        CTIMER4_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - SPIFI interrupt wake-up. 0 = Wake-up disabled. 1 = Wake-up enabled."]
    #[inline(always)]
    pub fn spifi(&self) -> SPIFI_R {
        SPIFI_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Flexcomm Interface 8 wake-up."]
    #[inline(always)]
    pub fn flexcomm8(&self) -> FLEXCOMM8_R {
        FLEXCOMM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Flexcomm Interface 9 wake-up."]
    #[inline(always)]
    pub fn flexcomm9(&self) -> FLEXCOMM9_R {
        FLEXCOMM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - USB 1 wake-up."]
    #[inline(always)]
    pub fn usb1(&self) -> USB1_R {
        USB1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - USB 1 activity wake-up."]
    #[inline(always)]
    pub fn usb1_act(&self) -> USB1_ACT_R {
        USB1_ACT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Ethernet."]
    #[inline(always)]
    pub fn enet_int1(&self) -> ENET_INT1_R {
        ENET_INT1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Ethernet."]
    #[inline(always)]
    pub fn enet_int2(&self) -> ENET_INT2_R {
        ENET_INT2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Ethernet."]
    #[inline(always)]
    pub fn enet_int0(&self) -> ENET_INT0_R {
        ENET_INT0_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - Smart card 0 wake-up."]
    #[inline(always)]
    pub fn smartcard0(&self) -> SMARTCARD0_R {
        SMARTCARD0_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Smart card 1 wake-up."]
    #[inline(always)]
    pub fn smartcard1(&self) -> SMARTCARD1_R {
        SMARTCARD1_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO pin interrupt 4 wake-up."]
    #[inline(always)]
    pub fn pint4(&mut self) -> PINT4_W<0> {
        PINT4_W::new(self)
    }
    #[doc = "Bit 1 - GPIO pin interrupt 5 wake-up."]
    #[inline(always)]
    pub fn pint5(&mut self) -> PINT5_W<1> {
        PINT5_W::new(self)
    }
    #[doc = "Bit 2 - GPIO pin interrupt 6 wake-up."]
    #[inline(always)]
    pub fn pint6(&mut self) -> PINT6_W<2> {
        PINT6_W::new(self)
    }
    #[doc = "Bit 3 - GPIO pin interrupt 7 wake-up."]
    #[inline(always)]
    pub fn pint7(&mut self) -> PINT7_W<3> {
        PINT7_W::new(self)
    }
    #[doc = "Bit 4 - Standard counter/timer CTIMER2 wake-up."]
    #[inline(always)]
    pub fn ctimer2(&mut self) -> CTIMER2_W<4> {
        CTIMER2_W::new(self)
    }
    #[doc = "Bit 5 - Standard counter/timer CTIMER4 wake-up."]
    #[inline(always)]
    pub fn ctimer4(&mut self) -> CTIMER4_W<5> {
        CTIMER4_W::new(self)
    }
    #[doc = "Bit 7 - SPIFI interrupt wake-up. 0 = Wake-up disabled. 1 = Wake-up enabled."]
    #[inline(always)]
    pub fn spifi(&mut self) -> SPIFI_W<7> {
        SPIFI_W::new(self)
    }
    #[doc = "Bit 8 - Flexcomm Interface 8 wake-up."]
    #[inline(always)]
    pub fn flexcomm8(&mut self) -> FLEXCOMM8_W<8> {
        FLEXCOMM8_W::new(self)
    }
    #[doc = "Bit 9 - Flexcomm Interface 9 wake-up."]
    #[inline(always)]
    pub fn flexcomm9(&mut self) -> FLEXCOMM9_W<9> {
        FLEXCOMM9_W::new(self)
    }
    #[doc = "Bit 15 - USB 1 wake-up."]
    #[inline(always)]
    pub fn usb1(&mut self) -> USB1_W<15> {
        USB1_W::new(self)
    }
    #[doc = "Bit 16 - USB 1 activity wake-up."]
    #[inline(always)]
    pub fn usb1_act(&mut self) -> USB1_ACT_W<16> {
        USB1_ACT_W::new(self)
    }
    #[doc = "Bit 17 - Ethernet."]
    #[inline(always)]
    pub fn enet_int1(&mut self) -> ENET_INT1_W<17> {
        ENET_INT1_W::new(self)
    }
    #[doc = "Bit 18 - Ethernet."]
    #[inline(always)]
    pub fn enet_int2(&mut self) -> ENET_INT2_W<18> {
        ENET_INT2_W::new(self)
    }
    #[doc = "Bit 19 - Ethernet."]
    #[inline(always)]
    pub fn enet_int0(&mut self) -> ENET_INT0_W<19> {
        ENET_INT0_W::new(self)
    }
    #[doc = "Bit 23 - Smart card 0 wake-up."]
    #[inline(always)]
    pub fn smartcard0(&mut self) -> SMARTCARD0_W<23> {
        SMARTCARD0_W::new(self)
    }
    #[doc = "Bit 24 - Smart card 1 wake-up."]
    #[inline(always)]
    pub fn smartcard1(&mut self) -> SMARTCARD1_W<24> {
        SMARTCARD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start logic 0 wake-up enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [starter1](index.html) module"]
pub struct STARTER1_SPEC;
impl crate::RegisterSpec for STARTER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [starter1::R](R) reader structure"]
impl crate::Readable for STARTER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [starter1::W](W) writer structure"]
impl crate::Writable for STARTER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STARTER1 to value 0"]
impl crate::Resettable for STARTER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
