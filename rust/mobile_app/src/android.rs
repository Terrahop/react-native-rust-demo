
#[cfg(feature = "jni")]
#[allow(non_snake_case)]
extern crate jni;
extern crate ring;
extern crate base64;

use self::jni::JNIEnv;
use self::jni::objects::{JClass, JString};
use self::jni::sys::jstring;
use self::ring::digest;
use self::base64::encode;

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

#[no_mangle]
pub unsafe extern fn Java_com_demo_MobileAppBridge_sha256(
  env: JNIEnv,
  _: JClass,
  data: JString
) -> jstring {
  let data: String = env.get_string(data).unwrap().into();
  let bytes: &[u8] = data.as_bytes();

  let alg = &digest::SHA256;
  let mut ctx = digest::Context::new(alg);
  ctx.update(&bytes[..]);
  let result = ctx.finish();
  let result_bytes = result.as_ref();
  let encoded = encode(&result_bytes);

  let response = format!("sha256({}): {:?}", data, encoded);
  env.new_string(response).unwrap().into_inner()
}
