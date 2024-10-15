#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos", target_os = "android"))]
pub struct File{
    pub handle: std::fs::File,
}