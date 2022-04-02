SELECT
  p.product_id,
  p.product_name
FROM
  Product p
WHERE
  p.product_id NOT IN (
    SELECT
      s.product_id
    FROM
      Sales s
    WHERE
      s.sale_date < '2019-01-01'
      OR s.sale_date > '2019-03-31'
  )
