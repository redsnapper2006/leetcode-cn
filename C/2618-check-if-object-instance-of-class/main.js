var checkIfInstanceOf = function (obj, classFunction) {
  if (
    obj === null ||
    obj === undefined ||
    classFunction === null ||
    classFunction === undefined
  )
    return false;
  return (
    obj.__proto__ === classFunction.prototype ||
    checkIfInstanceOf(obj.__proto__, classFunction)
  );
};
