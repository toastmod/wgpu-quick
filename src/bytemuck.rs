pub use bytemuck::*;

#[macro_export]
macro_rules! impl_bytemuck {
    ($dataname:expr) => {
        unsafe impl wgpu_quick::bytemuck::Pod for $dataname {}
        unsafe impl wgpu_quick::bytemuck::Zeroable for $dataname {}
    };
}