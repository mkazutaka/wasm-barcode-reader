import * as wasm from "wasm-barcode-reader"

let constraints = {
    "video": {
        // height:
        facingMode: { exact: "environment" }
    },
}

let video = document.getElementById("video")

// Canvas
let imageCapture;
let canvas = document.getElementById("c1")
let canvasCtx = canvas.getContext("2d")

// Result
let result = document.getElementById("result")
result.hidden = true
let barcode = document.getElementById("barcode")

navigator.mediaDevices.getUserMedia(constraints).then(stream => {
    video.srcObject = stream
    video.play()
    imageCapture = new ImageCapture(stream.getVideoTracks()[0])
    timerCallback()
}).catch(err => {
    console.log(err)
})

function timerCallback() {
    computeFrame()
    setTimeout(function () {
        timerCallback()
    }, 100)
}

function computeFrame() {
    imageCapture.grabFrame().then(function(imageBitmap) {
        let width = video.videoWidth;
        let height = video.videoHeight;
        canvas.width = width;
        canvas.height = height;
        canvasCtx.drawImage(imageBitmap, 0, 0);

        let frame = canvasCtx.getImageData(0, 0, width, height)

        let c = wasm.detect(imageBitmap.width, imageBitmap.height, frame.data)
        if (c !== "") {
            result.hidden = false
            barcode.innerText = c
            console.log(c)
        }
        canvasCtx.putImageData(frame, 0, 0)

    }).catch(function(error) {
        console.log('grabFrame() error: ', error);
    });
}
