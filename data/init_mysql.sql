-- Seed data awal untuk SMRT MCP

USE smrt_mcp;

-- Insert default settings
INSERT INTO settings (system_prompt, model, temperature, top_p)
VALUES (
  'You are an MCP intent router for SMRT IT Department. Detect intent and route to proper API endpoints.',
  'gpt-4o-mini',
  0.20,
  0.90
);

INSERT INTO settings (system_prompt, response_prompt, model, temperature, top_p)
VALUES (
  'You are an MCP intent router for SMRT IT Department. Detect intent and return JSON with "intent" and "endpoints".',
  'You are an assistant for SMRT Singapore IT Department. Summarize and explain monitoring data clearly to the user.',
  'gpt-4o-mini',
  0.20,
  0.90
);


-- Insert admin user
INSERT INTO users (username, email, role)
VALUES
  ('admin', 'admin@smrt.local', 'admin');

-- Contoh chat log dummy
INSERT INTO chat_logs (user_id, user_query, detected_intent, routed_endpoints, response_summary)
VALUES
  ('admin', 'Kenapa pipeline CI/CD gagal semalam?',
   'ci_cd_investigation',
   JSON_ARRAY('/api/gitlab-ci', '/api/runtime-logs'),
   'Pipeline gagal karena error pada job deploy di staging cluster.');

-- Contoh debug trace dummy
INSERT INTO debug_traces (trace_id, phase, payload)
VALUES
  ('trace-12345', 'received', JSON_OBJECT('query','Kenapa pipeline CI/CD gagal semalam?')),
  ('trace-12345', 'llm_done', JSON_OBJECT('intent','ci_cd_investigation','endpoints',JSON_ARRAY('/api/gitlab-ci','/api/runtime-logs'))),
  ('trace-12345', 'done', JSON_OBJECT('summary','Pipeline gagal karena error pada job deploy di staging cluster.'));
