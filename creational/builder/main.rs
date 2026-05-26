#[derive(Debug, PartialEq, Eq)]
pub struct Product {
    name: String,
    color: String,
    dimensions: (u32, u32, u32),
}

#[derive(Default)]
pub struct ProductBuilder {
    name: Option<String>,
    color: Option<String>,
    dimensions: Option<(u32, u32, u32)>,
}

impl Product {
    fn builder() -> ProductBuilder {
        ProductBuilder::default()
    }
}

impl ProductBuilder {
    fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    fn color(mut self, name: impl Into<String>) -> Self {
        self.color = Some(name.into());
        self
    }

    fn dimensions(mut self, name: impl Into<(u32, u32, u32)>) -> Self {
        self.dimensions = Some(name.into());
        self
    }

    fn build(self) -> Product {
        Product {
            name: self.name.unwrap_or("default_name".to_string()),
            color: self.color.unwrap(),
            dimensions: self.dimensions.unwrap(),
        }
    }
}

fn main() {
    let prod1 = Product::builder()
        .name("foobar")
        .color("blue")
        .dimensions((1, 1, 1))
        .build();

    assert_eq!(
        prod1,
        Product {
            name: "foobar".to_string(),
            color: "blue".to_string(),
            dimensions: (1, 1, 1)
        }
    );

    // not setting default_name (use its default value)
    let prod2 = Product::builder()
        .color("blue")
        .dimensions((1, 1, 1))
        .build();

    assert_eq!(
        prod2,
        Product {
            name: "default_name".to_string(),
            color: "blue".to_string(),
            dimensions: (1, 1, 1)
        }
    );
}
