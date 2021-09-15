use base64::{decode, encode};
use log::{error, trace};

pub trait EncDec {
    fn encode(key_buffer: Vec<u8>) -> String {
        trace!("Encoding buffer {:?}", key_buffer);
        let encoded_key = encode(key_buffer.to_vec());
        trace!("Encoded value {}", encoded_key);
        encoded_key
    }

    fn decode(encoded_str: String) -> Result<Vec<u8>, base64::DecodeError> {
        trace!("Decoding buffer {:?}", encoded_str);
        let res = match decode(encoded_str) {
            Ok(decoded_val) => {
                Ok(decoded_val)
                /*let mut decoded_buffer = [0u8; 32];
                for i in 0..32 {
                  decoded_buffer[i] = decoded_key[i];*/
                //}
                //Ok(decoded_buffer)
            }
            Err(e) => {
                error!("Error decoding base64 string {:?}", e);
                Err(e)
            }
        };
        res
    }
}
