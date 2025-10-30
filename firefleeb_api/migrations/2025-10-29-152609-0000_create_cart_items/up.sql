CREATE TABLE cart_items (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  item_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
  cart_id UUID NOT NULL REFERENCES carts(id) ON DELETE CASCADE,
  quantity INT NOT NULL CHECK (quantity > 0),
  unit_price NUMERIC(10, 2) NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
  CONSTRAINT uq_cart_items_cart_id_item_id UNIQUE (cart_id, item_id)
);