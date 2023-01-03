CREATE FUNCTION getNthHighestSalary(N INT) RETURNS INT BEGIN RETURN (
  SELECT
    IFNULL(e.salary, NULL) AS getNthHighestSalary
  FROM
    (
      SELECT
        e.salary,
        @rownum := @rownum + 1 AS rank
      FROM
        (
          SELECT
            DISTINCT salary
          FROM
            employee
          ORDER BY
            salary DESC
        ) e,
        (
          SELECT
            @rownum := 0
        ) r
      ORDER BY
        e.salary DESC
      LIMIT
        N
    ) e
  WHERE
    e.rank = N
);

END
