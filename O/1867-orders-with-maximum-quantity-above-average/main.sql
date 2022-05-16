SELECT
  od.order_id
FROM
  OrdersDetails od
GROUP BY
  od.order_id
HAVING
  max(od.quantity) > (
    SELECT
      max(mo.aq)
    FROM
      (
        SELECT
          avg(o.quantity) as aq
        FROM
          OrdersDetails o
        GROUP BY
          o.order_id
      ) mo
  )
