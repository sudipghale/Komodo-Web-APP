#![allow(warnings)]
/**
 * Disclosure
The following RPC calls interact with the komodod software, and are made available through the komodo-cli software
 */
use super::komodorpcutil;
use komodorpcutil::KomodoRPC;

/**
 * z_getpaymentdisclosure
EXPERIMENTAL FEATURE: Payment disclosure is currently DISABLED. This call always fails.

z_getpaymentdisclosure transaction js_index output_index ("message")

The z_getpaymentdisclosure method generates a payment disclosure for a given joinsplit output.

# Arguments
Name	Type	Description
"txid"	(string, required)	(in development)
"js_index"	(string, required)
"output_index"	(string, required)
"message"	(string, optional)
 */

pub fn z_get_payment_disclosure(
    someUser: komodorpcutil::KomodoRPC,
    txid: String,
    js_index: String,
    output_index: String,
    message: Option<String>,
) -> Result<String, reqwest::Error> {
    let method_body: String;
    let temp_message = message.unwrap_or("".to_string()); //Default value is 0
    if (!temp_message.is_empty()) {
        method_body = String::from("[\"")
            + &txid
            + &"\",".to_string()
            + &js_index
            + &",".to_string()
            + &output_index
            + &",\"".to_string()
            + &temp_message
            + "\"]"; // String::from(format!("[/"{0}/"]",temp_top));
    } else {
        method_body = String::from("[\"")
            + &txid
            + &"\",".to_string()
            + &js_index
            + &",".to_string()
            + &output_index
            + "]";
    }

    let method_name: String = String::from("z_getpaymentdisclosure");

    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));
    komodorpcutil::request(someUser.clone(), data)
}

/**
  *
  z_validatepaymentdisclosure
z_validatepaymentdisclosure "paymentdisclosure"

The z_validatepaymentdisclosure method validates a payment disclosure.

# Arguments
Name	Type	Description
"paymentdisclosure"	(string, required)	hex data string, with "zpd:" prefix
# Response
Name	Type	Description
(currently disabled)
  */

pub fn z_validate_payment_disclosure(
    someUser: komodorpcutil::KomodoRPC,
    paymentdisclosure: String,
) -> Result<String, reqwest::Error> {
    let params = String::from("[\"") + &paymentdisclosure + "\"]"; // String::from(format!("[/"{0}/"]",temp_top));

    let method_name: String = String::from("z_validatepaymentdisclosure");
    let method_body: String = String::from(params);
    let data: String = String::from(komodorpcutil::generate_body(
        someUser.clone(),
        method_name,
        method_body,
    ));

    komodorpcutil::request(someUser.clone(), data)
}
