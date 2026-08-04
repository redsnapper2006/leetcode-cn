select
  total.customer_id,
  total.total as total_orders,
  round(rush.cnt / total.total * 100, 0) as peak_hour_percentage,
  rate.average_rating
from
  (
    select
      customer_id,
      count(*) as total
    from
      restaurant_orders
    group by
      customer_id
  ) as total,
  (
    select
      customer_id,
      count(*) as cnt
    from
      restaurant_orders
    where
      (
        DATE_FORMAT (order_timestamp, '%H:%i') >= '11:00'
        AND DATE_FORMAT (order_timestamp, '%H:%i') <= '14:00'
      )
      OR DATE_FORMAT (order_timestamp, '%H:%i') >= '18:00'
      AND DATE_FORMAT (order_timestamp, '%H:%i') <= '21:00'
    group by
      customer_id
  ) as rush,
  (
    select
      customer_id,
      count(order_rating) as rating,
      round(avg(order_rating), 2) as average_rating
    from
      restaurant_orders
    group by
      customer_id
  ) as rate
where
  total.customer_id = rush.customer_id
  and total.total >= 3
  and rush.cnt * 5 >= total.total * 3
  and rate.customer_id = rush.customer_id
  and rate.average_rating >= 4.0
  and rate.rating * 2 >= total.total
order by
  rate.average_rating desc,
  total.customer_id desc
