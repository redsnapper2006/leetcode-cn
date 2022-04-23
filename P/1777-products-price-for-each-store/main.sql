select
  distinct p.product_id,
  (
    select
      price
    from
      Products s1
    where
      s1.product_id = p.product_id
      and s1.store = "store1"
  ) as store1,
  (
    select
      price
    from
      Products s1
    where
      s1.product_id = p.product_id
      and s1.store = "store2"
  ) as store2,
  (
    select
      price
    from
      Products s1
    where
      s1.product_id = p.product_id
      and s1.store = "store3"
  ) as store3
from
  Products p
