
select distinct p.product_id, ifnull(pppp.price, 10) price
from Products p
left join (
select p.product_id, p.new_price price
from Products p
  join (
select pp.product_id, max(pp.change_date) mcd from Products pp where  date_format(pp.change_date, "%Y-%m-%d") <= '2019-08-16'
group by pp.product_id
) ppp
  on p.product_id = ppp.product_id
  and p.change_date = ppp.mcd
) pppp
on p.product_id = pppp.product_id
