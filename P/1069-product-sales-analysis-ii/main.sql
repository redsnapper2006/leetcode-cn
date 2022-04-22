SELECT
  product_id,
  sum(quantity) as total_quantity
FROM
  Sales
GROUP BY
  product_id
