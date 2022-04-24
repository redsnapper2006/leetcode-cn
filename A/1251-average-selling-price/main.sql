select
  pp.product_id,
  round(pp.sum_total / pp.sum_units, 2) as average_price
from
  (
    select
      p.product_id,
      sum(us.units) as sum_units,
      sum(p.price * us.units) as sum_total
    from
      Prices p,
      UnitsSold us
    where
      p.product_id = us.product_id
      and p.start_date <= us.purchase_date
      and p.end_date >= us.purchase_date
    group by
      p.product_id
  ) as pp
