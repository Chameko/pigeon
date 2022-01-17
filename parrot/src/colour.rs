use bytemuck::{Pod, Zeroable};

/// A RGBA colour with 8-bit colour channels
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Default, Pod, Zeroable)]
pub struct Rgba8 {
    /// Red
    pub r: u8,
    /// Green
    pub g: u8,
    /// Blue
    pub b: u8,
    /// Alpha
    pub a: u8,
}

impl Rgba8 {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {r, g, b, a}
    }

    /// Given a slice of bytes, return a slice of [`Rgba8`] values
    pub fn align<'a, S: 'a, T: AsRef<[S]> + ?Sized> (bytes: &'a T) -> &'a [Rgba8] {
        let bytes = bytes.as_ref();
        let (head, body, tail) = unsafe { bytes.align_to::<Rgba8>() };

        // Panic if the bytes weren't correctly alligned
        if !(head.is_empty() && tail.is_empty()) {
            panic!("Rgba8::align: input is not a valid Rgba8 buffer")
        }
        body
    }
}

/// A BGRA colour with 8-bit colour channels
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Pod, Zeroable)]
pub struct Bgra8 {
    /// Blue
    pub b: u8,
    /// Green
    pub g: u8,
    /// Red
    pub r: u8,
    /// Alpha
    pub a: u8,
}

impl Bgra8 {
    pub const fn new(b: u8, g: u8, r: u8, a: u8) -> Self {
        Bgra8 { b, g, r, a }
    }

    /// Given a byte slice, returns a slice of `Bgra8` values.
    pub fn align<'a, S: 'a, T: AsRef<[S]> + ?Sized>(bytes: &'a T) -> &'a [Self] {
        let bytes = bytes.as_ref();
        let (head, body, tail) = unsafe { bytes.align_to::<Self>() };

        // Panic if the bytes weren't correctly alligned
        if !(head.is_empty() && tail.is_empty()) {
            panic!("Bgra8::align: input is not a valid Rgba8 buffer");
        }
        body
    }
}

/// A RGBA colour represented as a float between 0 and 1
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Rgba {
    /// Red
    pub r: f32,
    /// Green
    pub g: f32,
    /// Blue
    pub b: f32,
    /// Alpha
    pub a: f32,
}

impl Rgba {
    pub const TRANSPARENT: Self = Self::new(0.0, 0.0, 0.0, 0.0);

    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0, 1.0);

    pub const GREEN: Self = Self::new(0.0, 1.0, 0.0, 1.0);

    pub const RED: Self = Self::new(1.0, 0.0, 0.0, 1.0);

    /// Create a new colour with corresponding values between 0 and 1
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r,
            g,
            b,
            a
        }
    }
}

impl From<Bgra8> for Rgba8 {
    fn from(bgra: Bgra8) -> Rgba8 {
        Rgba8 {
            r: bgra.r,
            g: bgra.g,
            b: bgra.b,
            a: bgra.a,
        }
    }
}

impl From<Rgba8> for Rgba {
    fn from(c: Rgba8) -> Self {
        Self {
            r: (c.r as f32 / 255.0),
            b: (c.b as f32 / 255.0),
            g: (c.g as f32 / 255.0),
            a: (c.a as f32 / 255.0),
        }
    }
}

impl From<Rgba> for wgpu::Color {
    fn from(rgba: Rgba) -> wgpu::Color {
        wgpu::Color {
            r: rgba.r as f64,
            g: rgba.g as f64,
            b: rgba.b as f64,
            a: rgba.a as f64,
        }
    }
}