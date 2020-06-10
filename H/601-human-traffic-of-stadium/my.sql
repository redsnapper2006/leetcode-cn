SELECT
  *
FROM
  (
    SELECT
      s1.id,
      s1.visit_date,
      s1.people
    FROM
      stadium s1,
      stadium s2,
      stadium s3
    WHERE
      s1.people >= 100
      AND s2.people >= 100
      AND s3.people >= 100
      AND s1.id + 1 = s2.id
      AND s1.id + 2 = s3.id
    UNION
    SELECT
      s1.id,
      s1.visit_date,
      s1.people
    FROM
      stadium s1,
      stadium s2,
      stadium s3
    WHERE
      s1.people >= 100
      AND s2.people >= 100
      AND s3.people >= 100
      AND s1.id -1 = s2.id
      AND s1.id + 1 = s3.id
    UNION
    SELECT
      s1.id,
      s1.visit_date,
      s1.people
    FROM
      stadium s1,
      stadium s2,
      stadium s3
    WHERE
      s1.people >= 100
      AND s2.people >= 100
      AND s3.people >= 100
      AND s1.id -1 = s2.id
      AND s1.id -2 = s3.id
  ) t
ORDER BY
  1
