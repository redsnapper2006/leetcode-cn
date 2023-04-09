select round(sum(TIV_2016),2) as tiv_2016
from (
  SELECT distinct ins3.PID, ins3.TIV_2016
  FROM (
    SELECT distinct ins1.PID, ins1.TIV_2015,ins1.TIV_2016, ins1.LAT, ins1.LON
    FROM insurance ins1
    INNER JOIN insurance ins2
      ON ins1.PID != ins2.PID
      and ins1.TIV_2015 = ins2.TIV_2015
  ) ins3
  WHERE NOT EXISTS (
    select ins4.PID
    from  insurance ins4
    where
      ins3.PID != ins4.PID
      and ins3.LAT = ins4.LAT
      and ins3.LON = ins4.LON
  )
) a


---


select round(sum(tiv_2016), 2) tiv_2016 from (
  select
    tiv_2016,
    count(*) over(partition by tiv_2015) count_tiv_2015,
    count(*) over(partition by lat, lon) count_lat_lon
  from insurance
) as temp where count_lat_lon = 1 and count_tiv_2015 > 1
;
