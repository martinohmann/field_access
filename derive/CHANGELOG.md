# Changelog

## [0.1.10](https://github.com/martinohmann/field_access/compare/field_access_derive-v0.1.9...field_access_derive-v0.1.10) - 2024-10-04

### Other

- *(deps)* bump syn from 2.0.77 to 2.0.79 ([#51](https://github.com/martinohmann/field_access/pull/51))

## [0.1.9](https://github.com/martinohmann/field_access/compare/field_access_derive-v0.1.8...field_access_derive-v0.1.9) - 2024-09-02

### Other
- *(deps)* bump quote from 1.0.36 to 1.0.37 ([#47](https://github.com/martinohmann/field_access/pull/47))
- *(deps)* bump proc-macro2 from 1.0.85 to 1.0.86 ([#44](https://github.com/martinohmann/field_access/pull/44))
- *(deps)* bump syn from 2.0.66 to 2.0.77 ([#48](https://github.com/martinohmann/field_access/pull/48))

## [0.1.8](https://github.com/martinohmann/field_access/compare/field_access_derive-v0.1.7...field_access_derive-v0.1.8) - 2024-06-02

### Other
- *(deps)* bump syn from 2.0.58 to 2.0.66 ([#41](https://github.com/martinohmann/field_access/pull/41))
- *(deps)* bump proc-macro2 from 1.0.81 to 1.0.85 ([#39](https://github.com/martinohmann/field_access/pull/39))

## [0.1.7](https://github.com/martinohmann/field_access/compare/field_access_derive-v0.1.6...field_access_derive-v0.1.7) - 2024-05-02

### Other
- *(deps)* bump quote from 1.0.35 to 1.0.36 ([#32](https://github.com/martinohmann/field_access/pull/32))
- *(deps)* bump syn from 2.0.57 to 2.0.58 ([#33](https://github.com/martinohmann/field_access/pull/33))
- *(deps)* bump proc-macro2 from 1.0.79 to 1.0.81 ([#31](https://github.com/martinohmann/field_access/pull/31))

## [0.1.6](https://github.com/martinohmann/field_access/compare/field_access_derive-v0.1.5...field_access_derive-v0.1.6) - 2024-04-02

### Other
- *(deps)* bump syn from 2.0.52 to 2.0.57 ([#26](https://github.com/martinohmann/field_access/pull/26))
- *(deps)* bump proc-macro2 from 1.0.78 to 1.0.79 ([#25](https://github.com/martinohmann/field_access/pull/25))

## [0.1.5](https://github.com/martinohmann/field_access/compare/field_access_derive-v0.1.4...field_access_derive-v0.1.5) - 2024-03-01

### Other
- *(deps)* bump syn from 2.0.48 to 2.0.52 ([#22](https://github.com/martinohmann/field_access/pull/22))

## [0.1.4](https://github.com/martinohmann/field_access/compare/field_access_derive-v0.1.3...field_access_derive-v0.1.4) - 2024-02-03

### Other
- *(deps)* bump syn from 2.0.46 to 2.0.48 ([#18](https://github.com/martinohmann/field_access/pull/18))
- *(deps)* bump proc-macro2 from 1.0.74 to 1.0.78 ([#19](https://github.com/martinohmann/field_access/pull/19))

## [0.1.3](https://github.com/martinohmann/field_access/compare/field_access_derive-v0.1.2...field_access_derive-v0.1.3) - 2024-01-03

### Other
- *(deps)* bump proc-macro2 from 1.0.69 to 1.0.70 ([#6](https://github.com/martinohmann/field_access/pull/6))
- *(deps)* bump syn from 2.0.38 to 2.0.39 ([#7](https://github.com/martinohmann/field_access/pull/7))

## [0.1.2](https://github.com/martinohmann/field_access/compare/field_access_derive-v0.1.1...field_access_derive-v0.1.2) (2023-10-19)


### Bug Fixes

* add categories and keywords to `Cargo.toml` ([40a9278](https://github.com/martinohmann/field_access/commit/40a9278c280d8adf1a91f9d6d2736fc4ca2a23b2))

## [0.1.1](https://github.com/martinohmann/field_access/compare/field_access_derive-v0.1.0...field_access_derive-v0.1.1) (2023-10-18)


### Bug Fixes

* add missing repository link to `Cargo.toml` ([8862bf8](https://github.com/martinohmann/field_access/commit/8862bf8a890179a10605ee2e8aac36294322895e))

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
