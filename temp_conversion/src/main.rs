use std::io;
use std::str::FromStr;
enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

struct ParseTemperatureUnitError;

impl FromStr for TemperatureUnit {
    type Err = ParseTemperatureUnitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "celsius" | "c" => Ok(TemperatureUnit::Celsius),
            "fahrenheit" | "f" => Ok(TemperatureUnit::Fahrenheit),
            // Any other string results in an error
            _ => Err(ParseTemperatureUnitError),
        }
    }
}

fn main(){


    loop {
        let mut input_unit = String::new();
        let mut input_temp = String::new();
        println!("Please input your temperature unit (Celsius or Fahrenheit):");

        io::stdin()
            .read_line(&mut input_unit)
            .expect("Failed to read line");

        let mut unit: TemperatureUnit = match input_unit.trim().parse() {
            Ok(TemperatureUnit::Celsius) => TemperatureUnit::Celsius,
            Ok(TemperatureUnit::Fahrenheit) => TemperatureUnit::Fahrenheit,
            Err(_) => {
                println!("Did not enter a proper temperature unit.");
                continue;
            }


        };

        match unit {
            TemperatureUnit::Celsius => {
                println!("Please enter temperature in Celsius to be converted: ");
            }
            TemperatureUnit::Fahrenheit => {
                println!("Please enter temperature in Fahrenheit to be converted: ");
            }
        }

        io::stdin()
            .read_line(&mut input_temp)
            .expect("Failed to read line");


        let mut temp: f64 = match input_temp.trim().parse() {
            Ok(num) => num,
            Err(_) =>  {
                println!("Please type a goddamn number!");
                continue;
            },
        };

        let mut converted_temp = convert_temp(temp, &unit);

        match unit {
            TemperatureUnit::Celsius => {
                println!("{input_temp} Celsius is {converted_temp} Fahrenheit");
            }
            TemperatureUnit::Fahrenheit => {
                println!("{input_temp} Fahrenheit is {converted_temp} Celsius");()
            }
        }
    }

}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

fn convert_temp(temp:f64, unit: &TemperatureUnit) -> f64 {
    match unit {
        TemperatureUnit::Celsius => {
            celsius_to_fahrenheit(temp)
        },
        TemperatureUnit::Fahrenheit => {
            fahrenheit_to_celsius(temp)
        }

    }
}