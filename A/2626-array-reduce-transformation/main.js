var reduce = function (nums, fn, init) {
  let aggr = init;
  for (let i = 0; i < nums.length; i++) {
    aggr = fn(aggr, nums[i]);
  }
  return aggr;
};
