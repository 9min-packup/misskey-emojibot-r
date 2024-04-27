use std::fmt::Debug;

use std::{env, str::FromStr};

pub fn get_string_env( key: &str, default: &str) -> String {
    let check_var= env::var(key);
    match check_var {
        Ok(value) => value,
        Err(_e) => String::from(default),
    }
}

pub fn get_env<T>( key: &str, default: T) -> T 
where 
    T : FromStr + Debug,
    T::Err : Debug,
{
    let check_var= env::var(key);
    match check_var {
        Ok(value) => value.parse::<T>().expect(format!("{:?} : invalid type", value).as_str()),
        Err(_e) => default,
    }
}