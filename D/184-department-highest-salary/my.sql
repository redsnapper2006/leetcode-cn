SELECT
  d.Name AS Department,
  e.Name AS Employee,
  e.Salary
FROM
  Department d,
  (
    SELECT
      MAX(Salary) AS Salary,
      DepartmentId
    FROM
      Employee
    GROUP BY
      DepartmentId
  ) ee,
  Employee e
WHERE
  e.DepartmentId = ee.DepartmentId
  AND e.Salary = ee.Salary
  AND d.Id = e.DepartmentId
