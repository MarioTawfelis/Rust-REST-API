mod common;

use std::str::FromStr;

use bigdecimal::BigDecimal;
use common::setup_postgres;
use firefleeb_api::db::{PgPool, get_conn, product_repository, user_repository};
use firefleeb_api::handlers::dtos::CartResponse;
use firefleeb_api::models::{CartItemResponse, NewProduct, NewUser, Product, User};
use firefleeb_api::routes::{cart_routes::cart_routes, handle_rejection};
use firefleeb_api::types::email::Email;
use serde_json::json;
use warp::Filter;

fn cart_filter(
    pool: PgPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = std::convert::Infallible> + Clone {
    cart_routes(pool).recover(handle_rejection)
}

#[tokio::test]
async fn create_cart_defaults_active_zero_total() {
    let test_db = setup_postgres();
    let pool = test_db.pool.clone();
    let filter = cart_filter(pool.clone());

    let user = insert_user(&pool, "cart-defaults@example.com");

    let resp = warp::test::request()
        .method("POST")
        .path("/carts")
        .json(&json!({ "user_id": user.id }))
        .reply(&filter)
        .await;

    assert_eq!(resp.status(), 200);
    let cart: CartResponse = serde_json::from_slice(resp.body()).expect("cart response");
    assert_eq!(cart.user_id, user.id);
    assert_eq!(cart.cart_status, "active");
    assert_eq!(cart.cart_total, BigDecimal::from(0));
    assert!(cart.created_at.is_some());
}

#[tokio::test]
async fn cart_item_crud_updates_cart_total() {
    let test_db = setup_postgres();
    let pool = test_db.pool.clone();
    let filter = cart_filter(pool.clone());

    let user = insert_user(&pool, "cart-items@example.com");
    let product = insert_product(&pool, "Route Beans", "5.50");

    let cart_resp = warp::test::request()
        .method("POST")
        .path("/carts")
        .json(&json!({ "user_id": user.id }))
        .reply(&filter)
        .await;
    assert_eq!(cart_resp.status(), 200);
    let cart: CartResponse = serde_json::from_slice(cart_resp.body()).expect("cart");

    let add_payload = json!({
        "item_id": product.id,
        "quantity": 2,
        "unit_price": "5.50"
    });
    let add_resp = warp::test::request()
        .method("POST")
        .path(&format!("/carts/{}/items", cart.cart_id))
        .json(&add_payload)
        .reply(&filter)
        .await;
    println!("raw body: {}", String::from_utf8_lossy(add_resp.body()));
    assert_eq!(add_resp.status(), 201);
    let added: CartItemResponse = serde_json::from_slice(add_resp.body()).expect("add item");
    assert_eq!(added.cart_id, cart.cart_id);
    assert_eq!(added.quantity, 2);
    assert_eq!(
        added.total_price,
        BigDecimal::from_str("11.00").expect("total")
    );

    let fetched_resp = warp::test::request()
        .method("GET")
        .path(&format!("/carts/{}", user.id))
        .reply(&filter)
        .await;
    assert_eq!(fetched_resp.status(), 200);
    let fetched: CartResponse = serde_json::from_slice(fetched_resp.body()).expect("cart response");
    assert_eq!(
        fetched.cart_total,
        BigDecimal::from_str("11.00").expect("cart total")
    );

    let update_payload = json!({ "quantity": 3 });
    let update_resp = warp::test::request()
        .method("PUT")
        .path(&format!("/carts/{}/items/{}", cart.cart_id, product.id))
        .json(&update_payload)
        .reply(&filter)
        .await;
    assert_eq!(update_resp.status(), 200);
    let updated: CartItemResponse = serde_json::from_slice(update_resp.body()).expect("updated");
    assert_eq!(updated.quantity, 3);
    assert_eq!(
        updated.total_price,
        BigDecimal::from_str("16.50").expect("updated total")
    );

    let fetched_after_update_resp = warp::test::request()
        .method("GET")
        .path(&format!("/carts/{}", user.id))
        .reply(&filter)
        .await;
    assert_eq!(fetched_after_update_resp.status(), 200);
    let fetched_after_update: CartResponse =
        serde_json::from_slice(fetched_after_update_resp.body()).expect("cart");
    assert_eq!(
        fetched_after_update.cart_total,
        BigDecimal::from_str("16.50").expect("cart total")
    );

    let delete_resp = warp::test::request()
        .method("DELETE")
        .path(&format!("/carts/{}/items/{}", cart.cart_id, product.id))
        .reply(&filter)
        .await;
    assert_eq!(delete_resp.status(), 204);

    let cleared_resp = warp::test::request()
        .method("GET")
        .path(&format!("/carts/{}", user.id))
        .reply(&filter)
        .await;
    assert_eq!(cleared_resp.status(), 200);
    let cleared: CartResponse = serde_json::from_slice(cleared_resp.body()).expect("cart response");
    assert_eq!(cleared.cart_total, BigDecimal::from(0));
}

fn insert_user(pool: &PgPool, email: &str) -> User {
    let mut conn = get_conn(pool).expect("conn");
    let new_user = NewUser {
        email: Email::parse(email).expect("email"),
        password_hash: "test-hash".into(),
    };
    user_repository::create_user(&mut conn, &new_user).expect("create user")
}

fn insert_product(pool: &PgPool, name: &str, price: &str) -> Product {
    let mut conn = get_conn(pool).expect("conn");
    let new_product = NewProduct {
        product_name: name.into(),
        product_description: Some("cart test product".into()),
        price: BigDecimal::from_str(price).expect("price"),
        stock: 100,
    };
    let created =
        product_repository::create_product(&mut conn, &new_product).expect("create product");
    product_repository::get_product_by_id(&mut conn, created.id)
        .expect("db lookup")
        .expect("product should exist");
    created
}
