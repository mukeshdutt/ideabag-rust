use std::io;

// get tax with the help of base_cost and tax rate
fn get_tax(base_cost: f32, tax_rate: f32) -> f32 {
    return (base_cost / (100 as f32)) * tax_rate;
}

// apply tax to the base cost
fn apply_tax(base_cost: f32, tax_rate: f32) -> f32 {
    return base_cost + get_tax(base_cost, tax_rate);
}

fn main() {

    let mut cost = String::new();
    let mut tax: String = String::new();

    println!("Enter the base cost:");
    io::stdin()
        .read_line(&mut cost)
        .expect("not a base cost string");

    println!("Enter the valid tax:");
    io::stdin()
        .read_line(&mut tax)
        .expect("not a valid tax string");

    let base_cost = cost.trim().parse().expect("not valid float");
    let tax_rate = tax.trim().parse().expect("not valid float");

    println!("Your calculated tax is: {}", apply_tax(base_cost, tax_rate));

}
