mod common;

use bigdecimal::BigDecimal;
use common::setup_postgres;
use firefleeb_api::db::{PgPool, get_conn, product_repository};
use firefleeb_api::handlers::dtos::ProductResponse;
use firefleeb_api::routes::{handle_rejection, product_routes::product_routes};
use serde_json::json;
use std::str::FromStr;
use uuid::Uuid;
use warp::Filter;

fn product_filter(
    pool: PgPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = std::convert::Infallible> + Clone {
    product_routes(pool).recover(handle_rejection)
}

#[tokio::test]
async fn create_and_get_product_round_trip() {
    let test_db = setup_postgres();
    let filter = product_filter(test_db.pool.clone());

    let payload = json!({
        "product_name": "Route Coffee",
        "product_description": "Fresh beans for route tests",
        "price": "19.99",
        "stock": 25
    });

    let resp = warp::test::request()
        .method("POST")
        .path("/products")
        .json(&payload)
        .reply(&filter)
        .await;

    assert_eq!(resp.status(), 200);
    let created: ProductResponse = serde_json::from_slice(resp.body()).expect("product response");

    let fetch_resp = warp::test::request()
        .method("GET")
        .path(&format!("/products/{id}", id = created.id))
        .reply(&filter)
        .await;

    assert_eq!(fetch_resp.status(), 200);
    let fetched: ProductResponse =
        serde_json::from_slice(fetch_resp.body()).expect("fetch response");

    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.product_name, "Route Coffee");
    assert_eq!(fetched.stock, 25);
}

#[tokio::test]
async fn update_product_via_route_overwrites_fields() {
    let test_db = setup_postgres();
    let filter = product_filter(test_db.pool.clone());

    let create_resp = warp::test::request()
        .method("POST")
        .path("/products")
        .json(&json!({
            "product_name": "Original Widget",
            "product_description": "First revision",
            "price": "9.99",
            "stock": 5
        }))
        .reply(&filter)
        .await;

    assert_eq!(create_resp.status(), 200);
    let created: ProductResponse = serde_json::from_slice(create_resp.body()).expect("created");

    let update_payload = json!({
        "product_name": "Updated Widget",
        "product_description": "Second revision",
        "price": "14.50",
        "stock": 9
    });

    let update_resp = warp::test::request()
        .method("PUT")
        .path(&format!("/products/{id}", id = created.id))
        .json(&update_payload)
        .reply(&filter)
        .await;

    assert_eq!(update_resp.status(), 200);
    let updated: ProductResponse = serde_json::from_slice(update_resp.body()).expect("updated");

    assert_eq!(updated.id, created.id);
    assert_eq!(updated.product_name, "Updated Widget");
    assert_eq!(
        updated.product_description.as_deref(),
        Some("Second revision")
    );
    assert_eq!(updated.stock, 9);
    let expected_price = BigDecimal::from_str("14.50").expect("decimal");
    assert_eq!(updated.price, expected_price);
}

#[tokio::test]
async fn delete_product_removes_record() {
    let test_db = setup_postgres();
    let pool = test_db.pool.clone();
    let filter = product_filter(pool.clone());

    let create_resp = warp::test::request()
        .method("POST")
        .path("/products")
        .json(&json!({
            "product_name": "Disposable Widget",
            "product_description": null,
            "price": "4.99",
            "stock": 1
        }))
        .reply(&filter)
        .await;

    assert_eq!(create_resp.status(), 200);
    let created: ProductResponse = serde_json::from_slice(create_resp.body()).expect("created");

    let delete_resp = warp::test::request()
        .method("DELETE")
        .path(&format!("/products/{id}", id = created.id))
        .reply(&filter)
        .await;

    assert_eq!(delete_resp.status(), 204);

    let mut conn = get_conn(&pool).expect("conn");
    let db_record =
        product_repository::get_product_by_id(&mut conn, created.id).expect("db lookup");
    assert!(db_record.is_none());

    let fetch_resp = warp::test::request()
        .method("GET")
        .path(&format!("/products/{id}", id = created.id))
        .reply(&filter)
        .await;
    assert_eq!(fetch_resp.status(), 404);
}

#[tokio::test]
async fn getting_unknown_product_returns_404() {
    let test_db = setup_postgres();
    let filter = product_filter(test_db.pool.clone());
    let random_id = Uuid::new_v4();

    let resp = warp::test::request()
        .method("GET")
        .path(&format!("/products/{random_id}"))
        .reply(&filter)
        .await;

    assert_eq!(resp.status(), 404);

    let body: serde_json::Value = serde_json::from_slice(resp.body()).expect("json");
    assert_eq!(body.get("status").and_then(|s| s.as_u64()), Some(404));
}
