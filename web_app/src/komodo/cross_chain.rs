#![warn(missing_docs)]
#![allow(warnings)]
//!
//! This is the documentation for 'Cross-Chain API' module of Komodo.
//!
//! The 'Cross-Chain API' module of Komodo contains functionality of the 'Cross-Chain API' noted on the
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
//! [Komodo website]: https://docs.komodoplatform.com/basic-docs/smart-chains/smart-chain-api/crosschain.html
//!

// TODO: - run fmt and clippy
//       - document all methods
//          - more advanced examples
//       - ? potential fix for no function/method overloading if extra time
//              -> trait object static/dynamic dispatch for multi-type parameter
//       - ? some_user parameter may be simplified further

use super::komodorpcutil;
//use komodorpcutil::KomodoRPC;

///
/// The migrate_create_burn_transaction method creates a transaction burning a specific amount of
/// coins or tokens. This method also creates a payouts object which is later used to create an
/// import transaction for the value corresponding to the burned amount. This method should be
/// called on the source chain.
///
/// The method creates a burn transaction and returns it. This should be broadcast to the source chain using the sendrawtransaction method. After the burn transaction is successfully mined, the user might have to wait for some amount of time for the back notarization to reach the source chain. The back notarization contains the MoMoM fingerprints of the mined block that contains the burn transaction.
///
/// The hex value of the burn transaction along with the other returned value payouts are used as arguments for the migrate_createimporttransaction method.
///
/// # Arguments
///
/// * `some_user` - A required KomodoRPC object type that represents the user.
/// * `dest_chain` - A required string type that represents the name of the destination chain.
/// * `dest_address` - A required string that represents the address on the destination chain where coins are to be sent; the pubkey if tokens are to be sent.
/// * `amount` - A required u32 that represents the amount in coins or tokens that should be burned on the source chain and created on the destination chain; if the indicated assets are tokens, the amount can be set only to 1, as only migration of non-fungible tokens are supported at this time.
/// * `token_id` - An optional string that represents the token id in hex; if set, the software assumes that the user is migrating tokens.
///
/// # Response
///
/// * `payouts` - A hex string of the created payouts; this value is passed into the migrate_createimporttransaction method.
/// * `BurnTxHex` - A hex string of the returned burn transaction.
///
/// # Examples
/// ```
///
/// let result = komodo::cross_chain::migrate_create_burn_transaction(some_user,
///                                                                   "CFEKDRAGON".to_string(),
///                                                                   "RBQ1XwmzduHvciRJbXbWY9YBSNtaqZvfC4".to_string(),
///                                                                   7.77u32,
///                                                                   None);
///
/// ```
///
/// TODO: Provide more advanced examples
/// %%%
pub fn migrate_create_burn_transaction(
    some_user: komodorpcutil::KomodoRPC,
    dest_chain: String,
    dest_address: String,
    amount: f64,
    token_id: Option<String>,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("migrate_createburntransaction");
    let method_body: String;
    let temp_token_id: String = token_id.unwrap_or("".to_string());

    // user provides token to migrate
    if !temp_token_id.is_empty() {
        method_body = String::from("[\"")
            + &dest_chain.to_string()
            + &String::from("\",\"")
            + &dest_address.to_string()
            + &String::from("\",\"")
            + &amount.to_string()
            + &String::from("\",\"")
            + &temp_token_id.to_string()
            + &String::from("\"]");
    }
    // user did not provide token
    else {
        method_body = String::from("[\"")
            + &dest_chain.to_string()
            + &String::from("\",\"")
            + &dest_address.to_string()
            + &String::from("\",\"")
            + &amount.to_string()
            + &String::from("\"]");
    }

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

///
/// The migrate_converttoexport method allows the user to create a customized burn transaction (as opposed to a fully automated burn transaction). This method converts a given transaction to a burn transaction.
///
/// The method adds proof data to the transaction, extracts the transaction vouts, calculates their value, and burns the value by sending it to an opreturn vout. This vout is then added to the created transaction. (An opreturn vout cannot be spent at a later date, and therefore funds sent to an opreturn vout are permanently burnt.)
///
/// The other returned value, payouts, is used in the migrate_createimporttransaction method.
///
/// The caller of the method bears the responsibility to fund and sign the returned burn transaction using the methods fundrawtransaction and signrawtransaction.
///
/// The signed burn transaction must be broadcast to the source chain using the sendrawtansaction method.
///
/// # Limitations
///
/// * The migrate_converttoexport method supports only coins (tokens are not supported)
/// * The burn transaction must be stored in the import transaction's opreturn vout. Because an opreturn's data size is limited to 10,001 bytes, we recommend that the user limit the burn transaction's size to 30% of the opreturn object
///
/// # Arguments
///
/// * `burntx` 	(string, required) 	the burn transaction in hex format
/// * `destChain` 	(string, required) 	the name of the destination chain
///
/// # Response
///
/// * `payouts` 	(string) 	a hex string of the created payouts; this is passed into the migrate_createimporttransaction method
/// * `exportTx` 	(string) 	a hex string of the returned burn transaction
/// %%%
pub fn migrate_convert_to_export(
    some_user: komodorpcutil::KomodoRPC,
    burn_tx: String,
    dest_chain: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("migrate_converttoexport");
    //let method_body = format!("[\"{0}\",\"{1}\"]", burn_tx, dest_chain);
    let method_body: String = String::from("[\"")
        + &burn_tx.to_string()
        + &String::from("\",\"")
        + &dest_chain.to_string()
        + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

///
/// The migrate_createimporttransaction method performs the initial step in creating an import transaction. This method should be called on the source chain.
///
/// This method returns a created import transaction in hex format. This string should be passed to the migrate_completeimporttransaction method on the main KMD chain to be extended with the MoMoM proof object.
///
/// When using the MoMoM backup solution (described later), the created import transaction is not passed to the migrate_completeimporttransaction method.
///
/// The user may need to wait for some time before the back notarizations objects are stored in the destination chain.
///
/// # Arguments
///
/// * `burntx` 	(string, required) 	the burn transaction in hex format returned from the previous method
/// * `payouts` 	(string, required) 	the payouts object in hex format returned from the previous method and used for creating an import transaction
/// * `notaryTxid1` 	(string, optional) 	the notary approval transaction id 1, to be passed if the MoMoM backup solution is used for notarization
/// * `notaryTxidN` 	(string, optional) 	the notary approval transaction id N, to be passed if the MoMoM backup solution is used for notarization
///
/// # Response
///
/// * `ImportTxHex` 	(string) 	the created import transaction in hex format
/// %%%
pub fn migrate_create_import_transaction(
    some_user: komodorpcutil::KomodoRPC,
    burn_tx: String,
    payouts: String,
    notary_tx_id1: Option<String>,
    notary_tx_idN: Option<String>,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("migrate_createimporttransaction");
    let temp_notary_tx_id1: String = notary_tx_id1.unwrap_or("".to_string());
    let temp_notary_tx_idN: String = notary_tx_idN.unwrap_or("".to_string());

    let mut method_body: String =
        String::from("[\"") + &burn_tx.to_string() + &String::from("\",\"") + &payouts.to_string();

    // concatentate string for optional strings
    if !temp_notary_tx_id1.is_empty() {
        method_body = method_body + &String::from("\",\"") + &temp_notary_tx_id1.to_string();

        // we want to build up the 'notary_tx_idN' parameter if it exists
        if !temp_notary_tx_idN.is_empty() {
            method_body = method_body + &String::from("\",\"") + &temp_notary_tx_idN.to_string();
        }
    } else if !temp_notary_tx_idN.is_empty() {
        method_body = method_body + &String::from("\",\"") + &temp_notary_tx_idN.to_string();
    }

    method_body = method_body + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

///
/// The migrate_completeimporttransaction method performs the finalizing step in creating an import transaction. This method should be called on the KMD (Komodo) chain.
///
/// This method returns the import transaction in hex format, updated with the MoMoM proof object. This object provides confirmation that the burn transaction exists in the source chain.
///
/// The finalized import transaction should be broadcast on the destination chain through the sendrawtransaction method.
///
/// Komodo recommends that the user wait until the notarization objects are stored in the destination chain before broadcasting the import transaction. Otherwise an error message is returned.
///
/// In the event that an error is returned, simply wait until the notarization objects are stored in the KMD chain and try again.
///
/// # Arguments
///
/// * `importtx` 	(string, required) 	the import transaction in hex format created using the previous method
/// * `offset` 	(string, optional) 	the number of blocks below the current KMD(Komodo) blockchain height in which a MoMoM proof must be searched
///
/// # Response
///
/// * `ImportTxHex` 	(string) 	import transaction in hex extended with the MoMoM proof of burn transaction
/// %%%
pub fn migrate_complete_import_transaction(
    some_user: komodorpcutil::KomodoRPC,
    import_tx: String,
    offset: Option<String>,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("migrate_completeimporttransaction");
    let temp_offset: String = offset.unwrap_or("".to_string());
    let mut method_body: String = String::from("[\"") + &import_tx.to_string();

    if !temp_offset.is_empty() {
        method_body = method_body + &String::from("\",\"") + &temp_offset.to_string();
    }

    method_body = method_body + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

///
/// The migrate_checkburntransactionsource method allows a notary operator to check the burn transaction's structure and verify its presence in the source chain.
///
/// # Arguments
///
/// * `burntxid` 	(string, required) 	the burn transaction's id
///
/// # Response
///
/// * `sourceSymbol` 	(string) 	the source chain's name
/// * `targetSymbol` 	(string) 	the target chain's name
/// * `targetCCid` 	(number) 	the target chain's CCid
/// * `tokenid` 	(string, optional) 	the token id if a token is to be migrated
/// * `TxOutProof` 	(string) 	the proof of the burn transaction's existence in the source chain
/// %%%
pub fn migrate_check_burn_transaction_source(
    some_user: komodorpcutil::KomodoRPC,
    burn_tx_id: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("migrate_checkburntransactionsource");

    let method_body: String = String::from("[\"") + &burn_tx_id.to_string() + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

///
/// A notary operator uses the migrate_createnotaryapprovaltransaction method to create an approval transaction in the destination chain with the proof of the burn transaction's existence in the source chain.
///
/// The returned notary approval transaction should be broadcast to the destination chain using the sendrawtransaction method.
///
/// # Arguments
///
/// * `burntxid` 	(string, required) 	the burn transaction's id
/// * `txoutproof` 	(string, required) 	the proof of the burn transaction's existence
///
/// # Response
///
/// * `NotaryTxHex` 	(string) 	notary approval transaction in hex format
/// %%%
pub fn migrate_create_notary_approval_transaction(
    some_user: komodorpcutil::KomodoRPC,
    burn_tx_id: String,
    tx_out_proof: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("migrate_createnotaryapprovaltransaction");

    let method_body: String = String::from("[\"")
        + &burn_tx_id.to_string()
        + &String::from("\",\"")
        + &tx_out_proof.to_string()
        + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

///
/// The Self Import API allows a trusted pubkey to create more coins on the same chain.
///
/// # Requirements
///
/// * The chain must have the custom parameters -ac_import=PUBKEY and -ac_pubkey set to a pubkey which is allowed to create coins.
///
/// # Self Import Flow
///
/// * For creating more coins in the chain with -ac_import=PUBKEY enabled, use the selfimport method
/// * The method returns a source transaction that contains a parameter with the amount of coins to create
/// * The returned value is a proof of the trusted pubkey owner's intention to create new coins in the chain
/// * The returned source transaction should be broadcast to the chain using the sendrawtransaction method. The source transaction spends a txfee=10000 satoshis from the -ac_pubkey owner's uxtos
/// * After the source transaction is mined, the import transaction should also be broadcasted to the chain with the sendrawtransaction method. After this transaction is mined, its vout contains the amount of created coins in the chosen destination address
///
/// # Arguments
///
/// * `destAddress` 	(string, required) 	the address where the created coins should be sent
/// * `amount` 	(number, required) 	the amount in coins to create
///
/// # Response
///
/// * `SourceTxHex` 	(string) 	the source transaction in hex format
/// * `ImportTxHex` 	(string) 	the import transaction in hex format
/// %%%
pub fn self_import(
    some_user: komodorpcutil::KomodoRPC,
    dest_address: String,
    amount: f64,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("selfimport");

    let method_body: String = String::from("[\"")
        + &dest_address.to_string()
        + &String::from("\",\"")
        + &amount.to_string()
        + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

///
/// The calc_MoM method calculates the value of the merkle root of the blocks' merkle roots (MoM), starting from the block of the indicated height for the chosen depth.
///
/// # Note
///
/// * This method should be run on a Smart Chain.
///
/// # Arguments
///
/// * `height` 	(number, required) 	the block height from which the MoM calculation must begin
/// * `MoMdepth` 	(number, required) 	the number of blocks to include in the MoM calculation
///
/// # Response
///
/// * `coin` 	(string) 	the chain's name
/// * `height` 	(string) 	the starting block height
/// * `MoMdepth` 	(number) 	the number of blocks included in the MoM calculation
/// * `MoM` 	(string) 	the MoM value
/// %%%
pub fn calc_MoM(
    some_user: komodorpcutil::KomodoRPC,
    height: u32,
    MoM_depth: u32,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("calc_MoM");

    let method_body: String = String::from("[\"")
        + &height.to_string()
        + &String::from("\",\"")
        + &MoM_depth.to_string()
        + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

///
/// The MoMoMdata method calculates the value of the merkle root of merkle roots of the blocks' merkle roots (MoMoM), starting from the block of the indicated height for the data of the indicated chain.
///
/// # Note
///
/// * Execute this method on the KMD chain.
///
/// # Arguments
///
/// * `symbol` 	(string, required) 	the chain's name whose data's MoMoM value is to be calculated
/// * `kmdheight` 	(number, required) 	the number of blocks to include in the MoM calculation
/// * `ccid` 	(number, required) 	the chain's CCid
///
/// # Response
///
/// * `coin` 	(string) 	the chain's name
/// * `kmdheight` 	(string) 	the starting block's height
/// * `ccid` 	(number) 	the chain's CCid
/// * `MoMs` 	(string) 	the array of MoM values
/// * `notarizationHash` 	(string) 	the first found notarization transaction id for the chain
/// * `MoMoM` 	(string) 	the MoMoM value
/// %%%
pub fn MoMoM_data(
    some_user: komodorpcutil::KomodoRPC,
    symbol: String,
    kmd_height: u32,
    cc_id: u32,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("MoMoMdata");

    let method_body: String = String::from("[\"")
        + &symbol.to_string()
        + &String::from("\",\"")
        + &kmd_height.to_string()
        + &String::from("\",\"")
        + &cc_id.to_string()
        + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

///
/// The assetchainproof method scans the chain for the back MoM notarization for a transaction corresponding to the given transaction id and returns a proof object with MoM branch. Scanning is performed from the height up to the chain tip, with a limit of 1440 blocks.
///
/// # Arguments
///
/// * `txid` 	(string, required) 	the transaction id for which a proof object must be returned
///
/// # Response
///
/// * `proof object` 	(string) 	the returned proof object with MoM branch in hex format
/// %%%
pub fn asset_chain_proof(
    some_user: komodorpcutil::KomodoRPC,
    tx_id: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("assetchainproof");

    let method_body: String = String::from("[\"") + &tx_id.to_string() + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

///
/// The getNotarisationsForBlock method returns the notarization transactions within the block of the given block hash.
///
/// # Arguments
///
/// * `height` 	(number, required) 	the block number of the block to be searched
///
/// # Response
///
/// * `Notary Cluster` 	(string) 	refers to the notary group which performed the notarizations; KMD for the main Komodo notaries, LABS for the LABS notaries
/// * `txid` 	(string) 	the notarization transaction's id
/// * `chain` 	(string) 	the chain that has been notarized
/// * `height` 	(number) 	the notarization transaction's block height
/// * `blockhash` 	(string) 	the hash of the notarization transaction's block
/// * `notaries` 	(array) 	the ids of the notaries who performed the notarization
///
// get_notarisations_for_block vs scan_notarisations_db
//  both has a required numeric parameter,
//  but get_notar... does not need quotes in param
//  and scan_notar... does need quotes in param
pub fn get_notarisations_for_block(
    some_user: komodorpcutil::KomodoRPC,
    height: u32,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("getNotarisationsForBlock");

    let method_body: String = String::from("[") + &height.to_string() + &String::from("]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

///
/// The scanNotarisationsDB method scans the notarization database backwards from the given block height for a notarization of the chain with the given name (symbol).
///
/// # Arguments
///
/// * `blockHeight` 	(number, required) 	the starting block height from which notarizations are to be searched
/// * `symbol` 	(string, required) 	the chain's name whose notarizations are to be searched
/// * `blocksLimit` 	(number, optional) 	an optional block depth to search for notarizations
///
/// # Response
///
/// * `height` 	(number) 	the block height of the notarization transaction id that has been found
/// * `hash` 	(string) 	the hash of the notarization transaction id that has been found
/// * `opreturn` 	(string) 	the notarization data in hex format
///
// TODO: Fix string parsing for not empty
pub fn scan_notarisations_db(
    some_user: komodorpcutil::KomodoRPC,
    block_height: u32,
    symbol: String,
    blocks_limit: Option<u32>,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("scanNotarisationsDB");
    let temp_blocks_limit: String = blocks_limit.unwrap_or(0).to_string();
    let mut method_body: String = String::from("[\"")
        + &block_height.to_string()
        + &String::from("\",\"")
        + &symbol.to_string();

    if !temp_blocks_limit.is_empty() {
        method_body = method_body + &String::from("\",\"") + &temp_blocks_limit.to_string();
    }

    method_body = method_body + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

///
/// The getimports method lists import transactions in the indicated block of the chain.
///
/// # Note
///
/// * If the transaction id of an import is known, use the gettransaction method to retrieve its block hash.
///
/// # Arguments
///
/// * `hash` or `height` 	(string or number, required) 	the block's hash or height to be searched
///
/// # Response
///
/// * `imports` 	(array)
/// * `txid` 	(string) 	the import transaction id
/// * `amount` 	(number) 	the import transaction's value in coins
/// * `export` 	(json) 	the export or burn transaction's infomation
/// * `txid` 	(string) 	the export transaction's id
/// * `amount` 	(number) 	the export transaction's value
/// * `txid` 	(string) 	the export transaction's id
/// * `source` 	(string) 	the source chain's name
/// * `tokenid` 	(string,optional) 	the source chain's token id, if tokens are imported
/// * `TotalImported` 	(number) 	the total imported amount in coins
///
// NOTE: Komodo doc requires string OR number - currently forcing string only
pub fn get_imports(
    some_user: komodorpcutil::KomodoRPC,
    hash_or_height: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("getimports");
    let method_body: String =
        String::from("[\"") + &hash_or_height.to_string() + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

///
/// The getwalletburntransactions method lists all the burn transactions in the current wallet.
///
/// # Arguments
///
/// * `count` 	(number, optional) 	the number of burn transactions to be returned; if omitted, defaults to 10 burn transactions
///
/// # Response
///
/// * `txid` 	(string) 	the burn transaction's id
/// * `burnedAmount` 	(number) 	the burned value in coins
/// * `tokenid` 	(string, optional) 	the token id, if tokens are burned
/// * `targetSymbol` 	(string) 	the target chain's name
/// * `targetCCid` 	(number) 	the target chain's CCid
/// %%%
pub fn get_wallet_burn_transactions(
    some_user: komodorpcutil::KomodoRPC,
    count: Option<u32>,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("getwalletburntransactions");
    let temp_count = count.unwrap_or(10);
    let method_body: String = String::from("[\"") + &temp_count.to_string() + &String::from("\"]");

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}
