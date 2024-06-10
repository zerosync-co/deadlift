pub use deadlift_macro::modulify;
pub use rmp_serde;

#[repr(C)]
pub struct OutputSlice {
    pub len: usize,
    pub ptr: *mut u8,
}
