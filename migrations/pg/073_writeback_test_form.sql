-- Migration: 073_writeback_test_form
-- Version: (release-please)
-- Date: 2026-07-01
--
-- DATA migration on ht_feedback_forms (no schema change).
--
-- 1. Archive the two per-site re-verify forms (reverify_hfhotel / reverify_hfville,
--    seeded by migration 067). Those verification rounds are done; hide them from
--    the hub by clearing form_active instead of deleting the rows (audit trail +
--    their historical responses in ht_verification_responses keep their kind label).
-- 2. Add a new "legacy write-back test" form (form_key='writeback_test'). It rides
--    the existing results storage/kind by reusing form_kind='reverify' (submissions
--    land in ht_verification_responses.vr_answers tagged kind='reverify', so the
--    existing results hub picks it up with no code change). Applies to BOTH sites
--    (form_site='all'). Guides reception through the coordinated, one-switch-at-a-time
--    live verification of enabling legacy write-back (image + companion) into iHOTEL.
--
-- PG-CANONICAL ONLY (ht_feedback_forms has no legacy counterpart — no sync, no
-- writeback). Idempotent: the archive UPDATE is naturally repeatable and the INSERT
-- is ON CONFLICT (form_key) DO UPDATE. Per-site (connection-level scoping).

-- UP MIGRATION

-- 1. Archive the completed re-verify forms.
UPDATE ht_feedback_forms
   SET form_active = FALSE, updated_at = NOW()
 WHERE form_key IN ('reverify_hfhotel', 'reverify_hfville');

-- 2. Add the legacy write-back test form (active).
INSERT INTO ht_feedback_forms (form_key, form_site, form_kind, form_title, form_intro, form_schema, form_active, form_sort)
VALUES ('writeback_test', 'all', 'reverify', 'ทดสอบเขียนข้อมูลกลับ iHOTEL · Legacy write-back test', 'การทดสอบนี้เปิดใช้งาน "การเขียนข้อมูลจากแอปใหม่กลับเข้า iHOTEL" ทีละสวิตช์ โดยประสานกับทีมไอที ทีละขั้น — ไอทีเป็นผู้เปิดสวิตช์ให้บนเซิร์ฟเวอร์ แล้วรบกวนพนักงานต้อนรับทำรายการทดสอบ 1 รายการ และตรวจใน iHOTEL ว่าข้อมูลไปถึงถูกต้อง และ iHOTEL ยังทำงานปกติ (ไม่ค้าง/ไม่ error) ⚠️ ทำเฉพาะช่วงที่ไม่มีลูกค้าหน้าเคาน์เตอร์ และหยุดทันทีถ้า iHOTEL ผิดปกติ (แจ้งไอทีให้ปิดสวิตช์กลับ)', '{"questions": [{"id": "wb_site", "type": "radio", "label": "1) กำลังทดสอบสาขาใด?", "required": true, "options": [{"label": "HF Hotel", "value": "hfhotel"}, {"label": "HF Ville", "value": "hfville"}]}, {"id": "wb_img_result", "type": "radio", "label": "2) รูปบัตร — หลังไอทีแจ้งว่าเปิดสวิตช์ ''บันทึกรูปบัตรกลับ iHOTEL'' แล้ว: ทำเช็คอินทดสอบในแอปใหม่ 1 ราย แล้วสแกนบัตรประชาชน/พาสปอร์ต → เปิด iHOTEL ดูใบลงทะเบียน/รูปบัตรของรายนั้น → รูปบัตรปรากฏใน iHOTEL หรือไม่?", "required": true, "options": [{"label": "ปรากฏถูกต้อง", "value": "match"}, {"label": "ไม่ปรากฏ", "value": "missing"}, {"label": "iHOTEL แจ้ง error / บันทึกไม่ได้", "value": "error"}]}, {"id": "wb_img_note", "type": "text", "label": "→ ระบุสิ่งที่เห็น / ข้อความ error (ถ้าไม่ปรากฏหรือ error)", "placeholder": "เช่น เห็นเป็น… / ข้อความ…", "showIf": {"field": "wb_img_result", "equals": "missing"}}, {"id": "wb_img_error_note", "type": "text", "label": "→ ระบุข้อความ error", "placeholder": "ข้อความที่ iHOTEL แจ้ง…", "showIf": {"field": "wb_img_result", "equals": "error"}}, {"id": "wb_img_ihotel_ok", "type": "radio", "label": "3) หลังทดสอบข้อ 2 — iHOTEL ยังทำงานปกติหรือไม่ (บันทึก/เปิดบิลได้ ไม่ค้าง ไม่มี error)?", "required": true, "options": [{"label": "ปกติ", "value": "ok"}, {"label": "มีปัญหา (แจ้งไอทีปิดสวิตช์)", "value": "problem"}]}, {"id": "wb_comp_result", "type": "radio", "label": "4) ผู้ติดตาม — หลังไอทีแจ้งว่าเปิดสวิตช์ ''บันทึกผู้ติดตามกลับ iHOTEL'' แล้ว: เพิ่มผู้ติดตาม 1 คนในแอปใหม่ให้ผู้เข้าพักที่มีในระบบ → เปิด iHOTEL ดูรายชื่อผู้เข้าพักร่วมของรายนั้น → ปรากฏหรือไม่?", "required": true, "options": [{"label": "ปรากฏถูกต้อง", "value": "match"}, {"label": "ไม่ปรากฏ", "value": "missing"}, {"label": "iHOTEL แจ้ง error", "value": "error"}]}, {"id": "wb_comp_note", "type": "text", "label": "→ ระบุสิ่งที่เห็น / ข้อความ (ถ้าไม่ปรากฏหรือ error)", "placeholder": "เช่น เห็นเป็น… / ข้อความ…", "showIf": {"field": "wb_comp_result", "equals": "missing"}}, {"id": "wb_comp_error_note", "type": "text", "label": "→ ระบุข้อความ error", "placeholder": "ข้อความที่ iHOTEL แจ้ง…", "showIf": {"field": "wb_comp_result", "equals": "error"}}, {"id": "wb_comp_ihotel_ok", "type": "radio", "label": "5) หลังทดสอบข้อ 4 — iHOTEL ยังทำงานปกติหรือไม่?", "required": true, "options": [{"label": "ปกติ", "value": "ok"}, {"label": "มีปัญหา (แจ้งไอทีปิดสวิตช์)", "value": "problem"}]}, {"id": "wb_overall", "type": "radio", "label": "6) สรุปผลการทดสอบ", "required": true, "options": [{"label": "ผ่านทั้งสองส่วน — พร้อมเปิดใช้ถาวร", "value": "pass"}, {"label": "มีปัญหา — ให้ไอทีปิดสวิตช์กลับก่อน", "value": "fail"}]}, {"id": "wb_notes", "type": "text", "label": "7) หมายเหตุเพิ่มเติม (ถ้ามี)", "placeholder": "รายละเอียดอื่น ๆ / เวลาที่ทดสอบ / ชื่อผู้ประสานไอที"}]}'::jsonb, TRUE, 5)
ON CONFLICT (form_key) DO UPDATE SET
  form_site = EXCLUDED.form_site, form_kind = EXCLUDED.form_kind,
  form_title = EXCLUDED.form_title, form_intro = EXCLUDED.form_intro,
  form_schema = EXCLUDED.form_schema, form_active = EXCLUDED.form_active,
  form_sort = EXCLUDED.form_sort, updated_at = NOW();

-- DOWN MIGRATION (commented)
-- UPDATE ht_feedback_forms SET form_active = TRUE, updated_at = NOW()
--  WHERE form_key IN ('reverify_hfhotel', 'reverify_hfville');
-- DELETE FROM ht_feedback_forms WHERE form_key = 'writeback_test';
-- DELETE FROM schema_migrations WHERE version = '073';
