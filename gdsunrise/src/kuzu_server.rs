use kuzu::{Database, SystemConfig, Connection};
use godot::prelude::*;
use std::sync::Arc;


#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct KuzuServer {
    config: SystemConfig,
    db: Option<Arc<Database>>,
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
    fn new() -> Self {
        // let config = SystemConfig::default();

        // config.buffer_pool_size(512 * 1024 * 1024); // 512MB
        // config.max_num_threads(1); // Defaults to 1 
        // config.enable_compression(true);
        // config.read_only(false);
        // config.max_db_size(1 << 30); // 1 GB default
        // config.auto_checkpoint(true);
        // config.checkpoint_threshold(1024 * 1024 * 100); // 100MB
        
        Self {
            config: SystemConfig::default(),
            db: None,
        }
    }

    /*
        FUNCTION : init_db

        Initializes the Kuzu database with the specified path and configuration.
        Ability to have multiple databases by instantiating multiple KuzuServer's with different paths.

        Parameters:
            path (String): The path to the Kuzu database file.

        Returns:
            None
     */
    #[func]
    fn init_db(&mut self, path: String) {
        match Database::new(path, self.config.clone()) {
            Ok(db) => {
                let arc_db = Arc::new(db);
                self.db = Some(arc_db);
                godot_print!("GDSunrise | Kuzu DB initialized.");
            }
            Err(e) => godot_error!("GDSunrise | Database error: {:?}", e),
        }
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
    fn query(&self, q: String) -> String {
        match &self.db {
            Some(db) => match Connection::new(db) {
                Ok(conn) => match conn.query(&q) {
                    Ok(r) => format!("{}", r),
                    Err(e) => format!("GDSunrise | Query error: {:?}", e),
                },
                Err(e) => format!("GDSunrise | Connection init failed: {:?}", e),
            },
            None => "GDSunrise | No DB initialized.".to_string(),
        }
    }

    //TODO : Add Kuzu Config Setter Functions
    
    
}