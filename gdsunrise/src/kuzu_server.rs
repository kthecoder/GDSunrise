use kuzu::{Database, Connection};

#[derive(GodotClass)]
#[class(base=RefCounted)]
struct KuzuServer {
    db: Option<Database>,
    conn: Option<Connection>,
    config: SystemConfig,
    ws_tx: Option<broadcast::Sender<String>>, // WebSocket broadcaster
}

#[godot_api]
impl KuzuServer {

    /*
        FUNCTION : new

        Creates a new instance of the KuzuServer class.
        Initializes the database and sets default configuration values.

        Parameters:
            None
        Returns:
            KuzuServer: A new instance of the KuzuServer class.
     */
    #[func]
    fn new() -> Self {
        Self {
            db: None,
            conn: None,
            config: SystemConfig {
                buffer_pool_size: 512 * 1024 * 1024, // 512MB
                max_num_threads: 1, // Defaults to 1
                enable_compression: true,
                read_only: false,
                max_db_size: 1 << 30, // 1 GB default
                auto_checkpoint: true,
                checkpoint_threshold: 1024 * 1024 * 100, // 100MB
            },
            ws_tx: None,
        }
    }

    /*
        FUNCTION : init_db

        Initializes the Kuzu database with the specified path and configuration.

        Parameters:
            path (String): The path to the Kuzu database file.

        Returns:
            None
     */
    #[func]
    fn init_db(&mut self, path: String) {
        self.db = Some(Database::new(&path, self.config.clone()));
        self.conn = Some(Connection::new(self.db.as_ref().unwrap()));
    }

    /*
        FUNCTION : query

        Executes a query on the Kuzu database and returns the result as a string.

        Parameters:
            query (String): The query string to execute.
        Returns:
            String: The result of the query as a string, or an error message if the connection is not established.
     */
    #[func]
    fn query(&self, query: String) -> String {
        if let Some(conn) = &self.conn {
            let result = conn.query(&query);
            return format!("{:?}", result);
        }
        "GDSunrise | ERROR No connection to KuzuDB".to_string()
    }
    
}