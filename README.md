# rust-openssl-cannot-link-statically

This repository demonstrates rust-openssl cannot be statically linked with env `OPENSSL_STATIC` set in
[.cargo/config.toml](.cargo/config.toml) under project root.

## Summary

Related issues go as
  - https://github.com/rust-lang/pkg-config-rs/issues/102
  - https://github.com/sfackler/rust-openssl/issues/1792

The phenomenon is: **any library which falls into /usr/lib/ won't be statically linked (which is basically all libraries
found by pkg-config)**

## Prerequisite
- openssl: 0.10.42

## Reproduction

### 1. Start the container as building env
```bash
docker run -it --rm     \
  -v $PWD:/workspace    \
  -w /workspace         \
  rust:1.65.0-bullseye  \
  bash
```

### 2. Build the project within the started container
```bash
cargo build
```

### 3. Check linkage with ldd
```bash
ldd target/debug/rust-openssl-cannot-link-statically
```

Got output log as

```bash
        linux-vdso.so.1 (0x00007ffc00d2c000)
        libssl.so.1.1 => /usr/lib/x86_64-linux-gnu/libssl.so.1.1 (0x00007ff37e652000)
        libcrypto.so.1.1 => /usr/lib/x86_64-linux-gnu/libcrypto.so.1.1 (0x00007ff37e35e000)
        libdl.so.2 => /lib/x86_64-linux-gnu/libdl.so.2 (0x00007ff37e358000)
        libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007ff37e33e000)
        libpthread.so.0 => /lib/x86_64-linux-gnu/libpthread.so.0 (0x00007ff37e31c000)
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007ff37e147000)
        /lib64/ld-linux-x86-64.so.2 (0x00007ff37e745000)
```

where libssl.a and libcrypto.a should be used instead of libssl.so.1.1 and libcrypto.so.1.1.
