#[doc = "Register `DEVICE_ID` reader"]
pub struct R(crate::R<DEVICE_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICE_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICE_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICE_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DEVICEID` reader - Device ID for LPC13xx parts: 0x2C42 502B = LPC1311FHN33 0x2C40 102B = LPC1313FHN33 0x2C40 102B = LPC1313FBD48 0x3D01 402B = LPC1342FHN33 0x3D00 002B = LPC1343FHN33 0x3D00 002B = LPC1343FBD48 0x1816 902B = LPC1311FHN33/01 0x1830 102B = LPC1313FHN33/01 0x1830 102B = LPC1313FBD48/01"]
pub type DEVICEID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Device ID for LPC13xx parts: 0x2C42 502B = LPC1311FHN33 0x2C40 102B = LPC1313FHN33 0x2C40 102B = LPC1313FBD48 0x3D01 402B = LPC1342FHN33 0x3D00 002B = LPC1343FHN33 0x3D00 002B = LPC1343FBD48 0x1816 902B = LPC1311FHN33/01 0x1830 102B = LPC1313FHN33/01 0x1830 102B = LPC1313FBD48/01"]
    #[inline(always)]
    pub fn deviceid(&self) -> DEVICEID_R {
        DEVICEID_R::new(self.bits)
    }
}
#[doc = "Device ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [device_id](index.html) module"]
pub struct DEVICE_ID_SPEC;
impl crate::RegisterSpec for DEVICE_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [device_id::R](R) reader structure"]
impl crate::Readable for DEVICE_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVICE_ID to value 0"]
impl crate::Resettable for DEVICE_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
