#![allow(warnings)]
use super::komodorpcutil;
use komodorpcutil::KomodoRPC;

/**
 * addnode
addnode "node" "add|remove|onetry"

The addnode method attempts to add or remove a node from the addnode list, or to make a single attempt to connect to a node.

#Arguments
Name	Type	Description
"node"	(string, required)	the node (see getpeerinfo for nodes)
"command"	(string, required)	'add' to add a node to the list, 'remove' to remove a node from the list, 'onetry' to try a connection to the node once
#Response
Name	Type	Description
(none)
 */
pub fn add_node(
    someUser: komodorpcutil::KomodoRPC,
    node: String,
    command: String,
) -> Result<String, reqwest::Error> {
    let method_body =
        String::from("[\"") + &node + &String::from("\", \"") + &command + &String::from("\"]");
    let method_name: String = String::from("addnode");
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(someUser.clone(), data)
}

/**
 * clearbanned

The clearbanned method clears all banned IPs.

#Arguments
Name	Type	Description
(none)
#Response
Name	Type	Description
(none)
 */
pub fn clear_banned(SomeUser: komodorpcutil::KomodoRPC) -> Result<String, reqwest::Error> {
    let method_name: String = String::from("clearbanned");
    let method_body: String = String::from("[]");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}
/**
 * disconnectnode
disconnectnode "node"

The disconnectnode method instructs the daemon to immediately disconnect from the specified node.

Use getpeerinfo to determine the result.

#Arguments
Name	Type	Description
"node"	(string, required)	the node's address (see getpeerinfo for nodes)
#Response
Name	Type	Description
(none)
 */
pub fn disconnect_node(
    someUser: komodorpcutil::KomodoRPC,
    node: String,
) -> Result<String, reqwest::Error> {
    let method_body = String::from("[\"") + &node + "\"]"; // String::from(format!("[/"{0}/"]",temp_top));

    let method_name: String = String::from("disconnectnode");
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(someUser.clone(), data)
}

/**
 * getaddednodeinfo
getaddednodeinfo dns ( "node" )

The getaddednodeinfo method returns information about the given added node, or all added nodes.

If dns is set to false, only a list of added nodes is returned. Otherwise, connection information is also provided.

Nodes added via onetry are not listed here.

#Arguments
Name	Type	Description
dns	(boolean, required)	if false, only a list of added nodes will be provided; otherwise, connection information is also provided
"node"	(string, optional)	if provided, the method returns information about this specific node; otherwise, all nodes are returned
 */
pub fn get_added_node_info(
    SomeUser: komodorpcutil::KomodoRPC,
    dns: bool,
    node: Option<String>,
) -> Result<String, reqwest::Error> {
    let method_body: String;
    let temp_node: String = node.unwrap_or("".to_string()); // default -1///TO DO
    if (temp_node.is_empty()) {
        method_body = String::from("[") + &dns.to_string() + &String::from("]");
    } else
    //if (!temp_gen_proc_limit.is_empty())
    {
        method_body = String::from("[")
            + &dns.to_string()
            + &",\"".to_string()
            + &temp_node
            + &String::from("\"");
    }

    let method_name: String = String::from("getaddednodeinfo");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

/**
 * getconnectioncount

The getconnectioncount method returns the number of connections to other nodes.

#Arguments
Name	Type	Description
(none)

#Response
Name	Type	Description
n	(numeric)	the connection count
 */

pub fn get_connection_count(someUser: komodorpcutil::KomodoRPC) -> Result<String, reqwest::Error> {
    let params = String::from("[]");

    let method_name: String = String::from("getconnectioncount");
    let method_body: String = String::from(params);
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(someUser.clone(), data)
}

/**
 * getdeprecationinfo
getdeprecationinfo

The getdeprecationinfo method returns an object containing current version and deprecation block height.

This method is applicable only to the KMD main net.

#Arguments
Name	Type	Description
(none)
#Response
Name	Type	Description
"version"	(numeric)	the server version
"subversion"	(string)	the server sub-version string (i.e. "/MagicBean:x.y.z[-v]/")
"deprecationheight"	(numeric)	the block height at which this version will deprecate and shut down (unless disabledeprecation is set)
 */

pub fn get_deprecation_info(someUser: komodorpcutil::KomodoRPC) -> Result<String, reqwest::Error> {
    let params = String::from("[]");

    let method_name: String = String::from("getdeprecationinfo");
    let method_body: String = String::from(params);
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(someUser.clone(), data)
}

/**
 * getnettotals
getnettotals

The getnettotals method returns information about network traffic, including bytes in, bytes out, and current time.

#Arguments
Name	Type	Description
(none)
#Response
Name	Type	Description
"totalbytesrecv"	(numeric)	total bytes received
"totalbytessent"	(numeric)	total bytes sent
"timemillis"	(numeric)	total cpu time
 */

pub fn get_net_totals(someUser: komodorpcutil::KomodoRPC) -> Result<String, reqwest::Error> {
    let params = String::from("[]");

    let method_name: String = String::from("getnettotals");
    let method_body: String = String::from(params);
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(someUser.clone(), data)
}

/**
 * getnetworkinfo
getnetworkinfo

The getnetworkinfo method returns an object containing various state info regarding p2p networking.

#Arguments
Name	Type	Description
(none)
#Response
Name	Type	Description
"version"	(numeric)	the server version
"subversion"	(string)	the server subversion string (i.e. "/MagicBean:x.y.z[-v]/")
"protocolversion"	(numeric)	the protocol version
"localservices"	(string)	the services we offer to the network
"timeoffset"	(numeric)	the time offset
"connections"	(numeric)	the number of connections
"networks": [ ... ]	(array of jsons)	information per network
"name"	(string)	network (ipv4, ipv6 or onion)
"limited"	(boolean)	whether the network is limited using -onlynet
"reachable"	(boolean)	whether the network is reachable
"proxy"	(string)	(submitted as "host:port") the proxy that is used for this network, or empty if none
"relayfee"	(numeric)	minimum relay fee for non-free transactions in COIN/kB
"localaddresses": [ ... ]	(array of jsons)	list of local addresses
"address"	(string)	network address
"port"	(numeric)	network port
"score"	(numeric)	relative score
"warnings"	(string)	any network warnings (such as alert messages)
 */

pub fn get_network_info(someUser: komodorpcutil::KomodoRPC) -> Result<String, reqwest::Error> {
    let params = String::from("[]");

    let method_name: String = String::from("getnetworkinfo");
    let method_body: String = String::from(params);
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(someUser.clone(), data)
}

/**
 * getpeerinfo

The getpeerinfo method returns data about each connected network node as a json array of objects.

#Arguments
Name	Type	Description
(none)
#Response
Name	Type	Description
"id"	(numeric)	peer index
"addr":,	(string)	the ip address and port of the peer ("host:port")
"addrlocal"	(string)	local address ("ip:port")
"services"	(string)	the services offered
"lastsend"	(numeric)	the time in seconds since epoch (Jan 1 1970 GMT) of the last send
"lastrecv"	(numeric)	the time in seconds since epoch (Jan 1 1970 GMT) of the last receive
"bytessent"	(numeric)	the total bytes sent
"bytesrecv"	(numeric)	the total bytes received
"conntime"	(numeric)	the connection time in seconds since epoch (Jan 1 1970 GMT)
"timeoffset"	(numeric)	the time offset in seconds
"pingtime"	(numeric)	ping time
"pingwait"	(numeric)	ping wait
"version"	(numeric)	the peer version, such as 170002
"subver"	(string)	the string version (i.e. "/MagicBean:x.y.z[-v]/")
"inbound"	(boolean)	inbound (true) or outbound (false)
"startingheight"	(numeric)	the starting height (block) of the peer
"banscore"	(numeric)	the ban score
"synced_headers"	(numeric)	the last header we have in common with this peer
"synced_blocks"	(numeric)	the last block we have in common with this peer
"inflight": [ ... ]	(array)
number	(numeric)	the block height requested from this peer
 */

pub fn get_peer_info(someUser: komodorpcutil::KomodoRPC) -> Result<String, reqwest::Error> {
    let params = String::from("[]");

    let method_name: String = String::from("getpeerinfo");
    let method_body: String = String::from(params);
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(someUser.clone(), data)
}
/**
 * listbanned

The listbanned method lists all banned IP addresses and subnets.

#Arguments
Name	Type	Description
(none)
#Response
Name	Type	Description
"address"	(string)	the address/subnet that is banned
"banned_until"	(numeric)	the timestamp, at which point the ban will be removed
 */

pub fn list_banned(someUser: komodorpcutil::KomodoRPC) -> Result<String, reqwest::Error> {
    let params = String::from("[]");

    let method_name: String = String::from("listbanned");
    let method_body: String = String::from(params);
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(someUser.clone(), data)
}

/**
 * ping

The ping method requests that a ping be sent to all other nodes, to measure ping time.

Results provided in getpeerinfo, pingtime and pingwait fields are decimal seconds.

The ping command is handled in queue with all other commands, so it measures processing backlog, not just network ping.

Use getpeerinfo to see ping results.

#Arguments
Name	Type	Description
(none)
#Response
Name	Type	Description
(none)

 */
pub fn ping(someUser: komodorpcutil::KomodoRPC) -> Result<String, reqwest::Error> {
    let params = String::from("[]");

    let method_name: String = String::from("ping");
    let method_body: String = String::from(params);
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(someUser.clone(), data)
}

/**
 * T0 DO , NOT FULLU IMPLEMENTED AND TESTED
 * setban
setban "ip(/netmask)" "add|remove" (bantime) (absolute)

The setban method attempts to add or remove an IP address (and subnet, if indicated) from the banned list.

#Arguments
Name	Type	Description
"ip(/netmask)"	(string, ip required)	the IP/subnet (see getpeerinfo for nodes ip) with an optional netmask (default is /32 = single ip)
"command"	(string, required)	use "add" to add an IP/subnet to the list, or "remove" to remove an IP/subnet from the list
bantime	(numeric, optional)	indicates how long (in seconds) the ip is banned (or until when, if [absolute] is set). 0 or empty means the ban is using the default time of 24h, which can also be overwritten using the -bantime runtime parameter.
absolute	(boolean, optional)	if set to true, the bantime must be an absolute timestamp (in seconds) since epoch (Jan 1 1970 GMT)
 */
pub fn set_ban(
    SomeUser: komodorpcutil::KomodoRPC,
    ip: String,
    command: String,
    bantime: Option<u32>,
    absolute: Option<bool>,
) -> Result<String, reqwest::Error> {
    let mut method_body: String;
    method_body =
        String::from("[\"") + &ip + &String::from("\",\"") + &command + &String::from("\"");
    if let Some(temp_bantime) = bantime {
        method_body = method_body + &String::from(",") + &temp_bantime.to_string();
    }

    if let Some(temp_absolute) = absolute {
        method_body = method_body + &String::from(",") + &temp_absolute.to_string();
    }

    method_body = method_body + &String::from("]");
    let method_name: String = String::from("setban");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(SomeUser.clone(), data)
}
