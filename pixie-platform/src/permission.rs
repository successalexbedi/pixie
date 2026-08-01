#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    Camera,
    Microphone,
    Notifications,
    Storage,
    Location,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionStatus {
    Granted,
    Denied,
    NotDetermined,
}

/// &mut self because JNIEnv (Android's real implementation) needs
/// mutable access internally to track its local-reference frame —
/// every call into the JVM can push/pop local refs.
pub trait PlatformBridge {
    fn check_permission(&mut self, permission: Permission) -> PermissionStatus;
    fn request_permission(&mut self, permission: Permission) -> PermissionStatus;
}
