pub mod permission;
pub mod desktop;
pub mod clipboard;
pub mod haptics;
pub mod sensors;
pub mod notifications;
pub mod file_picker;
pub mod share_sheet;
pub mod billing;

#[cfg(target_os = "android")]
pub mod android;

pub use permission::{Permission, PermissionStatus, PlatformBridge};
pub use desktop::DesktopBridge;

#[cfg(target_os = "android")]
pub use android::AndroidBridge;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desktop_bridge_grants_by_default() {
        let mut bridge = DesktopBridge;
        assert_eq!(bridge.check_permission(Permission::Camera), PermissionStatus::Granted);
        assert_eq!(bridge.request_permission(Permission::Microphone), PermissionStatus::Granted);
    }

    #[test]
    fn permission_enum_values_are_distinct() {
        assert_ne!(Permission::Camera, Permission::Microphone);
    }
}
