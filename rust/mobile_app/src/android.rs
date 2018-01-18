
#[cfg(feature = "jni")]
#[allow(non_snake_case)]
extern crate jni;
extern crate ring;
extern crate base64;
extern crate untrusted;

use self::jni::JNIEnv;
use self::jni::objects::{JClass,JString};
use self::jni::sys::{jstring, jboolean};
use self::ring::{rand,digest};
use self::ring::signature::{VerificationAlgorithm, Ed25519KeyPair, EdDSAParameters};
use self::base64::{encode,decode};
use self::untrusted::Input;
use mdns::start_mdns;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern fn Java_com_demo_MobileAppBridge_start_1mdns(
  env: JNIEnv,
  _: JClass,
) -> jstring {
  start_mdns();
  let response = String::from("Started mdns");
  env.new_string(response).unwrap().into_inner()
}

#[no_mangle]
#[allow(non_snake_case)]
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
#[allow(non_snake_case)]
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
  env.new_string(encoded).unwrap().into_inner()
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern fn Java_com_demo_MobileAppBridge_ed25519GeneratePrivateKey(
  env: JNIEnv,
  _: JClass
) -> jstring {

  // Generate a key pair in PKCS#8 (v2) format.
  let rng = rand::SystemRandom::new(); //ceate new random number variable

  let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).expect("Failed to generate key");

  let encoded = encode(&pkcs8_bytes.as_ref());
  env.new_string(encoded).unwrap().into_inner()
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern fn Java_com_demo_MobileAppBridge_ed25519GetPublicKey(
  env: JNIEnv,
  _: JClass,
  key: JString
) -> jstring {

  let key: String = env.get_string(key).unwrap().into();
  let pkcs8_bytes = decode(&key).expect("Invalid Base64");

  let keypair = Ed25519KeyPair::from_pkcs8(Input::from(&pkcs8_bytes.as_ref())).expect("Failed to parse pkcs8");
  let public_bytes = keypair.public_key_bytes();

  let encoded = encode(&public_bytes);
  env.new_string(encoded).unwrap().into_inner()
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern fn Java_com_demo_MobileAppBridge_ed25519Sign(
  env: JNIEnv,
  _: JClass,
  key: JString,
  data: JString
) -> jstring  {
  let key: String = env.get_string(key).unwrap().into();
  let data: String = env.get_string(data).unwrap().into();
  let pkcs8_bytes = decode(&key).expect("Invalid Base64");
  let decoded_bytes = decode(&data).expect("Invalid Base64");

  let keypair = Ed25519KeyPair::from_pkcs8(Input::from(&pkcs8_bytes.as_ref())).expect("Failed to parse pkcs8");
  let signature = keypair.sign(&decoded_bytes.as_ref());

  let encoded = encode(signature.as_ref());
  env.new_string(encoded).unwrap().into_inner()
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern fn Java_com_demo_MobileAppBridge_ed25519Verify(
  env: JNIEnv,
  _: JClass,
  key: JString,
  msg: JString,
  sig: JString
) -> jboolean {

  let key: String = env.get_string(key).unwrap().into();
  let msg: String = env.get_string(msg).unwrap().into();
  let sig: String = env.get_string(sig).unwrap().into();

  let key_decoded = decode(&key).expect("Invalid Base64");
  let msg_decoded = decode(&msg).expect("Invalid Base64");
  let sig_decoded = decode(&sig).expect("Invalid Base64");

  let eddsa = EdDSAParameters;
  let verified = eddsa.verify(
    Input::from(&key_decoded.as_ref()),
    Input::from(&msg_decoded.as_ref()),
    Input::from(&sig_decoded.as_ref())
  );

  match verified {
    Ok(_) => 0x01u8,
    Err(_) => 0x00u8,
  }
}
