select p.product_name, sum(o.unit) unit
  from Orders o, Products p 
  where DATE_FORMAT(o.order_date, "%Y-%m") = "2020-02"
    and o.product_id = p.product_id
  group by p.product_id
having sum(unit) >= 100
