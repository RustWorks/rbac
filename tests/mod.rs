#![feature(test)]
extern crate test;
extern crate rbac;
#[use_macro] extern crate json;
mod tests {
    #[cfg(test)]
    use test::Bencher;
    use super::*;
    use std::collections::HashSet;
    use super::mods::*;
    use super::json;

    #[bench]
    fn bench_rua(b: &mut Bencher) {
        let dsn = env::var("DSN").ok()
            .expect("You should set mysql connection settings mysql://user:pass@ip:port in DSN env var");
        let pool = Pool::new_manual(1, 1, &dsn).unwrap();
        let d = load(&pool);
        let params = object! {
           "region" => "54",
           "project" => "1",
        };
        b.iter(|| {
            d.check_access(
                "14338667".to_string(),
                "ncc.records.update.access".to_string(),
                &params
            )
        });
    }

    #[bench]
    fn bench_rua2(b: &mut Bencher) {
        let dsn = env::var("DSN").ok()
            .expect("You should set mysql connection settings mysql://user:pass@ip:port in DSN env var");
        let pool = Pool::new_manual(1, 1, &dsn).unwrap();
        let d = load(&pool);
        let params = object! {
           "region" => "55",
           "project" => "1",
        };
        b.iter(|| {
            d.check_access(
                "14338667".to_string(),
                "ncc.records.update.access".to_string(),
                &params
            )
        });
    }

    #[bench]
    fn bench_regions(b: &mut Bencher) {
        let dsn = env::var("DSN").ok()
            .expect("You should set mysql connection settings mysql://user:pass@ip:port in DSN env var");
        let pool = Pool::new_manual(1, 1, &dsn).unwrap();
        let d = load(&pool);
        let regions = [
            "54", "24", "55", "22", "42", "70", "38", "123", "43403", "1077", "181490", "52", "45", "59", "76",
            "72", "74", "29", "27", "33", "73", "31", "23", "93", "66", "63", "2", "34", "61", "47", "44", "21",
            "69", "58", "56", "57", "71", "67", "46", "48", "62", "32", "36", "39", "30", "114160", "14", "16",
            "142982", "182028", "18", "26", "35", "43", "51", "53", "60", "64", "75", "86", "89", "68", "124",
            "155", "142", "138", "170", "166", "154"
        ];

        b.iter(|| {
            for region in regions.iter() {
                let params = object! {"region" => *region};
                d.check_access("11414968".to_string(), "ncc.region.access".to_string(), &params);
            }
        })
    }

    #[bench]
    fn bech_rule(b: &mut Bencher) {
        let item = object! {
            "paramsKey" => "pid",
            "data" => array!["23", "312", "545", "66", "14338727"]
            };
        let data = Data::new();
        let params = object! { "pid" => "14338727"};
        b.iter(|| {
            data.rule(&item, &params);
        });
    }

    #[test]
    fn it_works() {
        let data = mock_data();
        let params = object! { "r" => "1" };
        assert_eq!(true, data.check_access("1".to_string(), "action1".to_string(), &params));
        assert_eq!(false, data.check_access("1".to_string(), "action2".to_string(), &params));
        assert_eq!(false, data.check_access("2".to_string(), "action1".to_string(), &params));
        assert_eq!(true, data.check_access("2".to_string(), "action2".to_string(), &params));
        let params2 = object! {};
        assert_eq!(false, data.check_access("2".to_string(), "action2".to_string(), &params2));
    }

    #[test]
    fn parse_php() {
        let test = r#"a:2:{s:9:"paramsKey";s:3:"pid";s:4:"data";a:1:{i:0;s:8:"14338727";}}"#;
        let mut d = Deserializer::from_str(test);
        let res = d.parse();
        let r = object! {
            "paramsKey" => "pid",
            "data" => array!["14338727"]
        };
        assert_eq!(res, r);
    }

    #[test]
    fn rule() {
        let item = object! {
            "paramsKey" => "pid",
            "data" => array!["14338727"]
            };
        let data = Data::new();
        let params = object! { "pid" => "14338727"};
        assert!(data.rule(&item, &params));
    }

    pub fn mock_data() -> Data {
        let mut data = Data::new();
        data.map = [
            ("admin".to_string(), 0 as ItemId),
            ("user".to_string(), 1 as ItemId),
            ("action1".to_string(), 2 as ItemId),
            ("action2".to_string(), 3 as ItemId),
            ("task1".to_string(), 4 as ItemId)
        ].iter().cloned().collect();


        data.assignments = [
            (
                1,
                [0].iter().cloned().collect()
            ),
            (
                2,
                [1].iter().cloned().collect()
            )
        ].iter().cloned().collect();

        data.assignments_dict = [
            (
                0,
                Assignment {
                    name: 0,
                    user_id: 1,
                    data: object! {},
                }
            ),
            (
                1,
                Assignment {
                    name: 1,
                    user_id: 2,
                    data: object! {},
                }
            )
        ].iter().cloned().collect();
        data.items = [
            (0, Item {
                name: 0,
                data: json::JsonValue::new_object()
            }),
            (1, Item {
                name: 1,
                data: json::JsonValue::new_object()
            }), (2, Item {
                name: 2,
                data: json::JsonValue::new_object()
            }),
            (3, Item {
                name: 3,
                data: json::JsonValue::new_object()
            }),
            (4, Item {
                name: 4,
                data: object! {
                    "paramsKey" => "r",
                    "data" => array!["1", "2"]
                }
            })
        ].iter().cloned().collect();
        let action1_parents: HashSet<ItemId> = [0 as ItemId].iter().cloned().collect();
        let action2_parents: HashSet<ItemId> = [4 as ItemId].iter().cloned().collect();
        let task1_parents: HashSet<ItemId> = [1 as ItemId].iter().cloned().collect();
        data.parents = [
            (1, [
                (2 as ItemId, action1_parents)
            ].iter().cloned().collect()),
            (2, [
                (3 as ItemId, action2_parents),
                (4 as ItemId, task1_parents)
            ].iter().cloned().collect())
        ].iter().cloned().collect();

        return data;
    }
}