trait IAdminPanel {
    fn send_announcement(&self, message: String);
}

struct AdminPanel;

impl IAdminPanel for AdminPanel {
    fn send_announcement(&self, message: String) {
        println!("INFO: sending announcement \"{message}\" to all users")
    }
}

struct AuthMessagingProxy {
    service: AdminPanel,
    creds: String,
}

impl AuthMessagingProxy {
    fn new(service: AdminPanel) -> Self {
        Self {
            service,
            creds: "".to_string(),
        }
    }
    fn check_access(&self) -> bool {
        self.creds == "admin"
    }
}

impl IAdminPanel for AuthMessagingProxy {
    fn send_announcement(&self, message: String) {
        if self.check_access() {
            self.service.send_announcement(message);
        } else {
            println!("ERROR: you are not authorized to perform this operation")
        }
    }
}

fn main() {
    let panel: Box<dyn IAdminPanel> = Box::new(AuthMessagingProxy::new(AdminPanel {}));

    panel.send_announcement("hello, world".to_string());

    let panel: Box<dyn IAdminPanel> = Box::new({
        let mut panel = AuthMessagingProxy::new(AdminPanel {});
        panel.creds = "admin".to_string();
        panel
    });

    panel.send_announcement("hello, world".to_string());
}
