var createCounter = function (n) {
  let nn = n;
  return function () {
    nn += 1;
    return nn - 1;
  };
};
