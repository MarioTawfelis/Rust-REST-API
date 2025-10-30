// @generated automatically by Diesel CLI.

diesel::table! {
    cart_items (id) {
        id -> Uuid,
        item_id -> Uuid,
        cart_id -> Uuid,
        quantity -> Int4,
        unit_price -> Numeric,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    carts (id) {
        id -> Uuid,
        user_id -> Uuid,
        cart_status -> Text,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    products (id) {
        id -> Uuid,
        product_name -> Text,
        product_description -> Nullable<Text>,
        price -> Numeric,
        stock -> Int4,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        password_hash -> Text,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(cart_items -> carts (cart_id));
diesel::joinable!(cart_items -> products (item_id));
diesel::joinable!(carts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(cart_items, carts, products, users,);
