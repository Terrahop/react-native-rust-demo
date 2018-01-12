
#[cfg(feature = "jni")]
#[allow(non_snake_case)]
extern crate jni;
extern crate ring;
extern crate base64;
extern crate untrusted;

use self::jni::JNIEnv;
use self::jni::objects::{JClass,JString};
use self::jni::sys::{jstring,jboolean};
use self::ring::{rand,signature,digest};
use self::ring::signature::{VerificationAlgorithm};
use self::base64::{encode,decode};

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

#[no_mangle]
pub unsafe extern fn Java_com_demo_MobileAppBridge_ed25519GeneratePrivateKey(
  env: JNIEnv,
  _: JClass
) -> jstring {

  // Generate a key pair in PKCS#8 (v2) format.
  let rng = rand::SystemRandom::new(); //ceate new random number variable
  let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).expect("Failed to generate key");
  let encoded = encode(&pkcs8_bytes.as_ref());
  env.new_string(encoded).unwrap().into_inner()

}

#[no_mangle]
pub unsafe extern fn Java_com_demo_MobileAppBridge_ed25519GetPublicKey(
  env: JNIEnv,
  _: JClass,
  key: JString
) -> jstring {

  let key: String = env.get_string(key).unwrap().into();
  let pkcs8_bytes = decode(&key).expect("Invalid Base64");
  let keypair = signature::Ed25519KeyPair::from_pkcs8(untrusted::Input::from(&pkcs8_bytes.as_ref())).expect("Failed to parse pkcs8");
  let public_bytes = keypair.public_key_bytes();
  let encoded = encode(&public_bytes);
  env.new_string(encoded).unwrap().into_inner()
}

#[no_mangle]
pub unsafe extern fn Java_com_demo_MobileAppBridge_ed25519Sign(
  env: JNIEnv,
  _: JClass,
  key: JString,
  data: JString
) -> jstring  {

  let key: String = env.get_string(key).unwrap().into();
  let data: String = env.get_string(data).unwrap().into();;
  let pkcs8_bytes = decode(&key).expect("Invalid Base64");
  let decoded_bytes = decode(&data).expect("Invalid Base64");
  let keypair = signature::Ed25519KeyPair::from_pkcs8(untrusted::Input::from(&pkcs8_bytes.as_ref())).expect("Failed to parse pkcs8");
  let signature = keypair.sign(&decoded_bytes);
  let encoded = encode(&signature);
  env.new_string(encoded).unwrap().into_inner()

}

#[no_mangle]
pub unsafe extern fn Java_com_demo_MobileAppBridge_ed25519Verify(
  env: JNIEnv,
  _: JClass,
  key: JString,
  msg: JString,
  sig: JString
) -> jstring {

  let key: String = env.get_string(key).unwrap().into();
  let msg: String = env.get_string(msg).unwrap().into();
  let sig: String = env.get_string(sig).unwrap().into();

  let key_decoded = decode(&key).expect("Invalid Base64");
  let msg_decoded = decode(&msg).expect("Invalid Base64");
  let sig_decoded = decode(&sig).expect("Invalid Base64");

  let eddsa = signature::EdDSAParameters;
  let verified = eddsa.verify(
    untrusted::Input::from(&key_decoded),
    untrusted::Input::from(&msg_decoded),
    untrusted::Input::from(&sig_decoded)
  );

  let output = match verified {
    Ok(_) => 0x01u8,
    Err(_) => 0x00u8,
  };
  let response = format!("{:?}", output);
  env.new_string(response).unwrap().into_inner()
}
