#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSTPENDINGCLR` writer - Master Pending interrupt clear. Writing 1 to this bit clears the corresponding bit in the INTENSET register if implemented."]
pub type MSTPENDINGCLR_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 0>;
#[doc = "Field `MSTARBLOSSCLR` writer - Master Arbitration Loss interrupt clear."]
pub type MSTARBLOSSCLR_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 4>;
#[doc = "Field `MSTSTSTPERRCLR` writer - Master Start/Stop Error interrupt clear."]
pub type MSTSTSTPERRCLR_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 6>;
#[doc = "Field `SLVPENDINGCLR` writer - Slave Pending interrupt clear."]
pub type SLVPENDINGCLR_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 8>;
#[doc = "Field `SLVNOTSTRCLR` writer - Slave Not Stretching interrupt clear."]
pub type SLVNOTSTRCLR_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 11>;
#[doc = "Field `SLVDESELCLR` writer - Slave Deselect interrupt clear."]
pub type SLVDESELCLR_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 15>;
#[doc = "Field `MONRDYCLR` writer - Monitor data Ready interrupt clear."]
pub type MONRDYCLR_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 16>;
#[doc = "Field `MONOVCLR` writer - Monitor Overrun interrupt clear."]
pub type MONOVCLR_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 17>;
#[doc = "Field `MONIDLECLR` writer - Monitor Idle interrupt clear."]
pub type MONIDLECLR_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 19>;
#[doc = "Field `EVENTTIMEOUTCLR` writer - Event time-out interrupt clear."]
pub type EVENTTIMEOUTCLR_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 24>;
#[doc = "Field `SCLTIMEOUTCLR` writer - SCL time-out interrupt clear."]
pub type SCLTIMEOUTCLR_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 25>;
impl W {
    #[doc = "Bit 0 - Master Pending interrupt clear. Writing 1 to this bit clears the corresponding bit in the INTENSET register if implemented."]
    #[inline(always)]
    pub fn mstpendingclr(&mut self) -> MSTPENDINGCLR_W {
        MSTPENDINGCLR_W::new(self)
    }
    #[doc = "Bit 4 - Master Arbitration Loss interrupt clear."]
    #[inline(always)]
    pub fn mstarblossclr(&mut self) -> MSTARBLOSSCLR_W {
        MSTARBLOSSCLR_W::new(self)
    }
    #[doc = "Bit 6 - Master Start/Stop Error interrupt clear."]
    #[inline(always)]
    pub fn mstststperrclr(&mut self) -> MSTSTSTPERRCLR_W {
        MSTSTSTPERRCLR_W::new(self)
    }
    #[doc = "Bit 8 - Slave Pending interrupt clear."]
    #[inline(always)]
    pub fn slvpendingclr(&mut self) -> SLVPENDINGCLR_W {
        SLVPENDINGCLR_W::new(self)
    }
    #[doc = "Bit 11 - Slave Not Stretching interrupt clear."]
    #[inline(always)]
    pub fn slvnotstrclr(&mut self) -> SLVNOTSTRCLR_W {
        SLVNOTSTRCLR_W::new(self)
    }
    #[doc = "Bit 15 - Slave Deselect interrupt clear."]
    #[inline(always)]
    pub fn slvdeselclr(&mut self) -> SLVDESELCLR_W {
        SLVDESELCLR_W::new(self)
    }
    #[doc = "Bit 16 - Monitor data Ready interrupt clear."]
    #[inline(always)]
    pub fn monrdyclr(&mut self) -> MONRDYCLR_W {
        MONRDYCLR_W::new(self)
    }
    #[doc = "Bit 17 - Monitor Overrun interrupt clear."]
    #[inline(always)]
    pub fn monovclr(&mut self) -> MONOVCLR_W {
        MONOVCLR_W::new(self)
    }
    #[doc = "Bit 19 - Monitor Idle interrupt clear."]
    #[inline(always)]
    pub fn monidleclr(&mut self) -> MONIDLECLR_W {
        MONIDLECLR_W::new(self)
    }
    #[doc = "Bit 24 - Event time-out interrupt clear."]
    #[inline(always)]
    pub fn eventtimeoutclr(&mut self) -> EVENTTIMEOUTCLR_W {
        EVENTTIMEOUTCLR_W::new(self)
    }
    #[doc = "Bit 25 - SCL time-out interrupt clear."]
    #[inline(always)]
    pub fn scltimeoutclr(&mut self) -> SCLTIMEOUTCLR_W {
        SCLTIMEOUTCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Clear register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
