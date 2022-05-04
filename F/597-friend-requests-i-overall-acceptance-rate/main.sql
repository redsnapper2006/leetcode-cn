select
  round(
    ifnull(
      (
        select
          count(*) as cnt
        from
          (
            select
              distinct requester_id,
              accepter_id
            from
              RequestAccepted
          ) t
      ),
      0
    ) / if(
      (
        select
          count(*)
        from
          (
            select
              distinct sender_id,
              send_to_id
            from
              FriendRequest
          ) t
      ) = 0,
      100000,
(
        select
          count(*)
        from
          (
            select
              distinct sender_id,
              send_to_id
            from
              FriendRequest
          ) t
      )
    ),
    2
  ) as accept_rate
