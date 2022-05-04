SELECT
  u.user_id as buyer_id,
  u.join_date,
  IFNULL(
    (
      SELECT
        COUNT(*)
      FROM
        Orders o
      WHERE
        u.user_id = o.buyer_id
        AND DATE_FORMAT(o.order_date, "%Y") = '2019'
      GROUP BY
        o.buyer_id
    ),
    0
  ) as orders_in_2019
FROM
  Users u
