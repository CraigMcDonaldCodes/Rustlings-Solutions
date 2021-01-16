// quiz1.rs
// This is a quiz for the following sections:
// - Variables
// - Functions

// Mary is buying apples. One apple usually costs 2 Rustbucks, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the order amount. No hints this time!

fn calculate_apple_price(units: i32) -> i32 {

    const DISCOUNT_POINT: i32 = 40;
    const PRICE: i32 = 1;
    const DISCOUNT_PRICE: i32 = 2;

    if units > DISCOUNT_POINT {
        units * PRICE
    } else {
        units * DISCOUNT_PRICE
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(65);

    assert_eq!(70, price1);
    assert_eq!(65, price2);
}
