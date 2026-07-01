//! TD3 passport MRZ (Machine Readable Zone) parser.
//!
//! Parses the 2-line × 44-character TD3 zone printed at the bottom of a
//! passport data page and validates the ICAO 9303 check digits (7-3-1
//! weighting). The caller supplies the already-OCR'd MRZ string — this module
//! does NOT perform any OCR itself (that happens client-side in the browser via
//! tesseract.js, a dedicated scanner, or manual paste).
//!
//! An in-house parser is used instead of the `mrz` crate: the build
//! environment cannot fetch new crates from crates.io, and a TD3 parser is
//! small and fully unit-testable, so pulling in an external dependency buys
//! little. The check-digit algorithm and field offsets follow ICAO Doc 9303.

use chrono::Datelike;

/// Parsed fields extracted from a TD3 MRZ.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MrzData {
    pub passport_number: String,
    pub surname: String,
    pub given_names: String,
    pub nationality: String,
    /// ISO 8601 date (YYYY-MM-DD).
    pub date_of_birth: String,
    /// `M`, `F`, or `X` (unspecified).
    pub sex: String,
    /// ISO 8601 date (YYYY-MM-DD).
    pub expiry_date: String,
    /// True when passport-number, DOB, expiry and composite check digits all pass.
    pub checksum_valid: bool,
}

/// Parse a 2-line TD3 MRZ.
///
/// Accepts the two lines separated by a newline (`\n` or `\r\n`). Any embedded
/// spaces (a common OCR artefact) are stripped before validation. Returns
/// `Err` with a human-readable message when the geometry is not a valid TD3
/// zone (2 lines of exactly 44 chars). Check-digit failures do NOT error —
/// they are surfaced via [`MrzData::checksum_valid`] so the receptionist can
/// still see (and correct) the parsed fields.
pub fn parse_td3(mrz: &str) -> Result<MrzData, String> {
    let lines: Vec<String> = mrz
        .lines()
        .map(|l| l.chars().filter(|c| !c.is_whitespace()).collect::<String>())
        .filter(|l| !l.is_empty())
        .collect();

    if lines.len() != 2 {
        return Err(format!(
            "Expected a 2-line TD3 MRZ, got {} non-empty line(s)",
            lines.len()
        ));
    }

    let line1 = lines[0].to_uppercase();
    let line2 = lines[1].to_uppercase();

    if line1.chars().count() != 44 || line2.chars().count() != 44 {
        return Err(format!(
            "Each TD3 MRZ line must be 44 characters (got {} and {})",
            line1.chars().count(),
            line2.chars().count()
        ));
    }

    // TD3 line 2 layout (1-indexed):
    //  1-9   passport number
    //  10    passport number check digit
    //  11-13 nationality
    //  14-19 date of birth (YYMMDD)
    //  20    DOB check digit
    //  21    sex
    //  22-27 expiry date (YYMMDD)
    //  28    expiry check digit
    //  29-42 personal number
    //  43    personal number check digit
    //  44    composite check digit
    let l2: Vec<char> = line2.chars().collect();
    let passport_raw: String = l2[0..9].iter().collect();
    let passport_check = l2[9];
    let nationality_raw: String = l2[10..13].iter().collect();
    let dob_raw: String = l2[13..19].iter().collect();
    let dob_check = l2[19];
    let sex_raw = l2[20];
    let expiry_raw: String = l2[21..27].iter().collect();
    let expiry_check = l2[27];

    // Composite check digit covers positions 1-10, 14-20 and 22-43 of line 2.
    let composite_input: String = l2[0..10]
        .iter()
        .chain(l2[13..20].iter())
        .chain(l2[21..43].iter())
        .collect();
    let composite_check = l2[43];

    let passport_ok = check_matches(&passport_raw, passport_check);
    let dob_ok = check_matches(&dob_raw, dob_check);
    let expiry_ok = check_matches(&expiry_raw, expiry_check);
    let composite_ok = check_matches(&composite_input, composite_check);
    let checksum_valid = passport_ok && dob_ok && expiry_ok && composite_ok;

    // TD3 line 1 layout: 1='P', 2=type, 3-5=issuing country, 6-44=name.
    let name_field: String = line1.chars().skip(5).collect();
    let (surname, given_names) = parse_name(&name_field);

    Ok(MrzData {
        passport_number: strip_fillers(&passport_raw),
        surname,
        given_names,
        nationality: strip_fillers(&nationality_raw),
        date_of_birth: yymmdd_to_iso(&dob_raw, false),
        sex: normalize_sex(sex_raw),
        expiry_date: yymmdd_to_iso(&expiry_raw, true),
        checksum_valid,
    })
}

/// ICAO 9303 character value: digits are their face value, `A`-`Z` are 10-35,
/// the filler `<` is 0. Any other character is treated as a filler (0) so a
/// stray OCR glyph degrades the checksum rather than panicking.
fn char_value(c: char) -> u32 {
    match c {
        '0'..='9' => c as u32 - '0' as u32,
        'A'..='Z' => c as u32 - 'A' as u32 + 10,
        _ => 0,
    }
}

/// Compute the 7-3-1 weighted mod-10 check digit for a field.
fn check_digit(field: &str) -> u32 {
    const WEIGHTS: [u32; 3] = [7, 3, 1];
    field
        .chars()
        .enumerate()
        .map(|(i, c)| char_value(c) * WEIGHTS[i % 3])
        .sum::<u32>()
        % 10
}

/// True when `field`'s computed check digit equals the printed `check` char.
/// A `<` check digit (used for optional/empty fields) is treated as 0.
fn check_matches(field: &str, check: char) -> bool {
    let expected = if check == '<' {
        0
    } else if let Some(d) = check.to_digit(10) {
        d
    } else {
        return false;
    };
    check_digit(field) == expected
}

/// Strip trailing (and, defensively, interior) `<` fillers from a fixed-width
/// MRZ field.
fn strip_fillers(raw: &str) -> String {
    raw.trim_end_matches('<').replace('<', "")
}

/// Split the TD3 name field (`SURNAME<<GIVEN<NAMES<<<...`) into
/// (surname, given names). Runs of `<` become word separators.
fn parse_name(name_field: &str) -> (String, String) {
    let mut parts = name_field.splitn(2, "<<");
    let surname = clean_name(parts.next().unwrap_or(""));
    let given = clean_name(parts.next().unwrap_or(""));
    (surname, given)
}

/// Replace `<` with spaces, collapse whitespace, trim.
fn clean_name(raw: &str) -> String {
    raw.replace('<', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Normalize the sex field to `M`, `F`, or `X` (unspecified/filler).
fn normalize_sex(c: char) -> String {
    match c {
        'M' | 'm' => "M".to_string(),
        'F' | 'f' => "F".to_string(),
        _ => "X".to_string(),
    }
}

/// Convert a 6-char `YYMMDD` MRZ date to ISO `YYYY-MM-DD`.
///
/// MRZ dates carry no century. Pivot rules:
/// - Expiry dates always resolve to the 2000s (a passport with a 19xx expiry is
///   not something that will ever be scanned at reception).
/// - Birth dates use a "not in the future" pivot: `20YY` unless that would be a
///   future year, in which case `19YY`.
///
/// If the field cannot be parsed as a plausible date, the raw 6 digits are
/// returned unchanged so nothing is silently dropped.
fn yymmdd_to_iso(raw: &str, is_expiry: bool) -> String {
    let digits: Option<(i32, u32, u32)> = (|| {
        if raw.len() != 6 || !raw.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }
        let yy: i32 = raw[0..2].parse().ok()?;
        let mm: u32 = raw[2..4].parse().ok()?;
        let dd: u32 = raw[4..6].parse().ok()?;
        if !(1..=12).contains(&mm) || !(1..=31).contains(&dd) {
            return None;
        }
        Some((yy, mm, dd))
    })();

    match digits {
        Some((yy, mm, dd)) => {
            let year = if is_expiry {
                2000 + yy
            } else {
                let current = chrono::Utc::now().year();
                let candidate = 2000 + yy;
                if candidate > current {
                    1900 + yy
                } else {
                    candidate
                }
            };
            format!("{:04}-{:02}-{:02}", year, mm, dd)
        }
        None => raw.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Canonical ICAO Doc 9303 TD3 specimen (Anna Maria Eriksson, "Utopia").
    // All check digits are valid.
    const SAMPLE_LINE1: &str = "P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<";
    const SAMPLE_LINE2: &str = "L898902C36UTO7408122F1204159ZE184226B<<<<<10";

    fn sample() -> String {
        format!("{}\n{}", SAMPLE_LINE1, SAMPLE_LINE2)
    }

    #[test]
    fn test_parse_known_good_sample() {
        let data = parse_td3(&sample()).expect("sample should parse");
        assert_eq!(data.passport_number, "L898902C3");
        assert_eq!(data.surname, "ERIKSSON");
        assert_eq!(data.given_names, "ANNA MARIA");
        assert_eq!(data.nationality, "UTO");
        assert_eq!(data.date_of_birth, "1974-08-12");
        assert_eq!(data.sex, "F");
        assert_eq!(data.expiry_date, "2012-04-15");
        assert!(data.checksum_valid, "all check digits should validate");
    }

    #[test]
    fn test_crlf_and_spaces_tolerated() {
        // OCR often emits CRLF line breaks and stray spaces inside the zone.
        let noisy = format!("{}  \r\n {} \r\n", SAMPLE_LINE1, SAMPLE_LINE2);
        let data = parse_td3(&noisy).expect("noisy sample should parse");
        assert_eq!(data.passport_number, "L898902C3");
        assert!(data.checksum_valid);
    }

    #[test]
    fn test_corrupt_check_digit_flags_invalid() {
        // Flip the passport-number check digit 6 -> 5.
        let bad_line2 = "L898902C35UTO7408122F1204159ZE184226B<<<<<10";
        let data = parse_td3(&format!("{}\n{}", SAMPLE_LINE1, bad_line2))
            .expect("still parses geometrically");
        assert!(!data.checksum_valid, "corrupted check digit must be caught");
        // Field extraction is unaffected by the checksum failure.
        assert_eq!(data.passport_number, "L898902C3");
    }

    #[test]
    fn test_wrong_line_count_errors() {
        assert!(parse_td3(SAMPLE_LINE1).is_err());
    }

    #[test]
    fn test_wrong_line_length_errors() {
        let short = "P<UTOERIKSSON";
        assert!(parse_td3(&format!("{}\n{}", short, SAMPLE_LINE2)).is_err());
    }

    #[test]
    fn test_check_digit_algorithm() {
        // ICAO worked example: passport number "L898902C3" -> check digit 6.
        assert_eq!(check_digit("L898902C3"), 6);
        // DOB "740812" -> 2, expiry "120415" -> 9.
        assert_eq!(check_digit("740812"), 2);
        assert_eq!(check_digit("120415"), 9);
    }

    #[test]
    fn test_yymmdd_expiry_always_2000s() {
        assert_eq!(yymmdd_to_iso("300101", true), "2030-01-01");
    }

    #[test]
    fn test_yymmdd_birth_not_in_future() {
        // A birth year that would land in the future resolves to the 1900s.
        assert_eq!(yymmdd_to_iso("990101", false), "1999-01-01");
    }

    #[test]
    fn test_name_with_single_given_name() {
        let (surname, given) = parse_name("SMITH<<JOHN<<<<<<<<<<<<");
        assert_eq!(surname, "SMITH");
        assert_eq!(given, "JOHN");
    }
}
