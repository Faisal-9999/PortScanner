use postgres::{Client, NoTls};
use serde_json::{json, Value};

pub struct DBHandler;

impl DBHandler {
    pub fn save_to_db(scanned_ports: Vec<(u8, bool)>) -> Result<(), String> {
        let mut default_client = Client::connect(
            "host=localhost user=postgres dbname=postgres",
            NoTls,
        ).map_err(|e| e.to_string())?;

        let rows = default_client
            .query(
                "SELECT 1 FROM pg_database WHERE datname = 'scanhistory'",
                &[],
            )
            .map_err(|e| e.to_string())?;

        if rows.is_empty() {
            default_client
                .batch_execute(r#"CREATE DATABASE "scanhistory""#)
                .map_err(|e| e.to_string())?;
        }

        let mut client = Client::connect(
            "host=localhost user=postgres dbname=scanhistory",
            NoTls,
        ).map_err(|e| e.to_string())?;

        client
            .batch_execute(
                r#"
                CREATE TABLE IF NOT EXISTS history (
                    id SERIAL PRIMARY KEY,
                    date DATE NOT NULL,
                    time TIME NOT NULL,
                    ports JSONB NOT NULL
                )
                "#,
            )
            .map_err(|e| e.to_string())?;

        let mut map = serde_json::Map::new();
        for (port, status) in scanned_ports {
            map.insert(
                port.to_string(),
                Value::String(if status { "open" } else { "closed" }.into()),
            );
        }
        let ports_json = Value::Object(map);

        client
            .execute(
                "INSERT INTO history (date, time, ports) \
                 VALUES (CURRENT_DATE, CURRENT_TIME, $1)",
                &[&ports_json],
            )
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
