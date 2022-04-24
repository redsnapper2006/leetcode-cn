select
  lower(trim(product_name)) as product_name,
  DATE_FORMAT(sale_date, '%Y-%m') as sale_date,
  count(*) as total
from
  Sales
group by
  lower(trim(product_name)),
  DATE_FORMAT(sale_date, '%Y-%m')
order by
  lower(trim(product_name)),
  DATE_FORMAT(sale_date, '%Y-%m')
