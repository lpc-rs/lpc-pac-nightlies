#[doc = "Register `POSITION` reader"]
pub struct R(crate::R<POSITION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POSITION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POSITION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POSITION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POSITION` writer"]
pub struct W(crate::W<POSITION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POSITION_SPEC>;
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
impl From<crate::W<POSITION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POSITION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRAP` reader - This bit is set to 1 automatically when the POINTER value wraps as determined by the MASTER.MASK field in the MASTER Trace Control Register."]
pub type WRAP_R = crate::BitReader<bool>;
#[doc = "Field `WRAP` writer - This bit is set to 1 automatically when the POINTER value wraps as determined by the MASTER.MASK field in the MASTER Trace Control Register."]
pub type WRAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, POSITION_SPEC, bool, O>;
#[doc = "Field `POINTER` reader - Trace packet location pointer. Because a packet consists of two words, the POINTER field is the location of the first word of a packet. This field contains bits \\[31:3\\]
of the address, in the SRAM, where the next trace packet will be written. The field points to an unused location and is automatically incremented. A debug agent can calculate the system address, on the AHB-Lite bus, of the SRAM location pointed to by the POSITION register using the following equation: system address = BASE + ((P + (2AWIDTH - (BASE MOD 2AWIDTH))) MOD 2AWIDTH). Where P = POSITION AND 0xFFFF_FFF8. Where BASE is the BASE register value"]
pub type POINTER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `POINTER` writer - Trace packet location pointer. Because a packet consists of two words, the POINTER field is the location of the first word of a packet. This field contains bits \\[31:3\\]
of the address, in the SRAM, where the next trace packet will be written. The field points to an unused location and is automatically incremented. A debug agent can calculate the system address, on the AHB-Lite bus, of the SRAM location pointed to by the POSITION register using the following equation: system address = BASE + ((P + (2AWIDTH - (BASE MOD 2AWIDTH))) MOD 2AWIDTH). Where P = POSITION AND 0xFFFF_FFF8. Where BASE is the BASE register value"]
pub type POINTER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, POSITION_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bit 2 - This bit is set to 1 automatically when the POINTER value wraps as determined by the MASTER.MASK field in the MASTER Trace Control Register."]
    #[inline(always)]
    pub fn wrap(&self) -> WRAP_R {
        WRAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - Trace packet location pointer. Because a packet consists of two words, the POINTER field is the location of the first word of a packet. This field contains bits \\[31:3\\]
of the address, in the SRAM, where the next trace packet will be written. The field points to an unused location and is automatically incremented. A debug agent can calculate the system address, on the AHB-Lite bus, of the SRAM location pointed to by the POSITION register using the following equation: system address = BASE + ((P + (2AWIDTH - (BASE MOD 2AWIDTH))) MOD 2AWIDTH). Where P = POSITION AND 0xFFFF_FFF8. Where BASE is the BASE register value"]
    #[inline(always)]
    pub fn pointer(&self) -> POINTER_R {
        POINTER_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 2 - This bit is set to 1 automatically when the POINTER value wraps as determined by the MASTER.MASK field in the MASTER Trace Control Register."]
    #[inline(always)]
    pub fn wrap(&mut self) -> WRAP_W<2> {
        WRAP_W::new(self)
    }
    #[doc = "Bits 3:31 - Trace packet location pointer. Because a packet consists of two words, the POINTER field is the location of the first word of a packet. This field contains bits \\[31:3\\]
of the address, in the SRAM, where the next trace packet will be written. The field points to an unused location and is automatically incremented. A debug agent can calculate the system address, on the AHB-Lite bus, of the SRAM location pointed to by the POSITION register using the following equation: system address = BASE + ((P + (2AWIDTH - (BASE MOD 2AWIDTH))) MOD 2AWIDTH). Where P = POSITION AND 0xFFFF_FFF8. Where BASE is the BASE register value"]
    #[inline(always)]
    pub fn pointer(&mut self) -> POINTER_W<3> {
        POINTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "POSITION Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [position](index.html) module"]
pub struct POSITION_SPEC;
impl crate::RegisterSpec for POSITION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [position::R](R) reader structure"]
impl crate::Readable for POSITION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [position::W](W) writer structure"]
impl crate::Writable for POSITION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POSITION to value 0"]
impl crate::Resettable for POSITION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
