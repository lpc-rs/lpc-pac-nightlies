#[doc = "Register `FLASHCFG` reader"]
pub struct R(crate::R<FLASHCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASHCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASHCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASHCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASHCFG` writer"]
pub struct W(crate::W<FLASHCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASHCFG_SPEC>;
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
impl From<crate::W<FLASHCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASHCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FETCHCFG` reader - Instruction fetch configuration. This field determines how flash accelerator buffers are used for instruction fetches."]
pub type FETCHCFG_R = crate::FieldReader<u8, FETCHCFG_A>;
#[doc = "Instruction fetch configuration. This field determines how flash accelerator buffers are used for instruction fetches.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FETCHCFG_A {
    #[doc = "0: Instruction fetches from flash are not buffered. Every fetch request from the CPU results in a read of the flash memory. This setting may use significantly more power than when buffering is enabled."]
    NO_BUFFER = 0,
    #[doc = "1: One buffer is used for all instruction fetches."]
    ONE_BUFFER = 1,
    #[doc = "2: All buffers may be used for instruction fetches."]
    ALL_BUFFERS = 2,
}
impl From<FETCHCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: FETCHCFG_A) -> Self {
        variant as _
    }
}
impl FETCHCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FETCHCFG_A> {
        match self.bits {
            0 => Some(FETCHCFG_A::NO_BUFFER),
            1 => Some(FETCHCFG_A::ONE_BUFFER),
            2 => Some(FETCHCFG_A::ALL_BUFFERS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_BUFFER`"]
    #[inline(always)]
    pub fn is_no_buffer(&self) -> bool {
        *self == FETCHCFG_A::NO_BUFFER
    }
    #[doc = "Checks if the value of the field is `ONE_BUFFER`"]
    #[inline(always)]
    pub fn is_one_buffer(&self) -> bool {
        *self == FETCHCFG_A::ONE_BUFFER
    }
    #[doc = "Checks if the value of the field is `ALL_BUFFERS`"]
    #[inline(always)]
    pub fn is_all_buffers(&self) -> bool {
        *self == FETCHCFG_A::ALL_BUFFERS
    }
}
#[doc = "Field `FETCHCFG` writer - Instruction fetch configuration. This field determines how flash accelerator buffers are used for instruction fetches."]
pub type FETCHCFG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASHCFG_SPEC, u8, FETCHCFG_A, 2, O>;
impl<'a, const O: u8> FETCHCFG_W<'a, O> {
    #[doc = "Instruction fetches from flash are not buffered. Every fetch request from the CPU results in a read of the flash memory. This setting may use significantly more power than when buffering is enabled."]
    #[inline(always)]
    pub fn no_buffer(self) -> &'a mut W {
        self.variant(FETCHCFG_A::NO_BUFFER)
    }
    #[doc = "One buffer is used for all instruction fetches."]
    #[inline(always)]
    pub fn one_buffer(self) -> &'a mut W {
        self.variant(FETCHCFG_A::ONE_BUFFER)
    }
    #[doc = "All buffers may be used for instruction fetches."]
    #[inline(always)]
    pub fn all_buffers(self) -> &'a mut W {
        self.variant(FETCHCFG_A::ALL_BUFFERS)
    }
}
#[doc = "Field `DATACFG` reader - Data read configuration. This field determines how flash accelerator buffers are used for data accesses."]
pub type DATACFG_R = crate::FieldReader<u8, DATACFG_A>;
#[doc = "Data read configuration. This field determines how flash accelerator buffers are used for data accesses.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATACFG_A {
    #[doc = "0: Data accesses from flash are not buffered. Every data access from the CPU results in a read of the flash memory."]
    NOT_BUFFERED = 0,
    #[doc = "1: One buffer is used for all data accesses."]
    ONE_BUFFER = 1,
    #[doc = "2: All buffers may be used for data accesses."]
    ALL_BUFFERS = 2,
}
impl From<DATACFG_A> for u8 {
    #[inline(always)]
    fn from(variant: DATACFG_A) -> Self {
        variant as _
    }
}
impl DATACFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATACFG_A> {
        match self.bits {
            0 => Some(DATACFG_A::NOT_BUFFERED),
            1 => Some(DATACFG_A::ONE_BUFFER),
            2 => Some(DATACFG_A::ALL_BUFFERS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUFFERED`"]
    #[inline(always)]
    pub fn is_not_buffered(&self) -> bool {
        *self == DATACFG_A::NOT_BUFFERED
    }
    #[doc = "Checks if the value of the field is `ONE_BUFFER`"]
    #[inline(always)]
    pub fn is_one_buffer(&self) -> bool {
        *self == DATACFG_A::ONE_BUFFER
    }
    #[doc = "Checks if the value of the field is `ALL_BUFFERS`"]
    #[inline(always)]
    pub fn is_all_buffers(&self) -> bool {
        *self == DATACFG_A::ALL_BUFFERS
    }
}
#[doc = "Field `DATACFG` writer - Data read configuration. This field determines how flash accelerator buffers are used for data accesses."]
pub type DATACFG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASHCFG_SPEC, u8, DATACFG_A, 2, O>;
impl<'a, const O: u8> DATACFG_W<'a, O> {
    #[doc = "Data accesses from flash are not buffered. Every data access from the CPU results in a read of the flash memory."]
    #[inline(always)]
    pub fn not_buffered(self) -> &'a mut W {
        self.variant(DATACFG_A::NOT_BUFFERED)
    }
    #[doc = "One buffer is used for all data accesses."]
    #[inline(always)]
    pub fn one_buffer(self) -> &'a mut W {
        self.variant(DATACFG_A::ONE_BUFFER)
    }
    #[doc = "All buffers may be used for data accesses."]
    #[inline(always)]
    pub fn all_buffers(self) -> &'a mut W {
        self.variant(DATACFG_A::ALL_BUFFERS)
    }
}
#[doc = "Field `ACCEL` reader - Acceleration enable."]
pub type ACCEL_R = crate::BitReader<ACCEL_A>;
#[doc = "Acceleration enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACCEL_A {
    #[doc = "0: Flash acceleration is disabled. Every flash read (including those fulfilled from a buffer) takes FLASHTIM + 1 system clocks. This allows more determinism at a cost of performance."]
    DISABLED = 0,
    #[doc = "1: Flash acceleration is enabled. Performance is enhanced, dependent on other FLASHCFG settings."]
    ENABLED = 1,
}
impl From<ACCEL_A> for bool {
    #[inline(always)]
    fn from(variant: ACCEL_A) -> Self {
        variant as u8 != 0
    }
}
impl ACCEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACCEL_A {
        match self.bits {
            false => ACCEL_A::DISABLED,
            true => ACCEL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ACCEL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ACCEL_A::ENABLED
    }
}
#[doc = "Field `ACCEL` writer - Acceleration enable."]
pub type ACCEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASHCFG_SPEC, ACCEL_A, O>;
impl<'a, const O: u8> ACCEL_W<'a, O> {
    #[doc = "Flash acceleration is disabled. Every flash read (including those fulfilled from a buffer) takes FLASHTIM + 1 system clocks. This allows more determinism at a cost of performance."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ACCEL_A::DISABLED)
    }
    #[doc = "Flash acceleration is enabled. Performance is enhanced, dependent on other FLASHCFG settings."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ACCEL_A::ENABLED)
    }
}
#[doc = "Field `PREFEN` reader - Prefetch enable."]
pub type PREFEN_R = crate::BitReader<PREFEN_A>;
#[doc = "Prefetch enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREFEN_A {
    #[doc = "0: No instruction prefetch is performed."]
    NO_PREFETCH = 0,
    #[doc = "1: If the FETCHCFG field is not 0, the next flash line following the current execution address is automatically prefetched if it is not already buffered."]
    PREFETCH = 1,
}
impl From<PREFEN_A> for bool {
    #[inline(always)]
    fn from(variant: PREFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PREFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREFEN_A {
        match self.bits {
            false => PREFEN_A::NO_PREFETCH,
            true => PREFEN_A::PREFETCH,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PREFETCH`"]
    #[inline(always)]
    pub fn is_no_prefetch(&self) -> bool {
        *self == PREFEN_A::NO_PREFETCH
    }
    #[doc = "Checks if the value of the field is `PREFETCH`"]
    #[inline(always)]
    pub fn is_prefetch(&self) -> bool {
        *self == PREFEN_A::PREFETCH
    }
}
#[doc = "Field `PREFEN` writer - Prefetch enable."]
pub type PREFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASHCFG_SPEC, PREFEN_A, O>;
impl<'a, const O: u8> PREFEN_W<'a, O> {
    #[doc = "No instruction prefetch is performed."]
    #[inline(always)]
    pub fn no_prefetch(self) -> &'a mut W {
        self.variant(PREFEN_A::NO_PREFETCH)
    }
    #[doc = "If the FETCHCFG field is not 0, the next flash line following the current execution address is automatically prefetched if it is not already buffered."]
    #[inline(always)]
    pub fn prefetch(self) -> &'a mut W {
        self.variant(PREFEN_A::PREFETCH)
    }
}
#[doc = "Field `PREFOVR` reader - Prefetch override. This bit only applies when PREFEN = 1 and a buffered instruction is completing for which the next flash line is not already buffered or being prefetched."]
pub type PREFOVR_R = crate::BitReader<PREFOVR_A>;
#[doc = "Prefetch override. This bit only applies when PREFEN = 1 and a buffered instruction is completing for which the next flash line is not already buffered or being prefetched.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREFOVR_A {
    #[doc = "0: Any previously initiated prefetch will be completed."]
    PREFETCH_COMPLETED = 0,
    #[doc = "1: Any previously initiated prefetch will be aborted, and the next flash line following the current execution address will be prefetched if not already buffered."]
    PREFETCH_ABORT = 1,
}
impl From<PREFOVR_A> for bool {
    #[inline(always)]
    fn from(variant: PREFOVR_A) -> Self {
        variant as u8 != 0
    }
}
impl PREFOVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREFOVR_A {
        match self.bits {
            false => PREFOVR_A::PREFETCH_COMPLETED,
            true => PREFOVR_A::PREFETCH_ABORT,
        }
    }
    #[doc = "Checks if the value of the field is `PREFETCH_COMPLETED`"]
    #[inline(always)]
    pub fn is_prefetch_completed(&self) -> bool {
        *self == PREFOVR_A::PREFETCH_COMPLETED
    }
    #[doc = "Checks if the value of the field is `PREFETCH_ABORT`"]
    #[inline(always)]
    pub fn is_prefetch_abort(&self) -> bool {
        *self == PREFOVR_A::PREFETCH_ABORT
    }
}
#[doc = "Field `PREFOVR` writer - Prefetch override. This bit only applies when PREFEN = 1 and a buffered instruction is completing for which the next flash line is not already buffered or being prefetched."]
pub type PREFOVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASHCFG_SPEC, PREFOVR_A, O>;
impl<'a, const O: u8> PREFOVR_W<'a, O> {
    #[doc = "Any previously initiated prefetch will be completed."]
    #[inline(always)]
    pub fn prefetch_completed(self) -> &'a mut W {
        self.variant(PREFOVR_A::PREFETCH_COMPLETED)
    }
    #[doc = "Any previously initiated prefetch will be aborted, and the next flash line following the current execution address will be prefetched if not already buffered."]
    #[inline(always)]
    pub fn prefetch_abort(self) -> &'a mut W {
        self.variant(PREFOVR_A::PREFETCH_ABORT)
    }
}
#[doc = "Field `FLASHTIM` reader - Flash memory access time. The number of system clocks used for flash accesses is equal to FLASHTIM +1."]
pub type FLASHTIM_R = crate::FieldReader<u8, FLASHTIM_A>;
#[doc = "Flash memory access time. The number of system clocks used for flash accesses is equal to FLASHTIM +1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASHTIM_A {
    #[doc = "0: 1 system clock flash access time (for system clock rates up to 12 MHz)."]
    N_1_CLOCK_CYCLE = 0,
    #[doc = "1: 2 system clocks flash access time (for system clock rates up to 30 MHz)."]
    N_2_CLOCK_CYCLES = 1,
    #[doc = "2: 3 system clocks flash access time (for system clock rates up to 60 MHz)."]
    N_3_CLOCK_CYCLES = 2,
    #[doc = "3: 4 system clocks flash access time (for system clock rates up to 85 MHz)."]
    N_4_CLOCK_CYCLES = 3,
    #[doc = "4: 5 system clocks flash access time (for system clock rates up to 100 MHz)."]
    N_5_CLOCK_CYCLES = 4,
}
impl From<FLASHTIM_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASHTIM_A) -> Self {
        variant as _
    }
}
impl FLASHTIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLASHTIM_A> {
        match self.bits {
            0 => Some(FLASHTIM_A::N_1_CLOCK_CYCLE),
            1 => Some(FLASHTIM_A::N_2_CLOCK_CYCLES),
            2 => Some(FLASHTIM_A::N_3_CLOCK_CYCLES),
            3 => Some(FLASHTIM_A::N_4_CLOCK_CYCLES),
            4 => Some(FLASHTIM_A::N_5_CLOCK_CYCLES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `N_1_CLOCK_CYCLE`"]
    #[inline(always)]
    pub fn is_n_1_clock_cycle(&self) -> bool {
        *self == FLASHTIM_A::N_1_CLOCK_CYCLE
    }
    #[doc = "Checks if the value of the field is `N_2_CLOCK_CYCLES`"]
    #[inline(always)]
    pub fn is_n_2_clock_cycles(&self) -> bool {
        *self == FLASHTIM_A::N_2_CLOCK_CYCLES
    }
    #[doc = "Checks if the value of the field is `N_3_CLOCK_CYCLES`"]
    #[inline(always)]
    pub fn is_n_3_clock_cycles(&self) -> bool {
        *self == FLASHTIM_A::N_3_CLOCK_CYCLES
    }
    #[doc = "Checks if the value of the field is `N_4_CLOCK_CYCLES`"]
    #[inline(always)]
    pub fn is_n_4_clock_cycles(&self) -> bool {
        *self == FLASHTIM_A::N_4_CLOCK_CYCLES
    }
    #[doc = "Checks if the value of the field is `N_5_CLOCK_CYCLES`"]
    #[inline(always)]
    pub fn is_n_5_clock_cycles(&self) -> bool {
        *self == FLASHTIM_A::N_5_CLOCK_CYCLES
    }
}
#[doc = "Field `FLASHTIM` writer - Flash memory access time. The number of system clocks used for flash accesses is equal to FLASHTIM +1."]
pub type FLASHTIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASHCFG_SPEC, u8, FLASHTIM_A, 4, O>;
impl<'a, const O: u8> FLASHTIM_W<'a, O> {
    #[doc = "1 system clock flash access time (for system clock rates up to 12 MHz)."]
    #[inline(always)]
    pub fn n_1_clock_cycle(self) -> &'a mut W {
        self.variant(FLASHTIM_A::N_1_CLOCK_CYCLE)
    }
    #[doc = "2 system clocks flash access time (for system clock rates up to 30 MHz)."]
    #[inline(always)]
    pub fn n_2_clock_cycles(self) -> &'a mut W {
        self.variant(FLASHTIM_A::N_2_CLOCK_CYCLES)
    }
    #[doc = "3 system clocks flash access time (for system clock rates up to 60 MHz)."]
    #[inline(always)]
    pub fn n_3_clock_cycles(self) -> &'a mut W {
        self.variant(FLASHTIM_A::N_3_CLOCK_CYCLES)
    }
    #[doc = "4 system clocks flash access time (for system clock rates up to 85 MHz)."]
    #[inline(always)]
    pub fn n_4_clock_cycles(self) -> &'a mut W {
        self.variant(FLASHTIM_A::N_4_CLOCK_CYCLES)
    }
    #[doc = "5 system clocks flash access time (for system clock rates up to 100 MHz)."]
    #[inline(always)]
    pub fn n_5_clock_cycles(self) -> &'a mut W {
        self.variant(FLASHTIM_A::N_5_CLOCK_CYCLES)
    }
}
impl R {
    #[doc = "Bits 0:1 - Instruction fetch configuration. This field determines how flash accelerator buffers are used for instruction fetches."]
    #[inline(always)]
    pub fn fetchcfg(&self) -> FETCHCFG_R {
        FETCHCFG_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Data read configuration. This field determines how flash accelerator buffers are used for data accesses."]
    #[inline(always)]
    pub fn datacfg(&self) -> DATACFG_R {
        DATACFG_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Acceleration enable."]
    #[inline(always)]
    pub fn accel(&self) -> ACCEL_R {
        ACCEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Prefetch enable."]
    #[inline(always)]
    pub fn prefen(&self) -> PREFEN_R {
        PREFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Prefetch override. This bit only applies when PREFEN = 1 and a buffered instruction is completing for which the next flash line is not already buffered or being prefetched."]
    #[inline(always)]
    pub fn prefovr(&self) -> PREFOVR_R {
        PREFOVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Flash memory access time. The number of system clocks used for flash accesses is equal to FLASHTIM +1."]
    #[inline(always)]
    pub fn flashtim(&self) -> FLASHTIM_R {
        FLASHTIM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Instruction fetch configuration. This field determines how flash accelerator buffers are used for instruction fetches."]
    #[inline(always)]
    pub fn fetchcfg(&mut self) -> FETCHCFG_W<0> {
        FETCHCFG_W::new(self)
    }
    #[doc = "Bits 2:3 - Data read configuration. This field determines how flash accelerator buffers are used for data accesses."]
    #[inline(always)]
    pub fn datacfg(&mut self) -> DATACFG_W<2> {
        DATACFG_W::new(self)
    }
    #[doc = "Bit 4 - Acceleration enable."]
    #[inline(always)]
    pub fn accel(&mut self) -> ACCEL_W<4> {
        ACCEL_W::new(self)
    }
    #[doc = "Bit 5 - Prefetch enable."]
    #[inline(always)]
    pub fn prefen(&mut self) -> PREFEN_W<5> {
        PREFEN_W::new(self)
    }
    #[doc = "Bit 6 - Prefetch override. This bit only applies when PREFEN = 1 and a buffered instruction is completing for which the next flash line is not already buffered or being prefetched."]
    #[inline(always)]
    pub fn prefovr(&mut self) -> PREFOVR_W<6> {
        PREFOVR_W::new(self)
    }
    #[doc = "Bits 12:15 - Flash memory access time. The number of system clocks used for flash accesses is equal to FLASHTIM +1."]
    #[inline(always)]
    pub fn flashtim(&mut self) -> FLASHTIM_W<12> {
        FLASHTIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash wait states configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashcfg](index.html) module"]
pub struct FLASHCFG_SPEC;
impl crate::RegisterSpec for FLASHCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flashcfg::R](R) reader structure"]
impl crate::Readable for FLASHCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flashcfg::W](W) writer structure"]
impl crate::Writable for FLASHCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASHCFG to value 0x0d1a"]
impl crate::Resettable for FLASHCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0d1a
    }
}
