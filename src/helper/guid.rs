extern crate guid_create;

use guid_create::GUID;

pub fn create_random_guid() -> String {
    let guid = GUID::rand();
    return guid.to_string();
}