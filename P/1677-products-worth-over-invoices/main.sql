SELECT
  p.name,
  IFNULL(sum(i.rest), 0) as rest,
  IFNULL(sum(i.paid), 0) as paid,
  IFNULL(sum(i.canceled), 0) as canceled,
  IFNULL(sum(i.refunded), 0) as refunded
from
  Product p
  left outer join Invoice I on p.product_id = i.product_id
group by
  p.product_id
order by
  p.name
