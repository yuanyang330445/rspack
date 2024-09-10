/**
 * @type {import('@rspack/core').RspackOptions}
 */
module.exports = {
	context: __dirname,
	entry: "./index.js",
	module: {
		rules: [
			{
				test: /lib\.js$/,
				use: [
					{
						loader: "./my-loader.js"
					}
				]
			}
		]
	}
};
