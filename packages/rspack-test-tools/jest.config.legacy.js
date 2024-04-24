const config = require("./jest.config");

/** @type {import('jest').Config} */
module.exports = {
	...config,
	// can only use filename otherwise will fail by snapshot obsolete
	testPathIgnorePatterns: [
		"Compiler.test.js",
		"Defaults.unittest.js",
		"Stats.test.js",
		"TreeShaking.test.js",
		"Builtin.test.js",
		"HotTestStepWeb.test.js",
		"ConfigTestCases.basictest.js",
		"TestCasesNormal.basictest.js",
		"HotTestCasesWeb.test.js",
		"HotTestCasesWebWorker.test.js",
		"HotTestCasesNode.test.js",
		"Diagnostics.test.js",
		".difftest.js"
	]
};