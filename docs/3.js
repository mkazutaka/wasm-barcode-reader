(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[3],{

/***/ "./src/demo/demo.js":
/*!**************************!*\
  !*** ./src/demo/demo.js ***!
  \**************************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var wasm_barcode_reader__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! wasm-barcode-reader */ \"../pkg/wasm_barcode_reader.js\");\n\n\nlet constraints = {\n    \"video\": {\n        // height:\n        facingMode: { exact: \"environment\" }\n    },\n}\n\nlet video = document.getElementById(\"video\")\n\n// Canvas\nlet imageCapture;\nlet canvas = document.getElementById(\"c1\")\nlet canvasCtx = canvas.getContext(\"2d\")\n\n// Result\nlet result = document.getElementById(\"result\")\nresult.hidden = true\nlet barcode = document.getElementById(\"barcode\")\n\nnavigator.mediaDevices.getUserMedia(constraints).then(stream => {\n    video.srcObject = stream\n    video.play()\n    imageCapture = new ImageCapture(stream.getVideoTracks()[0])\n    timerCallback()\n}).catch(err => {\n    console.log(err)\n})\n\nfunction timerCallback() {\n    computeFrame()\n    setTimeout(function () {\n        timerCallback()\n    }, 100)\n}\n\nfunction computeFrame() {\n    imageCapture.grabFrame().then(function(imageBitmap) {\n        let width = video.videoWidth;\n        let height = video.videoHeight;\n        canvas.width = width;\n        canvas.height = height;\n        canvasCtx.drawImage(imageBitmap, 0, 0);\n\n        let frame = canvasCtx.getImageData(0, 0, width, height)\n\n        let c = wasm_barcode_reader__WEBPACK_IMPORTED_MODULE_0__[\"detect\"](imageBitmap.width, imageBitmap.height, frame.data)\n        if (c !== \"\") {\n            result.hidden = false\n            barcode.innerText = c\n            console.log(c)\n        }\n        canvasCtx.putImageData(frame, 0, 0)\n\n    }).catch(function(error) {\n        console.log('grabFrame() error: ', error);\n    });\n}\n\n\n//# sourceURL=webpack:///./src/demo/demo.js?");

/***/ })

}]);