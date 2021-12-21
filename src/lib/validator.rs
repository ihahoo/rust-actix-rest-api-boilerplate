use regex::Regex;
use crate::lib::error;

pub fn not_none<T>(v: Option<T>, name: &str) -> Result<(), error::Error> {
    if let None = v {
        return Err(error::new(400002, &format!("{}不能为空", name)[..], 422));
    }

    Ok(())
}

pub fn required_str(v: &Option<String>, name: &str) -> Result<String, error::Error> {
    not_none(v.as_ref(), name)?;

    let v = v.as_ref().unwrap().to_string();
    if v.chars().count() == 0 {
        return Err(error::new(400002, &format!("{}不能为空", name)[..], 422));
    }

    Ok(v)
}

pub fn email(v: &str, name: &str) -> Result<(), error::Error> {
    let re = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
    if !re.is_match(v) {
        return Err(error::new(400002, &format!("{}格式不正确", name)[..], 422));
    }

    Ok(())
}

pub fn mobile(v: &str, name: &str) -> Result<(), error::Error> {
    let re = Regex::new(r"^(\+?0?86\-?)?1[345789]\d{9}$").unwrap();
    if !re.is_match(v) {
        return Err(error::new(400002, &format!("{}错误", name)[..], 422));
    }

    Ok(())
}

pub fn uuid(v: &str, name: &str) -> Result<(), error::Error> {
    let re = Regex::new(r"^[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}$").unwrap();
    if !re.is_match(&(v.to_uppercase())[..]) {
        return Err(error::new(400002, &format!("{}格式错误", name)[..], 422));
    }

    Ok(())
}