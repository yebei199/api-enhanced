# Refactor Login and Crypto Module

## Tasks

- [ ] Move `crypto.rs` to a new `utils` module at the root level <!-- id: 0 -->
  - [ ] Create `rust/utils/crypto.rs` from `rust/enhanced/crypto.rs` <!-- id: 1 -->
  - [ ] Create `rust/utils.rs` <!-- id: 2 -->
  - [ ] Update `rust/lib.rs` to include `mod utils` <!-- id: 3 -->
  - [ ] Update `rust/enhanced.rs` to remove `mod crypto` <!-- id: 4 -->
  - [ ] Update `rust/enhanced/login_cellphone.rs` to use `crate::utils::crypto` <!-- id: 5 -->
- [ ] Refactor `login_cellphone.rs` to hide implementation details <!-- id: 6 -->
  - [ ] Create `rust/enhanced/login_cellphone/` directory <!-- id: 7 -->
  - [ ] Move implementation logic and structs to `rust/enhanced/login_cellphone/inner.rs` <!-- id: 8 -->
  - [ ] Update `rust/enhanced/login_cellphone.rs` to provide a simple public API <!-- id: 9 -->
- [ ] Verify everything works <!-- id: 10 -->
  - [ ] Run `cargo check` or `lsp_diagnostics` <!-- id: 11 -->
  - [ ] Run tests <!-- id: 12 -->
