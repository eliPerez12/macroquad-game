

// Checks what type of os is running the game

#[cfg(target_os = "windows")]
pub fn is_windows() -> bool { true }

#[cfg(target_os = "linux")]
pub fn is_windows() -> bool { false }

