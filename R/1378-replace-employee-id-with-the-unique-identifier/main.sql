select
  (
    select
      u.unique_id
    from
      EmployeeUNI u
    where
      u.id = e.id
  ) unique_id,
  e.name
from
  Employees e
