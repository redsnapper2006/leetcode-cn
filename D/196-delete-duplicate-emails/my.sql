DELETE FROM
  Person
WHERE
  Id NOT IN (
    SELECT
      B.MINID
    FROM
      (
        SELECT
          MIN(t.Id) AS MINID
        FROM
          Person t
        GROUP BY
          t.Email
      ) B
  )
