SELECT s.product_id, s.year first_year, s.quantity, s.price
FROM Sales s
INNER JOIN (
    SELECT
      s.product_id,
      MIN(s.year) AS m_year
    FROM
      Sales s
    GROUP BY
      s.product_id
) ss
ON
  s.product_id = ss.product_id
  AND s.year = ss.m_year
