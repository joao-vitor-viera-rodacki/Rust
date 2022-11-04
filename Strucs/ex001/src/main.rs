fn main() {
    //Criando na mao uma struct
    struct User {
        name : String,
        idade : u32,
        cidade : String
    }
    // passando as informacoes de uma struct para um variavel
    let mut user1  = User{
        name : String::from("Joao"),
        idade : 23,
        cidade : String::from("Guaratuba")
    };
    //acessado os valores de uma variavel criada a partir de ums struct
    println!("O valor de idade e {}, cidade {}, name {}",user1.idade,user1.cidade,user1.name);

    //Usando uma struct apartir de uma funcao 
    let user2 = build_user(32, String::from("Hojoko"),String::from("Facoa"));
    //mostrando o resultado da etapa a acima 
    println!("O user dois se chama {} e sua cidade e {}",user2.name,user2.cidade);

    // usando os .. 
    let user3 = User {
        name : String::from("kolo"),
        ..user2
        //com isso vai ser apenar herdado a cidade e idade do user
    };
    println!("O user 3 se chama {} e sua cidade e {}",user3.name,user3.cidade);

fn build_user (idade:u32,name:String,cidade:String) -> User{  
    User{
    idade,
    name,
    cidade
}}}