var chunk = function (arr, size) {
  let idx = 0;
  let res = [];
  for (; idx < arr.length; ) {
    let t = [];
    for (i = 0; i < size && idx + i < arr.length; i++) {
      t.push(arr[idx + i]);
    }
    res.push(t);
    idx += size;
  }
  return res;
};
