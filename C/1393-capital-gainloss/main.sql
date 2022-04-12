SELECT
  S.stock_name,
  (S.sp - B.sp) as capital_gain_loss
FROM
  (
    SELECT
      stock_name,
      SUM(price) sp
    FROM
      Stocks
    WHERE
      operation = 'Buy'
    GROUP BY
      stock_name,
      operation
  ) B,
  (
    SELECT
      stock_name,
      SUM(price) sp
    FROM
      Stocks
    WHERE
      operation = 'Sell'
    GROUP BY
      stock_name,
      operation
  ) S
WHERE
  B.stock_name = S.stock_name
