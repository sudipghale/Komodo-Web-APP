#![allow(warnings)]
use super::komodorpcutil;
use komodorpcutil::KomodoRPC;

/*getaddressbalance
getaddressbalance '{ "addresses" : [ "address" , ... ] }'

The getaddressbalance method returns the confirmed balance for an address, or addresses. It requires addressindex to be enabled.

#Arguments
Name	     Type      	Description
"address"	(string)	the address

# Response
Name	    Type	     Description
"balance"	(number)	the current confirmed balance in satoshis
"received"	(number)	the total confirmed number of satoshis received (including change)
*/
pub fn get_address_balance(
    someUser: komodorpcutil::KomodoRPC,
    v_address: Vec<String>,
) -> Result<String, reqwest::Error> {
    // implement for vec of addrs
    let mut addr_list = String::from("[");
    for addr in &v_address {
        addr_list = addr_list + "\"" + addr + "\"" + &","; //parsing error
    }

    if (v_address.len() > 0)
    // >1
    {
        addr_list = addr_list[0..(addr_list.len() - 1)].to_string(); // cutting the last ,
    }

    addr_list = addr_list + &"]"; //& for String -> &string

    let params = String::from("[{\"addresses\": ") + &addr_list + "}]";

    let method_name: String = String::from("getaddressbalance");
    let method_body: String = String::from(params);
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));
    let result = komodorpcutil::request(someUser.clone(), data);
    return result;
}

/*

getaddressdeltas
getaddressdeltas '{ "addresses" : [ "address" , ... ] }'

getaddressdeltas '{ "addresses" : [ "address" , ... ] , "start": start, "end": end, "chainInfo": boolean }'

The getaddressdeltas method returns all confirmed balance changes of an address. The user can optionally limit the response to a given interval of blocks. The method requires addressindex to be enabled.

Arguments
Name	    Type	    Description
"address"	(string)	the address
"start"  	(number)	the start block height
"end"	    (number)	the end block height
"chainInfo"	(boolean)	include chain info in results (only applies if start and end specified)

# Response
Name	    Type	    Description
"satoshis"	(number)	the difference in satoshis
"txid"	(string)	the related transaction id
"index"	(number)	the related input or output index
"height"	(number)	the block height
"address"	(string)	the address

*/
pub fn get_address_deltas(
    someUser: komodorpcutil::KomodoRPC,
    v_address: Vec<String>,
    start: u32,
    end: u32,
    chainInfo: bool,
) -> Result<String, reqwest::Error> {
    let mut addr_list = String::from("[");

    for addr in &v_address {
        addr_list = addr_list + "\"" + addr + "\"" + &","; //parsing error
    }

    if (v_address.len() > 0)
    // >1
    {
        // why need to cut last substring
        addr_list = addr_list[0..(addr_list.len() - 1)].to_string();
    }

    addr_list = addr_list + &"]"; //& for String -> &string

    let params = String::from("[{\"addresses\": ")
        + &addr_list
        + ",\"start\":"
        + &start.to_string()
        + ",\"end\":"
        + &end.to_string()
        + ",\"chainInfo\":"
        + &chainInfo.to_string()
        + "}]";

    let method_name: String = String::from("getaddressdeltas");
    let method_body: String = String::from(params);
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));
    let result = komodorpcutil::request(someUser.clone(), data);
    return result;
}

/*
getaddressmempool '{ "addresses" : [ "address" , ... ] }'

The getaddressmempool method returns all mempool deltas for an address, or addresses. The method requires addressindex to be enabled.

# Arguments
Name	    Type	    Description
"address"	(string)	the address

# Response
Name	    Type     	Description
"address"	(string)	the address
"txid"	    (string)	the related txid
"index"	    (number)	the related input or output index
"satoshis"	(number)	the difference in satoshis
"timestamp"	(number)	the time the transaction entered the mempool (seconds)
"prevtxid"	(string)	the previous txid (if spending)
"prevout"	(string)	the previous transaction output index (if spending)

*/
pub fn get_address_mem_pool(
    someUser: komodorpcutil::KomodoRPC,
    v_address: Vec<String>,
) -> Result<String, reqwest::Error> {
    let mut addr_list = String::from("[");

    for addr in &v_address {
        addr_list = addr_list + "\"" + addr + "\"" + &","; //parsing error
    }

    if (v_address.len() > 0)
    // >1
    {
        // why need to cut last substring
        addr_list = addr_list[0..(addr_list.len() - 1)].to_string();
    }

    addr_list = addr_list + &"]"; //& for String -> &string

    let params = String::from("[{\"addresses\": ") + &addr_list + "}]";

    let method_name: String = String::from("getaddressmempool");
    let method_body: String = String::from(params);
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));

    let result = komodorpcutil::request(someUser.clone(), data);
    return result;
}
/*
#getaddresstxids
getaddresstxids '{ "addresses" : [ "address" , ... ] }'

The getaddresstxids method returns the txids for an address, or addresses. It requires addressindex to be enabled.

# Arguments
Name	    Type	    Description
"address"	(string)	the address
"start"	    (number)	the start block height
"end"	    (number)	the end block height

# Response
Name	            Type	    Description
"transaction_id"	(string)	the transaction id


*/
pub fn get_address_tx_ids(
    someUser: komodorpcutil::KomodoRPC,
    v_address: Vec<String>,
    start: u32,
    end: u32,
) -> Result<String, reqwest::Error> {
    let mut addr_list = String::from("[");

    for addr in &v_address {
        addr_list = addr_list + "\"" + addr + "\"" + &","; //parsing error
    }

    if (v_address.len() > 0)
    // >1
    {
        // why need to cut last substring
        addr_list = addr_list[0..(addr_list.len() - 1)].to_string();
    }

    addr_list = addr_list + &"]"; //& for String -> &string

    let params = String::from("[{\"addresses\": ")
        + &addr_list
        + ",\"start\":"
        + &start.to_string()
        + ",\"end\":"
        + &end.to_string()
        + "}]";
    //let params = format!("[{\"addresses\": {0}}]", addr_list); //need to fix to use format

    let method_name: String = String::from("getaddresstxids");
    let method_body: String = String::from(params);
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));
    let result = komodorpcutil::request(someUser.clone(), data);
    return result;
}
/*
#getaddressutxos
getaddressutxos '{ "addresses" : [ "address" , ... ], "chaininfo" }'

The getaddressutxos method returns all unspent outputs for an address. It requires addressindex to be enabled.

# Arguments
Name	    Type	    Description
"address"	(string)	the address
"chainInfo"	(boolean)	include chain info with results

# Response
Name	        Type     	Description
"address"	   (string)  	the address
"txid"	       (string) 	the output txid
"height"	   (number) 	the block height
"outputIndex"  (number) 	the output index
"script"	   (string) 	the script hex encoded
"satoshis"     (number)	    the number of satoshis of the output


        */

pub fn get_address_utxos(
    someUser: komodorpcutil::KomodoRPC,
    v_address: Vec<String>,
    chainInfo: bool,
) -> Result<String, reqwest::Error> {
    let mut addr_list = String::from("[");

    for addr in &v_address {
        addr_list = addr_list + "\"" + addr + "\"" + &","; //parsing error
    }

    if (v_address.len() > 0)
    // >1
    {
        // why need to cut last substring
        addr_list = addr_list[0..(addr_list.len() - 1)].to_string();
    }

    addr_list = addr_list + &"]"; //& for String -> &string

    let params = String::from("[{\"addresses\": ")
        + &addr_list
        + ",\"chainInfo\":"
        + &chainInfo.to_string()
        + "}]";
    let method_name: String = String::from("getaddressutxos");
    let method_body: String = String::from(params);
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));

    let result = komodorpcutil::request(someUser.clone(), data);
    return result;
}
/*
#getsnapshot
getsnapshot top

The getsnapshot method returns a snapshot of addresses and their amounts at the Smart Chain's current height.

The method requires addressindex to be enabled.

# Arguments
Name	 Type	            Description
"top"	(number, optional)	Only return this many addresses, i.e. top N rich list

# Response
Name	            Type            	Description
"addresses"	        (array of jsons)	the array containing the address and amount details
"addr"	            (string)        	an address
"amount"        	(number)	        the amount of coins in the above address
"total"	            (numeric)         	the total amount in snapshot
"average"	        (numeric)	        the average amount in each address
"utxos"	            (number)    	    the total number of utxos in snapshot
"total_addresses"	(number)	        the total number of addresses in snapshot,
"start_height"	    (number)	        the block height snapshot began
"ending_height"	    (number)	        the block height snapshot finished,
"start_time"	    (number)	        the unix epoch time snapshot started
"end_time"	        (number)	        the unix epoch time snapshot finished

 */
pub fn get_snapshot(
    someUser: komodorpcutil::KomodoRPC,
    top: Option<u32>,
) -> Result<String, reqwest::Error> {
    //TODO: need to implement optinal arg
    let method_body: String;
    let temp_top = top.unwrap_or(0); //Default value is 0
    if (temp_top > 0) {
        method_body = String::from("[\"") + &temp_top.to_string() + "\"]"; // String::from(format!("[/"{0}/"]",temp_top));
    } else {
        method_body = String::from("[]");
    }

    //let params = String::from("[") + &top.to_string()+"]";

    let method_name: String = String::from("getsnapshot");
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));
    //println!("the payload is{}",payload);
    let result = komodorpcutil::request(someUser.clone(), data);
    return result;
}
