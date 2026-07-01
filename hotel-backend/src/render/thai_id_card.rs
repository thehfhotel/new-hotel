//! Thai national-ID card compositing — clean reimplement of the iHOTEL
//! `ReportReg` card face (NO vendor lib).
//!
//! A check-in captured via OUR chip card-reader should produce the same *style*
//! of card iHOTEL prints. We composite it server-side (rather than on a client
//! `<canvas>`) for ONE reason: correct Thai complex-script shaping. resvg drives
//! `rustybuzz`, which lays out Thai combining vowels / tone marks correctly;
//! naïve canvas text draws them mis-positioned.
//!
//! ## Layout recipe (extracted from the iHOTEL layout — a 446×273 space)
//!
//! | element                | size | colour        | anchor (x,y) |
//! |------------------------|------|---------------|--------------|
//! | template background    |  —   | —             | fill 446×273 |
//! | chip face photo        |  —   | —             | 332,129 103×119 |
//! | CID (formatted)        | 22px | black #000000 | 200,25       |
//! | Thai full name         | 22px | black #000000 | 110,52       |
//! | English title + first  | 18px | navy  #000080 | 180,78       |
//! | English last name      | 18px | navy  #000080 | 205,96       |
//! | date of birth          | 18px | black #000000 | 190,115      |
//! | issue date             | 18px | navy  #000080 | 220,135      |
//! | address line 1         | 18px | black #000000 | 71,175       |
//! | address line 2         | 18px | black #000000 | 44,195       |
//!
//! All text is BOLD, alphabetic baseline (the SVG default), rendered in the
//! loaded Thai font's own family name.

use base64::Engine as _;

/// Card canvas geometry (the coordinate space every recipe number is in).
const CARD_W: u32 = 446;
const CARD_H: u32 = 273;

/// The typed inputs for one card. All fields are plain `String`; blank fields
/// simply render empty. The caller (route handler) maps its request body onto
/// this — the renderer takes no request/DB types so it stays unit-testable.
#[derive(Debug, Clone, Default)]
pub struct ThaiIdCardFields {
    pub cid: String,
    pub thai_title: String,
    pub thai_first_name: String,
    pub thai_last_name: String,
    pub english_title: String,
    pub english_first_name: String,
    pub english_last_name: String,
    pub date_of_birth: String,
    pub issue_date: String,
    pub address: String,
}

/// Why a render could not be produced. Every variant is a *graceful* failure —
/// the route handler maps any of them to `{success:false}` (HTTP 200) so the
/// client falls back to uploading the raw face photo. We NEVER panic.
#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("template unreadable ({0})")]
    Template(String),
    #[error("font unreadable ({0})")]
    Font(String),
    #[error("svg parse failed: {0}")]
    SvgParse(String),
    #[error("render failed: {0}")]
    Render(String),
}

/// Composite a Thai national-ID card and return it as PNG bytes.
///
/// Reads `template_path` (PNG) and `font_path` (TTF/OTF) from disk, base64-
/// embeds the template + `face_jpeg` into an SVG, shapes the text with the
/// loaded font via resvg/rustybuzz, and rasterises to a 446×273 PNG.
///
/// Returns [`RenderError`] (never panics) when an asset is missing/unreadable
/// or the SVG fails to parse/render.
pub fn render_card(
    fields: &ThaiIdCardFields,
    face_jpeg: &[u8],
    template_path: &str,
    font_path: &str,
) -> Result<Vec<u8>, RenderError> {
    // Template PNG → base64 data-URL payload.
    let template_bytes = std::fs::read(template_path)
        .map_err(|e| RenderError::Template(format!("{template_path}: {e}")))?;
    let template_b64 = base64::engine::general_purpose::STANDARD.encode(&template_bytes);
    let face_b64 = base64::engine::general_purpose::STANDARD.encode(face_jpeg);

    // Load the Thai font into its own fontdb; the family name it advertises is
    // what we set as `font-family` on every <text> so resvg resolves to it
    // exactly (an empty db has no fallback, so the name must match).
    let mut db = fontdb::Database::new();
    db.load_font_file(font_path)
        .map_err(|e| RenderError::Font(format!("{font_path}: {e}")))?;
    let family = db
        .faces()
        .next()
        .and_then(|face| face.families.first().map(|(name, _lang)| name.clone()))
        .ok_or_else(|| RenderError::Font(format!("no usable face in {font_path}")))?;

    let svg = build_svg(fields, &template_b64, &face_b64, &family);

    let mut options = usvg::Options::default();
    // Move our single-font db in. Every <text> carries an explicit
    // `font-family` set to exactly this db's advertised family, so resvg
    // resolves to it directly — no reliance on the default-family fallback.
    *options.fontdb_mut() = db;

    let tree =
        usvg::Tree::from_str(&svg, &options).map_err(|e| RenderError::SvgParse(e.to_string()))?;

    let mut pixmap = tiny_skia::Pixmap::new(CARD_W, CARD_H)
        .ok_or_else(|| RenderError::Render(format!("cannot allocate {CARD_W}x{CARD_H} pixmap")))?;
    resvg::render(&tree, tiny_skia::Transform::identity(), &mut pixmap.as_mut());

    pixmap
        .encode_png()
        .map_err(|e| RenderError::Render(format!("png encode: {e}")))
}

/// Assemble the SVG the renderer rasterises. Split out so tests can assert the
/// exact coordinates/sizes/colours without touching disk or the font stack.
fn build_svg(fields: &ThaiIdCardFields, template_b64: &str, face_b64: &str, family: &str) -> String {
    let fam = xml_escape(family);
    let cid = xml_escape(&format_cid(&fields.cid));
    let thai_name = xml_escape(
        format!(
            "{} {} {}",
            fields.thai_title.trim(),
            fields.thai_first_name.trim(),
            fields.thai_last_name.trim()
        )
        .trim(),
    );
    let english_first = xml_escape(
        format!(
            "{} {}",
            fields.english_title.trim(),
            fields.english_first_name.trim()
        )
        .trim(),
    );
    let english_last = xml_escape(fields.english_last_name.trim());
    let dob = xml_escape(&format_date(&fields.date_of_birth));
    let issue = xml_escape(&format_date(&fields.issue_date));
    let (addr1, addr2) = split_address(&fields.address);
    let addr1 = xml_escape(&addr1);
    let addr2 = xml_escape(&addr2);

    // Note: base64 (A–Z a–z 0–9 + / =) carries no XML-special chars, so the
    // data-URL payloads need no escaping. Only the human text is escaped above.
    format!(
        concat!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\">",
            "<image href=\"data:image/png;base64,{template}\" x=\"0\" y=\"0\" width=\"{w}\" height=\"{h}\" preserveAspectRatio=\"none\"/>",
            "<image href=\"data:image/jpeg;base64,{face}\" x=\"332\" y=\"129\" width=\"103\" height=\"119\" preserveAspectRatio=\"xMidYMid slice\"/>",
            "<text x=\"200\" y=\"25\" font-family=\"{fam}\" font-weight=\"bold\" font-size=\"22\" fill=\"#000000\">{cid}</text>",
            "<text x=\"110\" y=\"52\" font-family=\"{fam}\" font-weight=\"bold\" font-size=\"22\" fill=\"#000000\">{thai_name}</text>",
            "<text x=\"180\" y=\"78\" font-family=\"{fam}\" font-weight=\"bold\" font-size=\"18\" fill=\"#000080\">{english_first}</text>",
            "<text x=\"205\" y=\"96\" font-family=\"{fam}\" font-weight=\"bold\" font-size=\"18\" fill=\"#000080\">{english_last}</text>",
            "<text x=\"190\" y=\"115\" font-family=\"{fam}\" font-weight=\"bold\" font-size=\"18\" fill=\"#000000\">{dob}</text>",
            "<text x=\"220\" y=\"135\" font-family=\"{fam}\" font-weight=\"bold\" font-size=\"18\" fill=\"#000080\">{issue}</text>",
            "<text x=\"71\" y=\"175\" font-family=\"{fam}\" font-weight=\"bold\" font-size=\"18\" fill=\"#000000\">{addr1}</text>",
            "<text x=\"44\" y=\"195\" font-family=\"{fam}\" font-weight=\"bold\" font-size=\"18\" fill=\"#000000\">{addr2}</text>",
            "</svg>",
        ),
        w = CARD_W,
        h = CARD_H,
        template = template_b64,
        face = face_b64,
        fam = fam,
        cid = cid,
        thai_name = thai_name,
        english_first = english_first,
        english_last = english_last,
        dob = dob,
        issue = issue,
        addr1 = addr1,
        addr2 = addr2,
    )
}

/// Format a 13-digit CID as `X XXXX XXXXX XX X` (the iHOTEL card grouping).
/// Non-digits are stripped first; anything that isn't exactly 13 digits is
/// passed through trimmed (defensive — a partial read still prints *something*).
fn format_cid(raw: &str) -> String {
    let digits: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() == 13 {
        format!(
            "{} {} {} {} {}",
            &digits[0..1],
            &digits[1..5],
            &digits[5..10],
            &digits[10..12],
            &digits[12..13],
        )
    } else {
        raw.trim().to_string()
    }
}

/// Render a date as `DD/MM/YYYY` when the input is ISO (`YYYY-MM-DD`, optionally
/// followed by `T…`/` …` time); otherwise pass it through unchanged (trimmed).
fn format_date(raw: &str) -> String {
    let s = raw.trim();
    if s.is_empty() {
        return String::new();
    }
    // Isolate the date part before any time separator.
    let date_part = s.split(['T', ' ']).next().unwrap_or(s);
    let parts: Vec<&str> = date_part.split('-').collect();
    if parts.len() == 3 {
        let (y, m, d) = (parts[0], parts[1], parts[2]);
        let iso = y.len() == 4
            && m.len() == 2
            && d.len() == 2
            && y.chars().all(|c| c.is_ascii_digit())
            && m.chars().all(|c| c.is_ascii_digit())
            && d.chars().all(|c| c.is_ascii_digit());
        if iso {
            return format!("{d}/{m}/{y}");
        }
    }
    s.to_string()
}

/// Split one address string into two lines at the first `ตำบล`/`ต.` marker:
/// everything before the marker is line 1, the marker onward is line 2. With no
/// marker the whole string is line 1 and line 2 is empty. `str::find` returns a
/// char-boundary byte index (the markers begin on one), so slicing is safe.
fn split_address(addr: &str) -> (String, String) {
    let a = addr.trim();
    for marker in ["ตำบล", "ต."] {
        if let Some(idx) = a.find(marker) {
            let line1 = a[..idx].trim().to_string();
            let line2 = a[idx..].trim().to_string();
            return (line1, line2);
        }
    }
    (a.to_string(), String::new())
}

/// Escape the five XML-significant characters so dynamic text can't break the
/// SVG document (or inject markup).
fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cid_groups_thirteen_digits() {
        assert_eq!(format_cid("1234567890123"), "1 2345 67890 12 3");
    }

    #[test]
    fn cid_strips_existing_spaces_then_groups() {
        assert_eq!(format_cid("1 2345 67890 12 3"), "1 2345 67890 12 3");
    }

    #[test]
    fn cid_passthrough_when_not_thirteen_digits() {
        assert_eq!(format_cid("  AB12  "), "AB12");
    }

    #[test]
    fn date_iso_becomes_ddmmyyyy() {
        assert_eq!(format_date("1990-05-09"), "09/05/1990");
    }

    #[test]
    fn date_iso_with_time_drops_time() {
        assert_eq!(format_date("2026-01-22T11:59:00.000Z"), "22/01/2026");
    }

    #[test]
    fn date_non_iso_passthrough() {
        assert_eq!(format_date("9 พ.ค. 2533"), "9 พ.ค. 2533");
        assert_eq!(format_date(""), "");
    }

    #[test]
    fn address_splits_on_tambon_word() {
        let (l1, l2) = split_address("123 หมู่ 4 ถนนสุข ตำบลบางนา อำเภอเมือง จังหวัดชลบุรี");
        assert_eq!(l1, "123 หมู่ 4 ถนนสุข");
        assert_eq!(l2, "ตำบลบางนา อำเภอเมือง จังหวัดชลบุรี");
    }

    #[test]
    fn address_splits_on_abbreviation() {
        let (l1, l2) = split_address("99/1 ถนนพระราม 4 ต.คลองเตย อ.คลองเตย");
        assert_eq!(l1, "99/1 ถนนพระราม 4");
        assert_eq!(l2, "ต.คลองเตย อ.คลองเตย");
    }

    #[test]
    fn address_no_marker_all_line1() {
        let (l1, l2) = split_address("123 Sukhumvit Road");
        assert_eq!(l1, "123 Sukhumvit Road");
        assert_eq!(l2, "");
    }

    #[test]
    fn xml_escape_covers_the_five() {
        assert_eq!(xml_escape("a<b>&\"'"), "a&lt;b&gt;&amp;&quot;&apos;");
    }

    #[test]
    fn svg_carries_exact_coords_sizes_and_colours() {
        let fields = ThaiIdCardFields {
            cid: "1234567890123".into(),
            thai_title: "นาย".into(),
            thai_first_name: "สมชาย".into(),
            thai_last_name: "ใจดี".into(),
            english_title: "Mr.".into(),
            english_first_name: "Somchai".into(),
            english_last_name: "Jaidee".into(),
            date_of_birth: "1990-05-09".into(),
            issue_date: "2020-01-01".into(),
            address: "123 ถนนสุข ตำบลบางนา".into(),
        };
        let svg = build_svg(&fields, "UFBB", "SkpH", "Loma");

        // Canvas + both images.
        assert!(svg.contains("width=\"446\" height=\"273\" viewBox=\"0 0 446 273\""));
        assert!(svg.contains("data:image/png;base64,UFBB"));
        assert!(svg.contains(
            "data:image/jpeg;base64,SkpH\" x=\"332\" y=\"129\" width=\"103\" height=\"119\" preserveAspectRatio=\"xMidYMid slice\""
        ));
        // CID anchor + grouping + colour + size.
        assert!(svg
            .contains("x=\"200\" y=\"25\" font-family=\"Loma\" font-weight=\"bold\" font-size=\"22\" fill=\"#000000\">1 2345 67890 12 3<"));
        // Thai full name.
        assert!(svg.contains("x=\"110\" y=\"52\"") && svg.contains(">นาย สมชาย ใจดี<"));
        // English title+first (navy).
        assert!(svg.contains("x=\"180\" y=\"78\"") && svg.contains("fill=\"#000080\">Mr. Somchai<"));
        // English last (navy).
        assert!(svg.contains("x=\"205\" y=\"96\"") && svg.contains("fill=\"#000080\">Jaidee<"));
        // Dates reformatted to DD/MM/YYYY.
        assert!(svg.contains("x=\"190\" y=\"115\"") && svg.contains(">09/05/1990<"));
        assert!(svg.contains("x=\"220\" y=\"135\"") && svg.contains(">01/01/2020<"));
        // Address split.
        assert!(svg.contains("x=\"71\" y=\"175\"") && svg.contains(">123 ถนนสุข<"));
        assert!(svg.contains("x=\"44\" y=\"195\"") && svg.contains(">ตำบลบางนา<"));
    }
}
