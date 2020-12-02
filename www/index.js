import * as wasm from "wasm-barcode-reader"

let constraints = {
    "video": {
    },
}

let video = document.querySelector("video")

let canvas1 = document.getElementById("c1")
let ctx1 = canvas1.getContext("2d")
let imageCapture;

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
        canvas1.width = imageBitmap.width;
        canvas1.height = imageBitmap.height;
        ctx1.drawImage(imageBitmap, 0, 0);

        let frame = ctx1.getImageData(0, 0, imageBitmap.width, imageBitmap.height)

        let c = wasm.detect(imageBitmap.width, imageBitmap.height, frame.data)
        if (c !== "") {
            console.log(c)
        }
        ctx1.putImageData(frame, 0, 0)

    }).catch(function(error) {
        console.log('grabFrame() error: ', error);
    });
}
