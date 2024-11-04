# Changelog

## [0.1.11](https://github.com/martinohmann/field_access/compare/field_access-v0.1.10...field_access-v0.1.11) - 2024-11-04

### Other

- *(deps)* bump trybuild from 1.0.99 to 1.0.101 ([#54](https://github.com/martinohmann/field_access/pull/54))

## [0.1.10](https://github.com/martinohmann/field_access/compare/field_access-v0.1.9...field_access-v0.1.10) - 2024-10-04

### Other

- updated the following local packages: field_access_derive

## [0.1.9](https://github.com/martinohmann/field_access/compare/field_access-v0.1.8...field_access-v0.1.9) - 2024-09-02

### Other
- *(deps)* bump quote from 1.0.36 to 1.0.37 ([#47](https://github.com/martinohmann/field_access/pull/47))
- *(deps)* bump proc-macro2 from 1.0.85 to 1.0.86 ([#44](https://github.com/martinohmann/field_access/pull/44))
- *(deps)* bump syn from 2.0.66 to 2.0.77 ([#48](https://github.com/martinohmann/field_access/pull/48))
- *(deps)* bump trybuild from 1.0.96 to 1.0.99 ([#46](https://github.com/martinohmann/field_access/pull/46))

## [0.1.8](https://github.com/martinohmann/field_access/compare/field_access-v0.1.7...field_access-v0.1.8) - 2024-06-02

### Fixed
- add missing `usize` and `isize` conversions ([#42](https://github.com/martinohmann/field_access/pull/42))

### Other
- *(deps)* bump paste from 1.0.14 to 1.0.15 ([#37](https://github.com/martinohmann/field_access/pull/37))
- *(deps)* bump syn from 2.0.58 to 2.0.66 ([#41](https://github.com/martinohmann/field_access/pull/41))
- *(deps)* bump proc-macro2 from 1.0.81 to 1.0.85 ([#39](https://github.com/martinohmann/field_access/pull/39))
- *(deps)* bump trybuild from 1.0.91 to 1.0.96 ([#38](https://github.com/martinohmann/field_access/pull/38))

## [0.1.7](https://github.com/martinohmann/field_access/compare/field_access-v0.1.6...field_access-v0.1.7) - 2024-05-02

### Other
- updated the following local packages: field_access_derive

## [0.1.6](https://github.com/martinohmann/field_access/compare/field_access-v0.1.5...field_access-v0.1.6) - 2024-04-02

### Other
- *(deps)* bump syn from 2.0.52 to 2.0.57 ([#26](https://github.com/martinohmann/field_access/pull/26))
- *(deps)* bump proc-macro2 from 1.0.78 to 1.0.79 ([#25](https://github.com/martinohmann/field_access/pull/25))
- *(deps)* bump trybuild from 1.0.89 to 1.0.91 ([#27](https://github.com/martinohmann/field_access/pull/27))
- *(deps)* bump actions/cache from 4.0.1 to 4.0.2 ([#28](https://github.com/martinohmann/field_access/pull/28))
- `Cargo.toml` link to correct repository ([#29](https://github.com/martinohmann/field_access/pull/29))

## [0.1.5](https://github.com/martinohmann/field_access/compare/field_access-v0.1.4...field_access-v0.1.5) - 2024-03-01

### Other
- updated the following local packages: field_access_derive

## [0.1.4](https://github.com/martinohmann/field_access/compare/field_access-v0.1.3...field_access-v0.1.4) - 2024-02-03

### Other
- *(deps)* bump syn from 2.0.46 to 2.0.48 ([#18](https://github.com/martinohmann/field_access/pull/18))
- *(deps)* bump proc-macro2 from 1.0.74 to 1.0.78 ([#19](https://github.com/martinohmann/field_access/pull/19))
- *(deps)* bump actions/cache from 3.3.2 to 4.0.0 ([#20](https://github.com/martinohmann/field_access/pull/20))
- *(deps)* bump trybuild from 1.0.87 to 1.0.89 ([#17](https://github.com/martinohmann/field_access/pull/17))

## [0.1.3](https://github.com/martinohmann/field_access/compare/field_access-v0.1.2...field_access-v0.1.3) - 2024-01-03

### Other
- *(release)* switch to release-plz
- *(deps)* bump trybuild from 1.0.85 to 1.0.87 ([#15](https://github.com/martinohmann/field_access/pull/15))
- remove release-please command
- update trybuild expectation to new compiler output
- *(deps)* bump google-github-actions/release-please-action ([#8](https://github.com/martinohmann/field_access/pull/8))
- *(deps)* bump proc-macro2 from 1.0.69 to 1.0.70 ([#6](https://github.com/martinohmann/field_access/pull/6))
- *(deps)* bump syn from 2.0.38 to 2.0.39 ([#7](https://github.com/martinohmann/field_access/pull/7))
- disable running tests on nightly for now
- label example value in `as_type_mut_method!` macro
- add doc test examples for `AnyFieldAccess` methods
- add ui tests for type with non-static generics
- ensure `derive` feature is enabled in tests

## [0.1.2](https://github.com/martinohmann/field_access/compare/field_access-v0.1.1...field_access-v0.1.2) (2023-10-19)


### Features

* add `as_*_mut` methods to `FieldMut` ([5e85611](https://github.com/martinohmann/field_access/commit/5e856118f1cd8f54e9c7e79e8c5555ec014b9cbd))
* add methods for `usize` and `isize` ([21a74d0](https://github.com/martinohmann/field_access/commit/21a74d0efbd1578d4749f7ceb34ea15b9742d88a))


### Bug Fixes

* add categories and keywords to `Cargo.toml` ([40a9278](https://github.com/martinohmann/field_access/commit/40a9278c280d8adf1a91f9d6d2736fc4ca2a23b2))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * field_access_derive bumped from 0.1.1 to 0.1.2

## [0.1.1](https://github.com/martinohmann/field_access/compare/field_access-v0.1.0...field_access-v0.1.1) (2023-10-18)


### Features

* add `Field::{as,is}_bool` ([25779c4](https://github.com/martinohmann/field_access/commit/25779c48db18705eb11048df8595eb21e6ce80fd))
* add `Field::is_{slice,string,vec}` ([1a3ec90](https://github.com/martinohmann/field_access/commit/1a3ec9038b12ec87fe7293c1b3a870817d9a2e8a))
* add examples for `Field::is_*` ([25744dd](https://github.com/martinohmann/field_access/commit/25744dd0f8740d95886f5c95b423005013c9e5e0))
* derive `Clone` for `Field` ([d3bb23a](https://github.com/martinohmann/field_access/commit/d3bb23a21eb0b4233cf36c5bfc275796189aceed))
* derive `Debug` for `Field{,Mut}` ([a9a6b33](https://github.com/martinohmann/field_access/commit/a9a6b33b7cfc2a15dbbf97fdf5e267a27fee6414))
* implement `Debug` for `Fields` ([acd46bb](https://github.com/martinohmann/field_access/commit/acd46bbdab62e97454e8436db26d53287e946e84))


### Bug Fixes

* add missing repository link to `Cargo.toml` ([8862bf8](https://github.com/martinohmann/field_access/commit/8862bf8a890179a10605ee2e8aac36294322895e))
* remove misplaced feature gate ([4c448e3](https://github.com/martinohmann/field_access/commit/4c448e3c82b8663d149c6292dc592f560ee0a1b7))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * field_access_derive bumped from 0.1.0 to 0.1.1

## [0.1.0](https://github.com/martinohmann/field_access/compare/field_access-v0.0.1...field_access-v0.1.0) (2023-10-16)


### ⚠ BREAKING CHANGES

* switch to `Option`-based API
* make `fields_as_any{,_mut}` return `Option`
* remove `FieldAccess::has_field`
* remove immutable methods from `FieldMut`
* remove `as_dyn_*` trait methods
* remove Send and Sync impls for now
* remove specialized getters for now
* remove `try_` methods

### Features

* add `AnyFieldAccess` for low-level access ([35d3908](https://github.com/martinohmann/field_access/commit/35d3908e75f7f633b99764daa867db1397d220d3))
* add `as_any` and `as_any_mut` ([479e7ab](https://github.com/martinohmann/field_access/commit/479e7abbd619c932f746447f38d0f22d86e3e025))
* add `derive` feature ([71c6035](https://github.com/martinohmann/field_access/commit/71c60351e062006ec68d69454a56ce722ca0a3ac))
* add `exists` method to `Field{Ref,Mut}` ([54de625](https://github.com/martinohmann/field_access/commit/54de625a061728f2e4fda3eea9126b16c63bf638))
* add `field_names` and `Fields` iterator ([590e6ff](https://github.com/martinohmann/field_access/commit/590e6ff7805fe198a5949e59fe5bb1d7b737d01e))
* add `Field{Ref,Mut}::name` ([42e8c74](https://github.com/martinohmann/field_access/commit/42e8c74342b95883dd1e7c94aedd7c0c3cd6a31b))
* add `FieldMut::swap` ([c848c22](https://github.com/martinohmann/field_access/commit/c848c22904a6c67acace9a21a3a9b2ddf64e577c))
* add `FieldRef::type_id` ([93f9b6c](https://github.com/martinohmann/field_access/commit/93f9b6c88e0a6d6c33bdc8c9c2c57c6476228965))
* add `FieldRef` and `FieldMut` ([6eab7e3](https://github.com/martinohmann/field_access/commit/6eab7e379c439f02fd9ec40064472782c9371cc6))
* add naive implementation ([0833297](https://github.com/martinohmann/field_access/commit/08332974d2d69909ac10092cc8b16ede8cda6002))
* add very basic version of FieldAccess proc macro ([a9cd116](https://github.com/martinohmann/field_access/commit/a9cd116e31832786c952b562abb565707069176d))


### Bug Fixes

* add missing fields to `Cargo.toml` ([aa9b127](https://github.com/martinohmann/field_access/commit/aa9b1271fae580e42ac49d29260289af1f5b7b72))
* correct doc issues ([5de8d86](https://github.com/martinohmann/field_access/commit/5de8d86ddad064cfb9bcac9bf1ad069750dc15ab))
* DoubleEndedIterator was moving into wrong direction ([4c17cc2](https://github.com/martinohmann/field_access/commit/4c17cc2b273298c76d4e453d153f7b0994b575f7))
* make `fields_as_any{,_mut}` return `Option` ([80c3920](https://github.com/martinohmann/field_access/commit/80c3920d7e8b2df7db191e6df16af30528395f42))
* remove `as_dyn_*` trait methods ([aafc455](https://github.com/martinohmann/field_access/commit/aafc455c4d8d9e6624593047480454505ce5fcb8))
* remove `FieldAccess::has_field` ([b7458a2](https://github.com/martinohmann/field_access/commit/b7458a2a1bee342307ef1a46efad886c76cc2668))
* remove broken doc test ([112e17e](https://github.com/martinohmann/field_access/commit/112e17e5b34eb9ac114ec5353bc0a66f12c02e99))
* remove immutable methods from `FieldMut` ([7ebf28e](https://github.com/martinohmann/field_access/commit/7ebf28eba3cbf2f8c8e3300280fbe71174dde779))
* remove misplaced function argument ([8173c44](https://github.com/martinohmann/field_access/commit/8173c44dd1a466a562891dc922fa4ffe35d5cc97))
* remove Send and Sync impls for now ([f1eaad9](https://github.com/martinohmann/field_access/commit/f1eaad92c54ddf66f5e709a671dd0e0bf0578e8e))
* reverse argument order for `Field{Ref,Mut}::new` ([162cd96](https://github.com/martinohmann/field_access/commit/162cd96de2c7c35ab3cd9aa0ece5912d4b8da835))
* unexport methods on `dyn FieldAccess` ([103d4b2](https://github.com/martinohmann/field_access/commit/103d4b23e95885cbab41b251e1a71c07f70c99aa))


### Code Refactoring

* remove `try_` methods ([1f72244](https://github.com/martinohmann/field_access/commit/1f7224432d38f2c06fe9472610ea2fe4d92f061d))
* remove specialized getters for now ([c65e418](https://github.com/martinohmann/field_access/commit/c65e4185cf7378000ef56fbb2b99b4a25fcce41f))
* switch to `Option`-based API ([eb8b7d4](https://github.com/martinohmann/field_access/commit/eb8b7d4e8ca50fe25757ba2db19518d2abf44dcb))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * field_access_derive bumped from =0.0.1 to 0.1.0
