
var inorderTraversal = function* (arr) {
  for (c of arr) {
    if (Array.isArray(c)) {
      yield* inorderTraversal(c);
    } else {
      yield c;
    }
  }
};
