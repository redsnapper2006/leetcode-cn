SELECT
  t.task_id,
  TL.SID as subtask_id
FROM
  Tasks t,
  (
    SELECT
      1 as SID
    FROM
      DUAL
    UNION
    SELECT
      2 as SID
    FROM
      DUAL
    UNION
    SELECT
      3 as SID
    FROM
      DUAL
    UNION
    SELECT
      4 as SID
    FROM
      DUAL
    UNION
    SELECT
      5 as SID
    FROM
      DUAL
    UNION
    SELECT
      6 as SID
    FROM
      DUAL
    UNION
    SELECT
      7 as SID
    FROM
      DUAL
    UNION
    SELECT
      8 as SID
    FROM
      DUAL
    UNION
    SELECT
      9 as SID
    FROM
      DUAL
    UNION
    SELECT
      10 as SID
    FROM
      DUAL
    UNION
    SELECT
      11 as SID
    FROM
      DUAL
    UNION
    SELECT
      12 as SID
    FROM
      DUAL
    UNION
    SELECT
      13 as SID
    FROM
      DUAL
    UNION
    SELECT
      14 as SID
    FROM
      DUAL
    UNION
    SELECT
      15 as SID
    FROM
      DUAL
    UNION
    SELECT
      16 as SID
    FROM
      DUAL
    UNION
    SELECT
      17 as SID
    FROM
      DUAL
    UNION
    SELECT
      18 as SID
    FROM
      DUAL
    UNION
    SELECT
      19 as SID
    FROM
      DUAL
    UNION
    SELECT
      20 as SID
    FROM
      DUAL
  ) TL
WHERE
  t.subtasks_count >= TL.SID
  AND TL.SID NOT IN (
    SELECT
      subtask_id
    FROM
      Executed e
    WHERE
      e.task_id = t.task_id
  )
ORDER BY
  t.task_id
