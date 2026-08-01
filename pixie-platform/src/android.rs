#![cfg(target_os = "android")]

use crate::permission::{Permission, PermissionStatus, PlatformBridge};
use jni::JNIEnv;
use jni::objects::{JObject, JValue};

pub struct AndroidBridge<'a> {
    pub env: JNIEnv<'a>,
    pub activity: JObject<'a>,
}

fn permission_manifest_string(permission: Permission) -> &'static str {
    match permission {
        Permission::Camera => "android.permission.CAMERA",
        Permission::Microphone => "android.permission.RECORD_AUDIO",
        Permission::Notifications => "android.permission.POST_NOTIFICATIONS",
        Permission::Storage => "android.permission.READ_EXTERNAL_STORAGE",
        Permission::Location => "android.permission.ACCESS_FINE_LOCATION",
    }
}

impl<'a> PlatformBridge for AndroidBridge<'a> {
    fn check_permission(&mut self, permission: Permission) -> PermissionStatus {
        let perm_str = permission_manifest_string(permission);
        let jperm = self.env.new_string(perm_str).expect("failed to create JString");
        let result = self.env.call_method(
            &self.activity,
            "checkSelfPermission",
            "(Ljava/lang/String;)I",
            &[JValue::Object(&jperm)],
        );
        match result.and_then(|v| v.i()) {
            Ok(0) => PermissionStatus::Granted,
            Ok(_) => PermissionStatus::Denied,
            Err(_) => PermissionStatus::NotDetermined,
        }
    }

    fn request_permission(&mut self, permission: Permission) -> PermissionStatus {
        let already = self.check_permission(permission);
        if already == PermissionStatus::Granted {
            return already;
        }
        let perm_str = permission_manifest_string(permission);
        let jperm = self.env.new_string(perm_str).expect("failed to create JString");
        let arr = self.env.new_object_array(1, "java/lang/String", &jperm)
            .expect("failed to build permission array");
        let _ = self.env.call_method(
            &self.activity,
            "requestPermissions",
            "([Ljava/lang/String;I)V",
            &[JValue::Object(&arr), JValue::Int(0)],
        );
        PermissionStatus::NotDetermined
    }
}
