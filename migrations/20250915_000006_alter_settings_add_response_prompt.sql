-- Tambahkan kolom response_prompt ke tabel settings
ALTER TABLE settings
ADD COLUMN response_prompt TEXT NULL AFTER system_prompt;

-- Isi default jika kosong
UPDATE settings
SET response_prompt = 'You are an assistant for SMRT Singapore IT Department. Summarize and explain monitoring data clearly to the user.'
WHERE response_prompt IS NULL;
