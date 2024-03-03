use std::ffi::OsString;
use ulid::Ulid;
use chrono::{DateTime, LocalResult, TimeZone, Utc};

pub fn gen_ulid() -> String {
    return Ulid::new().to_string();
}

pub fn convert_ulid_to_uuid() -> String {
    let ulid = gen_ulid();
    let ulid_bytes = ulid.as_bytes().to_vec();
    let mut uuid_bytes = [0u8; 16];

    for (i, byte) in ulid_bytes.iter().take(16).enumerate() {
        uuid_bytes[i] = *byte;
    }

    let mut binding = uuid::Builder::from_bytes(uuid_bytes);
    let uuidv4 = binding
        .set_variant(uuid::Variant::RFC4122)
        .set_version(uuid::Version::Random)
        .as_uuid();

    return uuidv4.to_string();
}

pub fn convert_ulid_to_datetime_atom(ulid: String) -> String {
    let ulid = Ulid::from_string(ulid.as_str()).unwrap();
    let datetime: LocalResult<DateTime<Utc>> = Utc.timestamp_millis_opt(ulid.timestamp_ms() as i64);
    let datetime_string = Ok::<String, OsString>(datetime.unwrap().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)).expect("TODO: panic message");
    return datetime_string;
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_gen_ulid() {
        let ulid = gen_ulid();
        assert!(Ulid::from_string(ulid.as_str()).is_ok());
    }

    #[test]
    fn test_convert_ulid_to_uuid() {
        let uuid_format_regex = Regex::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89ABab][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$").unwrap();
        let uuid = convert_ulid_to_uuid();

        assert!(uuid_format_regex.is_match(uuid.as_str()));
    }

    #[test]
    fn test_convert_ulid_to_datetime_atom() {
        let atom_format_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$").unwrap();
        let ulid = gen_ulid();
        let datetime = convert_ulid_to_datetime_atom(ulid);

        assert!(atom_format_regex.is_match(datetime.as_str()));
    }
}
