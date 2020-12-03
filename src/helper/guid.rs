extern crate guid_create;

use guid_create::GUID;

pub fn create_random_guid() -> String {
    return GUID::rand().to_string();
}