SELECT
  sale_date,
  sum(
    case
      when fruit = "apples" then sold_num
      else - sold_num
    end
  ) diff
FROM
  Sales
GROUP BY
  sale_date
ORDER BY
  sale_date
