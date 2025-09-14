-- Cache hasil API endpoint (optional)
CREATE TABLE IF NOT EXISTS api_results (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  chat_log_id BIGINT NULL,
  endpoint VARCHAR(128) NOT NULL,
  request_params JSON NULL,
  response_data JSON NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (chat_log_id) REFERENCES chat_logs(id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
