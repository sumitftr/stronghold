use super::ValidationError;
use std::str::FromStr;
use time::OffsetDateTime;

const SPECIAL: [char; 33] = [
    ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<',
    '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
];

pub fn is_birth_date_valid(
    year: u32,
    month: u8,
    day: u8,
    offset: time::UtcOffset,
) -> Result<OffsetDateTime, ValidationError> {
    // Convert month number to time::Month enum
    let month_enum = match time::Month::try_from(month) {
        Ok(m) => m,
        Err(_) => {
            tracing::error!("Invalid month: {month}");
            return Err(ValidationError::InvalidMonth);
        }
    };

    // Try to create a valid date
    let date = match time::Date::from_calendar_date(year as i32, month_enum, day) {
        Ok(d) => d,
        Err(e) => {
            tracing::error!("Invalid date: year={year}, month={month}, day={day}, error={e:?}");
            return Err(ValidationError::DateParseError(e));
        }
    };

    // Create a OffsetDateTime at midnight
    let birth_datetime = OffsetDateTime::new_in_offset(date, time::Time::MIDNIGHT, offset);

    // Check if birth date is in the future
    if birth_datetime > OffsetDateTime::now_utc() {
        tracing::error!("Birth date is in the future: {:?}", birth_datetime);
        return Err(ValidationError::InvalidDate("Birth Date cannot be in the future".to_string()));
    }

    Ok(birth_datetime)
}

pub fn is_password_strong(p: &str) -> Result<(), ValidationError> {
    if p.len() < 8 {
        return Err(ValidationError::PasswordTooShort);
    }
    if p.len() > 128 {
        return Err(ValidationError::PasswordTooLong);
    }
    let mut categories = 0u8;
    let (mut lower, mut upper, mut digit, mut special) = (false, false, false, false);
    for c in p.chars() {
        if !lower && c.is_lowercase() {
            lower = true;
            categories += 1;
        } else if !upper && c.is_uppercase() {
            upper = true;
            categories += 1;
        } else if !digit && c.is_numeric() {
            digit = true;
            categories += 1;
        } else if !special && SPECIAL.contains(&c) {
            special = true;
            categories += 1;
        }
        // Early exit once we have 3 categories
        if categories >= 3 {
            return Ok(());
        }
    }
    Err(ValidationError::InvalidPasswordFormat)
}

// a valid name contains two or more words
// each words should only contain english alphabets
pub fn is_legal_name_valid(s: &str) -> Result<String, ValidationError> {
    if s.len() > 64 {
        return Err(ValidationError::NameTooLong);
    }
    let mut result = String::new();
    let mut count = 0;
    for part in s.split_whitespace() {
        if part.chars().any(|b| !b.is_alphabetic()) {
            return Err(ValidationError::InvalidNameFormat(
                "Only alphabets are allowed inside name".to_string(),
            ));
        }
        if !result.is_empty() {
            result.push(' ');
        }
        count += 1;
        result.push_str(part);
    }
    if !result.is_empty() && count >= 2 {
        Ok(result)
    } else {
        Err(ValidationError::InvalidNameFormat("Name must contain two or more words".to_string()))
    }
}

pub fn is_display_name_valid(display_name: &str) -> Result<(), ValidationError> {
    if display_name.trim().len() < 2 {
        return Err(ValidationError::NameTooShort);
    }
    if display_name.len() > 64 {
        return Err(ValidationError::NameTooLong);
    }
    if !display_name.trim().is_ascii() {
        return Err(ValidationError::InvalidNameFormat(
            "Name can only contain ascii characters".to_string(),
        ));
    }
    Ok(())
}

pub fn is_bio_valid(bio: &str) -> Result<(), ValidationError> {
    if bio.len() > 3000 {
        return Err(ValidationError::BioTooLong);
    }
    Ok(())
}

pub fn is_gender_valid(gender: &str) -> Result<(), ValidationError> {
    if gender.chars().any(|c| !c.is_ascii_alphabetic()) {
        return Err(ValidationError::InvalidGender);
    }
    Ok(())
}

pub fn is_country_valid(country: &str) -> Result<String, ValidationError> {
    let c = celes::Country::from_str(country.trim()).map_err(|e| {
        tracing::error!("{e:?}");
        ValidationError::CountryNotFound
    })?;
    Ok(c.long_name.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! name_test {
        ($($name:ident: $exp:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (haystack, expected) = $exp;
                    assert_eq!(is_legal_name_valid(haystack).ok(), expected);
                }
            )*
        };
    }

    name_test! {
        name_test1: ("hello ", None),
        name_test2: ("   hello ", None),
        name_test3: ("   AB", None),
        name_test4: ("A", None),
        name_test5: ("SUMIT BRUH", Some("SUMIT BRUH".to_string())),
        name_test6: ("Sumit |", None),
        name_test7: (" RUST LANG", Some("RUST LANG".to_string())),
        name_test8: ("  hello  world broo  ", Some("hello world broo".to_string())),
        name_test9: ("RUST LANG   ", Some("RUST LANG".to_string())),
        name_test10: (" abc 82  cd  ", None),
        name_test11: (" abc_def  ", None),
        name_test12: (" abc-def  ", None),
        name_test13: (" abc@def  ", None),
    }
}
