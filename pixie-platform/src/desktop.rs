use crate::permission::{Permission, PermissionStatus, PlatformBridge};

pub struct DesktopBridge;

impl PlatformBridge for DesktopBridge {
    fn check_permission(&mut self, _permission: Permission) -> PermissionStatus {
        PermissionStatus::Granted
    }
    fn request_permission(&mut self, _permission: Permission) -> PermissionStatus {
        PermissionStatus::Granted
    }
}
