mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}
// pub use crate::front_of_house::hosting; //This technique is called re-exporting because we’re bringing an item into scope but also making that item available for others to bring into their scope.
