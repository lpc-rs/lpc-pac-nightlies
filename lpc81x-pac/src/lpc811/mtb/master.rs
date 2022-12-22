#[doc = "Register `MASTER` reader"]
pub struct R(crate::R<MASTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASTER` writer"]
pub struct W(crate::W<MASTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASTER_SPEC>;
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
impl From<crate::W<MASTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK` reader - This value determines the maximum size of the trace buffer in SRAM. It specifies the most-significant bit of the POSITION.POINTER field that can be updated by automatic increment. If the trace tries to advance past this power of two, the POSITION.WRAP bit is set to 1, the POSITION.POINTER\\[MASK:0\\]
bits are set to zero, and the POSITION.POINTER\\[AWIDTH-4:MASK+1\\]
bits remain unchanged. This field causes the trace packet information to be stored in a circular buffer of size 2(MASK+4) bytes, that can be positioned in memory at multiples of this size. Valid values of this field are zero to AWIDTH-4. Values greater than the maximum have the same effect as the maximum."]
pub type MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK` writer - This value determines the maximum size of the trace buffer in SRAM. It specifies the most-significant bit of the POSITION.POINTER field that can be updated by automatic increment. If the trace tries to advance past this power of two, the POSITION.WRAP bit is set to 1, the POSITION.POINTER\\[MASK:0\\]
bits are set to zero, and the POSITION.POINTER\\[AWIDTH-4:MASK+1\\]
bits remain unchanged. This field causes the trace packet information to be stored in a circular buffer of size 2(MASK+4) bytes, that can be positioned in memory at multiples of this size. Valid values of this field are zero to AWIDTH-4. Values greater than the maximum have the same effect as the maximum."]
pub type MASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MASTER_SPEC, u8, u8, 5, O>;
#[doc = "Field `TSTARTEN` reader - Trace start input enable. If this bit is 1 and the TSTART signal is HIGH, then the EN bit is set to 1. Tracing continues until a stop condition occurs."]
pub type TSTARTEN_R = crate::BitReader<bool>;
#[doc = "Field `TSTARTEN` writer - Trace start input enable. If this bit is 1 and the TSTART signal is HIGH, then the EN bit is set to 1. Tracing continues until a stop condition occurs."]
pub type TSTARTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASTER_SPEC, bool, O>;
#[doc = "Field `TSTOPEN` reader - Trace stop input enable. If this bit is 1 and the TSTOP signal is HIGH, then the EN bit is set to 0. If a trace packet is being written to memory, the write is completed before tracing is stopped."]
pub type TSTOPEN_R = crate::BitReader<bool>;
#[doc = "Field `TSTOPEN` writer - Trace stop input enable. If this bit is 1 and the TSTOP signal is HIGH, then the EN bit is set to 0. If a trace packet is being written to memory, the write is completed before tracing is stopped."]
pub type TSTOPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASTER_SPEC, bool, O>;
#[doc = "Field `SFRWPRIV` reader - Special Function Register Write Privilege bit. If this bit is 0, then User or Privileged AHB-Lite read and write accesses to the Special Function Registers are permitted. If this bit is 1, then only Privileged write accesses are permitted and User write accesses are ignored. The HPROT\\[1\\]
signal determines if an access is User or Privileged."]
pub type SFRWPRIV_R = crate::BitReader<bool>;
#[doc = "Field `SFRWPRIV` writer - Special Function Register Write Privilege bit. If this bit is 0, then User or Privileged AHB-Lite read and write accesses to the Special Function Registers are permitted. If this bit is 1, then only Privileged write accesses are permitted and User write accesses are ignored. The HPROT\\[1\\]
signal determines if an access is User or Privileged."]
pub type SFRWPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASTER_SPEC, bool, O>;
#[doc = "Field `RAMPRIV` reader - SRAM Privilege bit. If this bit is 0, then User or Privileged AHB-Lite read and write accesses to the SRAM are permitted. If this bit is 1, then only Privileged AHB-Lite read and write accesses to the SRAM are permitted and User accesses are RAZ/WI. The HPROT\\[1\\]
signal determines if an access is User or Privileged."]
pub type RAMPRIV_R = crate::BitReader<bool>;
#[doc = "Field `RAMPRIV` writer - SRAM Privilege bit. If this bit is 0, then User or Privileged AHB-Lite read and write accesses to the SRAM are permitted. If this bit is 1, then only Privileged AHB-Lite read and write accesses to the SRAM are permitted and User accesses are RAZ/WI. The HPROT\\[1\\]
signal determines if an access is User or Privileged."]
pub type RAMPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASTER_SPEC, bool, O>;
#[doc = "Field `HALTREQ` reader - Halt request bit. This bit is connected to the halt request signal of the trace logic, EDBGRQ. When HALTREQ is set to 1, EDBGRQ is asserted if DBGEN is also HIGH. The HALTREQ bit can be automatically set to 1 using the FLOW.WATERMARK field."]
pub type HALTREQ_R = crate::BitReader<bool>;
#[doc = "Field `HALTREQ` writer - Halt request bit. This bit is connected to the halt request signal of the trace logic, EDBGRQ. When HALTREQ is set to 1, EDBGRQ is asserted if DBGEN is also HIGH. The HALTREQ bit can be automatically set to 1 using the FLOW.WATERMARK field."]
pub type HALTREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASTER_SPEC, bool, O>;
#[doc = "Field `EN` reader - Main trace enable bit. When this bit is 1 trace data is written into the SRAM memory location addressed by POSITION.POINTER. The POSITION.POINTER value auto increments after the trace data packet is written. The EN bit can be automatically set to 0 using the FLOW.WATERMARK field and the FLOW.AUTOSTOP bit. The EN bit is automatically set to 1 if the TSTARTEN bit is 1 and the TSTART signal is HIGH. The EN bit is automatically set to 0 if TSTOPEN bit is 1 and the TSTOP signal is HIGH."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Main trace enable bit. When this bit is 1 trace data is written into the SRAM memory location addressed by POSITION.POINTER. The POSITION.POINTER value auto increments after the trace data packet is written. The EN bit can be automatically set to 0 using the FLOW.WATERMARK field and the FLOW.AUTOSTOP bit. The EN bit is automatically set to 1 if the TSTARTEN bit is 1 and the TSTART signal is HIGH. The EN bit is automatically set to 0 if TSTOPEN bit is 1 and the TSTOP signal is HIGH."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASTER_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - This value determines the maximum size of the trace buffer in SRAM. It specifies the most-significant bit of the POSITION.POINTER field that can be updated by automatic increment. If the trace tries to advance past this power of two, the POSITION.WRAP bit is set to 1, the POSITION.POINTER\\[MASK:0\\]
bits are set to zero, and the POSITION.POINTER\\[AWIDTH-4:MASK+1\\]
bits remain unchanged. This field causes the trace packet information to be stored in a circular buffer of size 2(MASK+4) bytes, that can be positioned in memory at multiples of this size. Valid values of this field are zero to AWIDTH-4. Values greater than the maximum have the same effect as the maximum."]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Trace start input enable. If this bit is 1 and the TSTART signal is HIGH, then the EN bit is set to 1. Tracing continues until a stop condition occurs."]
    #[inline(always)]
    pub fn tstarten(&self) -> TSTARTEN_R {
        TSTARTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trace stop input enable. If this bit is 1 and the TSTOP signal is HIGH, then the EN bit is set to 0. If a trace packet is being written to memory, the write is completed before tracing is stopped."]
    #[inline(always)]
    pub fn tstopen(&self) -> TSTOPEN_R {
        TSTOPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Special Function Register Write Privilege bit. If this bit is 0, then User or Privileged AHB-Lite read and write accesses to the Special Function Registers are permitted. If this bit is 1, then only Privileged write accesses are permitted and User write accesses are ignored. The HPROT\\[1\\]
signal determines if an access is User or Privileged."]
    #[inline(always)]
    pub fn sfrwpriv(&self) -> SFRWPRIV_R {
        SFRWPRIV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM Privilege bit. If this bit is 0, then User or Privileged AHB-Lite read and write accesses to the SRAM are permitted. If this bit is 1, then only Privileged AHB-Lite read and write accesses to the SRAM are permitted and User accesses are RAZ/WI. The HPROT\\[1\\]
signal determines if an access is User or Privileged."]
    #[inline(always)]
    pub fn rampriv(&self) -> RAMPRIV_R {
        RAMPRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Halt request bit. This bit is connected to the halt request signal of the trace logic, EDBGRQ. When HALTREQ is set to 1, EDBGRQ is asserted if DBGEN is also HIGH. The HALTREQ bit can be automatically set to 1 using the FLOW.WATERMARK field."]
    #[inline(always)]
    pub fn haltreq(&self) -> HALTREQ_R {
        HALTREQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Main trace enable bit. When this bit is 1 trace data is written into the SRAM memory location addressed by POSITION.POINTER. The POSITION.POINTER value auto increments after the trace data packet is written. The EN bit can be automatically set to 0 using the FLOW.WATERMARK field and the FLOW.AUTOSTOP bit. The EN bit is automatically set to 1 if the TSTARTEN bit is 1 and the TSTART signal is HIGH. The EN bit is automatically set to 0 if TSTOPEN bit is 1 and the TSTOP signal is HIGH."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - This value determines the maximum size of the trace buffer in SRAM. It specifies the most-significant bit of the POSITION.POINTER field that can be updated by automatic increment. If the trace tries to advance past this power of two, the POSITION.WRAP bit is set to 1, the POSITION.POINTER\\[MASK:0\\]
bits are set to zero, and the POSITION.POINTER\\[AWIDTH-4:MASK+1\\]
bits remain unchanged. This field causes the trace packet information to be stored in a circular buffer of size 2(MASK+4) bytes, that can be positioned in memory at multiples of this size. Valid values of this field are zero to AWIDTH-4. Values greater than the maximum have the same effect as the maximum."]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W<0> {
        MASK_W::new(self)
    }
    #[doc = "Bit 5 - Trace start input enable. If this bit is 1 and the TSTART signal is HIGH, then the EN bit is set to 1. Tracing continues until a stop condition occurs."]
    #[inline(always)]
    pub fn tstarten(&mut self) -> TSTARTEN_W<5> {
        TSTARTEN_W::new(self)
    }
    #[doc = "Bit 6 - Trace stop input enable. If this bit is 1 and the TSTOP signal is HIGH, then the EN bit is set to 0. If a trace packet is being written to memory, the write is completed before tracing is stopped."]
    #[inline(always)]
    pub fn tstopen(&mut self) -> TSTOPEN_W<6> {
        TSTOPEN_W::new(self)
    }
    #[doc = "Bit 7 - Special Function Register Write Privilege bit. If this bit is 0, then User or Privileged AHB-Lite read and write accesses to the Special Function Registers are permitted. If this bit is 1, then only Privileged write accesses are permitted and User write accesses are ignored. The HPROT\\[1\\]
signal determines if an access is User or Privileged."]
    #[inline(always)]
    pub fn sfrwpriv(&mut self) -> SFRWPRIV_W<7> {
        SFRWPRIV_W::new(self)
    }
    #[doc = "Bit 8 - SRAM Privilege bit. If this bit is 0, then User or Privileged AHB-Lite read and write accesses to the SRAM are permitted. If this bit is 1, then only Privileged AHB-Lite read and write accesses to the SRAM are permitted and User accesses are RAZ/WI. The HPROT\\[1\\]
signal determines if an access is User or Privileged."]
    #[inline(always)]
    pub fn rampriv(&mut self) -> RAMPRIV_W<8> {
        RAMPRIV_W::new(self)
    }
    #[doc = "Bit 9 - Halt request bit. This bit is connected to the halt request signal of the trace logic, EDBGRQ. When HALTREQ is set to 1, EDBGRQ is asserted if DBGEN is also HIGH. The HALTREQ bit can be automatically set to 1 using the FLOW.WATERMARK field."]
    #[inline(always)]
    pub fn haltreq(&mut self) -> HALTREQ_W<9> {
        HALTREQ_W::new(self)
    }
    #[doc = "Bit 31 - Main trace enable bit. When this bit is 1 trace data is written into the SRAM memory location addressed by POSITION.POINTER. The POSITION.POINTER value auto increments after the trace data packet is written. The EN bit can be automatically set to 0 using the FLOW.WATERMARK field and the FLOW.AUTOSTOP bit. The EN bit is automatically set to 1 if the TSTARTEN bit is 1 and the TSTART signal is HIGH. The EN bit is automatically set to 0 if TSTOPEN bit is 1 and the TSTOP signal is HIGH."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<31> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MASTER Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [master](index.html) module"]
pub struct MASTER_SPEC;
impl crate::RegisterSpec for MASTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [master::R](R) reader structure"]
impl crate::Readable for MASTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [master::W](W) writer structure"]
impl crate::Writable for MASTER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MASTER to value 0x80"]
impl crate::Resettable for MASTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
