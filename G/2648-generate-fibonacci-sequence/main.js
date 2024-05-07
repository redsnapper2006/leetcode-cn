var fibGenerator = function* () {
  let i1 = -1;
  let i2 = 1;
  while (true) {
    let v = i1 + i2;
    i1 = i2;
    i2 = v;
    yield v;
  }
};
