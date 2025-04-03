/**
 * @param {Function} fn
 * @return {Object}
 */
Array.prototype.groupBy = function (fn) {
  let ans = {};
  this.forEach((e) => {
    if (fn(e) in ans) {
      ans[fn(e)].push(e);
    } else {
      ans[fn(e)] = [e];
    }
  });
  return ans;
};

/**
 * [1,2,3].groupBy(String) // {"1":[1],"2":[2],"3":[3]}
 */
