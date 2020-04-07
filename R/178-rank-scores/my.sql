SELECT
  s.Score,
  t.Rank
FROM
  (
    SELECT
      ss.Score,
      @rownum := @rownum + 1 AS Rank
    FROM
      (
        SELECT
          DISTINCT Score
        FROM
          Scores
        ORDER BY
          Score DESC
      ) ss,
      (
        SELECT
          @rownum := 0
      ) r
  ) t,
  Scores s
WHERE
  s.Score = t.Score
ORDER BY
  t.Rank
