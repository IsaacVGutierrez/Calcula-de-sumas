use std::io;

fn main() {
    // Pedir al usuario que ingrese dos números enteros
    println!("Ingrese el primer número:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Error al leer la entrada.");
    let num1: i32 = input1.trim().parse().expect("Error al parsear el número.");

    println!("Ingrese el segundo número:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Error al leer la entrada.");
    let num2: i32 = input2.trim().parse().expect("Error al parsear el número.");

    // Calcular la suma de los dos números
    let suma = num1 + num2;

    // Imprimir el resultado en la consola
    println!("La suma de {} y {} es: {}", num1, num2, suma);
}
