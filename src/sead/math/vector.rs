#[repr(C, packed(4))]
pub struct VectorF<const L: usize> {
    pub value: [f32; L],
}
