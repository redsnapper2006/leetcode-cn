SELECT
  total.Request_at AS DAY,
  ROUND(IFNULL(cancel.COUNT, 0) / total.COUNT, 2) AS 'Cancellation Rate'
FROM
  (
    SELECT
      Request_at,
      COUNT(*) AS COUNT
    FROM
      Trips t,
      Users u
    WHERE
      t.Client_Id = u.Users_Id
      AND u.Banned = 'No'
    GROUP BY
      Request_at
  ) total
  LEFT JOIN (
    SELECT
      Request_at,
      COUNT(*) AS COUNT
    FROM
      Trips t,
      Users u
    WHERE
      t.Client_Id = u.Users_Id
      AND u.Banned = 'No'
      AND t.Status IN ('cancelled_by_driver', 'cancelled_by_client')
    GROUP BY
      Request_at
  ) cancel ON total.Request_at = cancel.Request_at
WHERE
  total.Request_at <= '2013-10-03'
  AND total.Request_at >= '2013-10-01'
