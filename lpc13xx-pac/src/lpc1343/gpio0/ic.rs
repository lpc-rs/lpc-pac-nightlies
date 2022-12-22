#[doc = "Register `IC` writer"]
pub struct W(crate::W<IC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC_SPEC>;
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
impl From<crate::W<IC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR0` writer - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
pub type CLR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IC_SPEC, bool, O>;
#[doc = "Field `CLR1` writer - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
pub type CLR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IC_SPEC, bool, O>;
#[doc = "Field `CLR2` writer - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
pub type CLR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IC_SPEC, bool, O>;
#[doc = "Field `CLR3` writer - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
pub type CLR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IC_SPEC, bool, O>;
#[doc = "Field `CLR4` writer - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
pub type CLR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IC_SPEC, bool, O>;
#[doc = "Field `CLR5` writer - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
pub type CLR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IC_SPEC, bool, O>;
#[doc = "Field `CLR6` writer - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
pub type CLR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IC_SPEC, bool, O>;
#[doc = "Field `CLR7` writer - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
pub type CLR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IC_SPEC, bool, O>;
#[doc = "Field `CLR8` writer - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
pub type CLR8_W<'a, const O: u8> = crate::BitWriter<'a, u32, IC_SPEC, bool, O>;
#[doc = "Field `CLR9` writer - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
pub type CLR9_W<'a, const O: u8> = crate::BitWriter<'a, u32, IC_SPEC, bool, O>;
#[doc = "Field `CLR10` writer - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
pub type CLR10_W<'a, const O: u8> = crate::BitWriter<'a, u32, IC_SPEC, bool, O>;
#[doc = "Field `CLR11` writer - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
pub type CLR11_W<'a, const O: u8> = crate::BitWriter<'a, u32, IC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
    #[inline(always)]
    pub fn clr0(&mut self) -> CLR0_W<0> {
        CLR0_W::new(self)
    }
    #[doc = "Bit 1 - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
    #[inline(always)]
    pub fn clr1(&mut self) -> CLR1_W<1> {
        CLR1_W::new(self)
    }
    #[doc = "Bit 2 - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
    #[inline(always)]
    pub fn clr2(&mut self) -> CLR2_W<2> {
        CLR2_W::new(self)
    }
    #[doc = "Bit 3 - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
    #[inline(always)]
    pub fn clr3(&mut self) -> CLR3_W<3> {
        CLR3_W::new(self)
    }
    #[doc = "Bit 4 - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
    #[inline(always)]
    pub fn clr4(&mut self) -> CLR4_W<4> {
        CLR4_W::new(self)
    }
    #[doc = "Bit 5 - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
    #[inline(always)]
    pub fn clr5(&mut self) -> CLR5_W<5> {
        CLR5_W::new(self)
    }
    #[doc = "Bit 6 - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
    #[inline(always)]
    pub fn clr6(&mut self) -> CLR6_W<6> {
        CLR6_W::new(self)
    }
    #[doc = "Bit 7 - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
    #[inline(always)]
    pub fn clr7(&mut self) -> CLR7_W<7> {
        CLR7_W::new(self)
    }
    #[doc = "Bit 8 - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
    #[inline(always)]
    pub fn clr8(&mut self) -> CLR8_W<8> {
        CLR8_W::new(self)
    }
    #[doc = "Bit 9 - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
    #[inline(always)]
    pub fn clr9(&mut self) -> CLR9_W<9> {
        CLR9_W::new(self)
    }
    #[doc = "Bit 10 - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
    #[inline(always)]
    pub fn clr10(&mut self) -> CLR10_W<10> {
        CLR10_W::new(self)
    }
    #[doc = "Bit 11 - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
    #[inline(always)]
    pub fn clr11(&mut self) -> CLR11_W<11> {
        CLR11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear register for port n\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic](index.html) module"]
pub struct IC_SPEC;
impl crate::RegisterSpec for IC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ic::W](W) writer structure"]
impl crate::Writable for IC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IC to value 0"]
impl crate::Resettable for IC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
