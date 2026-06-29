-- Migration: 067_create_ht_feedback_forms
-- Version: vNext
-- Date: 2026-06-29
-- Tier 1 data-driven feedback forms (follow-on to task #58/#71).
--
-- ## Background
--
-- The reception feedback / re-verification forms change FREQUENTLY (new questions,
-- edited wording, per-site item tweaks). Hardcoding them in the Next.js pages means
-- every question edit needs a full frontend image rebuild + CI/CD deploy. This table
-- moves the form DEFINITION out of code and into data: a generic renderer fetches a
-- form's schema from here and renders whatever it says, so editing a question is a
-- DB write (no frontend rebuild).
--
-- ## Schema notes
--
-- - PG-CANONICAL ONLY. App-internal config — iHOTEL has NO counterpart (legacy =
--   none, like `ht_users` / `ht_verification_responses`). No sync mapper, no writeback.
-- - `form_schema JSONB` is the question list, e.g.
--   {"questions":[
--      {"id":"rv_invoice","type":"radio","label":"…","required":true,
--       "options":[{"value":"match","label":"ตรง"},{"value":"mismatch","label":"ไม่ตรง"}]},
--      {"id":"rv_invoice_note","type":"text","label":"ระบุยอดที่เห็น",
--       "showIf":{"field":"rv_invoice","equals":"mismatch"}}
--   ]}.
--   JSONB keeps it flexible: questions can be added/edited WITHOUT a migration — just
--   UPDATE the row. Answers continue to land in `ht_verification_responses.vr_answers`
--   (already free-form JSONB) via POST /api/verification, tagged with `kind = form_kind`.
-- - `form_key` is the stable lookup id (e.g. 'reverify_hfhotel'); `form_site` is the
--   branch the form is ABOUT; `form_kind` groups responses (e.g. 'reverify').
-- - Per-site (connection-level scoping) — exists in BOTH hotelnew + hotelville, but
--   the API reads the primary pool (forms are global config, like the responses store).

-- UP MIGRATION

CREATE TABLE IF NOT EXISTS ht_feedback_forms (
    form_id      BIGSERIAL   PRIMARY KEY,
    -- Stable lookup id, e.g. 'reverify_hfhotel'.
    form_key     TEXT        NOT NULL UNIQUE,
    -- Branch the form is ABOUT ('hfhotel' / 'hfville' / 'all' / NULL).
    form_site    TEXT,
    -- Groups the submitted responses (written into vr_answers.kind), e.g. 'reverify'.
    form_kind    TEXT        NOT NULL DEFAULT 'reverify',
    -- Heading shown at the top of the rendered form.
    form_title   TEXT        NOT NULL,
    -- Optional intro/help paragraph.
    form_intro   TEXT,
    -- The question list (see header for shape). Rendered generically by the frontend.
    form_schema  JSONB       NOT NULL,
    -- Soft on/off without deleting the row.
    form_active  BOOLEAN     NOT NULL DEFAULT TRUE,
    -- Display ordering on the hub.
    form_sort    INTEGER     NOT NULL DEFAULT 0,
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS ix_ht_feedback_forms_active
    ON ht_feedback_forms (form_active, form_sort);

COMMENT ON TABLE ht_feedback_forms IS
    'Data-driven reception feedback / re-verification form definitions (Tier 1, '
    'migration 067). form_schema JSONB holds the question list rendered generically '
    'by the frontend, so editing questions is a DB write (no frontend rebuild). '
    'PG-CANONICAL ONLY (no legacy, no sync, no writeback). Answers land in '
    'ht_verification_responses.vr_answers tagged with kind=form_kind.';

-- Seed the two per-site re-verification forms (idempotent). These mirror the
-- questions the hardcoded /v2/verification/reverify page used to carry; editing
-- them from here on is just an UPDATE to form_schema.
INSERT INTO ht_feedback_forms (form_key, form_site, form_kind, form_title, form_intro, form_schema, form_sort)
VALUES
(
  'reverify_hfhotel', 'hfhotel', 'reverify', 'ตรวจสอบซ้ำ — HF Hotel',
  'ทีมไอทีแก้จุดที่แจ้งมาแล้ว รบกวนช่วยเปิดดูซ้ำแล้วเลือกคำตอบครับ/ค่ะ (แค่เปิดดู ไม่กระทบข้อมูลจริง)',
  '{"questions":[
     {"id":"rv_invoice","type":"radio","required":true,
      "label":"เปิดบิล INV2606-019832 ในระบบใหม่ — แสดงแยกเป็น 2 ห้อง ห้องละ 1,780 รวม 3,560 (เท่ากับบิล iHOTEL 2 ใบรวมกัน) หรือไม่?",
      "options":[{"value":"match","label":"ตรง"},{"value":"mismatch","label":"ไม่ตรง"}]},
     {"id":"rv_invoice_note","type":"text","label":"ระบุยอดที่เห็น","placeholder":"เช่น เห็นเป็น …",
      "showIf":{"field":"rv_invoice","equals":"mismatch"}},
     {"id":"rv_round_summary","type":"radio","required":true,
      "label":"เปิดหน้า ''สรุปรอบบิล / รายการรอบ'' — ยอด (โอน/รวม) ตรงกับ iHOTEL แล้วหรือไม่?",
      "options":[{"value":"match","label":"ตรง"},{"value":"mismatch","label":"ไม่ตรง"}]},
     {"id":"rv_round_summary_note","type":"text","label":"ระบุที่เห็น","placeholder":"เช่น รอบ/ยอดที่ไม่ตรง",
      "showIf":{"field":"rv_round_summary","equals":"mismatch"}}
   ]}'::jsonb,
  10
),
(
  'reverify_hfville', 'hfville', 'reverify', 'ตรวจสอบซ้ำ — HF Ville',
  'ทีมไอทีแก้จุดที่แจ้งมาแล้ว รบกวนช่วยเปิดดูซ้ำแล้วเลือกคำตอบครับ/ค่ะ (แค่เปิดดู ไม่กระทบข้อมูลจริง)',
  '{"questions":[
     {"id":"rv_round816","type":"radio","required":true,
      "label":"รายงานรอบบิล รอบ 816 (กะบ่าย 27/06) ยอดรวม = 14,280 หรือไม่?",
      "options":[{"value":"match","label":"ตรง"},{"value":"mismatch","label":"ไม่ตรง"}]},
     {"id":"rv_round816_note","type":"text","label":"ระบุยอดที่เห็น","placeholder":"เช่น เห็นเป็น …",
      "showIf":{"field":"rv_round816","equals":"mismatch"}},
     {"id":"rv_room114","type":"radio","required":true,
      "label":"สถานะห้อง 114 ขึ้นว่า ''ว่าง'' แล้วหรือไม่?",
      "options":[{"value":"vacant","label":"ว่างแล้ว"},{"value":"occupied","label":"ยังมีคนพัก"}]},
     {"id":"rv_arrivals","type":"radio","required":true,
      "label":"คำว่า ''เข้า'' ในรายชื่อผู้เข้าพัก (ที่เคยแจ้งว่าไม่ตรง) หมายถึงข้อใด?",
      "options":[{"value":"a","label":"ลูกค้าที่เช็คอินเข้าจริงวันนี้"},{"value":"b","label":"ลูกค้าที่จองไว้และยังรอเช็คอินวันนี้ (ยังไม่เข้า)"}]},
     {"id":"rv_arrivals_screen","type":"text","label":"ดูจากหน้าจอไหนของ iHOTEL","placeholder":"ชื่อหน้าจอ / เมนู"}
   ]}'::jsonb,
  20
)
ON CONFLICT (form_key) DO NOTHING;

-- DOWN MIGRATION (commented — destructive)
-- DROP INDEX IF EXISTS ix_ht_feedback_forms_active;
-- DROP TABLE IF EXISTS ht_feedback_forms;
-- DELETE FROM schema_migrations WHERE version = '067';
