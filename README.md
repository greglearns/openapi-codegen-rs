# OpenAPI Codegen

## Dependencies

```
[dependencies]
serde = "1"
serde_derive = "1"
serde_json = "1"
serde_yaml = "0.8"
url = "1"
hyper = "0.12"
base64 = "0.10"
futures = "0.1"
tokio-core = "0.1"
reqwest = "0.9"
failure = "*"

[build-dependencies]
openapi-codegen = "*"

[dev-dependencies]
testcontainers = { git = "https://github.com/testcontainers/testcontainers-rs", branch = "master" }
tc_generic = { git = "https://github.com/testcontainers/testcontainers-rs", branch = "master" }
tc_core = { git = "https://github.com/testcontainers/testcontainers-rs", branch = "master" }
```
