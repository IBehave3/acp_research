use mongodb::Database;

pub const ID_MAPPING_COLLECTION_NAME: &str = "id_mapping";
pub const PUSH_DATA_COLLECTION_NAME: &str = "push_data";
pub const NOTIFICATION_COLLECTION_NAME: &str = "notification";
pub const TEST_COLLECTION_NAME: &str = "test";

pub async fn create_collection(db: &Database, container_name: &str) {
    match db.create_collection(container_name, None).await {
        Err(_) => println!("Info container {container_name} already exists"),
        Ok(_) => println!("Infor contianer {container_name} created"),
    };
}
