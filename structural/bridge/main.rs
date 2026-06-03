use std::io::Write;

struct Ui {
    i: Option<Box<dyn UiBackend>>,
}

impl Ui {
    fn new(i: impl UiBackend + 'static) -> Self {
        Self {
            i: Some(Box::new(i)),
        }
    }

    fn display_hello_world(&mut self) {
        if let Some(backend) = &mut self.i {
            backend.hello_world();
        }
    }

    fn display_message(&mut self, message: String) {
        if let Some(mut backend) = self.i.take() {
            backend.set_message(message);
            backend.run();
        }
    }
}

trait UiBackend {
    fn hello_world(&mut self);
    fn set_message(&mut self, message: String);
    fn run(self: Box<Self>);
}

struct Tui {
    message: String,
}
struct Gui {
    message: String,
}

impl UiBackend for Tui {
    fn hello_world(&mut self) {
        println!("hello, world!");
    }

    fn set_message(&mut self, message: String) {
        self.message = message;
    }

    fn run(self: Box<Self>) {
        let mut buf = String::new();
        let mut this = self;
        loop {
            print!("set message, or enter exit: ");
            std::io::stdout().flush().unwrap();

            buf.clear();
            std::io::stdin().read_line(&mut buf).unwrap();

            if buf.trim() == "exit" {
                break;
            }

            this.set_message(buf.trim().to_string());
            println!("{}", this.message);
        }
    }
}

impl eframe::App for Gui {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.label(self.message.as_str());
        ui.horizontal(|ui| {
            ui.label("Name");
            ui.text_edit_singleline(&mut self.message);
        });
    }
}

impl UiBackend for Gui {
    fn set_message(&mut self, message: String) {
        self.message = message;
    }

    fn hello_world(&mut self) {
        self.set_message("Hello, world!".to_string());
    }

    fn run(self: Box<Self>) {
        eframe::run_native("Bridge Demo", Default::default(), Box::new(|_cc| Ok(self))).unwrap()
    }
}

// The "Client"
fn main() {
    let Some(ui) = std::env::args().nth(1) else {
        eprintln!("ERROR: missing ui argument. must be 'tui' or 'gui'");
        return;
    };

    let mut abstraction = match ui.as_str() {
        "tui" => Ui::new(Tui {
            message: String::new(),
        }),
        "gui" => Ui::new(Gui {
            message: String::new(),
        }),
        _ => {
            eprintln!("ERROR: invalid ui argument. must be 'tui' or 'gui'");
            return;
        }
    };

    abstraction.display_hello_world();
    abstraction.display_message("This is bridge pattern! :D".to_string());
}
