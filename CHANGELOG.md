# Changelog

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
