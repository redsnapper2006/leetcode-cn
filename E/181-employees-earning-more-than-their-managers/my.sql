SELECT
  e.Name AS Employee
FROM
  Employee e
WHERE
  e.Salary > (
    SELECT
      ee.Salary
    FROM
      Employee ee
    WHERE
      ee.Id = e.ManagerId
  )
