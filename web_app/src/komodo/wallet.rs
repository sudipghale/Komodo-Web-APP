#![warn(missing_docs)]
//!
//! This is the documentation for 'Wallet' module of Komodo.
//!
//! The 'Wallet' module of Komodo contains functionality of the 'Wallet' noted on the
//! [Komodo website].
//!
//! # Remarks
//!
//! * ? Some documentation of methods may reference a different method.
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
//! [Komodo website]: https://docs.komodoplatform.com/basic-docs/smart-chains/smart-chain-api/wallet.html
//!

// TODO: - run fmt and clippy
//       - document all methods
//          - more advanced examples
//       - ? potential fix for no function/method overloading if extra time
//              -> trait object static/dynamic dispatch for multi-type parameter
//       - ? some_user parameter may be simplified further

use super::komodorpcutil;
//use komodorpcutil::KomodoRPC;

// The move method in Wallet module has been deprecated.
//addmultisigaddress has been DEPRECATED

/// The backupwallet method safely copies the wallet.dat file to the indicated destination. The destination input accepts only alphanumeric characters.
/// This method requires that the coin daemon have the exportdir runtime parameter enabled.
/// # Arguments
/// * `destination` -	(string, required)	the destination filename, saved in the directory set by the exportdir runtime parameter
/// # Response
/// * `path`	(string)	the full path of the destination file
/// %%%
pub fn backup_wallet(
    someUser: komodorpcutil::KomodoRPC,
    destination: String,
) -> Result<String, reqwest::Error> {
    let method_body = String::from("[\"") + &destination + "\"]";
    let method_name: String = String::from("backupwallet");
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(someUser.clone(), data)
}

/// The dumpprivkey method reveals the private key corresponding to the indicated address.
/// See also importprivkey.
/// # Arguments
/// * `address` 	(string, required) 	the address for the private key
/// # Response
/// * `data` 	(string) 	the private key
/// %%%
pub fn dump_priv_key(
    someUser: komodorpcutil::KomodoRPC,
    address: String,
) -> Result<String, reqwest::Error> {
    let method_body = String::from("[\"") + &address + "\"]";
    let method_name: String = String::from("dumpprivkey");
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(someUser.clone(), data)
}

/// The dumpwallet method dumps all transparent-address wallet keys into a file, using a human-readable format.
/// Overwriting an existing file is not permitted. The destination parameter accepts only alphanumeric characters.
/// This method requires that the coin daemon have the exportdir runtime parameter enabled.
/// # Arguments
/// * `filename`	(string, required)	the filename, saved in the folder set by the exportdir runtime parameter
/// # Response
/// * `path`	(string)	the full path of the destination file
/// %%%
pub fn dump_wallet(
    someUser: komodorpcutil::KomodoRPC,
    filename: String,
) -> Result<String, reqwest::Error> {
    let method_body = String::from("[\"") + &filename + "\"]";
    let method_name: String = String::from("dumpwallet");
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(someUser.clone(), data)
}

/// Using the encryptwallet method will shutdown the Komodo daemon (komodod).
/// This feature is available only on chains where -ac_public is enabled. Chains that feature private transactions cannot use this feature.
/// The encryptwallet method encrypts the wallet with the indicated passphrase.
/// For more information, please see these instructions: Encrypt Komodo's wallet.dat File
/// This method is for first-time encryption only. After the first encryption, any calls that interact with private keys will require the passphrase via walletpassphrase prior to calling the corresponding method. This includes methods that create a transaction, dump a private key for an address, sign a transaction, etc.
/// # Arguments
/// * `passphrase`	(string)	the passphrase for wallet encryption; the passphrase must be at least 1 character, but should be many
/// # Response
/// * Text Response -
/// wallet encrypted; Komodo server stopping, restart to run with encrypted wallet. The keypool has been flushed, you need to make a new backup
/// %%%
pub fn encrypt_wallet(
    someUser: komodorpcutil::KomodoRPC,
    passphrase: String,
) -> Result<String, reqwest::Error> {
    let method_body = String::from("[\"") + &passphrase + "\"]";
    let method_name: String = String::from("encryptwallet");
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(someUser.clone(), data)
}

/// The getaccount method returns the account associated with the given address.
/// # Arguments
/// * `address`	(string, required)	the address
/// # Response
/// * `accountname`	(string)	the account address
/// %%%
pub fn get_account(
    someUser: komodorpcutil::KomodoRPC,
    address: String,
) -> Result<String, reqwest::Error> {
    let method_body = String::from("[\"") + &address + "\"]";
    let method_name: String = String::from("getaccount");
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(someUser.clone(), data)
}

/// getbalance (`account" minconf includeWatchonly ).
/// The getbalance method returns the server's total available balance.
/// The account input is deprecated.
/// # Arguments
/// * `account`	(string, optional)	DEPRECATED if provided, it MUST be set to the empty string "" or to the string "*"
/// minconf	(numeric, optional, default=1)	only include transactions confirmed at least this many times
/// includeWatchonly	(bool, optional, default=false)	also include balance in watchonly addresses (see importaddress)
/// # Response
/// * amount	(numeric)	the total amount

pub fn get_balance(
    //TODO check def value and if conditions
    SomeUser: komodorpcutil::KomodoRPC,
    minconf: Option<u32>,
    includeWatchonly: Option<bool>,
) -> Result<String, reqwest::Error> {
    let method_body: String;
    let temp_minconf = minconf.unwrap_or(1); //Default value is 1
    let temp_includeWatchonly: String = includeWatchonly.unwrap_or(false).to_string();
    method_body = String::from("[\"\",")
        + &temp_minconf.to_string()
        + &",".to_string()
        + &temp_includeWatchonly
        + &String::from("]");

    let method_name: String = String::from("getbalance");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

/*
getbalance64
This method is part of the new ac_staked functionality.// TO DO DIDNT IMPLEMENTED NOT ENOUGH INFO....
The getbalance64 method is used only on Smart Chains that are utilizing the ac_staked functionality.
 On KMD-based Proof-of-Stake (PoS) Smart Chains, all staked coins are placed into one of 64 segments (segid's').
 The getbalance64 method returns the balance of coins in each segid.
*/

/// getnewaddress ( "account" ).
/// The getnewaddress method returns a new address for receiving payments.
/// # Arguments
/// Name	Type	Description
/// * "account"	(string, optional)	DEPRECATED: If provided, the account MUST be set to the empty string "" to represent the default account; passing any other string will result in an error
/// # Response
/// Name	Type	Description
/// * "address"	(string)	the new address
/// %%%
pub fn get_new_address(SomeUser: komodorpcutil::KomodoRPC) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("getnewaddress");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

/// getrawchangeaddress
/// The getrawchangeaddress returns a new address that can be used to receive change.
/// This is for use with raw transactions, NOT normal use.
/// # Arguments
/// * (none)
/// # Response
/// * "address"	(string)	the address
/// %%%
pub fn get_raw_change_address(
    SomeUser: komodorpcutil::KomodoRPC,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("getrawchangeaddress");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

/// getreceivedbyaddress "address" ( minconf ).
/// The getreceivedbyaddress method returns the total amount received by the given address in transactions with at least minconf confirmations.
/// # Arguments
/// * "address"	(string, required)	the address for transactions
/// minconf  	(numeric, optional, default=1)	only include transactions confirmed at least this many times
/// # Response
/// * amount	(numeric)	the total amount of the relevant coin received at this address

pub fn get_receive_by_address(
    SomeUser: komodorpcutil::KomodoRPC,
    address: String,
    min_conf: Option<u32>,
) -> Result<String, reqwest::Error> {
    let method_body: String;
    let temp_min_conf: String = min_conf.unwrap_or(1).to_string(); // default 1///TO DO
    if (temp_min_conf.is_empty()) {
        method_body =
            String::from("[\"") + &address.to_string() + &String::from("\"") + &String::from("]");
    } else
    //if (!temp_gen_proc_limit.is_empty())
    {
        method_body = String::from("[\"")
            + &address.to_string()
            + &"\",".to_string()
            + &temp_min_conf
            + &String::from("]");
    }

    let method_name: String = String::from("getreceivedbyaddress");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

/// gettransaction "txid" ( includeWatchonly ).
/// The gettransaction method queries detailed information about transaction txid. This command applies only to txid's that are in the user's local wallet.
/// # Arguments
/// * "txid"	(string, required)	the transaction id
/// * "includeWatchonly"	(bool, optional, default=false)	whether to include watchonly addresses in the returned balance calculation and in the details[] returned values
/// # Response
/// * "amount"	(numeric)	the transaction amount
/// * "confirmations"	(numeric)	a confirmation number that is aware of the dPoW security service
/// * "rawconfirmations"	(numeric)	the raw confirmations (number of blocks on top of this transaction's block)
/// * "blockhash"	(string)	the block hash
/// * "blockindex"	(numeric)	the block index
/// * "blocktime"	(numeric)	the time in seconds since epoch (1 Jan 1970 GMT)
/// * "txid"	(string)	the transaction id
/// * "time"	(numeric)	the transaction time in seconds since epoch (1 Jan 1970 GMT)
/// * "timereceived"	(numeric)	the time received in seconds since epoch (1 Jan 1970 GMT)
/// * "details" : [ ... ]	(array)
/// * "account"	(string)	DEPRECATED the account name involved in the transaction; can be "" for the default account
/// * "address"	(string)	the address involved in the transaction
/// * "category"	(string)	the category - either send or receive
/// * "amount"	(numeric)	the amount
/// * "vout"	(numeric)	the vout value
/// * "vjoinsplit" : [ ... ]	(array of json objects)
/// * "anchor"	(string)	merkle root of note commitment tree
/// * "nullifiers" : [ ... ]	(array of strings)
/// * "hex"	(string)
/// * "commitments" : [ ... ]	(array of strings)
/// * "hex"	(string)
/// * "macs" : [ ... ]	(array of strings)
/// * "hex"	(string)
/// * "vpub_old"	(numeric)	the amount removed from the transparent value pool
/// * "vpub_new"	(numeric)	the amount added to the transparent value pool
/// * "hex"	(string)	transaction data translated into hex
/// %%%
pub fn get_transaction(
    SomeUser: komodorpcutil::KomodoRPC,
    tx_id: String,
    include_watch_only: Option<bool>,
) -> Result<String, reqwest::Error> {
    let method_body: String;
    let temp_include_watch_only: String = include_watch_only.unwrap_or(false).to_string();
    method_body = String::from("[\"")
        + &tx_id.to_string()
        + &String::from("\",")
        + &temp_include_watch_only
        + &String::from("]");
    let method_name: String = String::from("gettransaction");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

/// getunconfirmedbalance.
/// The getunconfirmedbalance method returns the server's total unconfirmed balance.
/// # Arguments
/// * (none)
/// # Response
/// * (none)
/// %%%
pub fn get_unconfirmed_balance(
    SomeUser: komodorpcutil::KomodoRPC,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("getunconfirmedbalance");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

/// The getwalletinfo method returns an object containing various information about the wallet state.
/// # Arguments
/// * (none)
/// # Response
/// * "walletversion"	(numeric)	the wallet version
/// * "balance"	(numeric)	the total confirmed balance of the wallet
/// * "unconfirmed_balance"	(numeric)	the total unconfirmed balance of the wallet
/// * "immature_balance"	(numeric)	the total immature balance of the wallet
/// * "txcount"	(numeric)	the total number of transactions in the wallet
/// * "keypoololdest"	(numeric)	the timestamp (seconds since GMT epoch) of the oldest pre-generated key in the key pool
/// * "keypoolsize"	(numeric)	how many new keys are pre-generated
/// * "unlocked_until"	(numeric)	the timestamp in seconds since epoch (midnight Jan 1 1970 GMT) that the wallet is unlocked for transfers, or 0 if the wallet is locked
/// * "paytxfee"	(numeric)	the transaction fee configuration, given as the relevant COIN per KB
/// %%%

pub fn get_wallet_info(SomeUser: komodorpcutil::KomodoRPC) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("getwalletinfo");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

/// importaddress "address" ( "label" rescan ).
/// The importaddress method adds an address or script (in hex) that can be watched as if it were in your wallet, although it cannot be used to spend.
/// This call can take an increased amount of time to complete if rescan is true.
/// # Arguments
/// * "address"	(string, required)	the address to watch
/// * "label"	(string, optional, default="")	an optional label
/// * rescan	(boolean, optional, default=true)	rescan the wallet for transactions
/// # Response
/// * (none)
/// %%%

pub fn import_address(
    //TODO check def value and if conditions
    SomeUser: komodorpcutil::KomodoRPC,
    address: String,
    label: Option<String>,
    rescan: Option<bool>,
) -> Result<String, reqwest::Error> {
    let method_body: String;
    let temp_label: String = label.unwrap_or("".to_string()).to_string();
    let temp_rescan: String = rescan.unwrap_or(true).to_string();

    if (temp_label.is_empty()) {
        method_body =
            String::from("[\"") + &address + &"\",".to_string() + &temp_rescan + &String::from("]");
    } else {
        method_body = String::from("[\"")
            + &address
            + &"\",\"".to_string()
            + &temp_label
            + &"\",".to_string()
            + &temp_rescan
            + &String::from("]");
    }
    let method_name: String = String::from("importaddress");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

/// importkey "komodoprivkey" ( "label" rescan ).
/// The importprivkey method adds a private key to your wallet.
/// This call can take minutes to complete if rescan is true.
/// See also dumpprivkey.
/// # Arguments
/// * "privkey"	(string, required)	the private key (see dumpprivkey)
/// * "label"	(string, optional, default="")	an optional label
/// * rescan	(boolean, optional, default=true)	rescan the wallet for transactions
/// # Response
/// * addresses	(string)	the public address
/// %%%
pub fn import_priv_key(
    //TODO check def value and if conditions
    SomeUser: komodorpcutil::KomodoRPC,
    priv_key: String,
    label: Option<String>,
    rescan: Option<bool>,
) -> Result<String, reqwest::Error> {
    let method_body: String;
    let temp_label: String = label.unwrap_or("".to_string()).to_string();
    let temp_rescan: String = rescan.unwrap_or(true).to_string();

    if (temp_label.is_empty()) {
        method_body = String::from("[\"")
            + &priv_key
            + &"\",".to_string()
            + &temp_rescan
            + &String::from("]");
    } else {
        method_body = String::from("[\"")
            + &priv_key
            + &"\",\"".to_string()
            + &temp_label
            + &"\",".to_string()
            + &temp_rescan
            + &String::from("]");
    }
    let method_name: String = String::from("importprivkey");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

/// importwallet "filename".
/// The importwallet method imports transparent-address keys from a wallet-dump file (see dumpwallet).
/// # Arguments
/// * "filename"	(string, required)	the wallet file
/// # Response
/// * (none)
///  %%%

pub fn import_wallet(
    someUser: komodorpcutil::KomodoRPC,
    file_name: String,
) -> Result<String, reqwest::Error> {
    let params = String::from("[\"") + &file_name + "\"]";

    let method_name: String = String::from("importwallet");
    let method_body: String = String::from(params);
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(someUser.clone(), data)
}

/// keypoolrefill ( newsize ).
/// The keypoolrefill method refills the keypool.
/// # Arguments
/// * newsize	(numeric, optional, default=100)	the new keypool size
/// # Response
/// * (none)
/// %%%
pub fn key_pool_refill(
    SomeUser: komodorpcutil::KomodoRPC,
    new_size: Option<u32>,
) -> Result<String, reqwest::Error> {
    let method_body: String;
    let temp_new_size: String = new_size.unwrap_or(100).to_string();
    method_body = String::from("[") + &temp_new_size + &String::from("]");
    let method_name: String = String::from("keypoolrefill");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

/// listaddressgroupings.
/// The listaddressgroupings method lists groups of addresses which have had their common ownership made public by common use as inputs or as the resulting change in past transactions.
/// # Arguments
/// * (none)
/// # Response
/// * "address",	(string)	the address
/// * amount,	(numeric)	the amount
/// * "account"	(string, optional)	(DEPRECATED) the account
///  %%%

pub fn list_address_groupings(
    SomeUser: komodorpcutil::KomodoRPC,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("listaddressgroupings");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

/// listlockunspent.
/// The listlockunspent method returns a list of temporarily non-spendable outputs.
/// See the lockunspent call to lock and unlock transactions for spending.
/// # Arguments
/// * (none)
/// # Response
/// * "txid"	(string)	the transaction id locked
/// * "vout"	(numeric)	the vout value

pub fn list_lock_unspent(SomeUser: komodorpcutil::KomodoRPC) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("listlockunspent");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

/// listreceivedbyaddress ( minconf includeempty includeWatchonly).
/// The listreceivedbyaddress method lists balances by receiving address.
/// # Arguments
/// * minconf	(numeric, optional, default=1)	the minimum number of confirmations before payments are included
/// * includeempty	(numeric, optional, default=false)	whether to include addresses that haven't received any payments// TO DO must be bool
/// * includeWatchonly	(bool, optional, default=false)	whether to include watchonly addresses (see 'importaddress')
/// # Response
/// * "involvesWatchonly"	(bool)	only returned if imported addresses were involved in transaction
/// * "address"	(string)	the receiving address
/// * "account"	(string)	DEPRECATED the account of the receiving address; the default account is ""
/// * "amount"	(numeric)	the total amount received by the address
/// * "confirmations"	(numeric)	a confirmation number that is aware of the dPoW security service
/// * "rawconfirmations"	(numeric)	the raw confirmations of the most recent transaction included (number of blocks on top of this transaction's block
/// %%%

pub fn list_received_by_address(
    SomeUser: komodorpcutil::KomodoRPC,
    min_conf: Option<u32>,
    include_empty: Option<bool>,
    include_watch_only: Option<bool>,
) -> Result<String, reqwest::Error> {
    let method_body: String;
    let temp_min_conf: String = min_conf.unwrap_or(1).to_string();
    let temp_include_empty: String = include_empty.unwrap_or(false).to_string();
    let temp_include_watch_only: String = include_watch_only.unwrap_or(false).to_string();

    method_body = String::from("[")
        + &temp_min_conf
        + &String::from(",")
        + &temp_include_empty
        + &String::from(",")
        + &temp_include_watch_only
        + &String::from("]");
    let method_name: String = String::from("listreceivedbyaddress");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

// TODO COMPLETE

/// listsinceblock ( "blockhash" target-confirmations includeWatchonly ).
/// The listsinceblock method queries all transactions in blocks since block blockhash, or all transactions if blockhash is omitted.
/// # Arguments
/// * "blockhash"	(string, optional)	the block hash from which to list transactions
/// * target-confirmations	(numeric, optional)	the confirmations required (must be 1 or more)
/// * includeWatchonly	(bool, optional, default=false)	include transactions to watchonly addresses (see also 'importaddress')
/// # Response
/// * "transactions":
/// * "account"	(string)	DEPRECATED the account name associated with the transaction; will be "" for the default account
/// * "address"	(string)	the address of the transaction (not present for move transactions -- category = move)
/// * "category"	(string)	the transaction category; send has negative amounts, receive has positive amounts
/// * "amount"	(numeric)	the amount of the relevant currency -- negative for the send category, and for the move category for moves outbound. It is positive for the receive category, and for the move category for inbound funds.
/// * "vout"	(numeric)	the vout value
/// * "fee"	(numeric)	the amount of the fee; this value is negative and only available for the send category of transactions
/// * "confirmations"	(numeric)	a confirmation number that is aware of the dPoW security service
/// * "rawconfirmations"	(numeric)	the raw confirmations of the transaction; available for send and receive category of transactions (number of blocks on top of this transaction's block)
/// * "blockhash"	(string)	the block hash containing the transaction; available for the send and receive categories of transactions
/// * "blockindex"	(numeric)	the block index containing the transaction; available for the send and receive categories of transactions
/// * "blocktime"	(numeric)	the block time in seconds since epoch (1 Jan 1970 GMT)
/// * "txid"	(string)	the transaction id; available for send and receive categories of transactions
/// * "time"	(numeric)	the transaction time in seconds since epoch (Jan 1 1970 GMT)
/// * "timereceived"	(numeric)	the time received in seconds since epoch (Jan 1 1970 GMT); available for send and receive category of transactions
/// * "comment"	(string)	whether a comment is associated with the transaction
/// * "to"	(string)	whether a 'to' comment is associated with the transaction
/// * "lastblock"	(string)	the hash of the last block
/// %%%

pub fn list_since_block(
    some_user: komodorpcutil::KomodoRPC,
    block_hash: Option<String>,
    target_conformations: Option<u32>,
    include_watch_only: Option<bool>,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("listsinceblock");
    let temp_block_hash: String = block_hash.unwrap_or("".to_string());
    let temp_target_conformations = target_conformations.unwrap_or(1);
    let temp_watch_only = include_watch_only.unwrap_or(false);

    let method_body: String = String::from("[\"")
        + &temp_block_hash
        + &String::from("\",")
        + &temp_target_conformations.to_string()
        + &String::from(",")
        + &temp_watch_only.to_string()
        + &String::from("]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

// TODO

/// The listtransactions method returns up to count most recent transactions skipping the first from transactions for account.
/// # Arguments
/// "account"       	(string, optional)            	DEPRECATED the account name; should be "*"
/// * count	            (numeric, optional, default=10)	the number of transactions to return
/// * from	            (numeric, optional, default=0)	the number of transactions to skip
/// includeWatchonly	(bool, optional, default=false)	include transactions to watchonly addresses (see importaddress)
/// # Response
/// * "account"	(string)	DEPRECATED the account name associated with the transaction; it will be "" for the default account
/// * "address"	(string)	the address of the transaction; not present for move transactions (category = move)
/// * "category"	(string)	The transaction category. This property can be send
/// * "amount"	(numeric)	The amount. This value is negative for the send category, and for the move category for moves outbound. It is positive for the receive category and for the move category for inbound funds.
/// * "vout"	(numeric)	the vout value
/// * "fee"	(numeric)	the amount of the fee; this is negative and only available for the send category of transactions
/// * "confirmations"	(numeric)	a confirmation number that is aware of the dPoW security service
/// * "rawconfirmations"	(numeric)	the raw confirmations of the transaction; available for send and receive category of transactions (number of blocks on top of this transaction's block)
/// * "blockhash"	(string)	the block hash containing the transaction; available for the send and receive categories of transactions
/// * "blockindex"	(numeric)	the block index containing the transaction; available for the send and receive categories of transactions
/// * "txid"	(string)	the transaction id; available for the send and receive categories of transactions
/// * "time"	(numeric)	the transaction time in seconds since epoch (midnight Jan 1 1970 GMT)
/// * "timereceived"	(numeric)	the time received in seconds since epoch (midnight Jan 1 1970 GMT); available for the send and receive categories of transactions
/// * "comment"	(string)	whether a comment is associated with the transaction
/// * "otheraccount"	(string)	for the move category of transactions; indicates the account which sent the funds (for receiving funds, positive amounts), or went to (for sending funds, negative amounts)
/// * "size"	(numeric)	transaction size in bytes
/// * %%%

pub fn list_transactions(
    some_user: komodorpcutil::KomodoRPC,
    accoount: Option<String>,
    count: Option<u32>,
    from: Option<u32>,
    include_watch_only: Option<bool>,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("listtransactions");

    let method_body: String = String::from("[\"")
        + &"*".to_string()
        + &String::from("\",")
        + &count.unwrap_or(10).to_string()
        + &from.unwrap_or(0).to_string()
        + &String::from("]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

//TODO
/*
/// * listunspent ( minconf maxconf ["address", ... ] ).
/// * The listunspent method returns an array of unspent transaction outputs, with a range between minconf and maxconf (inclusive) confirmations. The method can, optionally, filter to only include txouts paid to specified addresses.
/// # Arguments
/// * minconf	(numeric, optional, default=1)	the minimum confirmations to filter
/// * maxconf	(numeric, optional, default=9999999)	the maximum confirmations to filter
/// * "address"	(string)	a series of addresses
///  # Response
/// * "txid"	(string)	the transaction id
/// * "vout"	(numeric)	the vout value
/// * "generated"	(boolean)	true if txout is a coinbase transaction output
/// * "address"	(string)	the address
/// * "account"	(string)	DEPRECATED the associated account, or "" for the default account
/// * "scriptPubKey"	(string)	the script key
/// * "amount"	(numeric)	the transaction amount
/// * "confirmations"	(numeric)	a confirmation number that is aware of the dPoW security service
/// * "rawconfirmations"	(numeric)	the raw confirmations (number of blocks on top of this transaction's block)
/// * %%%
*/
pub fn list_unspent(
    some_user: komodorpcutil::KomodoRPC,
    minconf: Option<u32>,
    maxconf: Option<u32>,
    address: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("listunspent");

    let temp_minconf = minconf.unwrap_or(1);
    let temp_maxconf = maxconf.unwrap_or(9999999);

    let method_body: String = String::from("[\"")
        + &temp_minconf.to_string()
        + &String::from(", ")
        + &temp_maxconf.to_string()
        + &String::from(" [\"")
        + &address
        + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

//  TODO - unfinished

/// * lockunspent unlock [{ "txid": "txid", "vout": n }, ... ].
/// * The lockunspent method locks (unlock = false) or unlocks (unlock = true) specified transaction outputs. A locked transaction output will not be chosen by automatic coin selection, when spending the relevant coin. The locks are stored in memory only; at runtime a node always starts with zero locked outputs, and the locked output list is always cleared when a node stops or fails.
/// * See the listunspent and listlockunspent calls to determine local transaction state and info.
/// # Arguments
/// * unlock	(boolean, required)	whether to unlock (true) or lock (false) the specified transactions
/// * "txid"	(string)	the transaction id
/// * "vout"	(numeric)	the output number
/// # Response
/// * true/false	(boolean)	whether the command was successful
/// * %%%

pub fn lock_unspent(
    someUser: komodorpcutil::KomodoRPC,
    unlock: bool,
    txid: String,
    vout: u32,
) -> Result<String, reqwest::Error> {
    let method_body = String::from("[")
        + &unlock.to_string()
        + &(",[{\"txid\":\"").to_string()
        + &txid
        + &("\",\"vout\":").to_string()
        + &vout.to_string()
        + &("}]]").to_string();
    let method_name: String = String::from("lockunspent");
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(someUser.clone(), data)
}

/// * resendwallettransactions.
/// * The resendwallettransactions method immediately re-broadcasts unconfirmed wallet transactions to all peers. This method is intended only for testing; the wallet code periodically re-broadcasts automatically.
/// # Arguments
/// * (none)
/// # Response
/// * "transaction_id"	(string)	an array of the rebroadcasted transaction id's
/// * %%%  

pub fn resend_wallet_transactions(
    SomeUser: komodorpcutil::KomodoRPC,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("resendwallettransactions");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

/** TODO - unfinished
 * sendmany "account" { "address": amount, ... } ( minconf "comment" [ "address", ... ] )
The sendmany method can send multiple transactions at once. Amounts are double-precision floating point numbers.
#Arguments
Name	Type	Description
"account"	(string, required)	MUST be set to the empty string "" to represent the default account; passing any other string will result in an error
"amounts" { "address":amount, ... }	("string":numeric)	the address (string) and the value (double-precision floating numeric)
minconf	(numeric, optional, default=1)	only use the balance confirmed at least this many times
"comment"	(string, optional)	a comment
subtractfeefromamount	(string, optional)	a json array with addresses. The fee will be equally deducted from the amount of each selected address; the recipients will receive less than you enter in their corresponding amount field. If no addresses are specified here, the sender pays the fee.
"address"	(string)	subtract fee from this address
#Response
Name	Type	Description
"transaction_id"	(string)	the transaction id for the send; only 1 transaction is created regardless of the number of addresses
 */

/// * sendmany "account" { "address": amount, ... } ( minconf "comment" [ "address", ... ] ).
/// * The sendmany method can send multiple transactions at once. Amounts are double-precision floating point numbers.
/// # Arguments
/// * "account" 	(string, required) 	MUST be set to the empty string "" to represent the default account; passing any other string will result in an error
/// * "amounts" { "address":amount, ... } 	("string":numeric) 	the address (string) and the value (double-precision floating numeric)
/// * minconf 	(numeric, optional, default=1) 	only use the balance confirmed at least this many times
/// * "comment" 	(string, optional) 	a comment
/// * subtractfeefromamount 	(string, optional) 	a json array with addresses. The fee will be equally deducted from the amount of each selected address; the recipients will receive less than you enter in their corresponding amount field. If no addresses are specified here, the sender pays the fee.
/// * "address" 	(string) 	subtract fee from this address
/// # Response
/// * "transaction_id" 	(string) 	the transaction id for the send; only 1 transaction is created regardless of the number of addresses
/// * %%%
pub fn send_many(
    some_user: komodorpcutil::KomodoRPC,
    account: String,
    amounts: String,
    minconf: Option<u32>,
    comment: Option<String>,
    subtract_fee_from_amount: Option<String>,
    address: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("sendmany");
    let temp_minconf = minconf.unwrap_or(1);
    let temp_comment: String = comment.unwrap_or("".to_string());
    let temp_subtract_fee_from_amount: String = subtract_fee_from_amount.unwrap_or("".to_string());
    // dont use account?
    let method_body: String = String::from("[\"")
        + &String::from("\",")
        + &temp_minconf.to_string()
        + &String::from(",")
        + &String::from("\"")
        + &temp_comment.to_string()
        + &String::from("\"")
        + &String::from(",")
        + &temp_subtract_fee_from_amount.to_string()
        + &String::from("]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

/// * sendtoaddress "address" amount ( "comment" "comment-to" subtractfeefromamount )
/// *
/// * The sendtoaddress method sends an amount to a given address. The amount is real and is rounded to the nearest 0.00000001.
/// # Arguments
/// * "komodoaddress" 	(string, required) 	the receiving address
/// * "amount" 	(numeric, required) 	the amount to send (json requires all decimals values less than 1 begin with the characters '0.')
/// * "comment" 	(string, optional) 	a comment used to store what the transaction is for; this is not part of the transaction, just kept in your wallet
/// * "comment-to" 	(string, optional) 	a comment to store the name of the person or organization to which you're sending the transaction; this is stored in your local wallet file only
/// * subtractfeefromamount 	(boolean, optional, default=false) 	when true, the fee will be deducted from the amount being sent
/// # Response
/// * "transaction_id" 	(string) 	the transaction id

pub fn send_to_address(
    some_user: komodorpcutil::KomodoRPC,
    komodo_address: String,
    amount: f64,
    comment: Option<String>,
    comment_to: Option<String>,
    subtract_fee_from_amount: Option<bool>,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("sendtoaddress");
    let temp_comment: String = comment.unwrap_or("".to_string());
    let temp_comment_to: String = comment_to.unwrap_or("".to_string());
    let temp_subtract_fee_from_amount = subtract_fee_from_amount.unwrap_or(false);

    let mut method_body: String = String::from("[\"")
        + &komodo_address.to_string()
        + &String::from("\",")
        + &amount.to_string();

    if (!temp_comment.is_empty()) {
        let temp_string: String = String::from(",\"") + &temp_comment + &String::from("\"");
        method_body.push_str(&temp_string);
    }

    if (!temp_comment_to.is_empty()) {
        let temp_string: String = String::from(",\"") + &temp_comment_to + &String::from("\"");
        method_body.push_str(&temp_string);
    }
    if (temp_subtract_fee_from_amount) {
        method_body.push_str(&String::from(", true"));
    }

    method_body.push_str(&String::from("]"));
    println!("{}", method_body);
    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    let result = komodorpcutil::request(some_user.clone(), data);
    return result;
}
/// * setpubkey pubkey.
/// * The setpubkey method sets the indicated pubkey. This method can be used in place of the pubkey launch parameter, when necessary.
/// * Visit the section pubkey to understand when it is essential to set a pubkey and the consequences of setting it.
/// * This method works only once per daemon start. It can't be used to change the pubkey that has already been set.
/// # Arguments
/// * pubkey 	(string) 	the desired pubkey
/// # Response
/// * pubkey 	(string) 	the pubkey
/// * ismine 	(boolean) 	indicates whether the address belongs to the user
/// * R-address 	(string) 	the public address associated with the pubkey
/// * %%%
pub fn set_pub_key(
    some_user: komodorpcutil::KomodoRPC,
    pub_key: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("setpubkey");

    let method_body: String = String::from("[\"") + &pub_key.to_string() + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

/// * settxfee amount
/// * The settxfee method sets the transaction fee per kB.
/// * # Arguments
/// * amount 	(numeric, required) 	the transaction fee in COIN/kB rounded to the nearest 0.00000001
/// * # Response
/// * true/false 	(boolean) 	returns true if successful
/// * %%%
pub fn set_tx_fee(
    some_user: komodorpcutil::KomodoRPC,
    amount: f64,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("settxfee");

    let method_body: String = String::from("[") + &amount.to_string() + &String::from("]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

/// * signmessage "address" "message"
/// * The signmessage method signs a message via the private key of an address.
/// # Arguments
/// * "address" 	(string, required) 	the address to use for the private key
/// * "message" 	(string, required) 	the message
/// # Response
/// * "signature" 	(string) 	the signature of the message encoded in base 64

pub fn sign_message(
    some_user: komodorpcutil::KomodoRPC,
    address: String,
    message: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("signmessage");

    let method_body: String = String::from("[\"")
        + &address.to_string()
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

/// * walletlock
/// * The walletlock method is neither active nor visible in the help method until the encryptwallet passphrase is set.
/// * This feature is available only on chains where -ac_public is enabled. Chains that feature private transactions cannot use this feature.
/// * The walletlock method re-locks a wallet that has a passphrase enabled via encryptwallet.
/// # Arguments
/// * (none)
/// # Response
/// * (none)

pub fn wallet_lock(some_user: komodorpcutil::KomodoRPC) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("walletlock");

    let method_body: String = String::from("[]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}
/// * walletpassphrase "passphrase" (timeout)
/// * The walletpassphrase method is neither active nor visible in the help method until the encryptwallet passphrase is set.
/// * This feature is available only on chains where -ac_public is enabled. Chains that feature private transactions cannot use this feature.
/// * The walletpassphrase method unlocks the wallet using the passphrase that was set by the encryptwallet method.
/// * The timeout argument can be included to limit the length of time (in seconds) the wallet will remain unlocked.
/// # Arguments
/// * "passphrase" 	(string) 	the passphrase that was set by the encryptwallet method
/// * timeout 	(number in seconds, optional) 	the amount of time for which the wallet should remember the passphrase
/// # Response
/// * (none)
/// * %%%

pub fn wallet_pass_phrase(
    some_user: komodorpcutil::KomodoRPC,
    pass_phrase: String,
    timeout: Option<f64>,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("walletpassphrase");

    let mut method_body: String =
        String::from("[\"") + &pass_phrase.to_string() + &String::from("\"");

    if let Some(x) = timeout {
        method_body = method_body + &String::from(",") + &x.to_string();
    }
    method_body = method_body + &String::from("]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

/// * walletpassphrasechange "oldpassphrase" "newpassphrase"
/// * The walletpassphrasechange method is neither active nor visible in the help method until the encryptwallet passphrase is set.
/// * This feature is available only on chains where -ac_public is enabled. Chains that feature private transactions cannot use this feature.
/// * The walletpassphrasechange method changes "oldpassphrase" to "newpassphrase".
/// # Arguments
/// * "oldpassphrase" 	(string) 	the old passphrase
/// * "newpassphrase" 	(string) 	the new passphrase
/// # Response
/// * (none)
/// * %%%
pub fn wallet_pass_phrase_change(
    some_user: komodorpcutil::KomodoRPC,
    old_pass_phrase: String,
    new_pass_phrase: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("walletpassphrasechange");

    let method_body: String = String::from("[\"")
        + &old_pass_phrase.to_string()
        + &String::from("\",\"")
        + &new_pass_phrase.to_string()
        + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}
/// * z_exportkey "z_address"
/// * The z_exportkey method reveals the private z_key corresponding to z_address.
/// * See also z_importkey.
/// # Arguments
/// * "z_address" 	(string, required) 	the z_address for the private key
/// # Response
/// * "key" 	(string) 	the private key
/// * %%%
pub fn z_export_key(
    some_user: komodorpcutil::KomodoRPC,
    z_address: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("z_exportkey");

    let method_body: String = String::from("[\"") + &z_address.to_string() + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

/// * z_exportviewingkey "z_address"
/// * The z_exportviewingkey method reveals the viewing key corresponding to z_address.
/// * See also z_importviewingkey.
/// # Arguments
/// * "z_address" 	(string, required) 	the z_address for the viewing key
/// # Response
/// * "vkey" 	(string) 	the viewing key
/// * %%%

pub fn z_export_viewing_key(
    some_user: komodorpcutil::KomodoRPC,
    z_address: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("z_exportviewingkey");

    let method_body: String = String::from("[\"") + &z_address.to_string() + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

/// * z_exportwallet "filename"
/// * The z_exportwallet method exports all wallet keys, including both t address and z address types, in a human-readable format. Overwriting an existing file is not permitted.
/// # Arguments
/// * "filename" 	(string, required) 	the filename, saved to the directory indicated by the exportdir parameter at daemon runtime (required)
/// # Response
/// * "path" 	(string) 	the full path of the destination file
/// * %%%
pub fn z_export_wallet(
    some_user: komodorpcutil::KomodoRPC,
    file_name: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("z_exportwallet");

    let method_body: String = String::from("[\"") + &file_name.to_string() + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

/// * z_getbalance "address" ( minconf )
/// * The z_getbalance method returns the balance of a t address or z address belonging to the node’s wallet.
/// * CAUTION: If address is a watch-only z address, the returned balance may be larger than the actual balance, as spends cannot be detected with incoming viewing keys.
/// # Arguments
/// * "address" 	(string) 	the selected z or t address
/// * minconf 	(numeric, optional, default=1) 	only include transactions confirmed at least this many times
/// # Response
/// * amount 	(numeric) 	the total amount received at this address (in the relevant COIN value)
pub fn z_get_balance(
    some_user: komodorpcutil::KomodoRPC,
    address: String,
    minconf: Option<u32>,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("z_getbalance");
    let temp_minconf: String = minconf.unwrap_or(1).to_string();
    let method_body: String = String::from("[\"")
        + &address.to_string()
        + &String::from("\",")
        + &temp_minconf.to_string()
        + &String::from("]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}
/// * z_getnewaddress
/// * The z_getnewaddress method returns a new z_address for receiving payments.
/// # Arguments
/// * (none)
/// # Response
/// * "z_address" 	(string) 	the new z_address

pub fn z_get_new_address(some_user: komodorpcutil::KomodoRPC) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("z_getnewaddress");

    let method_body: String = String::from("[]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

/// * z_getoperationresult ([ "operationid", ... ])
/// * The z_getoperationresult method retrieves the result and status of an operation which has finished, and then removes the operation from memory.
/// * See also z_getoperationstatus.
/// # Arguments
/// * "operationid" 	(string, optional) 	a list of operation ids to query; if not provided, the method examines all operations known to the node
/// # Response
/// * "id" 	(string) 	the operation id
/// * "status" 	(string) 	the result of the operation; can be success
/// * "creation_time" 	(numeric) 	the creation time, in seconds since epoch (Jan 1 1970 GMT)
/// * "result": { ... } 	(array of json objects)
/// * "txid": 	(string) 	the transaction id
/// * "execution_secs" 	(numeric) 	the length of time to calculate the transaction
/// * "method" 	(string) 	the name of the method used in the operation
/// * "params": { ... } 	(json)
/// * "fromaddress" 	(string) 	the address from which funds are drawn
/// * "amounts": [ ... ] 	(array of json objects)
/// * "address" 	(string) 	the receiving address
/// * "amount" 	(numeric) 	the amount to receive
/// * "minconf" 	(numeric) 	the minimum number of confirmations required
/// * "fee" 	(numeric) 	the transaction fee
/// * %%%
pub fn z_get_operation_result(
    some_user: komodorpcutil::KomodoRPC,
    operation_id: Option<String>,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("z_getoperationresult");

    let temp_operation_id: String = operation_id.unwrap_or("".to_string());
    let method_body: String =
        String::from("[[\"") + &temp_operation_id.to_string() + &String::from("\"]]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

// TODO: arrays as argument?

/// * z_getoperationstatus ([ "operationid", ... ])
/// * The z_getoperationstatus message queries the operation status and any associated result or error data of any operationid stored in local memory. The operation will remain in memory (unlike z_getoperationresult, which removes the data from the local memory).
/// # Arguments
/// * "operationid" 	(array, optional) 	a list of operation ids we are interested in; if an array is not provided, the method examines all operations known to the node
/// # Response
/// * "id" 	(string) 	the operation id
/// * "status" 	(string) 	the status of the operation; can be success
/// * "creation_time" 	(numeric) 	the creation time, in seconds since epoch (Jan 1 1970 GMT)
/// * "error" : { ... } 	(array of json objects)
/// * "code" 	(numeric) 	the associated error code
/// * "message" 	(string) 	a message to indicate the nature of the error, if such a message is available
/// * "method" 	(string) 	the name of the method used in the operation
/// * "params" : { ... } 	(array of json objects)
/// * "fromaddress" 	(string) 	the address from which funds are drawn
/// * "amounts": [ ... ] 	(array of json objects)
/// * "address" 	(string) 	the receiving address
/// * "amount" 	(numeric) 	the amount to receive
/// * "minconf" 	(numeric) 	indicates the required number of mining confirmations
/// * "fee" 	(numeric) 	the fee
/// * %%%
pub fn z_get_operation_status(
    some_user: komodorpcutil::KomodoRPC,
    operation_id: Option<String>,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("z_getoperationstatus");

    let temp_operation_id: String = operation_id.unwrap_or("".to_string());
    let method_body: String =
        String::from("[[\"") + &temp_operation_id.to_string() + &String::from("\"]]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

/// * z_gettotalbalance ( minconf includeWatchonly )
/// * The z_gettotalbalance method returns the total value of funds, including both transparent and private, stored in the node’s wallet.
/// * CAUTION: If the wallet contains watch-only z addresses the returned private balance may be larger than the actual balance, as spends cannot be detected with incoming viewing keys.
/// * While the interest property is returned for all KMD-based coin daemons, only the main KMD chain utilizes the interest feature. KMD-based Smart Chains will always return a 0.00 interest value.
/// # Arguments
/// * minconf 	(numeric, optional, default=1) 	only include private and transparent transactions confirmed at least this many times
/// * includeWatchonly 	(bool, optional, default=false) 	also include balance in watchonly addresses (see 'importaddress' and 'z_importviewingkey')
/// # Response
/// * "transparent" 	(numeric) 	the total balance of transparent funds
/// * "interest" 	(numeric) 	the total balance of unclaimed interest earned
/// * "private" 	(numeric) 	the total balance of private funds
/// * "total" 	(numeric) 	the total balance of both transparent and private funds
/// * %%%
pub fn z_get_total_balance(
    some_user: komodorpcutil::KomodoRPC,
    minconf: Option<u32>,
    include_watch_only: Option<bool>,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("z_gettotalbalance");
    let temp_minconf = minconf.unwrap_or(1);
    let temp_include_watch_only = include_watch_only.unwrap_or(false);
    let method_body: String = String::from("[")
        + &temp_minconf.to_string()
        + &String::from(",")
        + &temp_include_watch_only.to_string()
        + &String::from("]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

/// * z_importkey "z_privatekey" ( rescan startHeight )
/// * The z_importkey method imports z_privatekey to your wallet.
/// * This call can take minutes to complete if rescan is true.
/// * The optional parameters are currently not functional with KMD-based blockchains.
/// * See also z_exportkey.
/// # Arguments
/// * "z_privatekey" 	(string, required) 	the z_privatekey (see z_exportkey)
/// * rescan 	(string, optional, default="whenkeyisnew") 	rescan the wallet for transactions; can be yes
/// * startHeight 	(numeric, optional, default=0) 	the block height at which to begin the rescan
/// # Response
/// * (none)
/// * %%%
pub fn z_import_key(
    some_user: komodorpcutil::KomodoRPC,
    z_private_key: String,
    rescan: Option<String>,
    start_height: Option<u32>,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("z_importkey");
    let temp_rescan = rescan.unwrap_or("whenkeyisnew".to_string());
    let temp_start_height = start_height.unwrap_or(0);
    let method_body: String = String::from("[\"")
        + &z_private_key.to_string()
        + &String::from("\",\"")
        + &temp_rescan.to_string()
        + &String::from("\",")
        + &temp_start_height.to_string()
        + &String::from("]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

/// * z_importviewingkey "viewing_key" ( rescan startHeight )
/// * The z_importviewingkey adds a viewing key to your wallet. This method allows you to view the balance in a z address that otherwise does not belong to your wallet.
/// * See also z_exportviewingkey.
/// * This call can take minutes to complete if rescan is true.
/// * The optional parameters are currently not functional for KMD-based blockchains.
/// # Arguments
/// * "viewing_key" 	(string, required) 	the viewing key
/// * rescan 	(string, optional, default="whenkeyisnew") 	whether to rescan the wallet for transactions; can be "yes"
/// * startHeight 	(numeric, optional, default=0) 	block height to start rescan
/// # Response
/// * (none)
pub fn z_import_viewing_key(
    some_user: komodorpcutil::KomodoRPC,
    z_private_key: String,
    rescan: Option<String>,
    start_height: Option<u32>,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("z_importviewingkey");
    let temp_rescan = rescan.unwrap_or("whenkeyisnew".to_string());
    let temp_start_height = start_height.unwrap_or(0);
    let method_body: String = String::from("[\"")
        + &z_private_key.to_string()
        + &String::from("\",\"")
        + &temp_rescan.to_string()
        + &String::from("\",")
        + &temp_start_height.to_string()
        + &String::from("]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

/// * z_importwallet "filename"
/// * The z_importwallet method imports t address and z address keys from a wallet export file.
/// * See also z_exportwallet.
/// # Arguments
/// * "filename" 	(string, required) 	the wallet file
/// # Response
/// * (none)
/// * %%%
pub fn z_import_wallet(
    some_user: komodorpcutil::KomodoRPC,
    file_name: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("z_importwallet");

    let method_body: String = String::from("[\"") + &file_name.to_string() + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}
/// * z_listaddresses ( includeWatchonly )
/// * The z_listaddresses method returns the list of z addresses belonging to the wallet.
/// * See also z_importviewingkey.
/// # Arguments
/// * includeWatchonly 	(bool, optional, default=false) 	also include watchonly addresses
/// # Response
/// * "z_address" 	(string) 	a z address belonging to the wallet
/// * %%%
pub fn z_list_addresses(
    some_user: komodorpcutil::KomodoRPC,
    include_watch_only: Option<bool>,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("z_listaddresses");
    let temp_include_watch_only = include_watch_only.unwrap_or(false);
    let method_body: String =
        String::from("[") + &temp_include_watch_only.to_string() + &String::from("]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

// TODO: handle if empty case

/// * z_listoperationids
/// * The z_listoperationids method returns the list of operation ids currently known to the wallet.
/// # Arguments
/// * "status" 	(string, optional) 	filter result by the operation's state e.g. "success"
/// # Response
/// * "operationid" 	(string) 	an operation id belonging to the wallet
/// * %%%
pub fn z_list_operation_ids(
    some_user: komodorpcutil::KomodoRPC,
    status: Option<String>,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("z_listoperationids");
    let temp_status = status.unwrap_or("".to_string());
    let method_body: String = String::from("[") + &temp_status.to_string() + &String::from("]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

/// * z_listreceivedbyaddress "address" ( minconf )
/// * The z_listreceivedbyaddress method returns a list of amounts received by a z address belonging to the node’s wallet.
/// # Arguments
/// * address 	(string) 	the private address.
/// * minconf 	(numeric, optional, default=1) 	only include transactions confirmed at least this many times
/// # Result
/// * An array of json objects, each having the properties below.
/// * txid 	(string) 	the transaction id
/// * amount 	(numeric) 	the amount of value in the note
/// * memo 	(string) 	hexadecimal string representation of memo field
/// * "confirmations" 	(numeric) 	a confirmation number that is aware of the dPoW security service
/// * "rawconfirmations" 	(numeric) 	the raw confirmations (number of blocks on top of this transaction's block)
/// * jsindex 	(sprout) 	(numeric, received only by sprout addresses) the joinsplit index
/// * jsoutindex 	(numeric, received only by sprout addresses) 	the output index of the joinsplit
/// * outindex 	(numeric, sapling) 	the output index
/// * change 	(boolean) 	true if the address that received the note is also one of the sending addresses
/// * %%%

pub fn z_list_received_by_address(
    SomeUser: komodorpcutil::KomodoRPC,
    address: String,
    min_conf: Option<u32>,
) -> Result<String, reqwest::Error> {
    let method_body: String;
    method_body = String::from("[\"")
        + &address.to_string()
        + &String::from(" \",")
        + &min_conf.unwrap_or(1).to_string()
        + &String::from("]");
    let method_name: String = String::from("z_listreceivedbyaddress");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

/// * z_listunspent ( minconf maxconf includeWatchonly ["zaddr", ...] )
/// * The z_listunspent method returns an array of unspent shielded notes.
/// * The method can also filter to only include results that have between minconf and maxconf (inclusive) confirmations, and also for specified z_addresses (["zaddr", ...]).
/// * When minconf is 0 unspent notes with zero confirmations are returned, even though they are not immediately spendable.
/// * Results are an array of Objects, each of which has: {txid, jsindex, jsoutindex, confirmations, address, amount, memo} (Sprout) {txid, outindex, confirmations, address, amount, memo} (Sapling)
/// # Arguments
/// * minconf 	(numeric, optional, default=1) 	the minimum confirmations to filter
/// * maxconf 	(numeric, optional, default=9999999) 	the maximum confirmations to filter
/// * includeWatchonly 	(bool, optional, default=false) 	whether to also include watchonly addresses (see z_importviewingkey)
/// * addresses 	(array) 	a json array of z addresses (both Sprout and Sapling) to act as a filter; duplicate addresses are not allowed
/// * address 	(string) 	a z address
/// # Results
/// * An array of json objects, each having the properties below.
/// * txid 	(string) 	the transaction id
/// * jsindex 	(numeric) 	the joinsplit index
/// * jsoutindex 	(numeric, only returned on sprout addresses) 	the output index of the joinsplit
/// * outindex 	(numeric, only returned on sapling addresses) 	the output index
/// * "confirmations" 	(numeric) 	a confirmation number that is aware of the dPoW security service
/// * "rawconfirmations" 	(numeric) 	the raw confirmations (number of blocks on top of this transaction's block)
/// * spendable 	(boolean) 	true if note can be spent by wallet, false if note has zero confirmations, false if address is watchonly
/// * address 	(string) 	the shielded address
/// * amount 	(numeric) 	the amount of value in the note
/// * memo 	(string) 	hexadecimal string representation of memo field
/// * change 	(boolean) 	true if the address that received the note is also one of the sending addresses/
/// * %%%

pub fn z_list_unspent(
    SomeUser: komodorpcutil::KomodoRPC,
    min_conf: Option<u32>,
    max_conf: Option<u32>,
    include_watch_only: Option<bool>,
    addresses: Vec<String>,
    address: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("z_listunspent");
    let method_body: String;

    let mut addr_list = String::from("[");

    for addr in &addresses {
        addr_list = addr_list + "\"" + addr + "\"" + &","; //parsing error
    }
    if (addresses.len() > 0)
    // >1
    {
        addr_list = addr_list[0..(addr_list.len() - 1)].to_string(); // cutting the last ,
    }
    addr_list = addr_list + &"]"; //& for String -> &string

    method_body = String::from("[")
        + &min_conf.unwrap_or(1).to_string()
        + &String::from(",")
        + &max_conf.unwrap_or(9999999).to_string()
        + &String::from(",")
        + &include_watch_only.unwrap_or(false).to_string()
        + &String::from(",")
        + &addr_list.to_string()
        + &String::from(",")
        + &address.to_string()
        + &String::from("]");

    komodorpcutil::request(
        SomeUser.clone(),
        komodorpcutil::generate_body(SomeUser.clone(), method_name, method_body),
    )
}

/*
TODO - unfinished
                                        E X P E R I M E N T A L                    F E A T U R E
Check with Dr. Datta & Komodo Team
Function: z_mergetoaddress
Arguments
Name 	Type 	Description
fromaddresses 	(string, required)
"address" 	(string) 	can be a t address or a z address
"toaddress" 	(string, required) 	the t address or z address to receive the combined utxo
fee 	(numeric, optional, default=0.0001) 	the fee amount to attach to this transaction
transparent_limit 	(numeric, optional, default=50) 	limit on the maximum number of transparent utxos to merge; you may set this value to 0 to use the node option mempooltxinputlimit
shielded_limit 	(numeric, optional, default=10) 	limit on the maximum number of hidden notes to merge; you may set this value to 0 to merge as many as will fit in the transaction
"memo" 	(string, optional) 	encoded as hex; when toaddress is a z address, this value will be stored in the memo field of the new note
#
Response
Name 	Type 	Description
"remainingUTXOs" 	(numeric) 	the number of utxos still available for merging
"remainingTransparentValue" 	(numeric) 	the value of utxos still available for merging
"remainingNotes" 	(numeric) 	the number of notes still available for merging
"remainingShieldedValue" 	(numeric) 	the value of notes still available for merging
"mergingUTXOs" 	(numeric) 	the number of utxos being merged
"mergingTransparentValue" 	(numeric) 	the value of utxos being merged
"mergingNotes" 	(numeric) 	the number of notes being merged
"mergingShieldedValue" 	(numeric) 	the value of notes being merged
"opid" 	(string) 	an operationid to pass to z_getoperationstatus to get the result of the operation
*/

// - Incomplete documentation, missing complete function
//      (https://docs.komodoplatform.com/basic-docs/smart-chains/smart-chain-api/wallet.html#z-sendmany)
// - unfinished function: amounts array arguments, memo argument
/// * z_sendmany "fromaddress" [ { "address": ..., "amount": ... }, ... ] ( minconf ) ( fee )
/// * The z_sendmany method sends one or more transactions at once, and allows for sending transactions of types t --> t, t --> z, z --> z, z --> t. It is the principle method for dealing with shielded z transactions in the Komodo ecosystem.
/// * The amount values are double-precision floating point numbers. Change from a t address flows to a new t address address, while change from z address returns to itself. When sending coinbase utxos to a z address, change is not allowed. The entire value of the utxo(s) must be consumed. Currently, the maximum number of z address outputs is 54 due to transaction-size limits.
/// # Arguments
/// * "fromaddress" 	(string, required) 	the sending t address or z address
/// * "amounts" 	(array of json objects)
/// * "address" 	(string, required) 	the receiving address; can be a t address or z address
/// * "amount" 	(numeric, required) 	the numeric amount
/// * "memo" 	(string, optional) 	if the address is a z address, this property accepts raw data represented in hexadecimal string format
/// * minconf 	(numeric, optional, default=1) 	only use funds confirmed at least this many times
/// * fee 	(numeric, optional, default=0.0001) 	the fee amount to attach to this transaction
/// # Response
/// * "operationid" 	(string) 	an operationid to pass to z_getoperationstatus to get the result of the operation
/*
pub fn z_send_many(
    SomeUser: komodorpcutil::KomodoRPC,
    from_address: String,
    amounts: f32,
    address: String,
    amount: f32,
    memo: Option<String>,
    minconf: Option<u32>,
    fee: Option<f32>,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("z_sendmany");
    let method_body: String;
    let temp_memo: String = memo.unwrap_or("");
    let temp_minconf: u32 = minconf.unwrap_or(1);
    let temp_fee: f32 = fee.unwrap_or(.0001);

    method_body = String::from("[\"")
    + &from_address
    + &String::from("\", ")
    + &address
    + &String::from(",")
    + &amount.to_string()
    + &String::from(",")
    + &temp_memo.to_string()
    + &String::from(",")
    + &temp_minconf.to_string()
    + &String::from(",")
    + &temp_fee.to_string()
    + &String::from("]");

    let data: String = String::from(komodorpcutil::generate_body(
            SomeUser.clone(),
            method_name,
            method_body));
    komodorpcutil::request(SomeUser.clone(), data)
}*/

/// * z_shieldcoinbase "fromaddress" "tozaddress" ( fee ) ( limit )
/// * The z_shieldcoinbase method shields transparent coinbase funds by sending the funds to a shielded z address. This is an asynchronous operation and utxos selected for shielding will be locked. If there is an error, they are unlocked.
/// * The RPC call listlockunspent can be used to return a list of locked utxos. The number of coinbase utxos selected for shielding can be limited by the caller. If the limit parameter is set to zero, the mempooltxinputlimit option will determine the number of uxtos. Any limit is constrained by the consensus rule defining a maximum transaction size of 100000 bytes.
/// # Arguments
/// * "fromaddress" 	(string, required) 	the address is a t address or "*" for all t address belonging to the wallet
/// * "toaddress" 	(string, required) 	the address is a z address
/// * fee 	(numeric, optional, default=0.0001) 	the fee amount to attach to this transaction
/// * limit 	(numeric, optional, default=50) 	limit on the maximum number of utxos to shield; set to 0 to use node option mempooltxinputlimit
/// # Response
/// * "remainingUTXOs" 	(numeric) 	the number of coinbase utxos still available for shielding
/// * "remainingValue" 	(numeric) 	the value of coinbase utxos still available for shielding
/// * "shieldingUTXOs" 	(numeric) 	the number of coinbase utxos being shielded
/// * "shieldingValue" 	(numeric) 	the value of coinbase utxos being shielded
/// * "opid" 	(string) 	an operationid to pass to z_getoperationstatus to get the result of the operation
/// * %%%

pub fn z_shield_coinbase(
    SomeUser: komodorpcutil::KomodoRPC,
    from_address: String,
    to_address: String,
    fee: Option<f32>,
    limit: Option<u32>,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("z_shieldcoinbase");
    let method_body: String;
    let temp_fee: f32 = fee.unwrap_or(0.0001);
    let temp_limit: u32 = limit.unwrap_or(50);

    method_body = String::from("[\"")
        + &from_address.to_string()
        + &String::from("\", \"")
        + &to_address.to_string()
        + &String::from("\", ")
        + &temp_fee.to_string()
        + &String::from(",")
        + &temp_limit.to_string()
        + &String::from("]");

    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

//  TODO #zcbenchmark - unfinished, no example provided on Komodo documentation
/// * zcbenchmark benchmarktype samplecount
/// * The zcbenchmark method runs a benchmark of the selected benchmarktype. This benchmark is calculated samplecount times.
/// * When finished, the method returns the running times of each sample.
/// # Arguments
/// * "benchmarktype" 	(string, required) 	the type of the benchmark
/// * "samplecount" 	(numeric) 	the number of samples to take
/// # Response
/// * runningtime" 	(numeric) 	the time it took to run the selected benchmarktype
/// %%%
pub fn z_cbenchmark(
    SomeUser: komodorpcutil::KomodoRPC,
    benchmark_type: String,
    sample_count: u32,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("zcbenchmark");
    let method_body: String;

    method_body = String::from("[\"")
        + &benchmark_type.to_string()
        + &String::from("\", \"")
        + &sample_count.to_string()
        + &String::from("]");

    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}
