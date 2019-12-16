#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate simple_logger;

use lambda::error::HandlerError;

use std::any::*;
use std::error::Error;
use std::num::*;

#[derive(Deserialize, Clone)]
struct CustomEvent {
    #[serde(rename = "value")]
    value: String,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    popcount: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);

    Ok(())
}

fn my_handler(e: CustomEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
    if e.value == "" {
        error!("Empty value in request {}", c.aws_request_id);
        return Err(c.new_error("Empty value"));
    }

    let value: u64 = match convert(e.value) {
        Ok(value) => value,
        Err(value) => {
            error!(
                " value in request cannot convert to u64. c.aws_request_id: {}, value: {}",
                c.aws_request_id, value
            );
            return Err(c.new_error("value cannot convert to u64"));
        }
    };

    if !is_u64(&value) {
        error!(" value in request is not u64 {}", c.aws_request_id);
        return Err(c.new_error("is not u64"));
    }

    let popcount = value.count_ones() as u64;
    Ok(CustomOutput { popcount })
}

fn is_u64<T: ?Sized + Any>(_u: &T) -> bool {
    TypeId::of::<u64>() == TypeId::of::<T>()
}

fn convert(s: String) -> Result<u64, ParseIntError> {
    u64::from_str_radix(&s, 10)
}
