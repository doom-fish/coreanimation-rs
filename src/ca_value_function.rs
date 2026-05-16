use crate::private::handle_type;

handle_type!(ValueFunction);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ValueFunctionName {
    RotateX = 0,
    RotateY = 1,
    RotateZ = 2,
    Scale = 3,
    ScaleX = 4,
    ScaleY = 5,
    ScaleZ = 6,
    Translate = 7,
    TranslateX = 8,
    TranslateY = 9,
    TranslateZ = 10,
}

impl ValueFunctionName {
    pub(crate) const fn from_raw(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::RotateX),
            1 => Some(Self::RotateY),
            2 => Some(Self::RotateZ),
            3 => Some(Self::Scale),
            4 => Some(Self::ScaleX),
            5 => Some(Self::ScaleY),
            6 => Some(Self::ScaleZ),
            7 => Some(Self::Translate),
            8 => Some(Self::TranslateX),
            9 => Some(Self::TranslateY),
            10 => Some(Self::TranslateZ),
            _ => None,
        }
    }

    pub(crate) const fn raw(self) -> i32 {
        self as i32
    }
}

impl ValueFunction {
    #[must_use]
    pub fn new(name: ValueFunctionName) -> Option<Self> {
        unsafe { Self::from_raw(crate::ffi::ca_value_function_new(name.raw())) }
    }

    #[must_use]
    pub fn name(&self) -> Option<ValueFunctionName> {
        ValueFunctionName::from_raw(unsafe {
            crate::ffi::ca_value_function_get_name(self.as_ptr())
        })
    }
}
