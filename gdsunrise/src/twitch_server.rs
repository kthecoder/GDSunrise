use twitch_irc::{
    login::StaticLoginCredentials,
    ClientConfig,
    SecureTCPTransport,
    TwitchIRCClient,
};
use tokio::sync::mpsc::{unbounded_channel};


use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=RefCounted, init)]
struct TwitchServer {
    _base: Base<RefCounted>,
    client: Option<TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>>,
}


#[godot_api]
impl TwitchServer {

    /*
        FUNCTION : new

        Creates a new instance of the TwitchServer class.

        Parameters:
            None
        Returns:
            TwitchServer: A new instance of the TwitchServer class.
     */
    fn new() -> Self {

        Self {
            _base: Base::default(),
            client: None,
        }
    }

    /*
        FUNCTION : init_twitch_client

        Initializes the Twitch client with the provided channel name.
        This function sets up the client to listen for messages in the specified Twitch channel.
        It spawns a new task to handle incoming messages and emits signals for chat messages.

        Assumes that the Twitch Messages are in the format of `!command` to trigger actions in Godot.

        Parameters:
            twitch_channel (String): The name of the Twitch channel to connect to.
        Returns:
            Result<(), String>: Ok if successful, or an error message if initialization fails.
     */

    #[signal]
    fn twitch_chat_message_ingest(username: GString, command: GString);

    async fn init_twitch_client(&mut self, twitch_channel: String) -> Result<(), String> {
        let config = ClientConfig::default();

        let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);
        let (tx, mut rx) = unbounded_channel::<(String, String)>();


        let join_handle = tokio::spawn(async move {
            while let Some(message) = incoming_messages.recv().await {
                if let twitch_irc::message::ServerMessage::Privmsg(msg) = &message {
                    if msg.message_text.starts_with('!') {
                        let username = msg.sender.name.clone();
                        let command = msg.message_text.clone();
                        let _ = tx.send((username,command));
                    }
                }
            }
        });

        while let Some((username, command)) = rx.recv().await {
        self.signals()
            .twitch_chat_message_ingest()
            .emit(&GString::from(username), &GString::from(command));
        }

        client.join(twitch_channel.to_owned()).map_err(|e| format!("Failed to join Twitch channel: {}", e))?;

        join_handle.await.map_err(|e| format!("Async task failed: {}", e))?;

        Ok(())

    }

    #[func]
    fn start_twitch_client(&mut self, twitch_channel: GString) {
        let twitch_channel = twitch_channel.to_string();
        let mut cloned_self = self.clone(); // Requires Clone or clone relevant fields

        // Spawn the async method
        self._base
            .get_tree()
            .expect("No SceneTree")
            .create_task(async move {
                let _ = cloned_self.init_twitch_client(twitch_channel).await;
            });
    }
    
}