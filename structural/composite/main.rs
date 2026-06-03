// ----- Component -----
trait Messageable {
    fn message(&self, message: String);
}

// ----- Leaf -----
#[derive(PartialEq, Eq, Hash, Clone)]
struct GuildMember {
    id: u32,
    name: String,
}
impl Messageable for GuildMember {
    fn message(&self, message: String) {
        eprintln!(
            "INFO : sent message to GuildMember \"{}\" (id={})",
            self.name, self.id
        );
        eprintln!("DEBUG: message={message}");
    }
}

// ----- Composite -----
struct Guild {
    id: u32,
    name: String,
    members: std::collections::HashSet<GuildMember>,
}
impl Messageable for Guild {
    fn message(&self, message: String) {
        eprintln!(
            "INFO : sending message to all members of Guild \"{}\" (id={})",
            self.name, self.id
        );
        for member in &self.members {
            member.message(message.clone());
        }
        eprintln!(
            "INFO : sent message to all members of Guild \"{}\" (id={})",
            self.name, self.id
        );
    }
}

fn main() {
    let alice = GuildMember {
        id: 1,
        name: String::from("alice"),
    };
    let bob = GuildMember {
        id: 2,
        name: String::from("bob"),
    };
    let charlie = GuildMember {
        id: 3,
        name: String::from("charlie"),
    };
    let guild = Guild {
        id: 1,
        name: String::from("rustaceans"),
        members: std::collections::HashSet::from([alice, bob, charlie]),
    };

    guild.message(String::from("hello, team!"));
}
