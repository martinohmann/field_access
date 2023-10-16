# Changelog

## [0.1.0](https://github.com/martinohmann/field_access/compare/field_access_derive-v0.0.1...field_access_derive-v0.1.0) (2023-10-16)


### ⚠ BREAKING CHANGES

* make `fields_as_any{,_mut}` return `Option`

### Features

* add `AnyFieldAccess` for low-level access ([35d3908](https://github.com/martinohmann/field_access/commit/35d3908e75f7f633b99764daa867db1397d220d3))
* add `field_names` and `Fields` iterator ([590e6ff](https://github.com/martinohmann/field_access/commit/590e6ff7805fe198a5949e59fe5bb1d7b737d01e))
* add `FieldRef` and `FieldMut` ([6eab7e3](https://github.com/martinohmann/field_access/commit/6eab7e379c439f02fd9ec40064472782c9371cc6))
* add very basic version of FieldAccess proc macro ([a9cd116](https://github.com/martinohmann/field_access/commit/a9cd116e31832786c952b562abb565707069176d))


### Bug Fixes

* add missing fields to `Cargo.toml` ([aa9b127](https://github.com/martinohmann/field_access/commit/aa9b1271fae580e42ac49d29260289af1f5b7b72))
* make `fields_as_any{,_mut}` return `Option` ([80c3920](https://github.com/martinohmann/field_access/commit/80c3920d7e8b2df7db191e6df16af30528395f42))
* remove broken doc test ([112e17e](https://github.com/martinohmann/field_access/commit/112e17e5b34eb9ac114ec5353bc0a66f12c02e99))
