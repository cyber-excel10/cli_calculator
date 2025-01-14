 use std::io;

fn main(){
    println!("Hello, Welcome And Thanks For Choosing Excel2024 Calculator! We Perform Mathematical Operation.");
    println!("You Can Input Your Calculations Or If You Want To Quit You Can Input 'Exit' To Close The Page");
    loop{
        
        let mut input = String::new();
        println!("Please Input The calculation You Want Me To Perform:");
        io::stdin()
        .read_line(&mut input)
        .expect("Failed To Read Line");
        let input = input.trim();

        if input =="Exit"{
            println!("Are You Sure You Want To Exit?");
            break;
        } else{
            match calculate(input){
             Ok(result) => println!("The Result is: {}.",result),
             Err(err) => println!("Error:{}.",err),
            }
        }
    }
    println!("Thanks For Using Excel2024 Calculator.");
}

fn calculate(expression: &str) -> Result<f64, String> {
    let parts:Vec<&str> = expression.split_whitespace().collect();

    if parts.len() < 3 {
        return Err("Invaild Format!Please Consinder Using:<number> <operator> <number>".to_string());
    }

    let mut result = parts[0].parse::<f64>().map_err(|_| "Please Consinder Inputing A Numeric Value".to_string())?;

    for x in(1..parts.len()).step_by(2) {
        let operator = parts[x];
        let another_number = parts[x + 1].parse::<f64>().map_err(|_| "Please Consinder Inputing A Numeric Value".to_string())?;
    
      result = match operator{
        "+" => Ok(result + another_number),
        "-" => Ok(result - another_number),
        "*" => Ok(result * another_number),
        "/" => {
            if another_number == 0.0 {
           Err("Cannnot By Divided By Zero!".to_string())
            } else {
                Ok(result / another_number)
            }
        },
        "^" => Ok(result.powf(another_number)),
        _ => Err(format!("Unsupported Operator: {}",operator)),
    }?;
}
   if parts.len() == 2 {
    let operator = parts[0];
    let angle_or_value: f64 = parts[1].parse().map_err(|_| "Please Consinder Inputing A Numeric Value.".to_string())?;

   match operator{
        "sin" => Ok(angle_or_value.to_radians().sin()),
        "cos" => Ok(angle_or_value.to_radians().cos()),
        "tan" => Ok(angle_or_value.to_radians().tan()),
        "square_root" => {
            if angle_or_value < 0.0 {
               return Err("Computing A Negative Number Square Root Is Not Accepted!!! ".to_string());
            } else {
               Ok(angle_or_value.sqrt())
            }
        },
        "log" => {
            if angle_or_value <= 0.0 {
           return Err("Please Consinder Removing Negative Value When Dealing With Logarithm.".to_string());
            } else {
                Ok(angle_or_value.log(10.0))
            }
        },
   _ => Err(format!("Unsupported Operator: {}",operator)),
    }
} else {
    Ok(result)      
}
}
