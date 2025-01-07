import init, {greet} from '../pkg/wasmfusion_pdf.js';

async function run () {
    await init();
    document.querySelector(".text").textContent = greet("Frank")
}
run()