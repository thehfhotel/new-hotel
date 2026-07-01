# Backend bundled assets — Thai national-ID card render

These files are COPY'd into the runtime image by `hotel-backend/Dockerfile` and
are the defaults for `render::thai_id_card` (see `config.rs`
`thai_id_template_path()` / `thai_id_font_path()`).

| file | image path | purpose |
|---|---|---|
| `thai_id_template.png` | `/app/assets/thai_id_template.png` | blank card background (`THAI_ID_TEMPLATE_PATH`) |
| `fonts/DilleniaUPC-Bold.ttf` | `/usr/share/fonts/truetype/dillenia/DilleniaUPC-Bold.ttf` | card text face (`THAI_ID_FONT_PATH`) |

## Provenance

Both are transcribed from the licensed iHOTEL toolset so OUR server-side card
matches what iHOTEL prints (see `render/thai_id_card.rs` header for the full
port notes):

- **`thai_id_template.png`** — the `Untitled_1__Copy_` resource (504×322) inside
  `KPThaiNationalIDCard.exe`, the blank card the reference paint handler draws
  onto. Stretched to the 446×273 render canvas at runtime.
- **`DilleniaUPC-Bold.ttf`** — the exact face the reference draws with
  (`new Font("DilleniaUPC", …, Bold)` → Windows `upcdb.ttf`). Its ascent/em ratio
  underpins the GDI+→SVG baseline math in `render/thai_id_card.rs`.

## Licensing note

`DilleniaUPC-Bold.ttf` is a proprietary Microsoft/Thai UPC font and the template
is a government-card facsimile. They are committed here (per an explicit decision
to bundle them in the backend image) as an intentional exception to the
`.gitignore` keep-out policy for card/vendor artifacts. Do not redistribute
outside this project's images. The free `fonts-tlwg-loma-ttf` (also installed in
the image) remains available as a fallback via `THAI_ID_FONT_PATH`.
