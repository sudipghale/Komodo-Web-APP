#![warn(missing_docs)]
//!
//! This is the documentation for 'Block chain' module of Komodo.
//!
//! The 'Block chain' module of Komodo contains functionality of the 'Block Chain' noted on the
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
//! use super::komodorpcutil;
//! use komodorpcutil::KomodoRPC;
//! use std::any::Any;
//! ```
//! [Komodo website]: https://docs.komodoplatform.com/basic-docs/smart-chains/smart-chain-api/address.html
//!

use super::komodorpcutil;
use komodorpcutil::KomodoRPC;
use std::any::Any;

/// The coinsupply method returns the coin supply information for the indicated block height.
/// If no height is given, the method defaults to the blockchain's current height.
/// # Arguments
/// * `height`	(integer, optional)	the desired block height
///# Response
/// * `result`	(string)	whether the request was successful
/// * `coin`	(string)	the ticker symbol of the coin for Smart Chains, otherwise KMD
/// * `height`	(integer)	the height of this coin supply data
/// * `supply`	(float)	the transparent coin supply
/// * `zfunds`	(float)	the shielded coin supply (in zaddrs)
/// * `sprout`	(float)	the sprout coin supply (in zcaddrs)
/// * `total`	(float)	the total coin supply, i.e. sum of supply + zfunds
/// %%%
pub fn coin_supply(
    some_user: komodorpcutil::KomodoRPC,
    height_supplied: Option<u32>,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("coinsupply");
    let method_body: String;

    let height = height_supplied.unwrap_or(0); //Default value is 0
    if height > 0 {
        method_body = String::from(format!("[\"{0}\"]", height));
    } else {
        method_body = String::from("[]");
    }
    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

/// The getbestblockhash method returns the hash of the best (tip) block in the longest block chain.
/// # Arguments
/// * `(none)`
/// # Response
/// * `hex`	(string)	the block hash, hex encoded
pub fn get_best_block_hash(some_user: komodorpcutil::KomodoRPC) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("getbestblockhash");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}
/// The getblock method returns the block's relevant state information.
/// The verbose input is optional. The default value is true, and it will return a json object
/// with information about the indicated block. If verbose is false,
/// the command returns a string that is serialized hex-encoded data
/// for the indicated block.
/// # Arguments
/// * `hash OR height`	(string OR number), respectively	the block hash OR the block height
/// * `verbose`	(boolean, optional, default=true)	true returns a json object, false returns hex-encoded data
///
/// # Response (verbose = true)
/// * `hash`	(string)	the block hash (same as provided hash)
/// * `confirmations`	(numeric)	a confirmation number that is aware of the dPoW security service
/// * `rawconfirmations`	(numeric)	the raw confirmations (number of blocks on top of this block); the returned value is -1 if the block is not on the main chain
/// * `size`	(numeric)	the block size
/// * `height`	(numeric)	the block height or index (same as provided height)
/// * `version`	(numeric)	the block version
/// * `merkleroot`	(string)	the merkle root
/// * `tx` : [ `transaction_id` ,...]	(array of strings)
/// * `time`	(numeric)	the block time in seconds since epoch (Jan 1 1970 GMT)
/// * `nonce`	(numeric)	the nonce
/// * `bits`	(string)	the bits
/// * `difficulty`	(numeric)	the difficulty
/// * `previousblockhash`	(string)	the hash of the previous block
/// * `nextblockhash`	(string)	the hash of the next block
///
/// # Response (Verbose = False)
/// * `data` (string)	a string that is serialized, hex-encoded data for the indicated block
/// %%%
pub fn get_block(
    SomeUser: komodorpcutil::KomodoRPC,
    height_or_hash: String,
    verbose: Option<bool>,
) -> Result<String, reqwest::Error> {
    let temp_verbose: bool = verbose.unwrap_or(false);
    let method_name: String = String::from("getblock");
    let method_body: String;
    // if verbose is true

    if temp_verbose {
        method_body = String::from("[\"") + &height_or_hash + &String::from("\", true]");
    } else {
        method_body = String::from("[\"") + &height_or_hash + &String::from("\", false]");
    }

    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}
/// The getblockchaininfo method returns a json object containing state information about blockchain processing.
///
/// # Arguments
///	* `(none)`
///
/// # Response
/// * `chain`	(string)	the current network name, as defined in BIP70 (main, test, regtest)
/// * `blocks`	(numeric)	the current number of blocks processed in the server
/// * `headers`	(numeric)	the current number of headers we have validated
/// * `bestblockhash`	(string)	the hash of the currently best block
/// * `difficulty`	(numeric)	the current difficulty
/// * `verificationprogress`	(numeric)	an estimate of verification progress [0..1]
/// * `chainwork`	(string)	the total amount of work in the active chain, in hexadecimal
/// * `pruned`	(bool)	whether the current state is in pruning mode; if true, the blockchain will not keep all transaction and block information, to preserve disk space
/// * `size_on_disk`	(numeric)	the size of the blockchain on disk, measured in bytes
/// * `commitments`	(numeric)	the current number of note commitments in the commitment tree
/// * `softforks`: { ..... }	(array)	the status of softforks in progress
/// * `id`	(string)	the name of the softfork
/// * `version`	(numeric)	the block version
/// * `enforce`: { ... }	(object)	the progress toward enforcing the softfork rules for blocks of the new version
/// * `status`	(boolean)	true if threshold reached
/// * `found`	(numeric)	the number of blocks with the new version found
/// * `required`	(numeric)	the number of blocks required to trigger
/// * `window`	(numeric)	the maximum size of the examined window of recent blocks
/// * `reject`: { ... }	(object)	the progress toward rejecting pre-softfork blocks (same fields as `enforce`)
/// * `upgrades`:	(object)	the status of network upgrades
/// * `xxxxxxxxx_string`:	(string)	the branch ID of the upgrade
/// * `name`	(string)	the name of upgrade
/// * `activationheight`	(numeric)	the block height of activation
/// * `status`	(string)	the status of the upgrade
/// * `info`	(string)	additional information about the upgrade
/// * `consensus`: { ..... }	(object)	branch IDs of the current and upcoming consensus rules
/// * `chaintip`	(string)	branch ID used to validate the current chain tip
/// * `nextblock`	(string)	branch ID under which the next block will be validated
/// %%%
pub fn get_blockchain_info(some_user: komodorpcutil::KomodoRPC) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("getblockchaininfo");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

///	The getblockcount method returns the number of blocks in the best valid block chain.
///
/// # Arguments
///	* `(none)`
///
/// # Response
/// data	(numeric)	the current block count
/// %%%

pub fn get_block_count(SomeUser: komodorpcutil::KomodoRPC) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("getblockcount");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

///The getblockhash method returns the hash of the indicated block index, according to the best blockchain at the time provided.
/// # Arguments
/// * `index`	(numeric, required)	the block index
/// # Response
/// * `hash`	(string)	the block hash
pub fn get_block_hash(
    SomeUser: komodorpcutil::KomodoRPC,
    index: u32,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("getblockckhash");
    let method_body: String = String::from(format!("[{:?}]", index));
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}
///The getblockhashes method returns an array of hashes of blocks within the timestamp range provided.
///The method requires timestampindex to be enabled.
/// # Arguments
/// * `high`	(numeric, required)	the newer block timestamp
/// * `low`	(numeric, required)	the older block timestamp
/// * `options`	(string, required)	a json object
/// * `noOrphans`	(boolean)	a value of true implies that the method will only include blocks on the main chain
/// * `logicalTimes`	(boolean)	a value of true implies that the method will only include logical timestamps with hashes
/// # Response
/// * `hash`	(string)	the block hash
/// * `blockhash`	(string)	the block hash
/// * `logicalts`	(numeric)	the logical timestamp
/// %%%
///
/// SKIPPED DUE TO LACK OF PROPER DOCUMENTATION IN KOMODO PLATFORM
pub fn get_block_hashes(
    SomeUser: komodorpcutil::KomodoRPC,
    high: u32,
    low: u32,
    no_orphans: bool,
    logical_times: bool,
) -> Result<String, reqwest::Error> {
    let method_body: String;
    let temp_no_orphans: String;
    let temp_logical_times: String;
    if no_orphans {
        temp_no_orphans = String::from("true");
    } else {
        temp_no_orphans = String::from("false");
    }

    if logical_times {
        temp_logical_times = String::from("true");
    } else {
        temp_logical_times = String::from("false");
    }

    let json_data_for_request = String::from(format!(
        "{{\"noOrphans\":{0}, \"logicalTimes\":{1}}}",
        temp_no_orphans, temp_logical_times
    ));
    /*method_body = String::from("[")
                    + &high.to_string()
                    + &String::from(",")
                    + &low.to_string()
                    + &String::from(",\"")
                    + &json_data_for_request
                    + &String::from("\"]");
                    print!("{}", method_body);
    */
    //alternative
    method_body = String::from("[")
        + &high.to_string()
        + &String::from(",")
        + &low.to_string()
        + &String::from("]");

    let method_name: String = String::from("getblockhashes");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    let result = komodorpcutil::request(SomeUser.clone(), data);
    return result;
}

/// The getblockheader method returns information about the indicated block.
/// The verbose input is optional. If verbose is false, the method returns a string that is serialized,
/// hex-encoded data for the indicated blockheader. If verbose is true, the method returns a json object
/// with information about the indicated blockheader.
///
/// # Arguments
/// * `hash`	(string, required)	the block hash
/// * `verbose`	(boolean, optional, default=true)	true returns a json object, false returns hex-encoded data
/// # Response (verbose = `true`)
/// * `hash`	(string)	the block hash (same as provided)
/// * `confirmations`	(numeric)	a confirmation number that is aware of the dPoW security service
/// * `rawconfirmations`	(numeric)	the raw confirmations (number of blocks on top of this block); if the block is not on the main chain, a value of -1 is returned
/// * `height`	(numeric)	the block height or index
/// * `version`	(numeric)	the block version
/// * `merkleroot`	(string)	the merkle root
/// * `time`	(numeric)	the block time in seconds since epoch (Jan 1 1970 GMT)
/// * `nonce`	(numeric)	the nonce
/// * `bits`	(string)	the bits
/// * `difficulty`	(numeric)	the difficulty
/// * `previousblockhash`	(string)	the hash of the previous block
/// * `nextblockhash`	(string)	the hash of the next block
/// # Response (verbose = `false`)
/// * `data`	(string)	a string that is serialized hex-encoded data for the indicated block
/// %%%
///
pub fn get_block_header(
    SomeUser: komodorpcutil::KomodoRPC,
    hash: String,
    verbose: Option<bool>,
) -> Result<String, reqwest::Error> {
    /*
    Arguments
    Name	Type	Description
    "hash"	(string, required)	the block hash
    verbose	(boolean, optional, default=true)	true returns a json object, false returns hex-encoded data
    */
    let temp_verbose: bool = verbose.unwrap_or(true);
    let method_name: String = String::from("getblockheader");
    let method_body: String;
    // if verbose is true

    if temp_verbose {
        method_body = String::from("[\"") + &hash + &String::from("\", true]");
    } else {
        method_body = String::from("[\"") + &hash + &String::from("\", false]");
    }

    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    let result = komodorpcutil::request(SomeUser.clone(), data);
    return result;
}

///The getchaintips method returns information about all known tips
/// in the block tree, including the main chain and any orphaned branches.
/// # Arguments
/// * `(none)`
/// # Response
/// *`height`	(numeric)	the height of the chain tip
/// *`hash`	(string)	the block hash of the tip
/// *`branchlen`	(numeric)	0 for main chain
/// *`status`	(string)	`active` for the main chain
/// *`height`	(numeric)	the height of the branch tip
/// *`hash`	(string)	the blockhash of the branch tip
/// *`branchlen`	(numeric)	the length of the branch connecting the tip to the main chain
/// *`status`	(string)	the status of the chain
/// %%%
pub fn get_chain_tips(SomeUser: komodorpcutil::KomodoRPC) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("getchaintips");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

/// The method getchaintxstats returns statistics about
/// the total number and rate of transactions in the chain.
///
/// # Arguments
/// * `nblocks`	(numeric, optional)	the number of blocks in the averaging window.
/// * `blockhash`	(string, optional)	the hash of the block which ends the window
/// # Response
/// * `time`	(numeric)	the timestamp for the final block in the window in UNIX format
/// * `txcount`	(numeric)	the total number of transactions in the chain up to this point
/// * `window_final_block_hash`	(string)	the hash of the final block in the window
/// * `window_block_count`	(numeric)	the size of the window in the number of blocks
/// * `window_tx_count`	(numeric)	the number of transactions in the window; this value is only returned if window_block_count is > 0.
/// * `window_interval`	(numeric)	the elapsed time in the window in seconds; this value is only returned if window_block_count is > 0.
/// * `txrate`	(numeric)	the average rate of transactions per second in the window; this value is only returned if window_interval is > 0.
/// %%%
pub fn get_chain_tx_stats(
    SomeUser: komodorpcutil::KomodoRPC,
    n_blocks: Option<u32>,
    block_hash: Option<String>,
) -> Result<String, reqwest::Error> {
    let method_body: String;
    let temp_nblocks = n_blocks.unwrap_or(0); //Default value is 0
    let temp_block_hash: String = block_hash.unwrap_or("".to_string());
    if (temp_nblocks > 0) && !(temp_block_hash.is_empty()) {
        method_body = String::from("[\"blockhash\": \"")
            + &temp_block_hash.to_string()
            + &String::from(" \", \"nblocks\" : ")
            + &temp_nblocks.to_string()
            + &String::from("]");
    } else if (temp_nblocks <= 0) && !(temp_block_hash.is_empty()) {
        method_body = String::from("[\"blockhash\": \"")
            + &temp_block_hash.to_string()
            + &String::from(" \"]");
    } else if (temp_nblocks > 0) && (temp_block_hash.is_empty()) {
        method_body =
            String::from("[\"nblocks\": ") + &temp_nblocks.to_string() + &String::from("]");
    } else {
        method_body = String::from("[]");
    }
    let method_name: String = String::from("getchaintxstats");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

/// The getdifficulty method returns the proof-of-work difficulty as a multiple of the minimum difficulty.
/// # Arguments
/// * `(none)`
/// # Response
/// * `number`	(numeric)	the proof-of-work difficulty as a multiple of the minimum difficulty
/// %%%
pub fn get_difficulty(SomeUser: komodorpcutil::KomodoRPC) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("getdifficulty");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}
///The getlastsegidstakes method returns an object containing the number
/// of blocks staked by each segid in the last X number of blocks,
///  where the value of X is equal to the indicated depth.
///
/// # Arguments
/// * `depth`	(numeric, required)	the number of blocks to scan, starting from the current height and working backwards
/// # Response
/// * `NotSet`	(numeric)	the number of blocks that have no SegId set
///* `PoW`	(numeric)	the number of blocks created through PoW
///* `PoSPerc`	(numeric)	the percentage of blocks created through PoS
///* `SegIds`	(json object)	the json containing the data of number of blocks in each SegId
///* `n`	(numeric)	the number of blocks staked from SegId n in the last X blocks, where X is equal to the indicated depth
/// %%%
pub fn get_last_segid_stakes(
    SomeUser: komodorpcutil::KomodoRPC,
    depth: u32,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("getlastsegidstakes");
    let method_body: String = String::from(format!("[{}]", depth));
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

/// The getmempoolinfo method returns details on the active state of the transaction memory pool
///
/// # Arguments
/// * `(none)`
///
/// # Resonse
/// * `size`	(numeric)	the current transaction count
/// * `bytes`	(numeric)	the sum of all transaction sizes
/// * `usage`	(numeric)	the total memory usage for the mempool
/// %%%
pub fn get_mempool_info(SomeUser: komodorpcutil::KomodoRPC) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("getmempoolinfo");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

///The getrawmempool method returns all transaction ids in the memory pool as a json array of transaction ids.
/// The verbose input is optional and is false by default. When it is true, the method instead returns a json object with various related data.
///
/// # Arguments
/// * `verbose`	(boolean, optional, default=false)	true for a json object, false for a json array of transaction ids
///
/// # Response (verbose = false)
/// * `transaction_id`	(string)	the transaction id
///
/// # Response (verbose = true)
/// * `transaction_id`: { .... }	(json object)
/// * `size`	(numeric)	the transaction size in bytes
/// * `fee`	(numeric)	the transaction fee
/// * `time`	(numeric)	the local time transaction entered pool in seconds since 1 Jan 1970 GMT
/// * `height`	(numeric)	the block height wherein the transaction entered the mempool
/// * `startingpriority`	(numeric)	the priority when the transaction entered the mempool
/// * `currentpriority`	(numeric)	the transaction priority at the current height
/// * `depends`: { ... }	(array)	unconfirmed transactions used as inputs for this transaction
/// * `transaction_id`	(string)	the parent transaction id
/// %%%
pub fn get_raw_mempool(
    SomeUser: komodorpcutil::KomodoRPC,
    verbose: Option<bool>,
) -> Result<String, reqwest::Error> {
    let temp_verbose: bool = verbose.unwrap_or(false);
    let method_name: String = String::from("getrawmempool");

    let method_body: String;
    if temp_verbose {
        method_body = String::from("[true]");
    } else {
        method_body = String::from("[false]");
    }
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

///The getspentinfo method returns the transaction id and index where the given output is spent.
/// The method requires spentindex to be enabled.
/// # Arguments
/// * `txid`	(string)	the hex string of the transaction id
/// * `index`	(number)	the output's index
/// # Response
/// * `txid`	(string)	the transaction id
/// * `index`	(number)	the spending input index
/// %%%
///
pub fn get_spent_info(
    SomeUser: komodorpcutil::KomodoRPC,
    tx_id: String,
    index: u32,
) -> Result<String, reqwest::Error> {
    let method_body: String;
    method_body = String::from("[\"txid\": \"")
        + &tx_id.to_string()
        + &String::from(" \", \"index\" : ")
        + &index.to_string()
        + &String::from("]");
    let method_name: String = String::from("getspentinfo");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}
/// The gettxout method returns details about an unspent transaction output.
/// # Arguments
/// * `txid`	(string, required)	the transaction id
/// * `vout`	(numeric, required)	the vout value
/// * `includemempool`	(boolean, optional)	whether to include the mempool
/// # Response
/// * `bestblock`	(string)	the block hash
/// * `confirmations`	(numeric)	a confirmation number that is aware of the dPoW security service aware
/// * `rawconfirmations`	(numeric)	the raw confirmations (number of blocks on top of this block with this transaction)
/// * `value`	(numeric)	the transaction value
/// * `scriptPubKey`:	(json object)
/// * `asm`	(string)	scriptPubKey in assembly format
/// * `hex`	(string)	scriptPubKey in hex format
/// * `reqSigs`	(numeric)	the number of required signatures
/// * `type`	(string)	the type, e.g. pubkeyhash
/// * `addresses`	(array of strings)	an array of Komodo addresses
/// * `address`	(string)	the blockchain address
/// * `version`	(numeric)	the version
/// * `coinbase`	(boolean)	whether this is a coinbase transaction
/// %%%
pub fn get_tx_out(
    SomeUser: komodorpcutil::KomodoRPC,
    tx_id: String,
    vout: u32,
    include_mempool: Option<bool>,
) -> Result<String, reqwest::Error> {
    let method_body: String;
    if let Some(x) = include_mempool {
        method_body = String::from("[\"")
            + &tx_id.to_string()
            + &String::from(" \", ")
            + &vout.to_string()
            + &String::from(format!(", {:?}", x))
            + &String::from("]");
    } else {
        method_body = String::from("[\"")
            + &tx_id.to_string()
            + &String::from(",")
            + &vout.to_string()
            + &String::from(" ]");
    }
    let method_name: String = String::from("gettxout");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}
///The gettxoutproof method returns a hex-encoded proof showing that the indicated transaction was included in a block.
///The gettxoutproof method relies on the txindex runtime parameter.
/// This parameter is enabled by default on all KMD-based blockchains, and should never be disabled.
/// # Arguments
/// * `txid`	(string)	a transaction hash
/// * `blockhash`	(string, optional)	if specified, the method looks for the relevant transaction id in this block hash
/// # Response
/// * `data`	(string)	a string that is a serialized, hex-encoded data for the proof
/// %%%
pub fn get_tx_out_proof(
    SomeUser: komodorpcutil::KomodoRPC,
    tx_id: String,
    block_hash: Option<String>,
) -> Result<String, reqwest::Error> {
    let temp_block_hash: String = block_hash.unwrap_or("".to_string());
    let method_body: String;
    if (!temp_block_hash.is_empty()) {
        method_body = String::from("[\"txid\": \"")
            + &tx_id.to_string()
            + &String::from(" \", \"blockhash\" : ")
            + &temp_block_hash.to_string()
            + &String::from(" ]");
    } else {
        method_body = String::from("[\"txid\": \"") + &tx_id.to_string() + &String::from(" \"]");
    }
    let method_name: String = String::from("gettxoutproof");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

/// The gettxoutsetinfo method returns statistics about the unspent transaction output set.
/// # Arguments
/// * `(none)`
///
/// # Response
/// * `height`	(numeric)	the current block height (index)
/// * `bestblock`	(string)	the best block hash hex
/// * `transactions`	(numeric)	the number of transactions
/// * `txouts`	(numeric)	the number of output transactions
/// * `bytes_serialized`	(numeric)	the serialized size
/// * `hash_serialized`	(string)	the serialized hash
/// * `total_amount`	(numeric)	the total amount
pub fn tx_out_set_info(SomeUser: komodorpcutil::KomodoRPC) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("gettxoutsetinfo");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

/// The kvsearch method searches for a key stored via the kvupdate command.
/// This feature is only available for Smart Chains.
/// # Arguments
/// `key`	(string, required)	the key for which the user desires to search the chain
///
/// # Response
/// * `coin`	(string)	the chain on which the key is stored
/// * `currentheight`	(numeric)	the current height of the chain
/// * `key`	(string)	the key
/// * `keylen`	(string)	the length of the key
/// * `owner`	(string)	a hex string representing the owner of the key
/// * `height`	(numeric)	the height at which the key was stored
/// * `expiration`	(numeric)	the height at which the key will expire
/// * `flags`	(numeric)	1 if the key was created with a password; 0 otherwise
/// * `value`	(string)	the stored value
/// * `valuesize`	(string)	the amount of characters stored
pub fn kv_search(
    SomeUser: komodorpcutil::KomodoRPC,
    key: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("kvsearch");
    let method_body: String = String::from("[\"") + &key.to_string() + &String::from("\"]");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

///The kvupdate method stores a key/value pair via OP_RETURN.
/// This feature is available only for Smart Chains. The maximum value memory size is 8kB.
///
/// # Arguments
/// * `key`	(string, required)	key (should be unique)
/// * `value`	(string, required)	value
/// * `days`	(numeric, required)	amount of days before the key expires (1440 blocks/day); minimum 1 day
/// * `passphrase`	(string, optional)	passphrase required to update this key
///
/// # Response
/// * `coin`	(string)	the chain on which the key is stored
/// * `height`	(numeric)	the height at which the key was stored
/// * `expiration`	(numeric)	the height at which the key will expire
/// * `flags`	(string)	the amount of days the key will be stored
/// * `key`	(numeric)	the stored key
/// * `keylen`	(numeric)	the length of the key
/// * `value`	(numeric)	the stored value
/// * `valuesize`	(string)	the length of the stored value
/// * `fee`	(string)	the transaction fee paid to store the key
/// * `txid`	(string)	the transaction id
/// %%%
pub fn kv_update(
    SomeUser: komodorpcutil::KomodoRPC,
    key: String,
    value: String,
    days: u32,
    pass_phrase: Option<String>,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("kvupdate");
    let method_body: String;
    let temp_pass_phrase: String = pass_phrase.unwrap_or("".to_string());
    if (temp_pass_phrase.is_empty()) {
        method_body = String::from("[\"")
            + &key.to_string()
            + &String::from("\", \"")
            + &value.to_string()
            + &String::from("\", \"")
            + &days.to_string()
            + &String::from("\"]");
    } else {
        method_body = String::from("[\"")
            + &key.to_string()
            + &String::from("\", \"")
            + &value.to_string()
            + &String::from("\", \"")
            + &days.to_string()
            + &String::from("\", \"")
            + &temp_pass_phrase.to_string()
            + &String::from("\"]");
    }
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

///The minerids method returns information about the notary nodes and external miners at a specific block height.
/// The response will calculate results according to the 2000 blocks proceeding the indicated "height" block.
/// # Arguments
/// * `heights`	(number)	the block height for the query
/// # Response
/// * `mined`:
/// * `notaryid`	(number)	the id of the specific notary node
/// * `kmdaddress`	(string)	the KMD address of the notary node
/// * `pubkey`	(string)	the public signing key of the notary node
/// * `blocks`	(number)
/// %%%
pub fn miner_ids(
    SomeUser: komodorpcutil::KomodoRPC,
    height: u32,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("minerids");
    let method_body: String = String::from(format!("[\"{:?}\"]", height));
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    let result = komodorpcutil::request(SomeUser.clone(), data);
    return result;
}

/// The notaries method returns the public key, BTC address, and KMD address for each Komodo notary node.
/// Either or both of the height and timestamp parameters will suffice.
/// # Arguments
/// * `height`	(number)	the block height desired for the query
/// * `timestamp`	(number)	the timestamp of the block desired for the query
/// # Response
/// * `notaries`: [ ... ]	(array)
/// * `pubkey`	(string)	the public signing key of the indicated notary node, used on the KMD network to create notary-node authorized transactions
/// * `BTCaddress`	(string)	the public BTC address the notary node uses on the BTC blockchain to create notarizations
/// * `KMDaddress`	(string)	the public KMD address the notary node uses on the KMD blockchain to create notarizations
/// * `numnotaries`	(number)	the number of notary nodes; typically this value is 64, but the value may vary on rare circumstances, such as during election seasons
/// * `height`	(number)	the block height number at which the notary-node information applies
/// * `timestamp`	(number)	the timestamp at which the notary-node information applies
/// %%%
pub fn notaries(
    SomeUser: komodorpcutil::KomodoRPC,
    height: u32,
    timestamp: u32,
) -> Result<String, reqwest::Error> {
    //????? TODO Review this method again ??????
    // what happens when two parameters are sent together
    // Do we need to name the paramater too?
    let method_name: String = String::from("notaries");
    let method_body: String;

    method_body = String::from("[\"")
        + &height.to_string()
        + &String::from("\",\"")
        + &timestamp.to_string()
        + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    let result = komodorpcutil::request(SomeUser.clone(), data);
    return result;
}

///  The verifychain method verifies the coin daemon's blockchain database.
///  Depending on the state of your blockchain database and daemon, this call can take a prolonged period of time to complete.
/// # Arguments
/// * `checklevel`	(numeric, optional, 0-4, default=3)	indicates the thoroughness of block verification
/// * `numblocks`	(numeric, optional, default=288, 0=all)	indicates the number of blocks to verify
/// # Response
/// * `true/false`	(boolean)	whether the verification was successful
/// %%%
pub fn verify_chain(
    SomeUser: komodorpcutil::KomodoRPC,
    check_level: Option<u8>,
    num_blocks: Option<u16>,
) -> Result<String, reqwest::Error> {
    let temp_check_level: u8 = check_level.unwrap_or(3);
    let temp_num_blocks: u16 = num_blocks.unwrap_or(288);
    let method_name: String = String::from("verifychain");
    let method_body = String::from("[")
        + &temp_check_level.to_string()
        + &String::from(",")
        + &temp_num_blocks.to_string()
        + &String::from("]");

    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    let result = komodorpcutil::request(SomeUser.clone(), data);
    return result;
}
/// The verifytxoutproof method verifies that a proof points to a transaction in a block.
/// It returns the transaction to which the proof is committed,
/// or it will throw an RPC error if the block is not in the current best chain.
/// # Arguments
/// * `proof_string`	(string, required)	the hex-encoded proof generated by gettxoutproof
/// # Response
/// * `txid` (string)	the transaction ids to which the proof commits; the array is empty if the proof is invalid
/// %%%

pub fn verify_tx_out_proof(
    SomeUser: komodorpcutil::KomodoRPC,
    proof_string: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("verifytxoutproof");
    let method_body: String =
        String::from("[\"") + &proof_string.to_string() + &String::from("\"]");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}
