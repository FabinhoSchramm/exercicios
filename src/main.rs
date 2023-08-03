fn main() {
    let x = 32.0;
    conversor_fahrenheit_to_celsius(x);
}

fn conversor_fahrenheit_to_celsius(temp :f64) {
    let calculo = (temp - 32.0) * (5.0/9.0);
    println!("{temp}F == {calculo}C");
}
