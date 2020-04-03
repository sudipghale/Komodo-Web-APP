#![allow(warnings)]
use super::komodorpcutil;
use komodorpcutil::KomodoRPC;

/**
 * generate
generate numblocks

This function can only be used in the regtest mode (for testing purposes).

The generate method instructs the coin daemon to immediately mine the indicated number of blocks.

#Arguments
Name	Type	Description
numblocks	(numeric)	the desired number of blocks to generate
#Response
Name	Type	Description
blockhashes	(array)	hashes of blocks generated
 */

pub fn generate(
    someUser: komodorpcutil::KomodoRPC,
    numblocks: u32,
) -> Result<String, reqwest::Error> {
    ///THE DOCUMENTATION IN DOCS.KOMODOPLATFORM.COM IS INCOMPLETE
    let params = String::from("[") + &numblocks.to_string() + "]";

    let method_name: String = String::from("generate");
    let method_body: String = String::from(params);
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(someUser.clone(), data)
}
/**
  * getgenerate

The getgenerate method returns a boolean value indicating the server's mining status.

The default value is false.

See also gen.

#Arguments
Name	Type	Description
(none)	(none)
#Response
Name	Type	Description
true/false	(boolean)	indicates whether the server is set to generate coins
  */
pub fn get_generate(someUser: komodorpcutil::KomodoRPC) -> Result<String, reqwest::Error> {
    let params = String::from("[]");

    let method_name: String = String::from("getgenerate");
    let method_body: String = String::from(params);
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(someUser.clone(), data)
}

/**
  * setgenerate
setgenerate generate ( genproclimit )

The setgenerate method allows the user to set the generate property in the coin daemon to true or false, thus turning generation (mining/staking) on or off.

Generation is limited to genproclimit processors. Set genproclimit to -1 to use maximum available processors.

See also the getgenerate method to query the current setting, and genproclimit for setting the default number of processors the daemon uses through the .conf file.

#Arguments
Name	Type	Description
generate	(boolean, required)	set to true to turn on generation; set to off to turn off generation
genproclimit	(numeric, optional)	set the processor limit for when generation is on; use value "-1" for unlimited
#Response
Name	Type	Description
(none)	(none)
  */

pub fn set_generate(
    SomeUser: komodorpcutil::KomodoRPC,
    generate: bool,
    gen_proc_limit: Option<u32>,
) -> Result<String, reqwest::Error> {
    let method_body: String;
    let temp_gen_proc_limit: String = gen_proc_limit.unwrap_or(1).to_string(); // default -1///TO DO
    if (temp_gen_proc_limit.is_empty()) {
        method_body = String::from("[") + &generate.to_string() + &String::from("]");
    } else
    //if (!temp_gen_proc_limit.is_empty())
    {
        method_body = String::from("[")
            + &generate.to_string()
            + &",".to_string()
            + &temp_gen_proc_limit
            + &String::from("]");
    }

    let method_name: String = String::from("setgenerate");
    let data: String = String::from(komodorpcutil::generate_body(
        SomeUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(SomeUser.clone(), data)
}

pub fn set_staking_split(
    someUser: komodorpcutil::KomodoRPC,
    split_percentage: f64,
) -> Result<String, reqwest::Error> {
    ///THE DOCUMENTATION IN DOCS.KOMODOPLATFORM.COM IS INCOMPLETE
    let params = String::from("[") + &split_percentage.to_string() + "]";

    let method_name: String = String::from("setstakingsplit");
    let method_body: String = String::from(params);
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(someUser.clone(), data)
}
