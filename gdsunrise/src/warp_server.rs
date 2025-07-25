use godot::prelude::*;
use warp::{Filter, Rejection, Reply};
use warp::http::StatusCode;
use warp::ws::{Message, WebSocket};
use futures::{StreamExt, SinkExt};
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use std::net::SocketAddr;
use futures::stream::SplitSink;
use std::convert::Infallible;

#[derive(GodotClass)]
#[class(base=RefCounted, init)]
struct WarpServer {
    runtime: Option<Runtime>,
    static_files_path: String,
    server_port: u16, // Store port number
    senders: Arc<Mutex<Vec<SplitSink<WebSocket, Message>>>>,
}

#[godot_api]
impl WarpServer {
    /*
        FUNCTION : new

        Creates a new instance of the WarpServer class.
        Initializes the runtime and sets default values for static files path and server port.

        Parameters:
            None
        Returns:
            WarpServer: A new instance of the WarpServer class.
     */
    fn new() -> Self {
        Self {
            runtime: None,
            static_files_path: "dist".to_string(), // Default folder
            server_port: 6699, // Default port
            senders: Arc::new(Mutex::new(Vec::new())),
        }
    }

    //======================================================================//
    //
    //  Web Server
    //
    //======================================================================//

    /*
        FUNCTION : set_static_folder

        Sets the folder for static files to be served by the Warp server.
        This folder should contain the built Vue.js application files.

        Parameters:
            folder (String): The path to the static files folder.
        Returns:
            None
     */
    #[func]
    fn set_static_folder(&mut self, folder: String) {
        self.static_files_path = folder;
    }

    /*
        FUNCTION : set_port

        Sets the port number for the Warp server to listen on.

        Parameters:
            port (u16): The port number to set for the server.
        Returns:
            None
     */
    #[func]
    fn set_port(&mut self, port: u16) {
        self.server_port = port;
    }

    /*
        FUNCTION : start_server

        Starts the Warp server to serve static files and handle static HTML (i.e. Vue.js) routing.
        This function will serve files from the specified static folder and
        ensure that all unmatched routes return index.html for Vue Router.

        Parameters:
            None
        Returns:
            None
     */
    #[func]
    fn start_server(&self) {
        let folder = self.static_files_path.clone();
        let port = self.server_port;

        self.runtime
        .as_ref()
        .expect("Tokio runtime not initialized")
        .spawn(async move {
            let folder = folder.clone(); // move-friendly
            let static_files = warp::fs::dir(&folder);


            let index_fallback = warp::any().map(move || {
                    warp::fs::file(format!("{}/index.html", folder))
                });



            let routes = warp::get()
                    .and(static_files.or(index_fallback))
                    .recover(|_err: Rejection| async move {
                        Ok::<_, Infallible>(warp::reply::with_status(
                            "<h1>404: Not Found</h1>",
                            StatusCode::NOT_FOUND,
                        ))
                    });



            warp::serve(routes)
                    .run(([127, 0, 0, 1], port))
                    .await;
        });
    }

    //======================================================================//
    //
    //  WebSocket 
    //
    //======================================================================//
    

    /*
        FUNCTION : start_websocket

        Starts a WebSocket server that listens for updates and broadcasts them to connected clients.
        This function uses the Warp framework to handle WebSocket connections.

        Parameters:
            None
        Returns:
            None
     */
    pub async fn start_websocket(self, port: u16) {
        let senders = self.senders.clone(); // Clone just what’s needed

        let routes = warp::path("ws")
            .and(warp::ws())
            .map(move |ws: warp::ws::Ws| {
                let senders = senders.clone();
                ws.on_upgrade(move |socket| WarpServer::handle_connection(socket, senders))
            });

        let addr = SocketAddr::from(([127, 0, 0, 1], port + 1));
        println!("Warp WebSocket listening on ws://{}", addr);
        warp::serve(routes).run(addr).await;
    }

    fn spawn_websocket(&self) {
        let port = self.server_port + 1;
        let cloned_self: &WarpServer = self.clone(); // Derive Clone or clone individual fields

        self.runtime
            .as_ref()
            .expect("Runtime not initialized")
            .spawn(async move {
                cloned_self.start_websocket(port).await;
            });
    }


    pub fn websocket_route(&self) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let senders = self.senders.clone();

        warp::path("ws")
            .and(warp::ws())
            .map(move |ws: warp::ws::Ws| {
                let senders = senders.clone();
                ws.on_upgrade(move |socket| Self::handle_connection(socket, senders))
            })
    }

    async fn handle_connection(ws: WebSocket, senders: Arc<Mutex<Vec<SplitSink<WebSocket, Message>>>>) {
        let (tx, mut rx) = ws.split();
        senders.lock().unwrap().push(tx);

        while let Some(Ok(_msg)) = rx.next().await {
            // We're ignoring browser messages; it's a broadcast-only server
        }
    }

    #[func]
    fn init_websocket(&mut self) {
        self.spawn_websocket();
    }

    #[func]
    fn get_websocket_port(&self) -> u16 {
        self.server_port + 1
    }

    #[func]
    fn send_json(&self, json: String) {
        let senders = self.senders.clone();

        // Spawn an async task that runs independently
        tokio::spawn(async move {
            let msg = Message::text(json);
            let mut locked = senders.lock().await;

            let mut i = 0;
            while i < locked.len() {
                match locked[i].send(msg.clone()).await {
                    Ok(_) => i += 1,
                    Err(_) => { locked.remove(i); } // Drop failed sender
                }
            }
        });
    }

}