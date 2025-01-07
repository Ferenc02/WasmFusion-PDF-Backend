use wasm_bindgen::prelude::*;

use lopdf::dictionary;
use lopdf::{Document, Object, Stream };
use lopdf::content::{Content, Operation};







#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn create_document() -> Vec<u8>{
    //Specify which PDF version the document should compile with
    let mut doc = Document::with_version("1.5");

    //Pages is the root node of the page tree
    let pages_id = doc.new_object_id();

    //Fonts are dictionaries.

    let font_id = doc.add_object(dictionary! {
        //Type of dictionary
        "Type" => "Font",
        //Type of font, type1 iis simple postscript font
        "Subtype" => "Type1",
        //Basefont is postscript name of font for type1 font
        "BaseFont" => "Courier",
    });

    //Used for adding the font dictionary to the resource

    let resources_id = doc.add_object(dictionary! {
        "Font" => dictionary! {
            "F1" => font_id,
        }
    });

    //Content with the wrapper which contains a vector of operations.

    let content = Content {
        operations: vec![
            //Text element
            Operation::new("BT", vec![]),

            //Give font size and font

            Operation::new("Tf", vec!["F1".into(), 32.into()]),

            //Put the font on the document. Y=0 bottom Y=600 almost at the top

            Operation::new("Td", vec![0.into(), 600.into()]),

            //Print the text to the document

            Operation::new("Tj", vec![Object::string_literal("This is a write test to a pdf")]),

            //ET end the text element

            Operation::new("ET", vec![]),
        ],
    };

    //Add to content
    let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));

    //Page is a dictionary that represents one page of a PDF file

    let page_id = doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content_id,
    });

    //Put the pages into the pdf

    let pages = dictionary!{
        "Type" => "Pages",
        "Kids" => vec![page_id.into()],
        "Count" => 1,
        "Resources" => resources_id,
        //Page size
        "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
    };

    //Insert everything to the doc

    doc.objects.insert(pages_id, Object::Dictionary(pages));

    //Create the document catalog

    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });

    //Compress the document and set the id to the document catalog
    doc.trailer.set("Root", catalog_id);
    doc.compress();

    //Store file in current directory


    let mut buffer = Vec::new();
    doc.save_to(&mut buffer).unwrap();

    buffer


    //doc.save("example.pdf").unwrap();


    //println!("{:#?}", doc);
}
