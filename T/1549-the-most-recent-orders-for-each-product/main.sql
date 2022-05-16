select
  p.product_name,
  p.product_id,
  o.order_id,
  o.order_date
from
  Orders o,
  Products p,
  (
    select
      max(o.order_date) as max_order_date,
      o.product_id
    from
      Orders o
    group by
      o.product_id
  ) mo
where
  o.product_id = mo.product_id
  and o.order_date = mo.max_order_date
  and o.product_id = p.product_id
order by
  p.product_name,
  p.product_id,
  o.order_id
