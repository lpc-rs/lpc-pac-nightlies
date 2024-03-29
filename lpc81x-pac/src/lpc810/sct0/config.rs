#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UNIFY` reader - SCT operation"]
pub type UNIFY_R = crate::BitReader<UNIFY_A>;
#[doc = "SCT operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNIFY_A {
    #[doc = "0: The SCT operates as two 16-bit counters named COUNTER_L and COUNTER_H."]
    DUAL_COUNTER = 0,
    #[doc = "1: The SCT operates as a unified 32-bit counter."]
    UNIFIED_COUNTER = 1,
}
impl From<UNIFY_A> for bool {
    #[inline(always)]
    fn from(variant: UNIFY_A) -> Self {
        variant as u8 != 0
    }
}
impl UNIFY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNIFY_A {
        match self.bits {
            false => UNIFY_A::DUAL_COUNTER,
            true => UNIFY_A::UNIFIED_COUNTER,
        }
    }
    #[doc = "Checks if the value of the field is `DUAL_COUNTER`"]
    #[inline(always)]
    pub fn is_dual_counter(&self) -> bool {
        *self == UNIFY_A::DUAL_COUNTER
    }
    #[doc = "Checks if the value of the field is `UNIFIED_COUNTER`"]
    #[inline(always)]
    pub fn is_unified_counter(&self) -> bool {
        *self == UNIFY_A::UNIFIED_COUNTER
    }
}
#[doc = "Field `UNIFY` writer - SCT operation"]
pub type UNIFY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, UNIFY_A, O>;
impl<'a, const O: u8> UNIFY_W<'a, O> {
    #[doc = "The SCT operates as two 16-bit counters named COUNTER_L and COUNTER_H."]
    #[inline(always)]
    pub fn dual_counter(self) -> &'a mut W {
        self.variant(UNIFY_A::DUAL_COUNTER)
    }
    #[doc = "The SCT operates as a unified 32-bit counter."]
    #[inline(always)]
    pub fn unified_counter(self) -> &'a mut W {
        self.variant(UNIFY_A::UNIFIED_COUNTER)
    }
}
#[doc = "Field `CLKMODE` reader - SCT clock mode"]
pub type CLKMODE_R = crate::FieldReader<u8, CLKMODE_A>;
#[doc = "SCT clock mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKMODE_A {
    #[doc = "0: System Clock Mode. The system clock clocks the entire SCT module including the counter(s) and counter prescalers."]
    SYSTEM_CLOCK_MODE = 0,
    #[doc = "1: Sampled System Clock Mode. The system clock clocks the SCT module, but the counter and prescalers are only enabled to count when the designated edge is detected on the input selected by the CKSEL field. The minimum pulse width on the selected clock-gate input is 1 bus clock period. This mode is the high-performance, sampled-clock mode."]
    SAMPLED_SYSTEM_CLOCK_MODE = 1,
    #[doc = "2: SCT Input Clock Mode. The input/edge selected by the CKSEL field clocks the SCT module, including the counters and prescalers, after first being synchronized to the system clock. The minimum pulse width on the clock input is 1 bus clock period. This mode is the low-power, sampled-clock mode."]
    SCT_INPUT_CLOCK_MODE = 2,
    #[doc = "3: Asynchronous Mode. The entire SCT module is clocked directly by the input/edge selected by the CKSEL field. In this mode, the SCT outputs are switched synchronously to the SCT input clock - not the system clock. The input clock rate must be at least half the system clock rate and can be the same or faster than the system clock."]
    ASYNCHRONOUS_MODE = 3,
}
impl From<CLKMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKMODE_A) -> Self {
        variant as _
    }
}
impl CLKMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKMODE_A {
        match self.bits {
            0 => CLKMODE_A::SYSTEM_CLOCK_MODE,
            1 => CLKMODE_A::SAMPLED_SYSTEM_CLOCK_MODE,
            2 => CLKMODE_A::SCT_INPUT_CLOCK_MODE,
            3 => CLKMODE_A::ASYNCHRONOUS_MODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYSTEM_CLOCK_MODE`"]
    #[inline(always)]
    pub fn is_system_clock_mode(&self) -> bool {
        *self == CLKMODE_A::SYSTEM_CLOCK_MODE
    }
    #[doc = "Checks if the value of the field is `SAMPLED_SYSTEM_CLOCK_MODE`"]
    #[inline(always)]
    pub fn is_sampled_system_clock_mode(&self) -> bool {
        *self == CLKMODE_A::SAMPLED_SYSTEM_CLOCK_MODE
    }
    #[doc = "Checks if the value of the field is `SCT_INPUT_CLOCK_MODE`"]
    #[inline(always)]
    pub fn is_sct_input_clock_mode(&self) -> bool {
        *self == CLKMODE_A::SCT_INPUT_CLOCK_MODE
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS_MODE`"]
    #[inline(always)]
    pub fn is_asynchronous_mode(&self) -> bool {
        *self == CLKMODE_A::ASYNCHRONOUS_MODE
    }
}
#[doc = "Field `CLKMODE` writer - SCT clock mode"]
pub type CLKMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CONFIG_SPEC, u8, CLKMODE_A, 2, O>;
impl<'a, const O: u8> CLKMODE_W<'a, O> {
    #[doc = "System Clock Mode. The system clock clocks the entire SCT module including the counter(s) and counter prescalers."]
    #[inline(always)]
    pub fn system_clock_mode(self) -> &'a mut W {
        self.variant(CLKMODE_A::SYSTEM_CLOCK_MODE)
    }
    #[doc = "Sampled System Clock Mode. The system clock clocks the SCT module, but the counter and prescalers are only enabled to count when the designated edge is detected on the input selected by the CKSEL field. The minimum pulse width on the selected clock-gate input is 1 bus clock period. This mode is the high-performance, sampled-clock mode."]
    #[inline(always)]
    pub fn sampled_system_clock_mode(self) -> &'a mut W {
        self.variant(CLKMODE_A::SAMPLED_SYSTEM_CLOCK_MODE)
    }
    #[doc = "SCT Input Clock Mode. The input/edge selected by the CKSEL field clocks the SCT module, including the counters and prescalers, after first being synchronized to the system clock. The minimum pulse width on the clock input is 1 bus clock period. This mode is the low-power, sampled-clock mode."]
    #[inline(always)]
    pub fn sct_input_clock_mode(self) -> &'a mut W {
        self.variant(CLKMODE_A::SCT_INPUT_CLOCK_MODE)
    }
    #[doc = "Asynchronous Mode. The entire SCT module is clocked directly by the input/edge selected by the CKSEL field. In this mode, the SCT outputs are switched synchronously to the SCT input clock - not the system clock. The input clock rate must be at least half the system clock rate and can be the same or faster than the system clock."]
    #[inline(always)]
    pub fn asynchronous_mode(self) -> &'a mut W {
        self.variant(CLKMODE_A::ASYNCHRONOUS_MODE)
    }
}
#[doc = "Field `CKSEL` reader - SCT clock select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register."]
pub type CKSEL_R = crate::FieldReader<u8, CKSEL_A>;
#[doc = "SCT clock select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKSEL_A {
    #[doc = "0: Rising edges on input 0."]
    INPUT_0_RISING_EDGES = 0,
    #[doc = "1: Falling edges on input 0."]
    INPUT_0_FALLING_EDGE = 1,
    #[doc = "2: Rising edges on input 1."]
    INPUT_1_RISING_EDGES = 2,
    #[doc = "3: Falling edges on input 1."]
    INPUT_1_FALLING_EDGE = 3,
    #[doc = "4: Rising edges on input 2."]
    INPUT_2_RISING_EDGES = 4,
    #[doc = "5: Falling edges on input 2."]
    INPUT_2_FALLING_EDGE = 5,
    #[doc = "6: Rising edges on input 3."]
    INPUT_3_RISING_EDGES = 6,
    #[doc = "7: Falling edges on input 3."]
    INPUT_3_FALLING_EDGE = 7,
}
impl From<CKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CKSEL_A) -> Self {
        variant as _
    }
}
impl CKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKSEL_A> {
        match self.bits {
            0 => Some(CKSEL_A::INPUT_0_RISING_EDGES),
            1 => Some(CKSEL_A::INPUT_0_FALLING_EDGE),
            2 => Some(CKSEL_A::INPUT_1_RISING_EDGES),
            3 => Some(CKSEL_A::INPUT_1_FALLING_EDGE),
            4 => Some(CKSEL_A::INPUT_2_RISING_EDGES),
            5 => Some(CKSEL_A::INPUT_2_FALLING_EDGE),
            6 => Some(CKSEL_A::INPUT_3_RISING_EDGES),
            7 => Some(CKSEL_A::INPUT_3_FALLING_EDGE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT_0_RISING_EDGES`"]
    #[inline(always)]
    pub fn is_input_0_rising_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_0_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_0_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_input_0_falling_edge(&self) -> bool {
        *self == CKSEL_A::INPUT_0_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `INPUT_1_RISING_EDGES`"]
    #[inline(always)]
    pub fn is_input_1_rising_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_1_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_1_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_input_1_falling_edge(&self) -> bool {
        *self == CKSEL_A::INPUT_1_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `INPUT_2_RISING_EDGES`"]
    #[inline(always)]
    pub fn is_input_2_rising_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_2_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_2_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_input_2_falling_edge(&self) -> bool {
        *self == CKSEL_A::INPUT_2_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `INPUT_3_RISING_EDGES`"]
    #[inline(always)]
    pub fn is_input_3_rising_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_3_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_3_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_input_3_falling_edge(&self) -> bool {
        *self == CKSEL_A::INPUT_3_FALLING_EDGE
    }
}
#[doc = "Field `CKSEL` writer - SCT clock select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register."]
pub type CKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, CKSEL_A, 4, O>;
impl<'a, const O: u8> CKSEL_W<'a, O> {
    #[doc = "Rising edges on input 0."]
    #[inline(always)]
    pub fn input_0_rising_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_0_RISING_EDGES)
    }
    #[doc = "Falling edges on input 0."]
    #[inline(always)]
    pub fn input_0_falling_edge(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_0_FALLING_EDGE)
    }
    #[doc = "Rising edges on input 1."]
    #[inline(always)]
    pub fn input_1_rising_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_1_RISING_EDGES)
    }
    #[doc = "Falling edges on input 1."]
    #[inline(always)]
    pub fn input_1_falling_edge(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_1_FALLING_EDGE)
    }
    #[doc = "Rising edges on input 2."]
    #[inline(always)]
    pub fn input_2_rising_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_2_RISING_EDGES)
    }
    #[doc = "Falling edges on input 2."]
    #[inline(always)]
    pub fn input_2_falling_edge(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_2_FALLING_EDGE)
    }
    #[doc = "Rising edges on input 3."]
    #[inline(always)]
    pub fn input_3_rising_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_3_RISING_EDGES)
    }
    #[doc = "Falling edges on input 3."]
    #[inline(always)]
    pub fn input_3_falling_edge(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_3_FALLING_EDGE)
    }
}
#[doc = "Field `NORELOAD_L` reader - A 1 in this bit prevents the lower match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
pub type NORELOAD_L_R = crate::BitReader<bool>;
#[doc = "Field `NORELOAD_L` writer - A 1 in this bit prevents the lower match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
pub type NORELOAD_L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `NORELOAD_H` reader - A 1 in this bit prevents the higher match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
pub type NORELOAD_H_R = crate::BitReader<bool>;
#[doc = "Field `NORELOAD_H` writer - A 1 in this bit prevents the higher match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
pub type NORELOAD_H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `INSYNC` reader - Synchronization for input N (bit 9 = input 0, bit 10 = input 1,, bit 12 = input 3); all other bits are reserved. A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is known to already be synchronous to the SCT clock, this bit may be set to 0 for faster input response. (Note: The SCT clock is the system clock for CKMODEs 0-2. It is the selected, asynchronous SCT input clock for CKMODE3). Note that the INSYNC field only affects inputs used for event generation. It does not apply to the clock input specified in the CKSEL field."]
pub type INSYNC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSYNC` writer - Synchronization for input N (bit 9 = input 0, bit 10 = input 1,, bit 12 = input 3); all other bits are reserved. A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is known to already be synchronous to the SCT clock, this bit may be set to 0 for faster input response. (Note: The SCT clock is the system clock for CKMODEs 0-2. It is the selected, asynchronous SCT input clock for CKMODE3). Note that the INSYNC field only affects inputs used for event generation. It does not apply to the clock input specified in the CKSEL field."]
pub type INSYNC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `AUTOLIMIT_L` reader - A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
pub type AUTOLIMIT_L_R = crate::BitReader<bool>;
#[doc = "Field `AUTOLIMIT_L` writer - A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
pub type AUTOLIMIT_L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `AUTOLIMIT_H` reader - A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
pub type AUTOLIMIT_H_R = crate::BitReader<bool>;
#[doc = "Field `AUTOLIMIT_H` writer - A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
pub type AUTOLIMIT_H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SCT operation"]
    #[inline(always)]
    pub fn unify(&self) -> UNIFY_R {
        UNIFY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - SCT clock mode"]
    #[inline(always)]
    pub fn clkmode(&self) -> CLKMODE_R {
        CLKMODE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:6 - SCT clock select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register."]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - A 1 in this bit prevents the lower match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub fn noreload_l(&self) -> NORELOAD_L_R {
        NORELOAD_L_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - A 1 in this bit prevents the higher match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub fn noreload_h(&self) -> NORELOAD_H_R {
        NORELOAD_H_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:16 - Synchronization for input N (bit 9 = input 0, bit 10 = input 1,, bit 12 = input 3); all other bits are reserved. A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is known to already be synchronous to the SCT clock, this bit may be set to 0 for faster input response. (Note: The SCT clock is the system clock for CKMODEs 0-2. It is the selected, asynchronous SCT input clock for CKMODE3). Note that the INSYNC field only affects inputs used for event generation. It does not apply to the clock input specified in the CKSEL field."]
    #[inline(always)]
    pub fn insync(&self) -> INSYNC_R {
        INSYNC_R::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bit 17 - A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub fn autolimit_l(&self) -> AUTOLIMIT_L_R {
        AUTOLIMIT_L_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub fn autolimit_h(&self) -> AUTOLIMIT_H_R {
        AUTOLIMIT_H_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCT operation"]
    #[inline(always)]
    pub fn unify(&mut self) -> UNIFY_W<0> {
        UNIFY_W::new(self)
    }
    #[doc = "Bits 1:2 - SCT clock mode"]
    #[inline(always)]
    pub fn clkmode(&mut self) -> CLKMODE_W<1> {
        CLKMODE_W::new(self)
    }
    #[doc = "Bits 3:6 - SCT clock select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register."]
    #[inline(always)]
    pub fn cksel(&mut self) -> CKSEL_W<3> {
        CKSEL_W::new(self)
    }
    #[doc = "Bit 7 - A 1 in this bit prevents the lower match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub fn noreload_l(&mut self) -> NORELOAD_L_W<7> {
        NORELOAD_L_W::new(self)
    }
    #[doc = "Bit 8 - A 1 in this bit prevents the higher match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub fn noreload_h(&mut self) -> NORELOAD_H_W<8> {
        NORELOAD_H_W::new(self)
    }
    #[doc = "Bits 9:16 - Synchronization for input N (bit 9 = input 0, bit 10 = input 1,, bit 12 = input 3); all other bits are reserved. A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is known to already be synchronous to the SCT clock, this bit may be set to 0 for faster input response. (Note: The SCT clock is the system clock for CKMODEs 0-2. It is the selected, asynchronous SCT input clock for CKMODE3). Note that the INSYNC field only affects inputs used for event generation. It does not apply to the clock input specified in the CKSEL field."]
    #[inline(always)]
    pub fn insync(&mut self) -> INSYNC_W<9> {
        INSYNC_W::new(self)
    }
    #[doc = "Bit 17 - A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub fn autolimit_l(&mut self) -> AUTOLIMIT_L_W<17> {
        AUTOLIMIT_L_W::new(self)
    }
    #[doc = "Bit 18 - A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub fn autolimit_h(&mut self) -> AUTOLIMIT_H_W<18> {
        AUTOLIMIT_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIG to value 0x1e00"]
impl crate::Resettable for CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1e00
    }
}
