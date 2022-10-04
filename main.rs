extern  crate rand; 

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Advinhe o número!");
    println!("Digite seu palpite.");

    let numero_secreto = rand::thread_rng().gen_range(1, 101);
    
    loop {
        let mut palpite = String::new();
        
        println!("Voce disse: {}",palpite);
        
        io::stdin()
        .read_line(&mut palpite)
        .expect("Falha ao ler entrada");
        

        let palpite : u32 = match palpite
        .trim() //remove espacos em brancos
            .parse(){
                Ok(num) => num,
                Err(_) => {
                    println!("Por favor insira um numero valido");
                    continue;}
                };
                
                match palpite.cmp(&numero_secreto) {
                    Ordering::Less => println!("Muito Baixo"),
                    Ordering::Greater => println!("Muito alto"),
                    Ordering::Equal => {println!("Muito acertou");
                            println!("Numero ale {}",numero_secreto);
                            break; },
                        }
    }
}
//rustfmt formatacao