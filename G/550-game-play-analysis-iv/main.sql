SELECT
  ROUND(
    (
      SELECT
        COUNT(*)
      FROM
        (
          SELECT
            DISTINCT aa.player_id
          FROM
            (
              SELECT
                a.player_id,
                min(a.event_date) as min_event_date
              FROM
                Activity a
              GROUP BY
                a.player_id
            ) aa,
            Activity b
          WHERE
            aa.player_id = b.player_id
            AND b.event_date = DATE_ADD(aa.min_event_date, INTERVAL 1 DAY)
        ) d
    ) / (
      SELECT
        COUNT(*)
      FROM
        (
          SELECT
            DISTINCT c.player_id
          FROM
            Activity c
        ) e
    ),
    2
  ) as fraction
FROM
  DUAL
