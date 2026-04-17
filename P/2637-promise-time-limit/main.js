var timeLimit = (fn, t) => async (...args) => new Promise((resolve, reject) => {
  setTimeout(() => reject("Time Limit Exceeded"), t);
  fn(...args).then((res) => resolve(res)).catch((err) =>
    reject(err))
});
