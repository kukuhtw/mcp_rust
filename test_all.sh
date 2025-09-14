#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${BASE_URL:-http://localhost:8080}"
TZ="${TZ:-Asia/Singapore}"
DATE_FROM="${DATE_FROM:-2025-09-14T00:00:00+08:00}"
DATE_TO="${DATE_TO:-2025-09-14T23:59:59+08:00}"

have_jq=0
if command -v jq >/dev/null 2>&1; then
  have_jq=1
fi

pass=0
fail=0
results=()

log() { printf "\n\033[1;34m[INFO]\033[0m %s\n" "$*"; }
ok()  { printf "\033[1;32m[PASS]\033[0m %s\n" "$*"; ((pass++)); results+=("PASS $*"); }
err() { printf "\033[1;31m[FAIL]\033[0m %s\n" "$*"; ((fail++)); results+=("FAIL $*"); }

get() {
  local url="$1"
  local label="$2"
  if out=$(curl -sS -f "$url"); then
    if [[ $have_jq -eq 1 ]]; then echo "$out" | jq . >/dev/null 2>&1 || true; fi
    ok "$label"
  else
    err "$label"
  fi
}

post() {
  local url="$1"
  local json="$2"
  local label="$3"
  if out=$(curl -sS -f -H "Content-Type: application/json" -d "$json" "$url"); then
    if [[ $have_jq -eq 1 ]]; then echo "$out" | jq . >/dev/null 2>&1 || true; fi
    ok "$label"
  else
    err "$label"
  fi
}

log "Base URL: $BASE_URL"
log "TZ=$TZ | DATE_FROM=$DATE_FROM | DATE_TO=$DATE_TO"
log "jq: $([[ $have_jq -eq 1 ]] && echo 'found' || echo 'not found (output not pretty)')"

# 1) Health
get "$BASE_URL/health" "health"

# 2) 10 dummy endpoints
get "$BASE_URL/api/gitlab-ci?date_from=$DATE_FROM&date_to=$DATE_TO&tz=$TZ" "gitlab-ci"
get "$BASE_URL/api/runtime-logs?tz=$TZ" "runtime-logs"
get "$BASE_URL/api/cloud-mon?date_from=$DATE_FROM&date_to=$DATE_TO&tz=$TZ" "cloud-mon"
get "$BASE_URL/api/db-perf?tz=$TZ" "db-perf"
get "$BASE_URL/api/observability?tz=$TZ" "observability"
get "$BASE_URL/api/mobile-telemetry?tz=$TZ" "mobile-telemetry"
get "$BASE_URL/api/security-auth?tz=$TZ" "security-auth"
get "$BASE_URL/api/incident-metrics?tz=$TZ" "incident-metrics"
get "$BASE_URL/api/user-feedback?tz=$TZ" "user-feedback"
get "$BASE_URL/api/data-integration-bi?tz=$TZ" "data-integration-bi"

# 3) Settings GET/POST
get  "$BASE_URL/api/settings" "settings GET"

post "$BASE_URL/api/settings" \
'{
  "system_prompt":"You are an MCP intent router for SMRT IT Department.",
  "response_prompt":"You are an assistant for SMRT IT Department. Summarize clearly.",
  "model":"gpt-4o-mini",
  "temperature":0.2,
  "top_p":0.9
}' \
"settings POST"

# 4) Joiner test
get "$BASE_URL/api/test-join?date_from=$DATE_FROM&date_to=$DATE_TO&tz=$TZ" "test-join"

# Summary
echo
log "Summary:"
for r in "${results[@]}"; do
  [[ "$r" == PASS* ]] && printf "  \033[1;32m%s\033[0m\n" "$r" || printf "  \033[1;31m%s\033[0m\n" "$r"
done
echo
printf "\033[1;34mDone.\033[0m PASS=%d FAIL=%d\n" "$pass" "$fail"

# exit non-zero jika ada fail
if [[ $fail -gt 0 ]]; then exit 1; fi
