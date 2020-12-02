import * as wasm from "wasm-barcode-reader"

const target = document.getElementById('target');
target.addEventListener('change', handleImage, false);
const rerun = document.getElementById('rerun');
rerun.addEventListener('click', reRunBench, false);
let canvas = document.getElementById('imgCanvas');
let ctx = canvas.getContext('2d');
let img = new Image();

function handleImage(e){
    let reader = new FileReader();
    reader.onload = function(event){
        img.onload = function(){
            canvas.width = img.width;
            canvas.height = img.height;
            ctx.drawImage(img,0,0);
            runBench()
        }
        img.src = event.target.result;
    }
    reader.readAsDataURL(e.target.files[0]);
}

function reRunBench() {
    runBench()
}

function runBench() {
    let frame = ctx.getImageData(0, 0, canvas.width, canvas.height)
    runMyApp(frame)
    runQuagga(img)
}

function runMyApp(image) {
    let t =  timer("MyApp");
    let c = wasm.detect(image.width, image.height, image.data)
    if (c !== "") {
        t.stop(c)
        return
    }
    t.stop("not detected")
}

// https://github.com/ericblade/quagga2
function runQuagga(image) {
    let t =  timer("Quagga");
    Quagga.decodeSingle({
        src: image.src,// The image doesn't seem matter, I used a sample image from the repo
        numOfWorkers: 0,  // Needs to be 0 when used within node
        inputStream: {
            width: image.width,
            height: image.height,
        },
        decoder: {
            readers: ["ean_reader"] // List of active readers
        },
    }, function(result) {
        if(result.codeResult) {
            t.stop(result.codeResult.code)
            return
        }
        t.stop("not detected")
    });
}

let timer = function(name) {
    let start = new Date();
    return {
        stop: function(result) {
            let end  = new Date();
            let time = end.getTime() - start.getTime();
            console.log('Timer:', name, 'finished in', time, 'ms.', 'result:', result);
        }
    }
};