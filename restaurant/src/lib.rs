mod front_of_house;

pub use crate::front_of_house::hosting;
pub use crate::front_of_house::serving;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    serving::take_order();
}