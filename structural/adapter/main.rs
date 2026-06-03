#![allow(dead_code)]
// -------- Model --------

struct User {
    id: u64,
    name: String,

    display_name: String,
    avatar_url: String,
    bio: String,

    email: String,
    discord: String,
    google: String,
    personal_site: String,
    steam: String,

    account_age_months: u32,
    last_login: String,
    status: String,
}

struct UserDisplay {
    name: String,
    status: String,
    is_veteran: bool,
}

// -------- Service --------

struct UserDisplayService {}

impl UserDisplayService {
    fn display(&self, profile: UserDisplay) {
        println!("{}", profile.name);
        println!("--------");
        println!("status: {}", profile.status);
        if profile.is_veteran {
            println!("veteran");
        }
    }
}

// -------- Adapter --------

struct Adapter {
    service: UserDisplayService,
}

impl Adapter {
    fn convert_user_to_user_display(user: User) -> UserDisplay {
        let is_veteran = user.account_age_months > 12;
        UserDisplay {
            name: user.name,
            status: user.status,
            is_veteran,
        }
    }
}

impl ClientInterface for Adapter {
    fn display_profile(&self, user: User) {
        let user_display = Self::convert_user_to_user_display(user);
        self.service.display(user_display)
    }
}

// -------- Client --------

trait ClientInterface {
    fn display_profile(&self, user: User);
}

// ---------------------------------------------------------
//  __  __   _   ___ _  _
// |  \/  | /_\ |_ _| \| |
// | |\/| |/ _ \ | || .` |
// |_|  |_/_/ \_\___|_|\_|
// ---------------------------------------------------------

fn main() {
    let user = User {
        id: 1,
        name: String::from("fazuh"),
        display_name: String::from("FAZuH"),
        avatar_url: String::from("https://example.com/avatar.png"),
        bio: String::from("stats student, rust enjoyer"),
        email: String::from("faz@example.com"),
        discord: String::from("fazuh#0000"),
        google: String::from("faz@gmail.com"),
        personal_site: String::from("https://fazuh.com"),
        steam: String::from("fazuh"),
        account_age_months: 24,
        last_login: String::from("2026-06-03"),
        status: String::from("online"),
    };

    let adapter = Adapter {
        service: UserDisplayService {},
    };

    adapter.display_profile(user);
}
