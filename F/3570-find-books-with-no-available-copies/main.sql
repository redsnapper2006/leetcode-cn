select
  lb.book_id,
  lb.title,
  lb.author,
  lb.genre,
  lb.publication_year,
  rd.current_borrowers
from
  library_books lb,
  (
    select
      book_id,
      count(*) current_borrowers
    from
      borrowing_records
    where
      return_date is null
    group by
      book_id
  ) rd
where
  lb.book_id = rd.book_id
  and lb.total_copies = rd.current_borrowers
order by
  rd.current_borrowers desc,
  lb.title
