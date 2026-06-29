-- =============================================================================
-- Canonical content for the data-driven feedback / re-verification forms.
-- =============================================================================
--
-- This file is the VERSION-CONTROLLED SOURCE OF TRUTH for feedback-form CONTENT
-- (questions, options, wording). The form SCHEMA (the `ht_feedback_forms` table)
-- lives in migrations/pg/067_create_ht_feedback_forms.sql — content lives HERE.
--
-- ## Editing form content (NO app rebuild, NO CI/CD)
--
--   1. Edit the form_schema JSONB below (add/edit/reorder questions), OR edit the
--      live DB directly with an UPDATE.
--   2. Apply to each canonical DB (idempotent — ON CONFLICT DO UPDATE re-syncs):
--        docker exec -i new-hotel-db psql -U postgres -d hotelnew  < scripts/seed-feedback-forms.sql
--        docker exec -i new-hotel-db psql -U postgres -d hotelville < scripts/seed-feedback-forms.sql
--   3. The renderer (/v2/verification/reverify) + results hub pick it up on next load.
--
-- If you edited the live DB directly instead, regenerate this file from it so the
-- repo stays the source of truth:
--   psql -d hotelnew -tA -c "SELECT format('INSERT INTO ht_feedback_forms (form_key,
--     form_site, form_kind, form_title, form_intro, form_schema, form_sort) VALUES
--     (%L,%L,%L,%L,%L,%L::jsonb,%s) ON CONFLICT (form_key) DO UPDATE SET ...;',
--     form_key, form_site, form_kind, form_title, form_intro, form_schema::text,
--     form_sort) FROM ht_feedback_forms ORDER BY form_sort;"
--
-- Idempotent: safe to re-run. Per-site (apply to both hotelnew + hotelville).
-- =============================================================================

INSERT INTO ht_feedback_forms (form_key, form_site, form_kind, form_title, form_intro, form_schema, form_sort)
VALUES ('reverify_hfhotel', 'hfhotel', 'reverify', 'ตรวจสอบซ้ำ — HF Hotel', 'ทีมไอทีแก้จุดที่แจ้งมาแล้ว รบกวนช่วยเปิดดูซ้ำแล้วเลือกคำตอบครับ/ค่ะ (แค่เปิดดู ไม่กระทบข้อมูลจริง)', '{"questions": [{"id": "rv_invoice", "type": "radio", "label": "บิลห้องพักหลายห้องในใบเดียว — เปิดบิล INV2606-019832 ในระบบใหม่ แล้วดูว่า: แสดงแยกบรรทัดต่อห้อง (2 ห้อง ห้องละ 1,780) และยอดรวมทั้งบิล = 3,560 ซึ่งต้องเท่ากับยอดรวมบิล iHOTEL ของลูกค้ารายนี้ทั้ง 2 ใบ ➜ ตรงกันหรือไม่?", "options": [{"label": "ตรง", "value": "match"}, {"label": "ไม่ตรง", "value": "mismatch"}], "required": true}, {"id": "rv_invoice_note", "type": "text", "label": "ถ้าไม่ตรง ระบุยอดที่เห็นในระบบใหม่", "showIf": {"field": "rv_invoice", "equals": "mismatch"}, "placeholder": "เช่น เห็นเป็น …"}, {"id": "rv_round_summary", "type": "radio", "label": "เปิดเมนู รายงาน ➜ หน้า สรุปรอบบิล ในระบบใหม่ → ดูช่อง รวมเงินรับ (สีแดง ไม่รวมเงินทอนตั้งต้น) ของรอบล่าสุด แล้วเทียบกับยอด รวมเงินรับ ในรายงานรายรับของ iHOTEL รอบเดียวกัน ➜ ตรงกันหรือไม่? (อย่าดูช่อง รวมทั้งหมด เพราะรวมเงินทอนตั้งต้นไว้ด้วย)", "options": [{"label": "ตรงแล้ว", "value": "match"}, {"label": "ยังไม่ตรง", "value": "mismatch"}], "required": true}, {"id": "rv_round_summary_note", "type": "text", "label": "ถ้ายังไม่ตรง ระบุรอบ + ยอดที่เห็น", "showIf": {"field": "rv_round_summary", "equals": "mismatch"}, "placeholder": "เช่น รอบ … ยอดในระบบใหม่ … / iHOTEL …"}]}'::jsonb, 10)
ON CONFLICT (form_key) DO UPDATE SET
  form_site = EXCLUDED.form_site, form_kind = EXCLUDED.form_kind,
  form_title = EXCLUDED.form_title, form_intro = EXCLUDED.form_intro,
  form_schema = EXCLUDED.form_schema, form_sort = EXCLUDED.form_sort,
  updated_at = now();

INSERT INTO ht_feedback_forms (form_key, form_site, form_kind, form_title, form_intro, form_schema, form_sort)
VALUES ('reverify_hfville', 'hfville', 'reverify', 'ตรวจสอบซ้ำ — HF Ville', 'ทีมไอทีแก้จุดที่แจ้งมาแล้ว รบกวนช่วยเปิดดูซ้ำแล้วเลือกคำตอบครับ/ค่ะ (แค่เปิดดู ไม่กระทบข้อมูลจริง)', '{"questions": [{"id": "rv_round816", "type": "radio", "label": "รายงานรอบบิล รอบ 816 (กะบ่าย 27/06) ยอดรวม = 14,280 หรือไม่?", "options": [{"label": "ตรง", "value": "match"}, {"label": "ไม่ตรง", "value": "mismatch"}], "required": true}, {"id": "rv_round816_note", "type": "text", "label": "ระบุยอดที่เห็น", "showIf": {"field": "rv_round816", "equals": "mismatch"}, "placeholder": "เช่น เห็นเป็น …"}, {"id": "rv_room114", "type": "radio", "label": "สถานะห้อง 114 ขึ้นว่า ''ว่าง'' แล้วหรือไม่?", "options": [{"label": "ว่างแล้ว", "value": "vacant"}, {"label": "ยังมีคนพัก", "value": "occupied"}], "required": true}, {"id": "rv_arrivals", "type": "radio", "label": "คำว่า ''เข้า'' ในรายชื่อผู้เข้าพัก (ที่เคยแจ้งว่าไม่ตรง) หมายถึงข้อใด?", "options": [{"label": "ลูกค้าที่เช็คอินเข้าจริงวันนี้", "value": "a"}, {"label": "ลูกค้าที่จองไว้และยังรอเช็คอินวันนี้ (ยังไม่เข้า)", "value": "b"}], "required": true}, {"id": "rv_arrivals_screen", "type": "text", "label": "ดูจากหน้าจอไหนของ iHOTEL", "placeholder": "ชื่อหน้าจอ / เมนู"}]}'::jsonb, 20)
ON CONFLICT (form_key) DO UPDATE SET
  form_site = EXCLUDED.form_site, form_kind = EXCLUDED.form_kind,
  form_title = EXCLUDED.form_title, form_intro = EXCLUDED.form_intro,
  form_schema = EXCLUDED.form_schema, form_sort = EXCLUDED.form_sort,
  updated_at = now();
