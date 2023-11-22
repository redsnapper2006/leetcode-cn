/**
 * @param {Array} arr
 * @param {Function} fn
 * @return {Array}
 */
const sortBy = function (arr, fn) {
  ret = arr.sort((a, b) => {
    const fa = fn(a);
    const fb = fn(b);
    if (fa < fb) {
      return -1;
    } else if (fa > fb) {
      return 1;
    }
    return 0;
  });
  return ret;
};
