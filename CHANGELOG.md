# Changelog

All notable changes to this project will be documented in this file.

## [2.3.0] - 2022-11-16

[4a3cb4c](4a3cb4c04a86d10985bb36f886d4b4b89b282920)...[f41d3f0](f41d3f0c1b9c79be471323710e84f08aa10a37d5)

### Features

- Use thiserror for error reporting ([f41d3f0](f41d3f0c1b9c79be471323710e84f08aa10a37d5) - 2022-11-16 by Tyr Chen)

## [2.2.0] - 2022-11-16

[1b83ec2](1b83ec2bc02c695b8a8464120e6bc456981c0a6a)...[4a3cb4c](4a3cb4c04a86d10985bb36f886d4b4b89b282920)

### Features

- Move the code to 2021 edition, support integer/float/boolean parsing, and add more tests ([4a3cb4c](4a3cb4c04a86d10985bb36f886d4b4b89b282920) - 2022-11-16 by Tyr Chen)

## [2.0.2] - 2020-06-06

[6dc53d0](6dc53d08f35679122fc6bcf1bfc416a849c79215)...[537461b](537461b030e291c96b49e6f43d5f218b9f2de89e)

### Bug Fixes

- Decode params before parsing ([25aba7a](25aba7af5793d89bc4fff1852aeddd5c4435787c) - 2017-02-14 by David Quintanel)

### Documentation

- Module import fixup ([6e65672](6e65672c6a212f9f30e4db5a9c3431a9f449b843) - 2020-05-11 by James Addison)

### Features

- Make regex crate optional ([8b62d50](8b62d5053e177d9f579267fee2b580c9298bb280) - 2020-01-29 by red75prime)
- Implement standalone key parsing ([64e380d](64e380d6e901ed8b83164fb0c91ba3e6cb1e1462) - 2020-05-11 by James Addison)
- Use Value::default to determine default value ([942e6f4](942e6f437a6e0054bf78e4d7fb0bbab7111683dd) - 2020-05-11 by James Addison)

### Miscellaneous Tasks

- Update serde & serde_json ([ede812f](ede812fdf43f74e1e2e186b64653a9706c20e88e) - 2016-08-14 by Thomas Heck)
- V1.0.1 ([af684e1](af684e14fba09e668532bf373dda44b640a317f2) - 2016-08-14 by Stanislav Panferov)
- V2.0.0 ([0246995](0246995025156d982b4a484acbec33add349e2ed) - 2018-03-07 by Stanislav Panferov)
- Specify MSRV (see rust-lang/rfcs#2495) ([25dfa3d](25dfa3d34038c602bdf665058169e45de9891326) - 2020-05-11 by James Addison)

### Refactor

- Update serde and serde_json to 0.9 ([2c34883](2c34883ee7c15657f2a395e7ce6aa71cc04e7733) - 2017-03-03 by Robert Lord)
- Update deps ([5618891](5618891f4fe7851b03e18f0aeac8c2fdcd5ae5e0) - 2018-03-07 by Stanislav Panferov)
- Cleanup 'try' linter warning (see rust-lang/rust#61000) ([ab0ed17](ab0ed17940a41bbedbba0390d01af7083eeee5fa) - 2020-05-11 by James Addison)
- Extract a 'parse_pair' method ([d49db89](d49db897442ee35dbc330b1b7a6f0d62783a4e1b) - 2020-05-11 by James Addison)

### Testing

- Add test case for intended behaviour ([61a6223](61a6223056a7d1419b0c1be1e6ee6eee73fc17d8) - 2020-05-11 by James Addison)
- Add test coverage over key-value parsing ([c794fac](c794faca0d14b9b70544871e1f8e7daeae47f55e) - 2020-05-11 by James Addison)

## [1.0.0] - 2016-05-01

### Bug Fixes

- Implicit arrays with more than two elements ([8364235](83642358923e7e585c390801e42ef7b3409664bc) - 2014-10-05 by Stanislav Panferov)

### Documentation

- Add some readme ([10f5f10](10f5f102f57dd7139a9877bac540e84e64ec224d) - 2014-09-26 by Stanislav Panferov)
- Fix readme ([33df84d](33df84d8e4f2cd4e5fa6fb0cce9ce78d294d0d83) - 2014-09-26 by Станислав Панферов)
- Fix readme ([67a7547](67a7547875544b33611a29ee60185bf6a82f9b6d) - 2014-09-26 by Станислав Панферов)
- Fix errors in the readme ([7f80103](7f80103c0aaa02354ab19c1f326100aca0e35af0) - 2014-09-26 by Станислав Панферов)
- Update README.md ([1f12d38](1f12d383f3ee704afef1e8c71d43761e81310a5b) - 2014-09-26 by Станислав Панферов)
- Update README.md ([80dbf33](80dbf33c6996e5b14958b9a94b8f00c91b3dcd2d) - 2014-10-05 by Stanislav Panferov)
- Update README.md ([eed2bb3](eed2bb30e1a5712af279de770e7cb2ddcf2ffe69) - 2014-10-08 by Stanislav Panferov)
- Rename to queryst ([e30cd82](e30cd82349f233bd68ac7faab95a4b69fd362c2d) - 2014-10-12 by Stanislav Panferov)
- Fix name in readme ([09d2384](09d2384ac85de471198509eeb7877beb75c4736e) - 2014-10-12 by Stanislav Panferov)
- Update README ([271cc6b](271cc6b909c9af61fe4523e132ad40bc0770843f) - 2014-10-13 by Stanislav Panferov)
- Update ([7010e61](7010e6108cfe1e4879397646c2b6ee997fdcd156) - 2016-05-01 by Stanislav Panferov)

### Features

- Add merges stuff; first working version ([74a5f42](74a5f4276ef1c7577e506936abeeffd122fb8e03) - 2014-09-26 by Stanislav Panferov)
- Working version ([9fc6cd1](9fc6cd1a4f7ef9cb8ad228acbf6c126b48564f9b) - 2014-09-26 by Stanislav Panferov)
- Update deps ([4de5346](4de53463ed4de5ed29cfcbe962d626a118de81a3) - 2016-05-01 by Stanislav Panferov)
- Move to serde ([3561877](356187734dfd211be828a940977813ce649eff6f) - 2016-05-01 by Stanislav Panferov)

### Miscellaneous Tasks

- License ([5bf1106](5bf1106f74f7df398c8d73e62da3ee9a9fbb39dd) - 2014-09-26 by Stanislav Panferov)
- Fix Cargo.toml ([681e941](681e9413ea9095280230e14602846ba95ac58559) - 2014-09-26 by Stanislav Panferov)
- Add travis ([447e76f](447e76f7a0e5436077e736ccec0ad763fe67e6c3) - 2014-10-08 by Stanislav Panferov)
- Fix travis ([abfadb5](abfadb5d4dd0f9fc6f95369ff40387d0d739d802) - 2014-10-08 by Stanislav Panferov)
- Fix travis ([e9b3c5f](e9b3c5fd3ae5195c0af4df1f951a62af5fcd3fe9) - 2014-10-08 by Stanislav Panferov)
- Add makefile to publish docs ([ca8dd46](ca8dd46967bd038ce188779080cbd3539978582b) - 2014-10-13 by Stanislav Panferov)
- Modify CNAME ([639465e](639465e5b1dba5fd908723bf929864979190e3f0) - 2014-10-13 by Stanislav Panferov)
- Move to crates.io ([cd54ab0](cd54ab0ef3d8e268eeea6e1026320e3a31ac998b) - 2014-11-23 by Stanislav Panferov)
- Bump version and update meta info. ([d0b878b](d0b878bc314cdc1377c5e31eee1c43efbb123760) - 2014-12-04 by Stanislav Panferov)
- Add github repo link to Cargo.toml ([e6e7af3](e6e7af3196c7f97d1ed632da5b8d5da9db02782d) - 2014-12-05 by Mordy Tikotzky)
- Update deps, (still get warnings about regex crate); 0.1.4 ([2297235](2297235db0147d5ba42548c42a0cb6acc6ebacd7) - 2014-12-27 by Stanislav Panferov)
- Add `regex_macros` to cargo deps; 0.1.6 ([40cffd3](40cffd3b42952ce6107fec28a5afa1a6cb1d4dc5) - 2015-01-07 by Stanislav Panferov)
- 0.1.7 ([13f38fd](13f38fd18f7227126fcfc5c0745840119f9d42ce) - 2015-01-19 by Stanislav Panferov)
- 0.1.10 ([f122de8](f122de8fc7dab2711e481a99de9263beff6e9455) - 2015-02-04 by Stanislav Panferov)
- 0.1.11 ([4531ff9](4531ff9f0332c99cb245cd1d7216639962659f7a) - 2015-02-12 by Stanislav Panferov)
- Add guide ([257c49e](257c49e46fd6d5f7bb8deb9f4107e9b03d55217f) - 2015-02-14 by Stanislav Panferov)
- 0.1.12 ([845b705](845b7054328e70d3c7a34534441122412fe4b045) - 2015-02-21 by Stanislav Panferov)
- 0.1.13 ([619fd84](619fd84b7775018eb53d4f1e454681c572aff7ab) - 2015-03-07 by Stanislav Panferov)
- 0.1.14 ([161426d](161426d3290fbd38b87d2c1bfed0e4c11862fc59) - 2015-04-15 by Stanislav Panferov)
- V0.1.15 ([7f5026c](7f5026cb96fa41057cfed54f605b38d43cb2a420) - 2015-05-10 by Stanislav Panferov)
- Bump deps ([e009485](e009485d0411ca3fe0e258c9a279aef3f8747e4a) - 2015-10-31 by Stanislav Panferov)
- Fix versions ([32e3616](32e3616a4617dc4b4a8cc9f59f000c2b3871394a) - 2015-11-25 by Stanislav Panferov)

### Refactor

- Less warnings ([64f45eb](64f45eb39801e3089ee1c5246c9718bd84967fb2) - 2014-09-26 by Stanislav Panferov)
- Fix naming to prevent warnings in modern Rust ([d1cef6f](d1cef6fdd494c8bb52026a4a87e78003dad79742) - 2014-10-04 by Stanislav Panferov)
- Rustup ([315ed18](315ed18e67bd4e216e13f3c4463cfd96c4b190f9) - 2014-11-01 by Stanislav Panferov)
- Rustup ([7bb6759](7bb675912aa6f9ffe533d8aa013f0125781218d7) - 2014-11-05 by Stanislav Panferov)
- Rustup ([8de61df](8de61dfc631f8f7f966ff73b5ede91c6315aa7c0) - 2014-11-10 by Stanislav Panferov)
- Rustup ([662805e](662805e517bd2504b6a84befee796a28a03a7b9b) - 2014-11-23 by Stanislav Panferov)
- Rustup ([3ddc5c7](3ddc5c7c219ffed307063f333b22f98d05093d8e) - 2014-12-04 by Stanislav Panferov)
- Rustup, 0.1.2 ([357c7d7](357c7d75fa5a347441e24121e0ef188a8bcadb90) - 2014-12-22 by Stanislav Panferov)
- Rustup, fix from_str ([aa0425b](aa0425b63e08dfade17d5a7fc826450d976fcc16) - 2014-12-27 by Stanislav Panferov)
- Rustup; rename `deriving` to `derive`; 0.1.5 ([995d8d6](995d8d61283112217d4b09116bca6117705da1d7) - 2015-01-04 by Stanislav Panferov)
- Rustup; 0.1.8 ([bc6571e](bc6571e7bcca114f881e758bd940cf34275438af) - 2015-01-24 by Stanislav Panferov)
- Rustup; 0.1.9 ([7ad1694](7ad169408959168ccaa2f2fa56f4600e13d35d4d) - 2015-01-30 by Stanislav Panferov)
- Update to new FromStr trait ([c9f11fa](c9f11fa1ec0925406a859a06d69a61719c46053d) - 2015-02-04 by Stanislav Panferov)
- Rustup; fix warning ([f79c220](f79c220eb0671f9dd95aa225cf5c924a076d5fa8) - 2015-02-21 by Stanislav Panferov)
- Rustup ([ffef941](ffef941cf67942b1971f005598d25a8529903c69) - 2015-03-07 by Stanislav Panferov)
- Rustup ([81a8358](81a83585d9c79563ecbb29d16cea8913d84ee51b) - 2015-04-15 by Stanislav Panferov)
- Use only beta features ([c0ff3a9](c0ff3a93f539eb1687480198b474b79a51874a20) - 2015-05-03 by Frank Laub)
- Remove StrExt from the library, use [x..y] instead ([066060e](066060e7ab846b80b133d1c5284b9c974f0ee84f) - 2015-05-10 by Stanislav Panferov)

### Styling

- Convert tabs to spaces ([89abba3](89abba398b051b31507d3cdd4d96a77cdc724a57) - 2014-09-30 by Stanislav Panferov)

<!-- generated by git-cliff -->
