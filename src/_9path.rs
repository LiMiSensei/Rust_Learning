

//定义条目
mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}
    }
}

//使用条目
fn add_to_waitlist(){
    front_of_house::hosting::add_to_waitlist();
}

//crate关键字

//super关键字