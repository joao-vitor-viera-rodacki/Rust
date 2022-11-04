
fn main() {
    let x = conto();
    println!("O valor vai ser {}",x);
    if x > 3 {
        println!("Voce e GOSTOZO");
    }
    else {
        println!("Voce nao e um GOSTOZO");
    }
}   

fn conto() -> i32 {
    3
}