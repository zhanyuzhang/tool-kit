module.exports = {
  "root": true,
  "globals": {
    "ds": true
  },
  "env": {
    "es6": true,
    "browser": true
  },
  "parser": "babel-eslint",
  "parserOptions": {
    "sourceType": "module",
    "ecmaVersion": 2017
  },
  "extends": [
    "eslint:recommended",
    "google",
    "plugin:prettier/recommended"
  ],
  "rules": {
    "prettier/prettier": ["error", {
      "singleQuote": true
    }],
    "require-jsdoc": 0,
    "no-console": 0
  }
}