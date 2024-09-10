module.exports = function (context) {
	let e;
	e = new Error("Failed to load content");
	e.hideStack = true;
  this.emitError(e);
	e = new Error("Failed to load content");
	e.hideStack = true;
	this.emitWarning(e);
	return ""
};
