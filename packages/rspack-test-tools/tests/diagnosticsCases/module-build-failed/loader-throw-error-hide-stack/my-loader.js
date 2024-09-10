module.exports = function (context) {
	let e = new Error("Failed to load");
	e.hideStack = true;
  throw e;
};
