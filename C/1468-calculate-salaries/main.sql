SELECT
  s.company_id,
  s.employee_id,
  s.employee_name,
  ROUND(
    (
      CASE
        WHEN c.max < 1000 then s.salary
        WHEN c.max >= 1000
        and c.max < 10000 then s.salary * 0.76
        ELSE s.salary * 0.51
      END
    ),
    0
  ) as salary
FROM
  Salaries s,
  (
    SELECT
      s.company_id,
      max(s.salary) as max
    FROM
      Salaries s
    GROUP BY
      s.company_id
  ) as c
WHERE
  s.company_id = c.company_id
