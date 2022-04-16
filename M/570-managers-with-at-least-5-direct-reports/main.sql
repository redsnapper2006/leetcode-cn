SELECT
  name
FROM
  Employee
WHERE
  id in (
    SELECT
      p.managerId
    FROM
      (
        select
          managerId,
          count(*) as cnt
        FROM
          Employee
        WHERE
          managerId is not null
        GROUP BY
          managerId
      ) p
    WHERE
      p.cnt >= 5
  )
