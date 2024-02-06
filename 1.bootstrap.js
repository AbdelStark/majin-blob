(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[1],{

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var majin_blob_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! majin-blob-wasm */ \"./node_modules/majin-blob-wasm/majin_blob_wasm.js\");\n\n\ndocument\n  .getElementById(\"blobForm\")\n  .addEventListener(\"submit\", async function (event) {\n    event.preventDefault(); // Prevent the form from submitting in the traditional way\n\n    const fileInput = document.getElementById(\"blobFileInput\");\n    const textInput = document.getElementById(\"blobTextInput\");\n    let blob;\n\n    if (fileInput.files.length > 0) {\n      // Read the blob from the file\n      const file = fileInput.files[0];\n      blob = await file.text();\n    } else {\n      // Use the manually entered blob data\n      blob = textInput.value;\n    }\n    const state_diffs = majin_blob_wasm__WEBPACK_IMPORTED_MODULE_0__[\"blob_recover\"](blob);\n\n    // Format the JSON string to be pretty\n    const formattedStateDiffs = JSON.stringify(\n      JSON.parse(state_diffs),\n      null,\n      2,\n    );\n\n    document.getElementById(\"stateDiffsOutput\").textContent =\n      formattedStateDiffs;\n  });\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ })

}]);