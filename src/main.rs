use server::Server;
use http::Request;
use http::Method;

mod server;
mod http;
fn main() {
    // let get = Method::GET;
    // let delete = Method::DELETE;
    // let post = Method::POST;
    // let put = Method::PUT;

    let server = Server::new ("127.0.0.1:8080".to_string());
    server.run();


}





























// use std::collections::HashMap;

// struct Player {
//     id: u32,
//     // Add more fields as needed
// }

// struct Lobby {
//     players: Vec<Player>,
//     // Add more fields as needed
// }

// struct Server {
//     addr: String,
//     players: HashMap<u32, Player>,
//     lobbies: Vec<Lobby>,
//     servers: HashMap<String, String>, // Map from server handles to addresses
// }

// impl Server {
//     fn new(addr: String) -> Self {
//         Self {
//             addr,
//             players: HashMap::new(),
//             lobbies: Vec::new(),
//             servers: HashMap::new(),
//         }
//     }

//     fn run(self) {
//         println!("Listening on {}", self.addr);
//         // Add code to accept connections, handle requests, etc.
//     }

//     fn add_player(&mut self, player: Player) {
//         // Add the player to the players list
//         self.players.insert(player.id, player);
//         // Add the player to a lobby
//         // If all lobbies are full, create a new one
//     }

//     fn spawn_server(&mut self, lobby: Lobby) {
//         // Spawn a new server for the given lobby
//         // Remove the players from the lobby and add them to the new server
//         // Generate a server handle and add it to the servers map
//     }

//     fn resolve_handle(&self, handle: &str) -> Option<&String> {
//         // Resolve a server handle into a server address
//         self.servers.get(handle)
//     }
// }

// fn main() {
//     let server = Server::new("127.0.0.1:8080".to_string());
//     server.run();
// }