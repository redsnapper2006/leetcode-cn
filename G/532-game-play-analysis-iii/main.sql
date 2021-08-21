SELECT
  a.player_id,
  a.event_date,
  (
    SELECT
      sum(games_played)
    FROM
      Activity sub
    WHERE
      sub.player_id = a.player_id
      AND sub.event_date <= a.event_date
  ) as games_played_so_far
FROM
  Activity a