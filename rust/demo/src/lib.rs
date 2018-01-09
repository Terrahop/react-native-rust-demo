#[cfg(feature = "jni")]
#[allow(non_snake_case)]
pub mod android {
extern crate jni;

use self::jni::JNIEnv;
use self::jni::objects::{JClass, JString};
use self::jni::sys::jstring;

#[no_mangle]
pub unsafe extern fn Java_com_demo_MobileAppBridge_helloWorld(
  env: JNIEnv,
  _: JClass,
  name: JString
) -> jstring {
  let name: String = env.get_string(name).unwrap().into();
  let response = format!("Hello {} from Rust!", name);
  env.new_string(response).unwrap().into_inner()
}
}
