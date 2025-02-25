select
  *
from
  Users
where
  LOCATE ('@', email) > 0
  and LOCATE ('@', email, LOCATE ('@', email) + 1) = 0
  and email REGEXP '\.com$'
  and SUBSTRING(email, 1, LOCATE ('@', email) -1) REGEXP '^[a-zA-Z0-9_]+$'
  and SUBSTRING(
    email,
    LOCATE ('@', email) + 1,
    LOCATE ('.com', email) - LOCATE ('@', email) -1
  ) REGEXP '^[a-zA-Z]+$'
order by
  user_id
