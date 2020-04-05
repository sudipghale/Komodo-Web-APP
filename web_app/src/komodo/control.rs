#![allow(warnings)]
/*
Control
The following RPC calls interact with the komodod software, and are made available through the komodo-cli software
*/

use super::komodorpcutil;
use komodorpcutil::KomodoRPC;

/*
getinfo

The getinfo method returns an object containing various state info.

# Arguments
Name	Type	Description
(none)

# Response
Name	Type	Description
"version"	(numeric)	the server version
"protocolversion"	(numeric)	the protocol version
"walletversion"	(numeric)	the wallet version
"balance"	(numeric)	the total balance of the wallet
"blocks"	(numeric)	the current number of blocks processed in the server
"timeoffset"	(numeric)	the time offset
"connections"	(numeric)	the number of connections
"proxy"	(string, optional)	the proxy used by the server
"difficulty"	(numeric)	the current difficulty
"testnet"	(boolean)	if the server is using testnet or not
"keypoololdest"	(numeric)	the timestamp (seconds since GMT epoch) of the oldest pre-generated key in the key pool
"keypoolsize"	(numeric)	how many new keys are pre-generated
"unlocked_until"	(numeric)	the timestamp in seconds since epoch (midnight Jan 1 1970 GMT) that the wallet is unlocked for transfers, or 0 if the wallet is locked
"paytxfee"	(numeric)	the transaction fee set in COIN/kB
"relayfee"	(numeric)	minimum relay fee for non-free transactions in COIN/kB
"errors"	(string)	any error messages
*/
pub fn get_info(SomeUser: komodorpcutil::KomodoRPC) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("getinfo");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    let result = komodorpcutil::request(SomeUser.clone(), data);
    return result;
}
/**
 *help
help ( "command" )

The help method lists all commands, or all information for a specified command.

# Arguments
Name	Type	Description
"command"	(string, optional)	the command requiring assistance
# Response
Name	Type	Description
"command"	(string, optional)	the command requiring assistance
 *
 */
pub fn help(
    someUser: komodorpcutil::KomodoRPC,
    command: Option<String>,
) -> Result<String, reqwest::Error> {
    //TODO: need to implement optinal arg
    let method_body: String;
    let temp_command = command.unwrap_or("".to_string()); //Default value is 0
    if (!temp_command.is_empty()) {
        method_body = String::from("[\"") + &temp_command + "\"]"; // String::from(format!("[/"{0}/"]",temp_top));
    } else {
        method_body = String::from("[]");
    }

    //let params = String::from("[") + &top.to_string()+"]";

    let method_name: String = String::from("help");

    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));

    let result = komodorpcutil::request(someUser.clone(), data);
    return result;
}
/**
 * stop

The stop method instructs the coin daemon to shut down.

The amount of time it takes to shut down the chain will vary depending on the chain's current state.

Forcefully stopping the chain should be avoided, as it may corrupt the local database. In the event of a corrupted database, the user will need to resync.

# Arguments
Name	Type	Description
(none)
# Response
Name	Type	Description
Komodo server stopping
[COIN] Komodo server stopping
 */
pub fn stop(SomeUser: komodorpcutil::KomodoRPC) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("stop");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    let result = komodorpcutil::request(SomeUser.clone(), data);
    return result;
}
