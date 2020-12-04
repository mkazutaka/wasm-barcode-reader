(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[2],{

/***/ "./src/bench/bench.js":
/*!****************************!*\
  !*** ./src/bench/bench.js ***!
  \****************************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var wasm_barcode_reader__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! wasm-barcode-reader */ \"../pkg/wasm_barcode_reader.js\");\n\n\nconst target = document.getElementById('target');\ntarget.addEventListener('change', handleImage, false);\nconst rerun = document.getElementById('rerun');\nrerun.addEventListener('click', reRunBench, false);\nlet canvas = document.getElementById('imgCanvas');\nlet ctx = canvas.getContext('2d');\nlet img = new Image();\n\nfunction handleImage(e){\n    let reader = new FileReader();\n    reader.onload = function(event){\n        img.onload = function(){\n            canvas.width = img.width;\n            canvas.height = img.height;\n            ctx.drawImage(img,0,0);\n            runBench()\n        }\n        img.src = event.target.result;\n    }\n    reader.readAsDataURL(e.target.files[0]);\n}\n\nfunction reRunBench() {\n    runBench()\n}\n\nfunction runBench() {\n    let frame = ctx.getImageData(0, 0, canvas.width, canvas.height)\n    runMyApp(frame)\n    runQuagga(img)\n}\n\nfunction runMyApp(image) {\n    let t =  timer(\"MyApp\");\n    let c = wasm_barcode_reader__WEBPACK_IMPORTED_MODULE_0__[\"detect\"](image.width, image.height, image.data)\n    if (c !== \"\") {\n        t.stop(c)\n        return\n    }\n    t.stop(\"not detected\")\n}\n\n// https://github.com/ericblade/quagga2\nfunction runQuagga(image) {\n    let t =  timer(\"Quagga\");\n    Quagga.decodeSingle({\n        src: image.src,// The image doesn't seem matter, I used a sample image from the repo\n        numOfWorkers: 0,  // Needs to be 0 when used within node\n        inputStream: {\n            width: image.width,\n            height: image.height,\n        },\n        decoder: {\n            readers: [\"ean_reader\"] // List of active readers\n        },\n    }, function(result) {\n        if(result.codeResult) {\n            t.stop(result.codeResult.code)\n            return\n        }\n        t.stop(\"not detected\")\n    });\n}\n\nlet timer = function(name) {\n    let start = new Date();\n    return {\n        stop: function(result) {\n            let end  = new Date();\n            let time = end.getTime() - start.getTime();\n            console.log('Timer:', name, 'finished in', time, 'ms.', 'result:', result);\n        }\n    }\n};\n\n//# sourceURL=webpack:///./src/bench/bench.js?");

/***/ })

}]);