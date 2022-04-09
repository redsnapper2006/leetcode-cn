SELECT
  employee_id,
  (
    CASE
      WHEN MOD(employee_id, 2) = 0 then 0
      WHEN LEFT(UPPER(name), 1) = 'M' then 0
      ELSE salary
    END
  ) as bonus
FROM
  Employees
