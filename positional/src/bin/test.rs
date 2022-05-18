use positional::*;

#[derive(PositionalRow)]
struct Data {
    #[positional(size = 5, filler = ' ', align = "left")]
    id: String,
    #[positional(size = 20, filler = ' ', align = "left")]
    value: String,
    #[positional(size = 5, filler = '0', align = "right")]
    count: i32,
}

pub fn main() {
    let data = Data {
        id: "1".to_string(),
        value: "test value".to_string(),
        count: 1_000_000,
    };
    let data2 = Data {
        id: "1".to_string(),
        value: "test value".to_string(),
        count: 1_000_000,
    };

    let positional_file: PositionalFile<Data> = PositionalFile::new(vec![data2]);
    assert_eq!("1    test value          10000", data.to_positional_row());
    assert_eq!(
        "1    test value          10000",
        positional_file.to_positional_file()
    );
}
