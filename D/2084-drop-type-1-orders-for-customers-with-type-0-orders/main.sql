SELECT
  o.order_id,
  o.customer_id,
  o.order_type
FROM
  Orders o
WHERE
  o.order_type = 0
UNION ALL
SELECT
  o.order_id,
  o.customer_id,
  o.order_type
FROM
  Orders o
WHERE
  o.customer_id NOT IN (
    SELECT
      oo.customer_id
    FROM
      Orders oo
    WHERE
      oo.order_type = 0
  )
