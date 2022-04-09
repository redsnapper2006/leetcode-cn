SELECT
  product_id,
  store,
  price
FROM
  (
    (
      select
        product_id,
        "store1" as store,
        store1 as price,
        "1" as r
      from
        products
      where
        store1 IS NOT null
    )
    UNION
    ALL (
      select
        product_id,
        "store2" as store,
        store2 as price,
        "2" as r
      from
        products
      where
        store2 IS NOT null
    )
    UNION
    ALL (
      select
        product_id,
        "store3" as store,
        store3 as price,
        "3" as r
      from
        products
      where
        store3 IS NOT null
    )
  ) p
ORDER BY
  p.product_id,
  p.r
