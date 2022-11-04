use std::io;

fn main() {
    struct Animal {
        especie : String,
        carnivoro : bool ,
        quant_patas : u32,
    }
    fn build_animal(especie:String ,carnivoro:bool ,quant_patas:u32) -> Animal {
        Animal {
            especie ,
            carnivoro,
            quant_patas,
        }
    }
    let mut info = (String , bool, u32);

    println!("Digite A especie");
    io::stdin().read_line(&mut info).expect("Erro na criacao ");
    //A fazer
}