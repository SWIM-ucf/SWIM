use std::fmt::Display;
use std::rc::Rc;

pub trait RegisterType: ToString + Display {
    fn get_register_name(&self) -> String;

    fn is_valid_register_value(&self, value: u64, pc_limit: usize) -> bool;
}

pub trait Registers {
    fn get_dyn_register_list(&self) -> Vec<(Rc<dyn RegisterType>, u64)>;
}

impl PartialEq for dyn RegisterType {
    fn eq(&self, other: &Self) -> bool {
        // The register names are unique, so use them to compare for equality
        self.get_register_name() == other.get_register_name()
    }
}
