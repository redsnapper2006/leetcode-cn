SELECT
  p.employee_id,
  p.name,
  t.reports_count,
  t.average_age
FROM Employees p
JOIN (
  SELECT
    reports_to,
    round(AVG(age)) as average_age,
    COUNT(*) as reports_count
  FROM Employees
  GROUP BY reports_to
) t ON p.employee_id = t.reports_to
order by p.employee_id
