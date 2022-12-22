#[doc = "Register `CCER` reader"]
pub struct R(crate::R<CCER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ExtendedExternalInputSelectors` reader - Extended external input selectors. The value of these bits is 0, indicating that extended external input selectors are not implemented."]
pub type EXTENDED_EXTERNAL_INPUT_SELECTORS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ExtendedExternalInputBus` reader - Extended external input bus. The value of these bits is 0, indicating that the extended external input bus is not implemented."]
pub type EXTENDED_EXTERNAL_INPUT_BUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ReadableRegisters` reader - Readable registers. The value of this bit is 1, indicating that all registers are readable."]
pub type READABLE_REGISTERS_R = crate::BitReader<bool>;
#[doc = "Field `DataAddressComparisons` reader - Data address comparisons. The value of this bit is 1, indicating that data address comparisons are not supported."]
pub type DATA_ADDRESS_COMPARISONS_R = crate::BitReader<bool>;
#[doc = "Field `InstrumentationResources` reader - Instrumentation resources. The value of these bits is 0b000, indicating that no Instrumentation resources are supported."]
pub type INSTRUMENTATION_RESOURCES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EmbeddedICEwatchpointInputs` reader - EmbeddedICE watchpoint inputs. The value of these bits is 0b0100, indicating that the number of EmbeddedICE watchpoint inputs implemented is four. These inputs come from the DWT."]
pub type EMBEDDED_ICEWATCHPOINT_INPUTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TraceStartStopBlockUsesEmbeddedICEwatchpointInputs` reader - Trace Start/Stop block uses EmbeddedICE watchpoint inputs. The value of this bit is 1, indicating that the Trace Start/Stop block uses the EmbeddedICE watchpoint inputs."]
pub type TRACE_START_STOP_BLOCK_USES_EMBEDDED_ICEWATCHPOINT_INPUTS_R = crate::BitReader<bool>;
#[doc = "Field `EmbeddedICEbehaviorControlImplemented` reader - EmbeddedICE behavior control implemented. The value of this bit is 0, indicating that the ETMEIBCR is not implemented."]
pub type EMBEDDED_ICEBEHAVIOR_CONTROL_IMPLEMENTED_R = crate::BitReader<bool>;
#[doc = "Field `TimestampingImplemented` reader - Timestamping implemented. This bit is set to 1, indicating that timestamping is implemented."]
pub type TIMESTAMPING_IMPLEMENTED_R = crate::BitReader<bool>;
#[doc = "Field `ReducedFunctionCounter` reader - Reduced function counter. Set to 1 to indicate that Counter 1 is a reduced function counter."]
pub type REDUCED_FUNCTION_COUNTER_R = crate::BitReader<bool>;
#[doc = "Field `TimestampEncoding` reader - Timestamp encoding. Set to 1 to indicate that the timestamp is encoded as a natural binary number."]
pub type TIMESTAMP_ENCODING_R = crate::BitReader<bool>;
#[doc = "Field `TimestampSize` reader - Timestamp size. Set to 0 to indicate a size of 48 bits."]
pub type TIMESTAMP_SIZE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:2 - Extended external input selectors. The value of these bits is 0, indicating that extended external input selectors are not implemented."]
    #[inline(always)]
    pub fn extended_external_input_selectors(&self) -> EXTENDED_EXTERNAL_INPUT_SELECTORS_R {
        EXTENDED_EXTERNAL_INPUT_SELECTORS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:10 - Extended external input bus. The value of these bits is 0, indicating that the extended external input bus is not implemented."]
    #[inline(always)]
    pub fn extended_external_input_bus(&self) -> EXTENDED_EXTERNAL_INPUT_BUS_R {
        EXTENDED_EXTERNAL_INPUT_BUS_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - Readable registers. The value of this bit is 1, indicating that all registers are readable."]
    #[inline(always)]
    pub fn readable_registers(&self) -> READABLE_REGISTERS_R {
        READABLE_REGISTERS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data address comparisons. The value of this bit is 1, indicating that data address comparisons are not supported."]
    #[inline(always)]
    pub fn data_address_comparisons(&self) -> DATA_ADDRESS_COMPARISONS_R {
        DATA_ADDRESS_COMPARISONS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Instrumentation resources. The value of these bits is 0b000, indicating that no Instrumentation resources are supported."]
    #[inline(always)]
    pub fn instrumentation_resources(&self) -> INSTRUMENTATION_RESOURCES_R {
        INSTRUMENTATION_RESOURCES_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - EmbeddedICE watchpoint inputs. The value of these bits is 0b0100, indicating that the number of EmbeddedICE watchpoint inputs implemented is four. These inputs come from the DWT."]
    #[inline(always)]
    pub fn embedded_icewatchpoint_inputs(&self) -> EMBEDDED_ICEWATCHPOINT_INPUTS_R {
        EMBEDDED_ICEWATCHPOINT_INPUTS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Trace Start/Stop block uses EmbeddedICE watchpoint inputs. The value of this bit is 1, indicating that the Trace Start/Stop block uses the EmbeddedICE watchpoint inputs."]
    #[inline(always)]
    pub fn trace_start_stop_block_uses_embedded_icewatchpoint_inputs(
        &self,
    ) -> TRACE_START_STOP_BLOCK_USES_EMBEDDED_ICEWATCHPOINT_INPUTS_R {
        TRACE_START_STOP_BLOCK_USES_EMBEDDED_ICEWATCHPOINT_INPUTS_R::new(
            ((self.bits >> 20) & 1) != 0,
        )
    }
    #[doc = "Bit 21 - EmbeddedICE behavior control implemented. The value of this bit is 0, indicating that the ETMEIBCR is not implemented."]
    #[inline(always)]
    pub fn embedded_icebehavior_control_implemented(
        &self,
    ) -> EMBEDDED_ICEBEHAVIOR_CONTROL_IMPLEMENTED_R {
        EMBEDDED_ICEBEHAVIOR_CONTROL_IMPLEMENTED_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Timestamping implemented. This bit is set to 1, indicating that timestamping is implemented."]
    #[inline(always)]
    pub fn timestamping_implemented(&self) -> TIMESTAMPING_IMPLEMENTED_R {
        TIMESTAMPING_IMPLEMENTED_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 27 - Reduced function counter. Set to 1 to indicate that Counter 1 is a reduced function counter."]
    #[inline(always)]
    pub fn reduced_function_counter(&self) -> REDUCED_FUNCTION_COUNTER_R {
        REDUCED_FUNCTION_COUNTER_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Timestamp encoding. Set to 1 to indicate that the timestamp is encoded as a natural binary number."]
    #[inline(always)]
    pub fn timestamp_encoding(&self) -> TIMESTAMP_ENCODING_R {
        TIMESTAMP_ENCODING_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Timestamp size. Set to 0 to indicate a size of 48 bits."]
    #[inline(always)]
    pub fn timestamp_size(&self) -> TIMESTAMP_SIZE_R {
        TIMESTAMP_SIZE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "Configuration Code Extension Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccer](index.html) module"]
pub struct CCER_SPEC;
impl crate::RegisterSpec for CCER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccer::R](R) reader structure"]
impl crate::Readable for CCER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CCER to value 0x1854_1800"]
impl crate::Resettable for CCER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1854_1800
    }
}
