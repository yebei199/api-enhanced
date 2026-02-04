## 2026-02-03 Task: Documentation
- **Issue**: Subagent accidentally broke `verify_captcha` by copying `send_captcha` body while adding comments.
- **Fix**: Reverted `verify_captcha` to its correct logic (correct URL and request struct) while keeping the newly added documentation.
