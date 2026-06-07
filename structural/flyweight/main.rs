mod without_flyweight {
    #[derive(Default)]
    pub struct Particle {
        coords: (i32, i32, i32),
        vector: (i32, i32, i32),
        speed: i32,
        color: (i32, i32, i32),
        sprit: String,
    }
}

mod with_flyweight {
    #[derive(Default)]
    pub struct MovingParticle {
        coords: (i32, i32, i32),
        vector: (i32, i32, i32),
        speed: i32,
    }

    #[derive(Default)]
    pub struct Particle {
        color: (i32, i32, i32),
        sprit: String,
    }
}

fn main() {
    // without_flyweight
    println!(
        "without_flyweight::Particle : {}",
        std::mem::size_of::<without_flyweight::Particle>() * 100_000
    );

    // with_flyweight
    println!(
        "with_flyweight::Particle : {}",
        std::mem::size_of::<with_flyweight::Particle>() * 100_000
    );
}
