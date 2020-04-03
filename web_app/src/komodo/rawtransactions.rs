#![allow(warnings)]
//!
//! This is the documentation for 'Raw Transactions' module of Komodo.
//!
//! The 'Raw Transactions' module of Komodo contains functionality of the 'Raw Transactions' noted on the
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
//! [Komodo website]: https://docs.komodoplatform.com/basic-docs/smart-chains/smart-chain-api/rawtransactions.html
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
/// The createrawtransaction method creates a transaction, spending the given inputs and sending to the given addresses. The method returns a hex-encoded raw transaction.
///
/// # Note
/// * This is a raw transaction, and therefore the inputs are not signed and the transaction is not stored in the wallet nor transmitted to the network.
///
/// # Arguments
///
/// * `transactions` 	(string, required) 	a json array of json objects
/// * `txid` 	(string, required) 	the transaction id
/// * `vout` 	(numeric, required) 	the output number
/// * `addresses` 	(string, required) 	a json object with addresses as keys and amounts as values
/// * `address` 	(numeric, required) 	the key is the address, the value is the COIN amount
///
/// # Response
///
/// * `transaction` 	(string) 	a hex string of the transaction
/// %%%
pub fn create_raw_transaction(
    // THE DOCUMENTATION IN DOCS.KOMODOPLATFORM.COM DOESN'T GIVE A VALID EXAMPLE FOR USING THE PARAMETERS
    some_user: komodorpcutil::KomodoRPC,
    transactions: String,
    tx_id: String,
    vout: u32,
    addresses: String,
    address: u32,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("createrawtransaction");
    let method_body: String;
    method_body = String::from("[[{\"txid\":\"")
        + &transactions.to_string()
        + &String::from("\",\"vout\":")
        + &vout.to_string()
        + &String::from("}], {\"")
        + &addresses.to_string()
        + &String::from("\":")
        + &address.to_string()
        + &String::from("} ]");
    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

///
/// The decoderawtransaction method returns a json object representing the serialized, hex-encoded transaction.
///
/// # Arguments
///
/// * `hex` 	(string, required) 	the transaction hex string
///
/// # Response
///
/// * `txid` 	(string) 	the transaction id
/// * `overwintered` 	(boolean) 	the overwintered flag
/// * `version` 	(numeric) 	the version
/// * `versiongroupid` 	(string, optional) 	the version group id (overwintered txs)
/// * `locktime` 	(numeric) 	the lock time
/// * `expiryheight` 	(numeric, optional) 	last valid block height for mining transaction (overwintered txs)
/// * `vin` : [ ... ] 	(array of json objects)
/// * `txid` 	(string) 	the transaction id
/// * `vout` : [ ... ] 	(numeric) 	the output number
/// * `scriptSig` 	(json object) 	the script
/// * `asm` 	(string) 	asm
/// * `hex` 	(string) 	hex
/// * `sequence` 	(numeric) 	the script sequence number
/// * `vout` 	(array of json objects)
/// * `value` 	(numeric) 	the value
/// * `number` 	(numeric) 	index
/// * `scriptPubKey` 	(json object)
/// * `asm` 	(string) 	the asm
/// * `hex` 	(string) 	the hex
/// * `reqSigs` 	(numeric) 	the required sigs
/// * `type` 	(string) 	the type, eg 'pubkeyhash'
/// * `addresses`
/// * `address` 	(string) 	the address
/// * `vjoinsplit` : [ ... ] 	(array of json objects, only for version >= 2)
/// * `vpub_old` 	(numeric) 	public input value
/// * `vpub_new` 	(numeric) 	public output value
/// * `anchor` 	(string) 	the anchor
/// * `nullifiers` : [ ... ] 	(array of strings)
/// * `hex` 	(string) 	input note nullifier
/// * `commitments` : [ ... ] 	(array of strings)
/// * `hex` 	(string) 	output note commitment
/// * `onetimePubKey` 	(string) 	the onetime public key used to encrypt the ciphertexts
/// * `randomSeed` 	(string) 	the random seed
/// * `macs` : [ ... ] 	(array of strings)
/// * `hex` 	(string) 	input note MAC
/// * `proof` 	(string) 	the zero-knowledge proof
/// * `ciphertexts` : [ ... ] 	(array of strings)
/// * `hex` 	(string) 	output note ciphertext
///
/// %%%
pub fn decode_raw_transaction(
    some_user: komodorpcutil::KomodoRPC,
    hex: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("decoderawtransaction");
    let method_body: String;
    method_body = String::from("[\"") + &hex.to_string() + &String::from("\"]");
    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

///
/// The decodescript method decodes a hex-encoded script.
///
/// # Arguments
///
/// * `transactions` 	(string, required) 	a json array of json objects
/// * `txid` 	(string, required) 	the transaction id
/// * `vout` 	(numeric, required) 	the output number
/// * `addresses` 	(string, required) 	a json object with addresses as keys and amounts as values
/// * `address` 	(numeric, required) 	the key is the address, the value is the COIN amount
///
/// # Response
///
/// * `asm` 	(string) 	the script public key
/// * `hex` 	(string) 	the hex-encoded public key
/// * `type` 	(string) 	the output type
/// * `reqSigs` 	(numeric) 	the required signatures
/// * `addresses`: [ ... ] 	(array of strings)
/// * `address` 	(string) 	the address
/// * `p2sh` 	(string) 	the script address
/// %%%
pub fn decode_script(
    some_user: komodorpcutil::KomodoRPC,
    hex: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("decodescript");
    let method_body: String;
    method_body = String::from("[\"") + &hex.to_string() + &String::from("\"]");
    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

///
/// The fundrawtransaction method adds inputs to a transaction until it has enough in value to meet its out value. This will not modify existing inputs, and will add one change output to the outputs.
///
/// # Arguments
///
/// * `hexstring` 	(string, required) 	the hex string of the raw transaction
///
/// # Response
///
/// * `hex` 	(string) 	the resulting raw transaction (hex-encoded string)
/// * `fee` 	(numeric) 	the fee added to the transaction
/// * `changepos` 	(numeric) 	the position of the added change output, or -1
/// %%%
pub fn fund_raw_transaction(
    some_user: komodorpcutil::KomodoRPC,
    hexstring: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("fundrawtransaction");
    let method_body: String;
    method_body = String::from("[\"") + &hexstring.to_string() + &String::from("\"]");
    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

///
/// The getrawtransaction method returns the raw transaction data.
///
/// If verbose=0, the method returns a string that is serialized, hex-encoded data for transaction_id. If verbose is non-zero, the method returns an object with information about transaction_id.
///
/// # Arguments
///
/// * `txid` 	(string, required) 	the transaction id
/// * `verbose` 	(numeric, optional, default=0) 	if 0, the method returns a string in hex; otherwise, it returns a json object
///
/// # Response
///
/// * `data` 	(string) 	the serialized, hex-encoded data for 'txid'
/// %%%
pub fn get_raw_transaction(
    some_user: komodorpcutil::KomodoRPC,
    txid: String,
    verbose_supplied: Option<u32>,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("getrawtransaction");
    let method_body: String;
    let verbose = verbose_supplied.unwrap_or(0);
    method_body = String::from("[\"")
        + &txid.to_string()
        + &String::from("\", ")
        + &verbose.to_string()
        + &String::from("]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

///
/// The sendrawtransction method submits raw transaction (serialized, hex-encoded) to local nodes and the network.
///
/// # Arguments
///
/// * `hexstring` 	(string, required) 	the hex string of the raw transaction
/// * `allowhighfees` 	(boolean, optional, default=false) 	whether to allow high fees
///
/// # Response
///
/// * `hex` 	(string) 	the transaction hash in hex
/// %%%
pub fn send_raw_transaction(
    some_user: komodorpcutil::KomodoRPC,
    hexstring: String,
    allow_high_fees_supplied: Option<bool>,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("sendrawtransaction");
    let method_body: String;
    let allow_high_fees = allow_high_fees_supplied.unwrap_or(false);
    method_body = String::from("[\"")
        + &hexstring.to_string()
        + &String::from("\",")
        + &allow_high_fees.to_string()
        + &String::from("]");
    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

///
/// The signrawtransaction method signs inputs for a raw transaction (serialized, hex-encoded). The second optional argument (may be null) is an array of previous transaction outputs that this transaction depends on, but may not yet be in the block chain. The third optional argument (may be null) is an array of base58-encoded private keys that, if given, will be the only keys used to sign the transaction.
///
/// # Arguments
///
/// * `hexstring` 	(string, required) 	the transaction hex string
/// * `prevtxs` 	(string, optional) 	a json array of previous dependent transaction outputs
/// * `txid` 	(string, required) 	the transaction id
/// * `vout` 	(numeric, required) 	the output number
/// * `scriptPubKey` 	(string, required) 	the script key
/// * `redeemScript` 	(string, required for P2SH) 	redeem script
/// * `amount` 	(numeric, required) 	the amount spent
/// * `privatekeys` 	(string, optional) 	a json array of base58-encoded private keys for signing
/// * `privatekey` 	(string) 	the private key in base58-encoding
/// * `sighashtype` 	(string, optional, default=ALL) 	the signature hash type; the following options are available: "ALL"
///
/// # Response
///
/// * `hex` 	(string) 	the hex-encoded raw transaction with signature(s)
/// * `complete` 	(boolean) 	whether the transaction has a complete set of signatures
/// * `errors`
/// * `txid` 	(string) 	the hash of the referenced, previous transaction
/// * `vout` 	(numeric) 	the index of the output to spend and used as input
/// * `scriptSig` 	(string) 	the hex-encoded signature script
/// * `sequence` 	(numeric) 	the script sequence number
/// * `error` 	(string) 	verification or signing error related to the input
/// %%%
// TODO: Other Komodo APIs have only 1 argument, site specifies 10 arguments
pub fn sign_raw_transaction(
    some_user: komodorpcutil::KomodoRPC,
    hexstring: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("signrawtransaction");
    let method_body: String;
    //let allow_high_fees = allow_high_fees_supplied.unwrap_or(false);
    method_body = String::from("[\"") + &hexstring.to_string() + &String::from("\"]");
    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}
