select date_add(target, interval 6 day) visited_on,
  sum(amount) amount,
  round(sum(amount)/7,2) average_amount
from (
    select @ss:=DATE_ADD(@ss, INTERVAL 1 day) as target
    from
        (SELECT @start := (select DATE_SUB(min(visited_on),INTERVAL 1 DAY) from Customer)) s,
        (SELECT @eee := (select DATE_SUB(max(visited_on), INTERVAL 6 DAY) from Customer)) e,
        (SELECT @ss:=@start) r, Customer c
    ) cc, Customer c
where c.visited_on >= cc.target
and   c.visited_on <= DATE_ADD(cc.target, interval 6 day)
and   cc.target <= (select DATE_SUB(max(visited_on), INTERVAL 6 DAY) from Customer)
group by target
