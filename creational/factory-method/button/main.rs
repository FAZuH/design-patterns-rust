trait Dialog {
    fn render(&self);
    fn create_button() -> Box<dyn Button>
    where
        Self: Sized;
}

trait Button {
    fn render(&self);
    fn on_click(&self);
}

struct WindowsDialog;
struct HtmlDialog;

impl Dialog for WindowsDialog {
    fn render(&self) {
        let button = Self::create_button();
        button.render();
        button.on_click();
    }

    fn create_button() -> Box<dyn Button> {
        Box::new(WindowsButton)
    }
}

impl Dialog for HtmlDialog {
    fn render(&self) {
        let button = Self::create_button();
        button.render();
        button.on_click();
    }

    fn create_button() -> Box<dyn Button> {
        Box::new(HtmlButton)
    }
}

struct WindowsButton;
struct HtmlButton;

impl Button for WindowsButton {
    fn render(&self) {
        println!("This is a windows button");
    }

    fn on_click(&self) {
        println!("Clicked windows button");
    }
}

impl Button for HtmlButton {
    fn render(&self) {
        println!("This is a html button");
    }

    fn on_click(&self) {
        println!("This is a html button");
    }
}

fn main() {
    let platform = "windows";

    let dialog: Box<dyn Dialog> = match platform {
        "windows" => Box::new(WindowsDialog),
        "html" => Box::new(HtmlDialog),
        _ => panic!("invalid platform"),
    };

    dialog.render();
}
