#![allow(warnings)]
//!
//! This is the documentation for 'Util' module of Komodo.
//!
//! The 'Util' module of Komodo contains functionality of the 'Util' noted on the
//! [Komodo website].
//!
//! # Remarks
//!
//! * A valid KomodoRPC object type must be passed in.
//!
//! * All examples for each method assumes a valid KomodoRPC object, named `some_user`, is used.
//!
//! * Use of any methods requires the following modules:
//! ```
//! mod komodorpcutil;
//! mod komodo;
//! use komodorpcutil::KomodoRPC;
//! ```
//!
//! [Komodo website]: https://docs.komodoplatform.com/basic-docs/smart-chains/smart-chain-api/util.html
//!

use super::komodorpcutil;
//use komodorpcutil::KomodoRPC;

// TODO: - run fmt and clippy
//       - document all methods
//          - more advanced examples
//       - ? potential fix for no function/method overloading if extra time
//              -> trait object static/dynamic dispatch for multi-type parameter
//       - ? some_user parameter may be simplified further

///
/// The create_multisig method creates a multi-signature address with n signature(s) of m key(s) required. The method returns a json object with the address and redeemScript.
///
/// # Arguments
///
/// * `number_required` - A required u32 that represents the number of required signatures out of the n key(s) or address(es).
/// * `keys` - A required string that represents a json array of keys which are addresses or hex-encoded public keys.
///
/// # Response
///
/// * `address` - The value of the new multisig address.
/// * `redeemScript` - The string value of the hex-encoded redemption script.
///
/// # Examples
/// ```
/// TODO: Update examples
/// let result = komodo::cross_chain::create_multisig(some_user,
///                                                                    "string".to_string(),
///                                                                    number_u32,
///                                                                    None);
///
/// ```
/// %%%
pub fn create_multisig(
    //THE DEVELOPERS.KOMODOPLATFORM.COM DOESN'T PROVIDE AN EXAMPLE TO IMPLEMENT THE PARAMETERS FOR THE API
    some_user: komodorpcutil::KomodoRPC,
    number_required: u32,
    keys: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("createmultisig");

    let method_body: String = String::from("[")
        + &number_required.to_string()
        + &String::from(",[\"")
        + &keys.to_string()
        + &String::from("\"]]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

///
/// The decodeccopret method decodes the OP RETURN data from a CC transaction to output the EVALCODE and function id of the method that produced the transaction.
///
/// # Arguments
///
/// * `scriptPubKey` - (string) 	the hex-string format scriptPubKey of the type : nulldata in the vout of a transaction produced by a CC module
///
/// # Response
///
/// * `result` 	(string) 	whether the call succeeded
/// * `OpRets` 	(json) 	a json containing the keys EVALCODE and function id
/// * `eval_code` 	(hexadecimal number) 	the EVALCODE of the method that produced the transaction
/// * `function` 	(string) 	the function id of the method that produced the transaction
///
/// %%%
pub fn decode_ccopret(
    some_user: komodorpcutil::KomodoRPC,
    script_pub_key: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("decodeccopret");

    let method_body: String =
        String::from("[\"") + &script_pub_key.to_string() + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

///
/// The estimatefee method estimates the approximate fee per kilobyte. The method is needed for a transaction to begin confirmation within nblocks blocks.
/// The value -1.0 is returned if not enough transactions and blocks have been observed to make an estimate.
///
/// # Arguments
///
/// * `nblocks` 	(numeric) 	the number of blocks within which the fee should be tested
///
/// # Response
///
/// * `n` 	(numeric) 	the estimated fee
/// %%%
pub fn estimate_fee(
    some_user: komodorpcutil::KomodoRPC,
    n_blocks: u32,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("estimatefee");

    let method_body: String = String::from("[") + &n_blocks.to_string() + &String::from("]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

///
/// The estimatepriority method estimates the approximate priority of a zero-fee transaction, when it needs to begin confirmation within nblocks blocks.
/// The value -1.0 is returned if not enough transactions and blocks have been observed to make an estimate.
///
/// # Arguments
///
/// * `nblocks` 	(numeric) 	a statement indicating within how many blocks the transaction should be confirmed
///
/// # Response
///
/// * `n` 	(numeric) 	the estimated priority
/// %%%
pub fn estimate_priority(
    some_user: komodorpcutil::KomodoRPC,
    n_blocks: u32,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("estimatepriority");

    let method_body: String = String::from("[") + &n_blocks.to_string() + &String::from("]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

///
/// The invalidateblock method permanently marks a block as invalid, as if it violated a consensus rule.
///
/// # Arguments
///
/// * `hash` 	(string, required) 	the hash of the block to mark as invalid
///
/// # Response
///
/// * No response
/// %%%
pub fn invalidate_block(
    some_user: komodorpcutil::KomodoRPC,
    hash: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("invalidateblock");

    let method_body: String = String::from("[\"") + &hash.to_string() + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

///
/// The reconsiderblock method removes invalidity status of a block and its descendants, reconsidering them for activation. This can be used to undo the effects of the invalidateblock method.
///
/// # Arguments
///
/// * `hash` 	(string, required) 	the hash of the block to reconsider
///
/// # Response
///
/// * No response
/// %%%
pub fn reconsider_block(
    some_user: komodorpcutil::KomodoRPC,
    hash: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("reconsiderblock");

    let method_body: String = String::from("[\"") + &hash.to_string() + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

///
/// The txnotarizedconfirmed method returns information about a transaction's state of confirmation.
/// If the transaction is on a chain that has Komodo's dPoW security service, the method returns true if the transaction is notarized.
/// If the chain does not have dPoW, the method returned true if the confirmation number is greater than 60.
///
/// # Arguments
///
/// * `txid` 	(string, required) 	the transaction id
///
/// # Response
///
/// * `result` 	(boolean) 	whether the transaction is confirmed, for dPoW-based chains; for non-dPoW chains, the value indicates whether the transaction has 60 or more confirmations
/// %%%
pub fn tx_notarized_confirmed(
    some_user: komodorpcutil::KomodoRPC,
    tx_id: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("txnotarizedconfirmed");

    let method_body: String = String::from("[\"") + &tx_id.to_string() + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

///
/// The validateaddress method returns information about the given address.
///
/// # Arguments
///
/// * `address` 	(string, required) 	the address to validate
///
/// # Response
///
/// * `isvalid` 	(boolean) 	indicates whether the address is valid. If it is not, this is the only property returned.
/// * `address` 	(string) 	the address validated
/// * `scriptPubKey` 	(string) 	the hex encoded scriptPubKey generated by the address
/// * `ismine` 	(boolean) 	indicates whether the address is yours
/// * `isscript` 	(boolean) 	whether the key is a script
/// * `pubkey` 	(string) 	the hex value of the raw public key
/// * `iscompressed` 	(boolean) 	whether the address is compressed
/// * `account` 	(string) 	DEPRECATED the account associated with the address; "" is the default account
/// %%%
pub fn validate_address(
    some_user: komodorpcutil::KomodoRPC,
    address: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("validateaddress");

    let method_body: String = String::from("[\"") + &address.to_string() + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

///
/// The verifymessage method verifies a signed message.
///
/// # Arguments
///
/// * `address` 	(string, required) 	the address to use for the signature
/// * `signature` 	(string, required) 	the signature provided by the signer in base 64 encoding
/// * `message` 	(string, required) 	the message that was signed
///
/// # Response
///
/// * `true` or `false` 	(boolean) 	indicates whether the signature is verified
/// %%%
pub fn verify_message(
    some_user: komodorpcutil::KomodoRPC,
    address: String,
    signature: String,
    message: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("verifymessage");

    let method_body: String = String::from("[\"")
        + &address.to_string()
        + &String::from("\",\"")
        + &signature.to_string()
        + &String::from("\",\"")
        + &message.to_string()
        + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

///
/// The z_validateaddress method returns information about the given z address.
///
/// # Arguments
///
/// * `zaddr` 	(string, required) 	the z address to validate
///
/// # Response
///
/// * `isvalid` 	(boolean) 	indicates whether the address is valid; if not, this is the only property returned
/// * `address` 	(string) 	the z address validated
/// * `ismine` 	(boolean) 	indicates if the address is yours or not
/// * `payingkey` 	(string) 	the hex value of the paying key, a_pk
/// * `transmissionkey` 	(string) 	the hex value of the transmission key, pk_enc
/// %%%
pub fn z_validate_address(
    some_user: komodorpcutil::KomodoRPC,
    z_addr: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("z_validateaddress");

    let method_body: String = String::from("[\"") + &z_addr.to_string() + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}
