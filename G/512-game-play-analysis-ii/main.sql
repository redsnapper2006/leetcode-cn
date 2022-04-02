SELECT
  a.player_id,
  a.device_id
FROM
  Activity a,
  (
    SELECT
      player_id,
      min(event_date) as min_event_date
    FROM
      Activity
    GROUP BY
      player_id
  ) med
WHERE
  a.player_id = med.player_id
  and a.event_date = med.min_event_date
