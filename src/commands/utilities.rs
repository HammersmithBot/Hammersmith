use crate::{Context, Error};

#[derive(Debug, poise::ChoiceParameter)]
pub enum ConvertType {
    Celsius,
    Fahrenheit,
    Kelvin,
}

#[poise::command(
slash_command,
)]
pub async fn convert(
    ctx: Context<'_>,
    #[description = "First value"]
    convert_type_1: ConvertType,
    #[description = "Second value"]
    convert_type_2: ConvertType,
    #[description = "Value to convert"]
    value: f64,
) -> Result<(), Error> {
    let result = match convert_type_1 {
        ConvertType::Celsius => {
            match convert_type_2 {
                ConvertType::Celsius => {
                    format!("{:?}C is {:?}C", value, value)
                }
                ConvertType::Fahrenheit => {
                    let result = (value as f32) * (9.0 / 5.0) + 32.0;
                    format!("{:?}C is {:?}F", value, result)
                }
                ConvertType::Kelvin => {
                    let result = (value as f32) + 273.15;
                    format!("{:?}C is {:?}K", value, result)
                }
            }
        }
        ConvertType::Fahrenheit => {
            match convert_type_2 {
                ConvertType::Celsius => {
                    let result = (value as f32) - 32.0 * (5.0 / 9.0);
                    format!("{:?}F is {:?}C", value, result)
                }
                ConvertType::Fahrenheit => {
                    format!("{:?}F is {:?}F", value, convert_type_2)
                }
                ConvertType::Kelvin => {
                    let result = (((value as f32) - 32.0) * (5.0 / 9.0)) + 273.15;
                    format!("{:?}F is {:?}K", value, result)
                }
            }
        }
        ConvertType::Kelvin => {
            match convert_type_2 {
                ConvertType::Celsius => {
                    let result = (value as f32) - 273.15;
                    format!("{:?}K is {:?}C", value, result)
                }
                ConvertType::Fahrenheit => {
                    let result = (value as f32) * (9.0 / 5.0) - 459.67;
                    format!("{:?}K is {:?}F", value, result)
                }
                ConvertType::Kelvin => {
                    format!("{:?}K is {:?}K", value, convert_type_2)
                }
            }
        }
    };

    ctx.say(result).await?;
    Ok(())
}