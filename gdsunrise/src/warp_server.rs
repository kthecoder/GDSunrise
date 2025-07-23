use gdext::prelude::*;
use warp::Filter;
use tokio::sync::broadcast;
use warp::ws::{Message, WebSocket};
use futures::{StreamExt, SinkExt};

#[derive(GodotClass)]
#[class(base=RefCounted)]
struct WarpServer {
    runtime: Option<Runtime>,
    static_files_path: String,
    server_port: u16, // Store port number

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
    #[func]
    fn new() -> Self {
        Self {
            runtime: Some(Runtime::new().unwrap()),
            static_files_path: "dist".to_string(), // Default folder
            server_port: 6699, // Default port
        }
    }

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

        let rt = self.runtime.as_ref().unwrap();
        rt.spawn(async move {
            let static_files = warp::fs::dir(&folder);

            // Ensure all unmatched routes return index.html for Vue Router
            let index_route = warp::path::end()
                .or(warp::any())
                .map(move || warp::fs::file(format!("{}/index.html", folder)));

            let routes = warp::get().and(static_files.or(index_route));

            warp::serve(routes)
                .run(([127, 0, 0, 1], port))
                .await;
        });
    }

    /*
        FUNCTION : start_websocket

        Starts a WebSocket server that listens for updates and broadcasts them to connected clients.
        This function uses the Warp framework to handle WebSocket connections.

        Parameters:
            None
        Returns:
            None
     */
    #[func]
    fn start_websocket(&mut self) {
        let port = self.server_port + 1; // WebSocket runs on a separate port
        let (tx, _rx) = broadcast::channel::<String>(10);
        self.ws_tx = Some(tx.clone());

        let rt = self.runtime.as_ref().unwrap();
        rt.spawn(async move {
            let websocket_route = warp::path("updates")
                .and(warp::ws())
                .map(move |ws: warp::ws::Ws| {
                    let tx = tx.clone();
                    ws.on_upgrade(move |socket| handle_connection(socket, tx))
                });

            warp::serve(websocket_route)
                .run(([127, 0, 0, 1], port))
                .await;
        });
    }

    /*
        FUNCTION : notify_clients

        Sends a message to all connected WebSocket clients.
        This function is used to broadcast updates to the clients.

        Parameters:
            message (String): The message to send to all connected clients.
        Returns:
            None
     */
    #[func]
    fn notify_clients(&self, message: String) {
        if let Some(tx) = &self.ws_tx {
            let _ = tx.send(message);
        }
    }

    /*
        FUNCTION : handle_connection

        Handles a new WebSocket connection. It listens for incoming messages
        and broadcasts them to all connected clients.

        Parameters:
            ws (WebSocket): The WebSocket connection.
            tx (broadcast::Sender<String>): The broadcast channel to send messages to clients.
        Returns:
            None
     */
    #[func]
    async fn handle_connection(ws: WebSocket, tx: broadcast::Sender<String>) {
        let (mut sender, mut receiver) = ws.split();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            while let Ok(update) = rx.recv().await {
                let _ = sender.send(Message::text(update)).await;
            }
        });

        while let Some(Ok(msg)) = receiver.next().await {
            let _ = tx.send(msg.to_str().unwrap().to_string());
        }
    }

}