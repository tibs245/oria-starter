use std::sync::{Mutex};
use once_cell::sync::Lazy;
use pasetors::keys::{AsymmetricPublicKey, AsymmetricSecretKey};

static PASETO_SECRET_KEY: Lazy<Mutex<Option<AsymmetricSecretKey::<pasetors::version4::V4>>>> = Lazy::new(|| {
    Mutex::new(None)
});

static PASETO_PUBLIC_KEY: Lazy<Mutex<Option<AsymmetricPublicKey::<pasetors::version4::V4>>>> = Lazy::new(|| {
    Mutex::new(None)
});

pub struct AuthSettings;

impl AuthSettings {
    pub fn set_secret_key(secret:  &[u8]) {
        let mut secret_key = PASETO_SECRET_KEY.lock().expect("Cannot lock secret key to write it");

        *secret_key = Some(AsymmetricSecretKey::<pasetors::version4::V4>::from(secret).expect("Cannot create secrete key from secret given"));
    }

    pub fn set_public_key(public: &[u8]) {
        let mut public_key = PASETO_PUBLIC_KEY.lock().expect("Cannot lock public key to write it");

        *public_key = Some(AsymmetricPublicKey::<pasetors::version4::V4>::from(public).expect("Cannot create public key from secret given"));
    }

    pub(crate) fn get_secret_key() -> AsymmetricSecretKey<pasetors::version4::V4> {
        PASETO_SECRET_KEY
            .lock()
            .unwrap()
            .clone()
            .expect("Secret key not configured")
    }

    pub(crate) fn get_public_key() -> AsymmetricPublicKey::<pasetors::version4::V4> {
        PASETO_PUBLIC_KEY
            .lock()
            .unwrap()
            .clone()
            .expect("Public key not configured")
    }
}

#[cfg(test)]
mod auth_settings_module_tests {
    use base64::Engine;
    use base64::engine::general_purpose;
    use super::*;
    
    const FAKE_SECRET_KEY: &'static [u8] = b"y8zar2SZhQoufiUpYSGF94eTzqJ8Q6xo4nFb3TeImqzVX9Bs0xCfK0fpt0g7OcrrQXnTgo2Sz3xBGOoc7ZJ50Q==";
    const FAKE_PUBLIC_KEY: &'static [u8] = b"1V/QbNMQnytH6bdIOznK60F504KNks98QRjqHO2SedE=";
    
    impl AuthSettings {
        pub fn init_fake() {
            AuthSettings::set_secret_key(&general_purpose::STANDARD.decode(FAKE_SECRET_KEY).expect("Unable decode key to init AuthSettings"));
            AuthSettings::set_public_key(&general_purpose::STANDARD.decode(FAKE_PUBLIC_KEY).expect("Unable decode key to init AuthSettings"));
        }
    }

    #[test]
    fn test_set_secret_key() {
        AuthSettings::set_secret_key(&general_purpose::STANDARD.decode(FAKE_SECRET_KEY).expect("Error on parse Base64 secret key"));
        AuthSettings::get_secret_key();
        assert!(PASETO_SECRET_KEY.lock().unwrap().is_some());
    }


    #[test]
    fn test_set_public_key() {
        AuthSettings::set_public_key(&general_purpose::STANDARD.decode(FAKE_PUBLIC_KEY).expect("Error on parse Base64 public key"));
        AuthSettings::get_public_key();
        assert!(PASETO_PUBLIC_KEY.lock().unwrap().is_some());
    }
}


#[cfg(test)]
mod auth_settings_module_tests_error {
    use crate::utils::settings::AuthSettings;

    /// Ignored because context test is shared.
    /// Work on its own only
    /// But create other behavior with cargo test
    #[test]
    #[ignore]
    #[should_panic(expected = "Secret key not configured")]
    fn test_get_secret_key_not_set() {
        AuthSettings::get_secret_key();
    }

    /// Ignored because context test is shared.
    /// Work on its own only
    /// But create other behavior with cargo test
    #[test]
    #[ignore]
    #[should_panic(expected = "Public key not configured")]
    fn test_get_public_key_not_set() {
        AuthSettings::get_public_key();
    }
}