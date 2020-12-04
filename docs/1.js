(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[1],{

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var wasm_barcode_reader__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! wasm-barcode-reader */ \"../pkg/wasm_barcode_reader.js\");\n\n\nlet constraints = {\n    \"video\": {\n    },\n}\n\nlet video = document.querySelector(\"video\")\n\nlet canvas1 = document.getElementById(\"c1\")\nlet ctx1 = canvas1.getContext(\"2d\")\nlet imageCapture;\n\nnavigator.mediaDevices.getUserMedia(constraints).then(stream => {\n    video.srcObject = stream\n    video.play()\n    imageCapture = new ImageCapture(stream.getVideoTracks()[0])\n    timerCallback()\n}).catch(err => {\n    console.log(err)\n})\n\nfunction timerCallback() {\n    computeFrame()\n    setTimeout(function () {\n        timerCallback()\n    }, 100)\n}\n\nfunction computeFrame() {\n    imageCapture.grabFrame().then(function(imageBitmap) {\n        canvas1.width = imageBitmap.width;\n        canvas1.height = imageBitmap.height;\n        ctx1.drawImage(imageBitmap, 0, 0);\n\n        let frame = ctx1.getImageData(0, 0, imageBitmap.width, imageBitmap.height)\n\n        let c = wasm_barcode_reader__WEBPACK_IMPORTED_MODULE_0__[\"detect\"](imageBitmap.width, imageBitmap.height, frame.data)\n        if (c !== \"\") {\n            console.log(c)\n        }\n        ctx1.putImageData(frame, 0, 0)\n\n    }).catch(function(error) {\n        console.log('grabFrame() error: ', error);\n    });\n}\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ })

}]);