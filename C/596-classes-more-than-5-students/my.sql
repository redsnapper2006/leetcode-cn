SELECT
  t.class
FROM
  (
    SELECT
      K.class,
      count(*) AS CNT
    FROM
      (
        SELECT
          DISTINCT class,
          student
        FROM
          courses
      ) K
    GROUP BY
      K.class
  ) t
WHERE
  t.CNT >= 5
