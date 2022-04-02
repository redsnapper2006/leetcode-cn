SELECT
  t.actor_id,
  t.director_id
FROM
  (
    SELECT
      actor_id,
      director_id,
      COUNT(*) as cnt
    FROM
      ActorDirector
    GROUP BY
      actor_id,
      director_id
  ) t
WHERE
  t.cnt >= 3
