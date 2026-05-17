use apple_cf::cg::CGAffineTransform;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform3D {
    pub m11: f64,
    pub m12: f64,
    pub m13: f64,
    pub m14: f64,
    pub m21: f64,
    pub m22: f64,
    pub m23: f64,
    pub m24: f64,
    pub m31: f64,
    pub m32: f64,
    pub m33: f64,
    pub m34: f64,
    pub m41: f64,
    pub m42: f64,
    pub m43: f64,
    pub m44: f64,
}

impl Transform3D {
    #[must_use]
    pub const fn new(elements: [f64; 16]) -> Self {
        Self {
            m11: elements[0],
            m12: elements[1],
            m13: elements[2],
            m14: elements[3],
            m21: elements[4],
            m22: elements[5],
            m23: elements[6],
            m24: elements[7],
            m31: elements[8],
            m32: elements[9],
            m33: elements[10],
            m34: elements[11],
            m41: elements[12],
            m42: elements[13],
            m43: elements[14],
            m44: elements[15],
        }
    }

    #[must_use]
    pub const fn identity() -> Self {
        Self::new([
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
    }

    #[must_use]
    pub const fn translation(tx: f64, ty: f64, tz: f64) -> Self {
        Self::new([
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, tx, ty, tz, 1.0,
        ])
    }

    #[must_use]
    pub const fn scale(sx: f64, sy: f64, sz: f64) -> Self {
        Self::new([
            sx, 0.0, 0.0, 0.0, 0.0, sy, 0.0, 0.0, 0.0, 0.0, sz, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
    }

    #[must_use]
    pub fn rotation(angle: f64, x: f64, y: f64, z: f64) -> Self {
        unsafe { CATransform3DMakeRotation(angle, x, y, z) }
    }

    #[must_use]
    pub fn translated(self, tx: f64, ty: f64, tz: f64) -> Self {
        unsafe { CATransform3DTranslate(self, tx, ty, tz) }
    }

    #[must_use]
    pub fn scaled(self, sx: f64, sy: f64, sz: f64) -> Self {
        unsafe { CATransform3DScale(self, sx, sy, sz) }
    }

    #[must_use]
    pub fn rotated(self, angle: f64, x: f64, y: f64, z: f64) -> Self {
        unsafe { CATransform3DRotate(self, angle, x, y, z) }
    }

    #[must_use]
    pub fn concat(self, other: Self) -> Self {
        unsafe { CATransform3DConcat(self, other) }
    }

    #[must_use]
    pub fn inverted(self) -> Self {
        unsafe { CATransform3DInvert(self) }
    }

    #[must_use]
    pub fn from_affine(transform: CGAffineTransform) -> Self {
        unsafe { CATransform3DMakeAffineTransform(transform) }
    }

    #[must_use]
    pub fn is_affine(self) -> bool {
        unsafe { CATransform3DIsAffine(self) }
    }

    #[must_use]
    pub fn to_affine(self) -> Option<CGAffineTransform> {
        self.is_affine()
            .then(|| unsafe { CATransform3DGetAffineTransform(self) })
    }

    #[must_use]
    pub const fn as_array(&self) -> [f64; 16] {
        [
            self.m11, self.m12, self.m13, self.m14, self.m21, self.m22, self.m23, self.m24,
            self.m31, self.m32, self.m33, self.m34, self.m41, self.m42, self.m43, self.m44,
        ]
    }
}

impl Default for Transform3D {
    fn default() -> Self {
        Self::identity()
    }
}

unsafe extern "C" {
    fn CATransform3DMakeRotation(angle: f64, x: f64, y: f64, z: f64) -> Transform3D;
    fn CATransform3DTranslate(transform: Transform3D, tx: f64, ty: f64, tz: f64) -> Transform3D;
    fn CATransform3DScale(transform: Transform3D, sx: f64, sy: f64, sz: f64) -> Transform3D;
    fn CATransform3DRotate(
        transform: Transform3D,
        angle: f64,
        x: f64,
        y: f64,
        z: f64,
    ) -> Transform3D;
    fn CATransform3DConcat(a: Transform3D, b: Transform3D) -> Transform3D;
    fn CATransform3DInvert(transform: Transform3D) -> Transform3D;
    fn CATransform3DMakeAffineTransform(transform: CGAffineTransform) -> Transform3D;
    fn CATransform3DIsAffine(transform: Transform3D) -> bool;
    fn CATransform3DGetAffineTransform(transform: Transform3D) -> CGAffineTransform;
}
