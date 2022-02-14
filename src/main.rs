use log::Level;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, EntityTrait, QueryOrder, QueryTrait};
use select_two_many_example::entity::{food, food_truck};

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Debug).unwrap();

    let url = "postgres://username:password@localhost:5432/db";

    let option = ConnectOptions::new(url.to_string());

    let db = Database::connect(option).await.expect("connect postgresql");

    /*

    CREATE TABLE IF NOT EXISTS "food_truck" (
        "id" int PRIMARY KEY,
        "name" varchar NOT NULL
    );

    CREATE TABLE IF NOT EXISTS "food" (
        "id" int PRIMARY KEY,
        "food_truck_id" int NOT NULL,
        "name" varchar NOT NULL,
        CONSTRAINT "food_truck_id" FOREIGN KEY ("food_truck_id") REFERENCES "food_truck" ("id") ON DELETE CASCADE
    );

    */

    food::Entity::delete_many().exec(&db).await.unwrap();
    food_truck::Entity::delete_many().exec(&db).await.unwrap();

    insert_all(&db).await;

    let food_trucks_with_foods = {
        let select = food_truck::Entity::find().find_with_related(food::Entity);

        log::debug!("{:#?}", select.as_query()); // why added `ORDER BY food_truck.id ASC`?

        select
            .order_by_desc(food_truck::Column::Name)
            .all(&db)
            .await
            .unwrap()
    };

    log::info!("{food_trucks_with_foods:#?}");
}

async fn insert_food_trucks(food_trucks: Vec<food_truck::ActiveModel>, db: &DatabaseConnection) {
    let _ = food_truck::Entity::insert_many(food_trucks)
        .exec(db)
        .await
        .unwrap();
}

async fn insert_foods(foods: Vec<food::ActiveModel>, db: &DatabaseConnection) {
    let _ = food::Entity::insert_many(foods).exec(db).await.unwrap();
}

fn food_trucks() -> Vec<food_truck::ActiveModel> {
    use sea_orm::ActiveValue::*;

    vec![
        food_truck::ActiveModel {
            id: Set(0),
            name: Set("Baskin Robbins".into()),
        },
        food_truck::ActiveModel {
            id: Set(1),
            name: Set("McDonald's".into()),
        },
        food_truck::ActiveModel {
            id: Set(2),
            name: Set("Kentucky Fried Chicken".into()),
        },
    ]
}

fn foods() -> Vec<food::ActiveModel> {
    use sea_orm::ActiveValue::*;

    vec![
        /* Baskin Robbins */
        food::ActiveModel {
            id: Set(0),
            name: Set("31 Yogurt".into()),
            food_truck_id: Set(0),
        },
        food::ActiveModel {
            id: Set(1),
            name: Set("Green Tea".into()),
            food_truck_id: Set(0),
        },
        food::ActiveModel {
            id: Set(2),
            name: Set("Rainbow Sherbet".into()),
            food_truck_id: Set(0),
        },
        food::ActiveModel {
            id: Set(3),
            name: Set("Mint Chocolate Chip".into()),
            food_truck_id: Set(0),
        },
        food::ActiveModel {
            id: Set(4),
            name: Set("Shooting Star".into()),
            food_truck_id: Set(0),
        },
        /* McDonald's */
        food::ActiveModel {
            id: Set(5),
            name: Set("Big Mac".into()),
            food_truck_id: Set(1),
        },
        food::ActiveModel {
            id: Set(6),
            name: Set("Double Quarter Pounder with Cheese".into()),
            food_truck_id: Set(1),
        },
        food::ActiveModel {
            id: Set(7),
            name: Set("Bacon Tomato Delux".into()),
            food_truck_id: Set(1),
        },
        food::ActiveModel {
            id: Set(8),
            name: Set("Cheese Burger".into()),
            food_truck_id: Set(1),
        },
        /* KFC */
        food::ActiveModel {
            id: Set(9),
            name: Set("Zinger Burger".into()),
            food_truck_id: Set(2),
        },
        food::ActiveModel {
            id: Set(10),
            name: Set("Tower Burger".into()),
            food_truck_id: Set(2),
        },
        food::ActiveModel {
            id: Set(11),
            name: Set("Zinger Double Down MAXX".into()),
            food_truck_id: Set(2),
        },
    ]
}

async fn insert_all(db: &DatabaseConnection) {
    insert_food_trucks(food_trucks(), db).await;
    insert_foods(foods(), db).await;
}
