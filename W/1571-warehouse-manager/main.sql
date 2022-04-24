select
  wh.name as WAREHOUSE_NAME,
  sum(wh.units * p.width * p.length * p.height) as VOLUME
from
  Warehouse wh,
  Products P
where
  wh.product_id = p.product_id
group by
  wh.name
