// Keystore implementation is temporary :P
// Propably should be written in Kotlin directly and called from Rust

use crate::zkp::UserDocument;
use android_native_keyring_store::Store;
use keyring_core::{set_default_store, Entry, Error};

const SERVICE: &str = "com.agh-eidas.wallet.storage";

const ID_KEYSTORE_NAME: &str = "document";

const TOKEN_KEYSTORE_NAME: &str = "token";
// For checking if token exists without loading it (Probably need to rethink that one)
const TOKEN_EXISTENCE: &str = "token_set";

pub fn init_keyring() -> Result<(), Error> {
    let store = Store::new()?;
    set_default_store(store);
    Ok(())
}

fn create_keystore(name: &str, value: &str) -> Result<(), Error> {
    let cred = Entry::new(SERVICE, name)?;
    cred.set_password(value)?;
    Ok(())
}

fn load_keystore(name: &str) -> Result<String, Error> {
    let cred = Entry::new(SERVICE, name)?;
    cred.get_password()
}

fn delete_keystore(name: &str) -> Result<(), Error> {
    let cred = Entry::new(SERVICE, name)?;
    cred.delete_credential()
}

pub fn get_token() -> Result<String, Box<dyn std::error::Error>> {
    // TO DO: Add token validation

    let data = match load_keystore(TOKEN_KEYSTORE_NAME) {
        Ok(s) => s,
        Err(_) => return Err("Could not load user document".into()),
    };

    Ok(data)
}

pub fn store_token(token: &str) -> Result<(), Box<dyn std::error::Error>> {
    // TO DO: Add token validation
    //

    match load_keystore(TOKEN_KEYSTORE_NAME) {
        Ok(_) => {
            // TO DO: Test if there is need to delete keystore;
            delete_keystore(TOKEN_KEYSTORE_NAME)?;
            create_keystore(TOKEN_KEYSTORE_NAME, token)?;
            create_keystore(TOKEN_EXISTENCE, "true");
        }
        Err(_) => {
            create_keystore(TOKEN_KEYSTORE_NAME, token)?;
            create_keystore(TOKEN_EXISTENCE, "true");
        }
    }

    Ok(())
}

pub fn delete_token() -> Result<(), Box<dyn std::error::Error>> {
    let _ = delete_keystore(TOKEN_KEYSTORE_NAME);
    let _ = delete_keystore(TOKEN_EXISTENCE);
    Ok(())
}

pub fn check_token_existence() -> bool {
    match load_keystore(TOKEN_EXISTENCE) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn get_user_document() -> Result<UserDocument, Box<dyn std::error::Error>> {
    let data = match load_keystore(ID_KEYSTORE_NAME) {
        Ok(s) => s,
        Err(_) => return Err("Could not load user document".into()),
    };

    let doc = serde_json::from_str(&data)?;

    Ok(doc)
}

pub fn store_user_document(document: UserDocument) -> Result<(), Box<dyn std::error::Error>> {
    //TO DO: Probably it should be rebuild but it is what it is for the time being

    let data = match serde_json::to_string(&document) {
        Ok(s) => s,
        Err(_) => {
            return Err("Failed to serialize document".into());
        }
    };

    match load_keystore(ID_KEYSTORE_NAME) {
        Ok(_) => {
            delete_keystore(ID_KEYSTORE_NAME)?;
            create_keystore(ID_KEYSTORE_NAME, &data)?;
        }
        Err(_) => {
            create_keystore(ID_KEYSTORE_NAME, &data)?;
        }
    }

    Ok(())
}

pub fn delete_user_document() -> Result<(), Box<dyn std::error::Error>> {
    let _ = delete_keystore(ID_KEYSTORE_NAME);
    Ok(())
}
