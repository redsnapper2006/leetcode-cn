-- SELECT
--   a.player_id,
--   b.event_date,
--   sum(a.games_played) as games_played_so_far
-- FROM
--   Activity a,
--   Activity b
-- WHERE
--   a.player_id = b.player_id
--   AND a.event_date <= b.event_date
-- GROUP BY
--   b.player_id,
--   b.event_date
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
