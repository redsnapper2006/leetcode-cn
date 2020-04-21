SELECT
  K.id,
  (
    SELECT
      revenue
    FROM
      Department
    WHERE
      id = K.id
      AND MONTH = 'Jan'
  ) AS Jan_Revenue,
  (
    SELECT
      revenue
    FROM
      Department
    WHERE
      id = K.id
      AND MONTH = 'Feb'
  ) AS Feb_Revenue,
  (
    SELECT
      revenue
    FROM
      Department
    WHERE
      id = K.id
      AND MONTH = 'Mar'
  ) AS Mar_Revenue,
  (
    SELECT
      revenue
    FROM
      Department
    WHERE
      id = K.id
      AND MONTH = 'Apr'
  ) AS Apr_Revenue,
  (
    SELECT
      revenue
    FROM
      Department
    WHERE
      id = K.id
      AND MONTH = 'May'
  ) AS May_Revenue,
  (
    SELECT
      revenue
    FROM
      Department
    WHERE
      id = K.id
      AND MONTH = 'Jun'
  ) AS Jun_Revenue,
  (
    SELECT
      revenue
    FROM
      Department
    WHERE
      id = K.id
      AND MONTH = 'Jul'
  ) AS Jul_Revenue,
  (
    SELECT
      revenue
    FROM
      Department
    WHERE
      id = K.id
      AND MONTH = 'Aug'
  ) AS Aug_Revenue,
  (
    SELECT
      revenue
    FROM
      Department
    WHERE
      id = K.id
      AND MONTH = 'Sep'
  ) AS Sep_Revenue,
  (
    SELECT
      revenue
    FROM
      Department
    WHERE
      id = K.id
      AND MONTH = 'Oct'
  ) AS Oct_Revenue,
  (
    SELECT
      revenue
    FROM
      Department
    WHERE
      id = K.id
      AND MONTH = 'Nov'
  ) AS Nov_Revenue,
  (
    SELECT
      revenue
    FROM
      Department
    WHERE
      id = K.id
      AND MONTH = 'Dec'
  ) AS Dec_Revenue
FROM
  (
    SELECT
      DISTINCT id
    FROM
      Department
  ) K
ORDER BY
  K.id
