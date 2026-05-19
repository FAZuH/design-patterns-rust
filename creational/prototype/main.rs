#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Model {
    field1: &'static str,
    field2: u32,
}

fn main() {
    let model1 = Model {
        field1: "foo",
        field2: 2,
    };

    let model2 = model1.clone();

    assert_eq!(model1, model2);
}
