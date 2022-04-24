select
  m.month,
  o.order_count,
  m.customer_count
from
  (
    select
      date_format(order_date, '%Y-%m') as month,
      count(distinct customer_id) as customer_count
    from
      Orders
    where
      invoice > 20
    group by
      date_format(order_date, '%Y-%m')
  ) m,
  (
    select
      date_format(order_date, '%Y-%m') as month,
      count(invoice) as order_count
    from
      Orders
    where
      invoice > 20
    group by
      date_format(order_date, '%Y-%m')
  ) o
where
  m.month = o.month
