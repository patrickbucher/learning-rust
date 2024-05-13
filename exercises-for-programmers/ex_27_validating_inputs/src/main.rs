use akshually::io::prompt_line;
use regex::Regex;

fn main() {
    let first_name: Option<String> = prompt_line("Enter the first name: ");
    let last_name: Option<String> = prompt_line("Enter the last name: ");
    let zip_code: Option<String> = prompt_line("Enter the ZIP code: ");
    let employee_id: Option<String> = prompt_line("Enter the employee ID: ");

    if validate_name(first_name).is_none() {
        eprintln!("The first name must be filled in.");
    }
    if validate_name(last_name).is_none() {
        eprintln!("The last name must be filled in.");
    }
    if validate_zip_code(zip_code).is_none() {
        eprintln!("The ZIP code must be numeric.");
    }
    if validate_employee_id(employee_id.clone()).is_none() {
        match employee_id {
            Some(eid) => eprintln!("{} is not a valid ID.", eid),
            None => eprintln!("The employee ID is invalid."),
        }
    }
}

fn validate_name(name: Option<String>) -> Option<String> {
    if name.clone()?.trim().chars().count() >= 2 {
        name
    } else {
        None
    }
}

fn validate_zip_code(code: Option<String>) -> Option<u16> {
    Some(code?.parse::<u16>().ok()?)
}

fn validate_employee_id(id: Option<String>) -> Option<String> {
    let s = id?;
    let pattern = Regex::new(r"^[A-Z]{2}-[0-9]{4}$").ok()?;
    if pattern.is_match(&s) {
        Some(s)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_name() {
        assert_eq!(validate_name(Some("X".into())), None);
        assert_eq!(validate_name(Some("XY".into())), Some("XY".into()));
    }

    #[test]
    fn test_validate_zip_code() {
        assert_eq!(validate_zip_code(Some("ABCDE".into())), None);
        assert_eq!(validate_zip_code(Some("12345".into())), Some(12345));
    }

    #[test]
    fn test_validate_employee_id() {
        assert_eq!(validate_employee_id(Some("foobar".into())), None);
        assert_eq!(
            validate_employee_id(Some("AZ-1234".into())),
            Some("AZ-1234".into())
        );
    }
}
