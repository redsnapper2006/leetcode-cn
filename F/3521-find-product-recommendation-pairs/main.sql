select
  pp1.product_id product1_id,
  pp2.product_id product2_id,
  pi1.category product1_category,
  pi2.category product2_category,
  count(pp1.user_id) customer_count
from
  (
    select
      product_id,
      user_id
    from
      ProductPurchases
  ) pp1,
  (
    select
      product_id,
      user_id
    from
      ProductPurchases
  ) pp2,
  ProductInfo pi1,
  ProductInfo pi2
where
  pp1.product_id < pp2.product_id
  and pp1.user_id = pp2.user_id
  and pi1.product_id = pp1.product_id
  and pi2.product_id = pp2.product_id
group by
  concat (
    cast(pp1.product_id as char),
    '_',
    cast(pp2.product_id as char)
  )
having
  count(pp1.user_id) >= 3
order by
  customer_count desc,
  pp1.product_id,
  pp2.product_id
