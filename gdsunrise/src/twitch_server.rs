use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

use tokio::runtime::Runtime;

#[derive(GodotClass)]
#[class(base=RefCounted)]
struct TwitchServer {
    config: twitch_irc::config,
    join_handle: Option<JoinHandle<()>>,
    client: Option<TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>>, // Add this field
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
    #[func]
    fn new() -> Self {
        Self {
            config: ClientConfig::default(),
            join_handle: None,
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
    #[func]
    pub async fn init_twitch_client(&mut self, twitch_channel: String) -> Result<(), String> {
        let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);
        
        join_handle = tokio::spawn(async move {
            while let Some(message) = incoming_messages.recv().await {
                if let twitch_irc::message::ServerMessage::Privmsg(msg) = &message {
                    if msg.message_text.starts_with('!') {
                        let username = msg.sender.name.clone();
                        let command = msg.message_text.clone();
                        godot::GodotObject::owner().emit_signal(
                            "twitch_chat_message_ingest".into(),
                            &[username.to_variant(), command.to_variant()],
                        );
                    }
                }
            }
        });

         client.join(twitch_channel.to_owned()).unwrap();

         join_handle.await.unwrap();
    }
    
}