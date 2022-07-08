use crate::serde::deserialize_program::ValueAddress;
use crate::types::instruction::Register;
use num_bigint::BigInt;
use std::ops::Rem;

// Checks if the hex string has an odd length.
// If that is the case, prepends '0' to it.
pub fn maybe_add_padding(mut hex: String) -> String {
    if hex.len().rem(2) != 0 {
        hex.insert(0, '0');
        return hex;
    }
    hex
}

pub fn parse_dereference(value: &str) -> Result<ValueAddress, ()> {
    let splitted: Vec<&str> = value.split(" + ").collect();

    match splitted.len() {
        1 => parse_dereference_no_offsets(splitted),
        2 => parse_dereference_with_one_offset(splitted),
        3 => parse_dereference_with_two_offsets(splitted),
        _ => Err(()),
    }
}

fn parse_dereference_no_offsets(splitted_value_str: Vec<&str>) -> Result<ValueAddress, ()> {
    let str_tmp: Vec<&str> = splitted_value_str[0].split(',').collect();

    let register = match str_tmp[0].split('(').collect::<Vec<_>>()[1] {
        "ap" => Some(Register::AP),
        "fp" => Some(Register::FP),
        _ => None,
    };

    Ok(ValueAddress {
        register,
        offset1: 0,
        offset2: 0,
        immediate: None,
        dereference: true,
    })
}

fn parse_dereference_with_one_offset(splitted_value_str: Vec<&str>) -> Result<ValueAddress, ()> {
    let register = match splitted_value_str[0].split('(').collect::<Vec<&str>>()[1] {
        "ap" => Some(Register::AP),
        "fp" => Some(Register::FP),
        _ => None,
    };

    let mut offset1_str = splitted_value_str[1].split(',').collect::<Vec<_>>()[0].to_string();
    offset1_str.retain(|c| !r#"()]"#.contains(c));

    let offset1: i32 = offset1_str.parse().unwrap();

    Ok(ValueAddress {
        register,
        offset1,
        offset2: 0,
        immediate: None,
        dereference: true,
    })
}

fn parse_dereference_with_two_offsets(splitted_value_str: Vec<&str>) -> Result<ValueAddress, ()> {
    let register = match splitted_value_str[0].split('[').collect::<Vec<&str>>()[2] {
        "ap" => Some(Register::AP),
        "fp" => Some(Register::FP),
        _ => None,
    };

    let mut offset1_str = splitted_value_str[1].to_string();
    offset1_str.retain(|c| !r#"()]"#.contains(c));

    let offset1: i32 = offset1_str.parse().unwrap();

    let mut offset2_str = splitted_value_str[2].split(',').collect::<Vec<_>>()[0].to_string();
    offset2_str.retain(|c| !r#"()"#.contains(c));

    let offset2: i32 = offset2_str.parse().unwrap();

    Ok(ValueAddress {
        register,
        offset1,
        offset2,
        immediate: None,
        dereference: true,
    })
}

pub fn parse_reference(value: &str) -> Result<ValueAddress, ()> {
    let splitted: Vec<_> = value.split(" + ").collect();

    match splitted.len() {
        1 => parse_reference_no_offsets(splitted),
        2 => parse_reference_with_one_offset(splitted),
        3 => parse_reference_with_two_offsets(splitted),
        _ => Err(()),
    }
}

fn parse_reference_no_offsets(splitted_value_str: Vec<&str>) -> Result<ValueAddress, ()> {
    let register = match splitted_value_str[0].split(',').collect::<Vec<_>>()[0] {
        "cast(ap" => Some(Register::AP),
        "cast(fp" => Some(Register::FP),
        _ => None,
    };

    Ok(ValueAddress {
        register,
        offset1: 0,
        offset2: 0,
        immediate: None,
        dereference: false,
    })
}

fn parse_reference_with_one_offset(splitted_value_str: Vec<&str>) -> Result<ValueAddress, ()> {
    let register = match splitted_value_str[0].split('(').collect::<Vec<_>>()[1] {
        "ap" => Some(Register::AP),
        "fp" => Some(Register::FP),
        _ => None,
    };

    let mut offset1_str = splitted_value_str[1].split(',').collect::<Vec<_>>()[0].to_string();
    offset1_str.retain(|c| !r#"()"#.contains(c));

    let offset1: i32 = offset1_str.parse().unwrap();

    Ok(ValueAddress {
        register,
        offset1,
        offset2: 0,
        immediate: None,
        dereference: false,
    })
}

fn parse_reference_with_two_offsets(splitted_value_str: Vec<&str>) -> Result<ValueAddress, ()> {
    let register = match splitted_value_str[0].split('[').collect::<Vec<_>>()[1] {
        "ap" => Some(Register::AP),
        "fp" => Some(Register::FP),
        _ => None,
    };

    let mut offset1_str = splitted_value_str[1].to_string();
    offset1_str.retain(|c| !r#"()]"#.contains(c));

    let offset1: i32 = offset1_str.parse().unwrap();

    let mut immediate_str = splitted_value_str[2].split(',').collect::<Vec<_>>()[0].to_string();
    immediate_str.retain(|c| !r#"()"#.contains(c));

    let immediate: BigInt = immediate_str.parse().unwrap();

    Ok(ValueAddress {
        register,
        offset1,
        offset2: 0,
        immediate: Some(immediate),
        dereference: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bigint;
    use num_traits::FromPrimitive;

    #[test]
    // parse value string of format `[cast(reg + offset1, felt*)]`
    fn parse_dereference_with_one_offset_test() {
        let value_string: &str = "[cast(fp + (-3), felt*)]";
        let splitted_value: Vec<&str> = value_string.split(" + ").collect();

        let parsed_value = parse_dereference_with_one_offset(splitted_value).unwrap();

        let value_address = ValueAddress {
            register: Some(Register::FP),
            offset1: -3,
            offset2: 0,
            immediate: None,
            dereference: true,
        };

        assert_eq!(value_address, parsed_value);
    }

    #[test]
    fn parse_dereference_with_two_offsets_test() {
        let value_string: &str = "[cast([fp + (-4)] + 1, felt*)]";
        let splitted_value: Vec<&str> = value_string.split(" + ").collect();

        let parsed_value = parse_dereference_with_two_offsets(splitted_value).unwrap();

        let value_address = ValueAddress {
            register: Some(Register::FP),
            offset1: -4,
            offset2: 1,
            immediate: None,
            dereference: true,
        };

        assert_eq!(value_address, parsed_value);
    }

    #[test]
    fn parse_dereference_no_offsets_test() {
        let value_string: &str = "[cast(fp, felt*)]";
        let splitted_value: Vec<&str> = value_string.split(" + ").collect();

        let parsed_value = parse_dereference_no_offsets(splitted_value).unwrap();

        let value_address = ValueAddress {
            register: Some(Register::FP),
            offset1: 0,
            offset2: 0,
            immediate: None,
            dereference: true,
        };

        assert_eq!(value_address, parsed_value);
    }

    #[test]
    // parse value string of format `[cast(reg + offset1, felt*)]`
    fn parse_reference_with_one_offset_test() {
        let value_string: &str = "cast(fp + (-3), felt*)";
        let splitted_value: Vec<&str> = value_string.split(" + ").collect();

        let parsed_value = parse_reference_with_one_offset(splitted_value).unwrap();

        let value_address = ValueAddress {
            register: Some(Register::FP),
            offset1: -3,
            offset2: 0,
            immediate: None,
            dereference: false,
        };

        assert_eq!(value_address, parsed_value);
    }

    #[test]
    fn parse_reference_with_two_offsets_test() {
        let value_string: &str = "cast([fp + (-4)] + 1, felt*)";
        let splitted_value: Vec<&str> = value_string.split(" + ").collect();

        let parsed_value = parse_reference_with_two_offsets(splitted_value).unwrap();

        let value_address = ValueAddress {
            register: Some(Register::FP),
            offset1: -4,
            offset2: 0,
            immediate: Some(bigint!(1)),
            dereference: false,
        };

        assert_eq!(value_address, parsed_value);
    }

    #[test]
    fn parse_reference_no_offsets_test() {
        let value_string: &str = "cast(fp, felt*)";
        let splitted_value: Vec<&str> = value_string.split(" + ").collect();

        let parsed_value = parse_reference_no_offsets(splitted_value).unwrap();

        let value_address = ValueAddress {
            register: Some(Register::FP),
            offset1: 0,
            offset2: 0,
            immediate: None,
            dereference: false,
        };

        assert_eq!(value_address, parsed_value);
    }
}
