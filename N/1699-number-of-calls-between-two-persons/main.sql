SELECT
  p.from_id as person1,
  p.to_id as person2,
  COUNT(*) as call_count,
  SUM(p.duration) as total_duration
FROM
  (
    (
      SELECT
        from_id,
        to_id,
        duration
      FROM
        Calls
      WHERE
        from_id < to_id
    )
    UNION
    ALL (
      SELECT
        to_id,
        from_id,
        duration
      FROM
        Calls
      WHERE
        from_id > to_id
    )
  ) p
GROUP BY
  p.from_id,
  p.to_id
