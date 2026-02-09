SELECT
  total.request_at AS DAY,
  ROUND(IFNULL (cancel.count, 0) / total.count, 2) AS 'Cancellation Rate'
FROM
  (
    SELECT
      request_at,
      COUNT(*) AS COUNT
    FROM
      trips t,
      users u1,
      users u2
    WHERE
      t.client_id = u1.users_id
      AND u1.banned = 'No'
      AND t.driver_id = u2.users_id
      AND u2.banned = 'No'
    GROUP BY
      request_at
  ) total
  LEFT JOIN (
    SELECT
      request_at,
      COUNT(*) AS COUNT
    FROM
      trips t,
      users u
    WHERE
      t.client_id = u.users_id
      AND u.banned = 'No'
      AND t.status IN ('cancelled_by_driver', 'cancelled_by_client')
    GROUP BY
      request_at
  ) cancel ON total.request_at = cancel.request_at
WHERE
  total.request_at <= '2013-10-03'
  AND total.request_at >= '2013-10-01'
