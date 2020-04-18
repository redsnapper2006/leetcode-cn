SELECT
  d.Name AS Department,
  e.Name AS Employee,
  t.Salary AS Salary
FROM
  (
    SELECT
      dist.DepartmentId,
      dist.Salary,
      IF(
        @lastDpt = dist.DepartmentId,
        @lastRank := @lastRank + 1,
        @lastRank := 1
      ) AS RANK,
      @lastDpt := dist.DepartmentId
    FROM
      (
        SELECT
          DISTINCT e.DepartmentId,
          e.Salary
        FROM
          Employee e
      ) dist,
      (
        SELECT
          @lastDpt := 0,
          @lastRank := 0
      ) SQLVars
    ORDER BY
      dist.DepartmentId ASC,
      dist.Salary DESC
  ) t,
  Employee e,
  Department d
WHERE
  t.DepartmentId = d.Id
  AND t.RANK <= 3
  AND t.DepartmentId = e.DepartmentId
  AND t.Salary = e.Salary
ORDER BY
  d.Id,
  t.Salary DESC
