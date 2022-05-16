select
  c.name as customer_name,
  oo.customer_id,
  oo.order_id,
  oo.order_date
from
  Customers c,
  (
    select
      o.order_id,
      o.order_date,
      o.customer_id,
      ROW_NUMBER() OVER (
        PARTITION BY o.customer_id
        ORDER BY
          o.order_date desc
      ) AS rk
    from
      Orders o
  ) oo
where
  c.customer_id = oo.customer_id
  and oo.rk <= 3
order by
  c.name,
  oo.customer_id,
  oo.order_date desc
