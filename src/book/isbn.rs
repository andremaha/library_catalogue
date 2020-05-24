use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum ISBN {
    V10(u16, u16, u16, u16, u16),
    V13(u16, u16, u16, u16, u16)
}

impl ISBN {


    /// Convert ISBN codes into lines
    fn convert_to_lines(digits: Vec<&u16>) -> String {
        let mut code = String::from("");

        for digit in digits.iter() {
            code.push_str(&ISBN::convert_digit_into_line(digit));
            code.push_str("-");
        }

        // remove trailing dash
        code.pop();

        code 
    }

    /// Converts single ISBN digit into the line
    fn convert_digit_into_line(digit: &u16) -> String {

        if digit == &1u16 {
            String::from("|")
        } else if digit > &100u16 && digit < &1000u16 {
            String::from("|||||||||")
        } else  {
            String::from("|||||||||||||||||")
        }
    }

    /// Prints a QR code for the current ISBN
    pub fn print_qr_code(&self) -> String {
        

        match self {
            ISBN::V10(a, b, c, d, e) => {

                format!("{}", ISBN::convert_to_lines(vec![a, b, c, d, e]))
                
            }, 
            ISBN::V13(a, b, c, d, e) => {
                
                format!("{}", ISBN::convert_to_lines(vec![a, b, c, d, e]))

            }
        }

    }
}

impl Display for ISBN {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            ISBN::V10(a, b, c, d, e) => f.write_fmt(format_args!("v10: {}-{}-{}-{}-{}", a, b, c, d, e)),
            ISBN::V13(a, b, c, d, e) => f.write_fmt(format_args!("v13: {}-{}-{}-{}-{}", a, b, c, d, e)),
        }
    }
}