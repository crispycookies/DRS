use crate::helper::guid;
use crate::helper::timestamp;

pub fn get_guid() -> String{
    return guid::create_random_guid();
}

pub fn get_timestamp() -> String{
    return timestamp::get_time_us();
}

pub fn serialize(str: const String){
    return json::parse(str);
}

pub fn deserialize() -> String{
    return guid::create_random_guid();
}

