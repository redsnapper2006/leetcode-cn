SELECT
  t.activity_date as day,
  COUNT(*) as active_users
FROM
  (
    SELECT
      DISTINCT activity_date,
      user_id
    FROM
      Activity
    WHERE
      activity_date BETWEEN '2019-6-28'
      AND '2019-07-27'
  ) t
GROUP BY
  t.activity_date
