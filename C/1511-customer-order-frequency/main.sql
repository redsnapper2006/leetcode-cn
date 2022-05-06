select
  c.customer_id,
  c.name
from
  Customers c,
  (
    select
      p.customer_id,
      count(*) as cnt
    from
      (
        select
          o.customer_id,
          sum(p.price * o.quantity) as total,
          DATE_FORMAT(o.order_date, '%Y-%m') as month
        from
          Product p,
          Orders o
        where
          p.product_id = o.product_id
        group by
          o.customer_id,
          DATE_FORMAT(o.order_date, '%Y-%m')
        having
          (
            month = '2020-06'
            and total >= 100
          )
          or (
            month = '2020-07'
            and total >= 100
          )
      ) p
    group by
      p.customer_id
  ) t
where
  t.customer_id = c.customer_id
  and t.cnt = 2
