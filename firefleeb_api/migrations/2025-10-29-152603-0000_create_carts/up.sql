CREATE TABLE carts (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  cart_status TEXT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);
-- Ensure a user can have at most one active cart
CREATE UNIQUE INDEX uq_carts_user_active ON carts (user_id) WHERE cart_status = 'active';
