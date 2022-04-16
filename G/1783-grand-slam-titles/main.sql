SELECT
  ps.player_id,
  ps.player_name,
  SUM(P.CNT) as grand_slams_count
FROM
  (
    (
      select
        Wimbledon as ID,
        count(*) cnt
      FROM
        Championships
      GROUP BY
        Wimbledon
    )
    UNION
    ALL (
      select
        Fr_open as ID,
        count(*) cnt
      FROM
        Championships
      GROUP BY
        Fr_open
    )
    UNION
    ALL (
      select
        US_open as ID,
        count(*) cnt
      FROM
        Championships
      GROUP BY
        US_open
    )
    UNION
    ALL (
      select
        Au_open as ID,
        count(*) cnt
      FROM
        Championships
      GROUP BY
        Au_open
    )
  ) P,
  Players ps
WHERE
  ps.player_id = P.ID
GROUP BY
  P.ID
