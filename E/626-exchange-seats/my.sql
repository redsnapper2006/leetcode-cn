SELECT
  s.id,
  CASE
    WHEN s.id % 2 = 1 THEN IFNULL(
      (
        SELECT
          sub.student
        FROM
          seat sub
        WHERE
          sub.id = s.id + 1
      ),
      s.student
    )
    WHEN s.id % 2 = 0 THEN (
      SELECT
        sub.student
      FROM
        seat sub
      WHERE
        sub.id = s.id -1
    )
  END AS student
FROM
  seat s
