#![allow(warnings)]
use super::komodorpcutil;
use komodorpcutil::KomodoRPC;

/*The get_block_subsidy method returns the block-subsidy reward.
The resulting calculation takes into account the mining slow start.
This method can be used in conjunction with custom mining rewards designed by the developers of a KMD-based Smart Chain.
# Arguments
Name	    Type	            Description
"height"	(u32, optional)	    the block height
# Response
Name	    Type     	        Description
"miner" 	(u32)	            the mining reward amount
*/

pub fn get_block_subsidy(
    some_user: komodorpcutil::KomodoRPC,
    height_supplied: Option<u32>,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("getblocksubsidy");
    let method_body: String;
    let height = height_supplied.unwrap_or(0);
    if height >= 0 {
        method_body = String::from(format!("[{0}]", height));
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
/*The getblocktemplate method returns data that is necessary to construct a block.
If the request parameters include a mode key, it is used to explicitly select between the default 'template' request, a 'proposal' or 'disablecb'.
# Arguments
Name	         Type	                Description
"mode"	         (string, optional)	    the block height
"capabilities"   (array, optional)      a list of strings
"support"        (string)               client side supported features: "longpoll", "coinbasetxn", "coinbasevalue", "proposal", "serverlist", "workid"
# Response
Large amount of return values.
*/
pub fn get_block_template(
    some_user: komodorpcutil::KomodoRPC,
    mode_supplied: Option<String>,
    capabilities: Vec<String>,
    support: String,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("getblocktemplate");
    let method_body: String;
    let mode = mode_supplied.unwrap_or("".to_string());
    let mut cap_list = String::from("[");
    if (capabilities[0] != "".to_string()) {
        for cap in &capabilities {
            cap_list = cap_list + "\"" + cap + "\"" + &",";
        }
        cap_list = cap_list[0..(cap_list.len() - 1)].to_string(); //cut last ,
        cap_list = cap_list + &"]";
    } else {
        cap_list = "".to_string();
    }
    if (mode != "".to_string()) {
        method_body = String::from("{\"mode\":\"")
            + &mode.to_string()
            + &String::from(", \"capabilities\":")
            + &cap_list
            + &String::from(", \"support\":")
            + &support.to_string()
            + "}";
    } else {
        method_body = String::from("{\"capabilities\":")
            + &cap_list
            + &String::from(", \"support\":")
            + &support.to_string()
            + "}";
    }

    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

/*The get_local_solps method returns average local solutions per second since this node was started.
# Arguments
Name	    Type	            Description
(none)      (none)
# Response
Name	    Type     	        Description
"data" 	    (u32)	            the solutions-per-second average
*/

pub fn get_local_solps(some_user: komodorpcutil::KomodoRPC) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("getlocalsolps");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

/*The getmininginfo method returns a json object containing mining-related information.
# Arguments
Name	    Type	            Description
# Response
Name	            Type     	        Description
"blocks"	        (numeric)	        the current block
"currentblocksize"	(numeric)	        the last block size
"currentblocktx"	(numeric)	        the last block transaction
"difficulty"	    (numeric)	        the current difficulty
"errors":
"generate"	        (boolean)	        if the generation is on or off (see getgenerate or setgenerate calls)
"genproclimit"	    (numeric)	        the processor limit for generation; -1 if no generation (see getgenerate or setgenerate calls)
"localsolps"	    (numeric)	        the average local solution rate (solutions per second) since this node was started
"networksolps"	    (numeric)	        the estimated network solution rate (solutions per second)
"pooledtx":
"testnet"	        (boolean)	        if using testnet or not
"chain"	            (string)	        the current network name as defined in BIP70 (main, test, regtest)
*/

pub fn get_mining_info(some_user: komodorpcutil::KomodoRPC) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("getmininginfo");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

/*The getnetworksolps method returns the estimated network solutions per second based on the last n blocks.
Pass in blocks to override the default number of blocks.
Use -1 to calculate according to the relevant difficulty averaging window.
Pass in height to estimate the network speed at the time when a certain block was found.
# Arguments
Name	    Type	            Description
"blocks"	(u32, optional)	    the number of blocks   (Defaults to 120)
"height"    (u32, optional)     the block height that corresponds to the requested data    (Defaults to -1)
# Response
Name	    Type     	        Description
"data"   	(u32)	            solutions per second, estimated
*/

pub fn get_network_solps(
    some_user: komodorpcutil::KomodoRPC,
    blocks_supplied: Option<u32>,
    height_supplied: Option<u32>,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("getnetworksolps");
    let method_body: String;
    let blocks = blocks_supplied.unwrap_or(120);
    let height = height_supplied.unwrap_or(1);

    method_body = String::from("[\"blocks\": \"")
        + &blocks.to_string()
        + &String::from(" \", \"height\" : ")
        + &height.to_string()
        + &String::from("]");
    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(some_user.clone(), data)
}

/* The prioritisetransaction method instructs the daemon to accept the indicated transaction into mined blocks at a higher (or lower) priority.
The transaction selection algorithm considers the transaction as it would have a higher priority.
# Arguments
Name	            Type	            Description
"transaction_id"	(string, required)	the transaction id
"priority_delta"    (u32, required)     the priority to add or subtract (if negative).
                                        The transaction selection algorithm assigns the tx a higher or lower priority.
                                        The transaction priority calculation: coinage * value_in_satoshis / txsize
"fee_delta"         (u32, required)     the fee value in satoshis to add or subtract (if negative);
                                        the fee is not actually paid, only the algorithm for selecting transactions into a block considers the transaction as if it paid a higher (or lower) fee
# Response
Name	    Type     	        Description
"true"   	(boolean)	        returns true
*/
pub fn prioritise_transaction(
    some_user: komodorpcutil::KomodoRPC,
    transaction_id: String,
    priority_delta: u32,
    fee_delta: u32,
) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("prioritisetransaction");
    let method_body: String;
    method_body = String::from("[\"")
        + &transaction_id.to_string()
        + &String::from(" \", ")
        + &priority_delta.to_string()
        + &String::from(",")
        + &fee_delta.to_string()
        + &String::from("]");
    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(some_user.clone(), data)
}

/* submitblock "hexdata" ( "jsonparametersobject" )

The submitblock method instructs the daemon to propose a new block to the network.
# Arguments
"hexdata"	(string, required)	the hex-encoded block data to submit
"jsonparametersobject" : { ... }	(string, optional)	object of optional parameters
"workid"	(string, sometimes optional)	if the server provides a workid, it MUST be included with submissions
# Response
"duplicate"		the node already has a valid copy of the block
"duplicate-invalid"		the node already has the block, but it is invalid
"duplicate-inconclusive"		the node already has the block but has not validated it
"inconclusive"		the node has not validated the block, it may not be on the node's current best chain
"rejected"		the block was rejected as invalid

*/
pub fn submit_block(
    some_user: komodorpcutil::KomodoRPC,
    hexdata: String,
    workid_supplied: Option<String>,
) -> Result<String, reqwest::Error> {
    let workid = workid_supplied.unwrap_or("".to_string());
    let method_name: String = String::from("submitblock");
    let method_body: String;
    if (workid == "".to_string()) {
        method_body = String::from("[\"") + &hexdata.to_string() + &String::from("]");
    } else {
        method_body = String::from("[\"")
            + &hexdata.to_string()
            + &String::from(" \", \" ")
            + &workid.to_string()
            + &String::from("\"]");
    }
    let data: String = String::from(komodorpcutil::generate_body(
        some_user.clone(),
        method_name,
        method_body,
    ));
    println!("the body is{:?}", data);
    let result = komodorpcutil::request(some_user.clone(), data);
    return result;
}
