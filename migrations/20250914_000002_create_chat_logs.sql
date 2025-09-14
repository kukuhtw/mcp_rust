-- Log percakapan user + intent detection
CREATE TABLE IF NOT EXISTS chat_logs (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  user_id VARCHAR(64) NULL,
  user_query TEXT NOT NULL,
  detected_intent VARCHAR(128) NULL,
  routed_endpoints JSON NULL,
  response_summary TEXT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
