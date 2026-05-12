trait Transport {
    fn deliver(&self);
}

struct Truck;

struct Ship;

impl Transport for Truck {
    fn deliver(&self) {
        println!("Delivering on road with a truck...");
    }
}

impl Transport for Ship {
    fn deliver(&self) {
        println!("Delivering across the sea with a ship...");
    }
}

trait Logistics {
    fn plan_delivery() -> Self;
    fn create_transport() -> Box<dyn Transport>;
}

struct RoadLogistics {
    transport: Box<dyn Transport>,
}

struct SeaLogistics {
    transport: Box<dyn Transport>,
}

impl Logistics for RoadLogistics {
    fn plan_delivery() -> Self {
        Self {
            transport: Self::create_transport(),
        }
    }

    fn create_transport() -> Box<dyn Transport> {
        Box::new(Truck)
    }
}

impl Logistics for SeaLogistics {
    fn plan_delivery() -> Self {
        Self {
            transport: Self::create_transport(),
        }
    }

    fn create_transport() -> Box<dyn Transport> {
        Box::new(Ship)
    }
}

fn main() {
    RoadLogistics::plan_delivery().transport.deliver();
    SeaLogistics::plan_delivery().transport.deliver();
}
