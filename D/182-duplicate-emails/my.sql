SELECT
  t.Email
FROM
  (
    SELECT
      Email,
      COUNT(*) AS count
    FROM
      Person
    GROUP BY
      Email
  ) t
WHERE
  t.count > 1
