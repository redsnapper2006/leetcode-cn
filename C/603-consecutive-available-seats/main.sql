
select distinct a.seat_id
from Cinema a,
Cinema b
where (a.seat_id + 1 = b.seat_id or a.seat_id -1 = b.seat_id)
and a.free = 1
and b.free =1
order by a.seat_id


