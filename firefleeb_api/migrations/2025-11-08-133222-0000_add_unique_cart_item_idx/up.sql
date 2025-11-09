ALTER TABLE cart_items
ADD CONSTRAINT uq_cart_items_cart_product UNIQUE (cart_id, item_id);