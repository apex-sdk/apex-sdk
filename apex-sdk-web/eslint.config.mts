import js from "@eslint/js";
import globals from "globals";

export default [
  js.configs.recommended,
  {
    files: ["**/*.{js,mjs,cjs,ts,tsx,jsx}"],
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node,
        React: "readonly",
        JSX: "readonly"
      }
    },
    rules: {
      "no-unused-vars": "warn",
      "no-console": "warn"
    }
  },
  {
    ignores: [
      ".next/**",
      "out/**",
      "build/**",
      "node_modules/**", 
      "*.config.js",
      "*.config.mjs",
      "*.config.ts"
    ]
  }
];
