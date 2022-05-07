SELECT
  distinct buyer_id
from
  Sales
where
  product_id = (
    select
      product_id
    from
      Product
    where
      product_name = 'S8'
  )
  and buyer_id not IN (
    SELECT
      buyer_id
    FROM
      Sales
    WHERE
      product_id = (
        select
          product_id
        from
          Product
        where
          product_name = 'iPhone'
      )
  )
