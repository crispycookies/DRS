extern crate guid_create;

use guid_create::GUID;

pub struct RandomGuid {
    pub guid : String
}

impl RandomGuid {
    pub fn create_random_guid(&mut self){
        self.guid = GUID::rand().to_string();
    }

}



