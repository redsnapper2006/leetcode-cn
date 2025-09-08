select
  s.store_id,
  s.store_name,
  s.location,
  imx.product_name as most_exp_product,
  imn.product_name as cheapest_product,
  round(mnii.mnq / mxii.mxq, 2) as imbalance_ratio
from
  stores s,
  inventory imx,
  inventory imn,
  (
    select
      i.store_id,
      mxi.mx,
      max(quantity) as mxq
    from
      inventory i,
      (
        select
          store_id,
          max(price) as mx
        from
          inventory
        group by
          store_id
      ) mxi
    where
      i.store_id = mxi.store_id
      and i.price = mxi.mx
    group by
      mxi.store_id,
      mxi.mx
  ) mxii,
  (
    select
      i.store_id,
      mni.mn,
      max(quantity) as mnq
    from
      inventory i,
      (
        select
          store_id,
          min(price) as mn
        from
          inventory
        group by
          store_id
      ) mni
    where
      i.store_id = mni.store_id
      and i.price = mni.mn
    group by
      mni.store_id,
      mni.mn
  ) mnii,
  (
    select
      store_id,
      count(*) as ct
    from
      inventory
    group by
      store_id
  ) cnt
where
  mxii.store_id = mnii.store_id
  and mnii.store_id = cnt.store_id
  and cnt.store_id = s.store_id
  and cnt.ct >= 3
  and mxii.mxq < mnii.mnq
  and imx.store_id = s.store_id
  and imn.store_id = s.store_id
  and imx.price = mxii.mx
  and imx.quantity = mxii.mxq
  and imn.price = mnii.mn
  and imn.quantity = mnii.mnq
order by
  imbalance_ratio desc,
  s.store_name
