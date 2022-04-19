select
  distinct cc1.user_id
from
  (
    SELECT
      user_id,
      time_stamp,
      ROW_NUMBER() OVER (
        PARTITION BY user_id
        ORDER BY
          time_stamp
      ) AS rk
    FROM
      Confirmations
  ) cc1,
(
    SELECT
      user_id,
      time_stamp,
      ROW_NUMBER() OVER (
        PARTITION BY user_id
        ORDER BY
          time_stamp
      ) AS rk
    FROM
      Confirmations
  ) cc2
where
  cc1.rk + 1 = cc2.rk
  and cc1.user_id = cc2.user_id
  and TIMESTAMPDIFF(SECOND, cc1.time_stamp, cc2.time_stamp) <= 86400
