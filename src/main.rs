use std::io;

fn convert_to_int(data_input: & String) -> i32
{
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main()
{
    let mut number1 = String::new();
    let mut number2 = String::new();

    println!("Digite o primeiro numero: ");
    io::stdin().read_line(&mut number1).expect("Erro ao ler number1.");
    println!("Digite o segundo numero: ");
    io::stdin().read_line(&mut number2).expect("Erro ao ler number2.");

    if convert_to_int(&number1) > convert_to_int(&number2)
    {
        println!("Resultado do teste: {} é maior que {}", number1, number2);
    }
    else if convert_to_int(&number1) < convert_to_int(&number2)
    {
        println!("Resultado do teste: {} é menor ou que {}", number1, number2);
    }
    else
    {
        println!("Resultado do teste: {} é igual a {}", number1, number2);
    }
}
