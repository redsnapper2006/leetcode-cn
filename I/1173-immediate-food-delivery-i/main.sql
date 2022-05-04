select
  round(
    (
      select
        count(*)
      from
        Delivery d
      where
        d.order_date = d.customer_pref_delivery_date
    ) /(
      select
        count(*)
      from
        Delivery
    ) * 100,
    2
  ) as immediate_percentage
