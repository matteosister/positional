use chrono::{DateTime, Utc};
use criterion::{criterion_group, criterion_main, Criterion};
use fake::{Dummy, Fake, Faker};
use positional::*;
use std::str::FromStr;

#[derive(ToPositionalRow, FromPositionalRow, Dummy, Debug)]
struct Data {
    #[field(size = 30)]
    name: String,
    #[field(size = 40, filler = '_', align = "right")]
    created: DateTime<Utc>,
    #[field(size = 20, filler = '0')]
    test1: String,
    #[field(size = 20, filler = '0')]
    test2: String,
    #[field(size = 20, filler = '0')]
    test3: String,
    #[field(size = 20, filler = '0')]
    test4: String,
    #[field(size = 20, filler = '0')]
    test5: String,
    #[field(size = 20, filler = '0')]
    test6: String,
    #[field(size = 20, filler = '0')]
    test7: String,
    #[field(size = 20, filler = '0')]
    test8: String,
    #[field(size = 20, filler = '0')]
    test9: CustomType,
}

#[derive(Dummy, Debug)]
struct CustomType(String);

impl ToPositionalField for CustomType {
    fn to_positional_field(&self) -> String {
        self.0.to_string()
    }
}

impl FromStr for CustomType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}

fn serialize_struct(c: &mut Criterion) {
    let iter = std::iter::repeat_with(|| Faker.fake::<Data>()).take(100_000);
    let positional_file = Writer::new(iter);
    c.bench_function("serialize 100.000 structs", |b| {
        b.iter(|| positional_file.to_string())
    });
}

fn deserialize_struct(c: &mut Criterion) {
    let rows = vec![Faker.fake::<Data>()];
    let positional_file = Writer::new(rows);
    let line = positional_file.to_string();
    c.bench_function("deserialize 100.000 structs", |b| {
        b.iter(|| {
            for _ in 1..=100_000 {
                let _reader: Reader<Data> = Reader::from_str(&line).unwrap();
            }
        })
    });
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().sample_size(60);
    targets = serialize_struct, deserialize_struct
}
criterion_main!(benches);
