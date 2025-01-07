import init, {greet,create_document} from '../pkg/wasmfusion_pdf.js';





async function run () {
    await init();
    document.querySelector(".text").textContent = greet("Frank")
}





async function submitButton () {
    await init();

    try {
        //Call the function
        const pdfData = create_document();

        //Create a blob from returned buffer
        const blob = new Blob([pdfData], {type: 'application/pdf'});

        //Create a link
        let link = document.querySelector('a');
        link.href = URL.createObjectURL(blob);
        link.download = "test.pdf";
        link.click();

        //Clean up the url
        URL.revokeObjectURL(link.href)
    }
    catch (e){
        console.error("Error creating PDF: ", e)
    }
}



document.querySelector("#button").addEventListener("click",submitButton)