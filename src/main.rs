mod graphql;
mod sqlite;

const DB_PATH: &str = "sqlite.db";

#[tokio::main]
async fn main() {
    let db = sqlite::SqliteService::connect(DB_PATH).await.unwrap();
    let gql = tokio::spawn(graphql::serve_graphql(db.clone()));

    // get the insertion devices from the db
    let insertion_devices = sqlite::SqliteService::get_insertion_devices(&db)
        .await
        .unwrap();
    for device in insertion_devices {
        println!(
            "uuid: {}, poles: {}, length: {}",
            device.uuid, device.poles, device.length
        );
    }

    // Get all of the devices.
    let device_results = sqlite::SqliteService::get_devices(&db).await.unwrap();
    for device in device_results {
        println!(
            "beamline: {}, device_name: {}, uuid: {}",
            device.beamline, device.device_name, device.uuid
        );
    }
    let _ = gql.await;
}
