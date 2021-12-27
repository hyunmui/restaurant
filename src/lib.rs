mod back_of_house;
mod front_of_house;

fn serve_order() {}

// like symbolic link
// use crate::front_of_house::hosting;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 여름에 아침 식사로 호밀빵을 주문한다.
    let mut meal = back_of_house::Breakfast::summer("호밀빵");

    // 고객이 마음을 바꿔서 빵 종류를 바꾼다.
    meal.toast = String::from("밀빵");
    println!("{} 토스트로 주세요", meal.toast);

    // 다음 줄의 주석을 해제하면 컴파일 되지 않는다.
    // 고객은 식사와 함께 제공되는 계절 과일을 선택할 수 없다.
    // meal.seasonal_fruit = String::from("블루베리");

    // enums
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // use crate
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
