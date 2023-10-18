// SPDX-License-Identifier: Apache-2.0 OR MIT

#![warn(rust_2018_idioms, single_use_lifetimes)]

use serde_derive::{Deserialize, Serialize};

use crate::{v1, v2, Dependabot};

#[test]
#[cfg_attr(miri, ignore)] // Miri is too slow
fn test_v2() {
    let v2_test_data = include_str!("fixtures/v2.yml");
    assert_eq!(from_str::<serde_yaml::Value>(v2_test_data).len(), 22);
    for case in from_str::<v2::Dependabot>(v2_test_data) {
        case.to_string();
    }
    for case in from_str::<serde_yaml::Value>(v2_test_data) {
        let s = serde_yaml::to_string(&case).unwrap();
        s.parse::<v2::Dependabot>().unwrap().to_string();
        serde_yaml::to_string(&serde_yaml::from_str::<v2::Dependabot>(&s).unwrap()).unwrap();
    }
    for case in from_str::<Dependabot>(v2_test_data) {
        case.to_string();
    }
    for case in from_str::<serde_yaml::Value>(v2_test_data) {
        let s = serde_yaml::to_string(&case).unwrap();
        s.parse::<Dependabot>().unwrap().to_string();
        serde_yaml::to_string(&serde_yaml::from_str::<Dependabot>(&s).unwrap()).unwrap();
    }

    let v2_registries_test_data = include_str!("fixtures/v2_registries.yml");
    assert_eq!(from_str::<serde_yaml::Value>(v2_registries_test_data).len(), 13);
    for case in from_str::<TestRegistriesV2>(v2_registries_test_data) {
        serde_yaml::to_string(&case).unwrap();
    }
    for case in from_str::<serde_yaml::Value>(v2_registries_test_data) {
        let s = serde_yaml::to_string(&case).unwrap();
        serde_yaml::to_string(&serde_yaml::from_str::<TestRegistriesV2>(&s).unwrap()).unwrap();
    }
}

#[test]
fn test_v1() {
    let v1_test_data = include_str!("fixtures/v1.yml");
    assert_eq!(from_str::<serde_yaml::Value>(v1_test_data).len(), 21);
    for case in from_str::<v1::Dependabot>(v1_test_data) {
        case.to_string();
    }
    for case in from_str::<serde_yaml::Value>(v1_test_data) {
        let s = serde_yaml::to_string(&case).unwrap();
        s.parse::<v1::Dependabot>().unwrap().to_string();
        serde_yaml::to_string(&serde_yaml::from_str::<v1::Dependabot>(&s).unwrap()).unwrap();
    }
    for case in from_str::<Dependabot>(v1_test_data) {
        case.to_string();
    }
    for case in from_str::<serde_yaml::Value>(v1_test_data) {
        let s = serde_yaml::to_string(&case).unwrap();
        s.parse::<Dependabot>().unwrap().to_string();
        serde_yaml::to_string(&serde_yaml::from_str::<Dependabot>(&s).unwrap()).unwrap();
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct TestRegistriesV2 {
    registries: v2::Registries,
}

#[track_caller]
#[allow(single_use_lifetimes)]
fn from_str<D>(s: &str) -> Vec<D>
where
    D: for<'de> Deserialize<'de>,
{
    let mut buf = vec![];
    for document in serde_yaml::Deserializer::from_str(s) {
        buf.push(D::deserialize(document).unwrap());
    }
    buf
}
