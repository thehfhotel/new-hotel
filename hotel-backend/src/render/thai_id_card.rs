//! Thai national-ID card compositing — a clean reimplementation of the iHOTEL
//! `KPThaiNationalIDCard` card face (NO vendor lib).
//!
//! A check-in captured via OUR chip card-reader should produce the same card
//! iHOTEL prints. We composite it server-side (rather than on a client
//! `<canvas>`) for ONE reason: correct Thai complex-script shaping. resvg drives
//! `rustybuzz`, which lays out Thai combining vowels / tone marks correctly;
//! naïve canvas text draws them mis-positioned.
//!
//! ## Provenance — this is a faithful port of the reference paint handler
//!
//! The layout, fonts, colours, date wording and address handling below are
//! transcribed 1:1 from `KPThaiNationalIDCard.exe`'s `uuu(...)` PictureBox.Paint
//! handler (the routine that draws onto the blank card and saves the PNG). The
//! reference draws onto a **446×273** picture box (`pictureBox1`, itself placed
//! at (121,91) inside a 703×996 white panel — which is exactly how our
//! `writeback::save_image` pads this card for iHOTEL's `Tb_Save_Image`).
//!
//! Two GDI+ → SVG coordinate-model differences are corrected here (they were the
//! cause of the previously "all over the place" text):
//!
//! 1. **Font size unit.** WinForms `new Font(name, 22f, Bold)` is 22 **points**;
//!    at 96 DPI that is `22 * 96/72 = 29.33` **px**. We render in px, so every
//!    point size is scaled by 4/3. Sizes used: 22pt→29.33px, 18pt→24px,
//!    14pt→18.67px.
//! 2. **Anchor origin.** GDI+ `DrawString(text, font, brush, PointF)` treats the
//!    point as the **top-left** of the text box; SVG `<text y>` is the
//!    **baseline**. The per-size baseline drop (and the small left inset GDI+
//!    adds) were measured from GDI+ with the real font — see
//!    `gdi_baseline_and_inset` — and applied so glyphs land where the reference
//!    put them.
//!
//! ## Layout recipe (446×273 space; `pt` = the reference's WinForms point size)
//!
//! | element                    | pt | colour        | anchor (x,y) |
//! |----------------------------|----|---------------|--------------|
//! | template background        | —  | —             | fill 446×273 |
//! | CID (grouped)              | 22 | black #000000 | 200,25       |
//! | Thai full name             | 22 | black #000000 | 110,52       |
//! | English title + first      | 18 | navy  #000080 | 180,78       |
//! | English last name          | 18 | navy  #000080 | 205,96       |
//! | date of birth — Thai (BE)  | 18 | black #000000 | 190,115      |
//! | date of birth — Eng (AD)   | 18 | navy  #000080 | 220,135      |
//! | address line 1             | 18 | black #000000 | 71,175       |
//! | address line 2             | 18 | black #000000 | 44,195       |
//! | issue date — Thai (BE)     | 14 | black #000000 | 44,215       |
//! | issue date — Eng (AD)      | 14 | navy  #000080 | 44,237       |
//! | expiry date — Thai (BE)    | 14 | black #000000 | 244,215      |
//! | expiry date — Eng (AD)     | 14 | navy  #000080 | 244,237      |
//! | chip face photo            | —  | —             | 332,129 103×119 |
//!
//! All text is BOLD in the loaded font's own family (the reference used
//! **DilleniaUPC Bold**; the backend image ships that face — see the Dockerfile).

use base64::Engine as _;

/// Card canvas geometry (the coordinate space every recipe number is in).
const CARD_W: u32 = 446;
const CARD_H: u32 = 273;

/// Colours the reference uses: plain black, and the navy (`Color.FromArgb(0,0,128)`)
/// it applies to every Latin-script / Gregorian row.
const BLACK: &str = "#000000";
const NAVY: &str = "#000080";

/// Thai month abbreviations (index 0 = January), verbatim from the reference's
/// decrypted string table (`ม.ค.` … `ธ.ค.`).
const TH_MONTHS: [&str; 12] = [
    "ม.ค.", "ก.พ.", "มี.ค.", "เม.ย.", "พ.ค.", "มิ.ย.", "ก.ค.", "ส.ค.", "ก.ย.", "ต.ค.", "พ.ย.",
    "ธ.ค.",
];

/// English month abbreviations (index 0 = January). The reference prints these
/// WITH a trailing dot (`Jan.` … `Dec.`, and notably `May.`).
const EN_MONTHS: [&str; 12] = [
    "Jan.", "Feb.", "Mar.", "Apr.", "May.", "Jun.", "Jul.", "Aug.", "Sep.", "Oct.", "Nov.", "Dec.",
];

/// The typed inputs for one card. All fields are plain `String`; blank fields
/// simply render empty. The caller (route handler) maps its request body onto
/// this — the renderer takes no request/DB types so it stays unit-testable.
///
/// `date_of_birth` / `issue_date` / `expire_date` arrive as the chip's
/// `YYYY-MM-DD` **Buddhist-era** dates (that is what our card-reader middleware
/// emits); the bilingual formatters below add 543 / subtract 543 as needed.
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
    pub expire_date: String,
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
/// exact coordinates/sizes/colours/wording without touching disk or the font
/// stack.
fn build_svg(fields: &ThaiIdCardFields, template_b64: &str, face_b64: &str, family: &str) -> String {
    let fam = xml_escape(family);

    let cid = format_cid(&fields.cid);
    let thai_name = join_parts(&[
        fields.thai_title.as_str(),
        fields.thai_first_name.as_str(),
        fields.thai_last_name.as_str(),
    ]);
    let english_first = join_parts(&[
        fields.english_title.as_str(),
        fields.english_first_name.as_str(),
    ]);
    let english_last = fields.english_last_name.trim().to_string();

    let dob_th = thai_date(&fields.date_of_birth);
    let dob_en = eng_date(&fields.date_of_birth);
    let issue_th = thai_date(&fields.issue_date);
    let issue_en = eng_date(&fields.issue_date);
    let expire_th = thai_date(&fields.expire_date);
    let expire_en = eng_date(&fields.expire_date);

    let (addr1, addr2) = split_address(&fields.address);

    let mut svg = String::with_capacity(template_b64.len() + face_b64.len() + 2048);
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{CARD_W}\" height=\"{CARD_H}\" viewBox=\"0 0 {CARD_W} {CARD_H}\">"
    ));
    // Template background — stretched to fill (the reference's PictureBox uses
    // SizeMode=StretchImage, so preserveAspectRatio="none" matches it exactly).
    svg.push_str(&format!(
        "<image href=\"data:image/png;base64,{template_b64}\" x=\"0\" y=\"0\" width=\"{CARD_W}\" height=\"{CARD_H}\" preserveAspectRatio=\"none\"/>"
    ));

    // Text rows — (anchor_x, anchor_y, point_size, colour, text).
    let rows: [(f32, f32, f32, &str, &str); 12] = [
        (200.0, 25.0, 22.0, BLACK, cid.as_str()),
        (110.0, 52.0, 22.0, BLACK, thai_name.as_str()),
        (180.0, 78.0, 18.0, NAVY, english_first.as_str()),
        (205.0, 96.0, 18.0, NAVY, english_last.as_str()),
        (190.0, 115.0, 18.0, BLACK, dob_th.as_str()),
        (220.0, 135.0, 18.0, NAVY, dob_en.as_str()),
        (71.0, 175.0, 18.0, BLACK, addr1.as_str()),
        (44.0, 195.0, 18.0, BLACK, addr2.as_str()),
        (44.0, 215.0, 14.0, BLACK, issue_th.as_str()),
        (44.0, 237.0, 14.0, NAVY, issue_en.as_str()),
        (244.0, 215.0, 14.0, BLACK, expire_th.as_str()),
        (244.0, 237.0, 14.0, NAVY, expire_en.as_str()),
    ];
    for (x, y, pt, fill, text) in rows {
        push_text(&mut svg, x, y, pt, fill, &fam, text);
    }

    // Chip face photo — drawn last (over the template), same rect the reference
    // uses. `slice` fills the box, cropping overflow like WinForms StretchImage.
    svg.push_str(&format!(
        "<image href=\"data:image/jpeg;base64,{face_b64}\" x=\"332\" y=\"129\" width=\"103\" height=\"119\" preserveAspectRatio=\"xMidYMid slice\"/>"
    ));
    svg.push_str("</svg>");
    svg
}

/// Emit one `<text>` element, converting the reference's GDI+ top-left anchor
/// (in points) to an SVG baseline anchor (in px). No-op for empty text.
fn push_text(out: &mut String, x: f32, y: f32, pt: f32, fill: &str, fam: &str, text: &str) {
    if text.trim().is_empty() {
        return;
    }
    let px = pt * 4.0 / 3.0; // WinForms points → px at 96 DPI
    let (baseline_dy, left_dx) = gdi_baseline_and_inset(pt, px);
    out.push_str(&format!(
        "<text x=\"{:.2}\" y=\"{:.2}\" font-family=\"{}\" font-weight=\"bold\" font-size=\"{:.3}\" fill=\"{}\">{}</text>",
        x + left_dx,
        y + baseline_dy,
        fam,
        px,
        fill,
        xml_escape(text),
    ));
}

/// GDI+ `DrawString(PointF)` places the text box's **top-left** at the anchor;
/// SVG places the **baseline**. Returns `(baseline_drop, left_inset)` in px to
/// add to the anchor so glyphs land where the reference drew them.
///
/// The three tiers are the exact values measured from GDI+ with DilleniaUPC Bold
/// (`.decompile/measure_baseline.ps1`: 22pt→25px, 18pt→20px, 14pt→16px baseline
/// drop; left inset ≈ em/6). The `_` arm keeps the renderer sane for any other
/// size a future field might use (≈0.85·em drop, em/6 inset).
fn gdi_baseline_and_inset(pt: f32, px: f32) -> (f32, f32) {
    match pt as u32 {
        22 => (25.0, 5.0),
        18 => (20.0, 4.0),
        14 => (16.0, 3.0),
        _ => ((px * 0.853).round(), (px / 6.0).round()),
    }
}

/// Join name parts with single spaces, dropping blanks (so a missing title
/// doesn't leave a double space). The reference concatenates with a literal
/// space between each part; for real card data this is identical.
fn join_parts(parts: &[&str]) -> String {
    parts
        .iter()
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Format a 13-digit CID as `X XXXX XXXXX XX X` (the card's grouping).
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

/// Parse a date into `(year, month, day)`. Accepts ISO `YYYY-MM-DD` (optionally
/// followed by `T…`/` …` time) and raw 8-digit `YYYYMMDD`. Returns `None` for
/// anything unparseable (the caller then renders that row empty).
fn date_parts(raw: &str) -> Option<(i32, u32, u32)> {
    let s = raw.trim();
    if s.is_empty() {
        return None;
    }
    let date_part = s.split(['T', ' ']).next().unwrap_or(s);

    let seg: Vec<&str> = date_part.split('-').collect();
    let (y, m, d) = if seg.len() == 3 {
        (
            seg[0].parse::<i32>().ok()?,
            seg[1].parse::<u32>().ok()?,
            seg[2].parse::<u32>().ok()?,
        )
    } else if date_part.len() == 8 && date_part.chars().all(|c| c.is_ascii_digit()) {
        (
            date_part[0..4].parse::<i32>().ok()?,
            date_part[4..6].parse::<u32>().ok()?,
            date_part[6..8].parse::<u32>().ok()?,
        )
    } else {
        return None;
    };

    if (1..=12).contains(&m) && (1..=31).contains(&d) {
        Some((y, m, d))
    } else {
        None
    }
}

/// The Thai (upper) date row: `DD <thai-month-abbr> <Buddhist-year>`, e.g.
/// `09 พ.ค. 2533`. Input is normally already Buddhist-era; a Gregorian-looking
/// year (< 2400) is bumped by 543 so the Thai row always shows the BE year.
fn thai_date(raw: &str) -> String {
    match date_parts(raw) {
        Some((y, m, d)) => {
            let be = if y >= 2400 { y } else { y + 543 };
            format!("{:02} {} {}", d, TH_MONTHS[(m - 1) as usize], be)
        }
        None => String::new(),
    }
}

/// The English (lower) date row: `DD <eng-month-abbr> <Gregorian-year>`, e.g.
/// `09 May. 1990`. Input is normally Buddhist-era, so 543 is subtracted; an
/// already-Gregorian year (< 2400) is left as-is.
fn eng_date(raw: &str) -> String {
    match date_parts(raw) {
        Some((y, m, d)) => {
            let ad = if y >= 2400 { y - 543 } else { y };
            format!("{:02} {} {}", d, EN_MONTHS[(m - 1) as usize], ad)
        }
        None => String::new(),
    }
}

/// Split the address into the card's two lines, replicating the reference's
/// transform exactly:
///
/// 1. Abbreviate the region words, inserting a newline before the district:
///    `ตำบล→ต.`, `อำเภอ→\nอ.`, `จังหวัด→จ.`, `แขวง→\nแขวง.`
/// 2. Line 2 is everything after the **last** newline; line 1 is everything
///    before it (with any remaining newline removed).
///
/// With no district marker the whole (abbreviated) string is line 1.
fn split_address(addr: &str) -> (String, String) {
    let s = addr
        .trim()
        .replace("ตำบล", "ต.")
        .replace("อำเภอ", "\nอ.")
        .replace("จังหวัด", "จ.")
        .replace("แขวง", "\nแขวง.");
    match s.rfind('\n') {
        // `\n` is ASCII, so the byte index is a valid char boundary.
        Some(i) => {
            let line1 = s[..i].replace('\n', "").trim().to_string();
            let line2 = s[i + 1..].trim().to_string();
            (line1, line2)
        }
        None => (s.trim().to_string(), String::new()),
    }
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
    fn thai_date_uses_be_year_and_thai_month() {
        // Buddhist-era input stays BE on the Thai row.
        assert_eq!(thai_date("2533-05-09"), "09 พ.ค. 2533");
        // Gregorian-looking input is bumped to BE.
        assert_eq!(thai_date("1990-05-09"), "09 พ.ค. 2533");
    }

    #[test]
    fn eng_date_uses_ad_year_and_dotted_month() {
        // Buddhist-era input drops 543 for the English row; month carries a dot.
        assert_eq!(eng_date("2533-05-09"), "09 May. 1990");
        assert_eq!(eng_date("2560-03-15"), "15 Mar. 2017");
    }

    #[test]
    fn dates_with_time_and_bad_input() {
        assert_eq!(thai_date("2533-05-09T00:00:00.000Z"), "09 พ.ค. 2533");
        assert_eq!(eng_date("25330509"), "09 May. 1990"); // raw 8-digit
        assert_eq!(thai_date(""), "");
        assert_eq!(eng_date("not-a-date"), "");
        assert_eq!(thai_date("2533-13-09"), ""); // month out of range
    }

    #[test]
    fn address_abbreviates_and_splits_before_district() {
        let (l1, l2) =
            split_address("50 หมู่ 5 ตำบลบางแก้ว อำเภอบางพลี จังหวัดสมุทรปราการ");
        assert_eq!(l1, "50 หมู่ 5 ต.บางแก้ว");
        assert_eq!(l2, "อ.บางพลี จ.สมุทรปราการ");
    }

    #[test]
    fn address_bangkok_breaks_before_khwaeng() {
        let (l1, l2) = split_address("99/1 ถนนพระราม 4 แขวงคลองเตย");
        assert_eq!(l1, "99/1 ถนนพระราม 4");
        assert_eq!(l2, "แขวง.คลองเตย");
    }

    #[test]
    fn address_no_marker_all_line1() {
        let (l1, l2) = split_address("123 Sukhumvit Road");
        assert_eq!(l1, "123 Sukhumvit Road");
        assert_eq!(l2, "");
    }

    #[test]
    fn join_parts_drops_blanks() {
        assert_eq!(join_parts(&["นาย", "สมชาย", "ใจดี"]), "นาย สมชาย ใจดี");
        assert_eq!(join_parts(&["", "Somchai", ""]), "Somchai");
    }

    #[test]
    fn xml_escape_covers_the_five() {
        assert_eq!(xml_escape("a<b>&\"'"), "a&lt;b&gt;&amp;&quot;&apos;");
    }

    #[test]
    fn point_size_is_scaled_to_px_and_baseline_dropped() {
        // 22pt → 29.333px, anchored baseline at y+25, x+5.
        let fields = ThaiIdCardFields {
            cid: "1234567890123".into(),
            ..Default::default()
        };
        let svg = build_svg(&fields, "UFBB", "SkpH", "DilleniaUPC");
        // CID: anchor (200,25) 22pt → x=205, y=50, font-size 29.333.
        assert!(svg.contains(
            "<text x=\"205.00\" y=\"50.00\" font-family=\"DilleniaUPC\" font-weight=\"bold\" font-size=\"29.333\" fill=\"#000000\">1 2345 67890 12 3</text>"
        ));
    }

    #[test]
    fn svg_carries_all_rows_sizes_and_colours() {
        let fields = ThaiIdCardFields {
            cid: "1234567890123".into(),
            thai_title: "นาย".into(),
            thai_first_name: "สมชาย".into(),
            thai_last_name: "ใจดี".into(),
            english_title: "Mr.".into(),
            english_first_name: "Somchai".into(),
            english_last_name: "Jaidee".into(),
            date_of_birth: "2533-05-09".into(),
            issue_date: "2560-03-15".into(),
            expire_date: "2568-03-14".into(),
            address: "50 หมู่ 5 ตำบลบางแก้ว อำเภอบางพลี จังหวัดสมุทรปราการ".into(),
        };
        let svg = build_svg(&fields, "UFBB", "SkpH", "DilleniaUPC");

        // Canvas + both images.
        assert!(svg.contains("width=\"446\" height=\"273\" viewBox=\"0 0 446 273\""));
        assert!(svg.contains("data:image/png;base64,UFBB"));
        assert!(svg.contains(
            "data:image/jpeg;base64,SkpH\" x=\"332\" y=\"129\" width=\"103\" height=\"119\" preserveAspectRatio=\"xMidYMid slice\""
        ));
        // Thai full name (22pt, black) at (110,52) → (115,77).
        assert!(svg.contains("y=\"77.00\"") && svg.contains(">นาย สมชาย ใจดี</text>"));
        // English rows are navy.
        assert!(svg.contains("fill=\"#000080\">Mr. Somchai</text>"));
        assert!(svg.contains("fill=\"#000080\">Jaidee</text>"));
        // Bilingual DOB.
        assert!(svg.contains(">09 พ.ค. 2533</text>"));
        assert!(svg.contains("fill=\"#000080\">09 May. 1990</text>"));
        // Bilingual issue + expiry (14pt tier).
        assert!(svg.contains(">15 มี.ค. 2560</text>"));
        assert!(svg.contains("fill=\"#000080\">15 Mar. 2017</text>"));
        assert!(svg.contains(">14 มี.ค. 2568</text>"));
        assert!(svg.contains("fill=\"#000080\">14 Mar. 2025</text>"));
        // Address split.
        assert!(svg.contains(">50 หมู่ 5 ต.บางแก้ว</text>"));
        assert!(svg.contains(">อ.บางพลี จ.สมุทรปราการ</text>"));
    }
}
