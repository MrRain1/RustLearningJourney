use std::io;

fn fahr_to_cels(temp: f64) -> f64 {
    (temp - 32.0)*5.0/9.0
}

fn main() {
    println!("Insert the temperature to convert:");
    
    let mut temperature_fahr = String::new();

    loop{
        io::stdin()
            .read_line(&mut temperature_fahr)
            .expect("Failed to read line");

        let temperature_fahr: f64 = match temperature_fahr.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        let temperature_cels = fahr_to_cels(temperature_fahr);

        println!("Celsius temperature: {temperature_cels}");
        break;
    }


}
