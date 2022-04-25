SELECT
  IFNULL(
    ROUND(
      count(distinct a.session_id) / count(distinct a.user_id),
      2
    ),
    0
  ) as average_sessions_per_user
FROM
  Activity a
WHERE
  a.activity_date <= '2019-07-27'
  AND a.activity_date >= '2019-06-28'
