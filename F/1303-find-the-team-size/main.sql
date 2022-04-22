select
  e.employee_id,
  t.cnt as team_size
FROM
  Employee e,
  (
    SELECT
      team_id,
      COUNT(*) as cnt
    FROM
      Employee
    group by
      team_id
  ) t
WHERE
  e.team_id = t.team_id
