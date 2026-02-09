SELECT
  s.score,
  t.rank1 AS 'rank'
FROM
  (
    SELECT
      ss.score,
      @rownum := @rownum + 1 AS rank1
    FROM
      (
        SELECT
          DISTINCT score
        FROM
          scores
        ORDER BY
          score DESC
      ) ss,
      (
        SELECT
          @rownum := 0
      ) r
  ) t,
  scores s
WHERE
  s.score = t.score
ORDER BY
  t.rank1
