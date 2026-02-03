set shell := ["fish", "-c"]
set dotenv-load := true
set export := true

# 在当前目录下，清理所有非当前环境的构建产物
clean:
    cargo sweep --time 0 .

# 清理 libduckdb-sys 构建目录中的大型临时 st* 文件 (fd 仅匹配文件，避免误删目录)
clean-duckdb:
    fd --type f --max-depth 1 "^st" target/*/build/libduckdb-sys-*/out/ --exec rm -f

release:
    cargo build --release --target x86_64-unknown-linux-musl
    ls -lh target/x86_64-unknown-linux-musl/release/sf
    cp --force target/x86_64-unknown-linux-musl/release/sf ~/PycharmProjects/pair3/src/my_dg/assets/rust_cli/bin/sf

check:
    ls -lh target/x86_64-unknown-linux-musl/release/sf
    ls -lh ~/PycharmProjects/pair3/src/my_dg/assets/rust_cli/bin/sf
