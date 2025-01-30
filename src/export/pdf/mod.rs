use crate::{errors::api_error::ApiError, models::ticket::TicketView};
use lopdf::{
    content::{Content, Operation},
    dictionary, {Document, Object, Stream},
};

pub async fn create_ticket_pdf(ticket: TicketView) -> Result<Vec<u8>, ApiError> {
    let mut doc = Document::with_version("1.5");

    let pages_id = doc.new_object_id();

    let font_id_arial = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Arial",
    });

    let font_id_arial_bold = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Arial-Bold",
    });

    let resources_id = doc.add_object(dictionary! {
        "Font" => dictionary! {
            "F1" => font_id_arial,
            "F2" => font_id_arial_bold,
        },
    });

    let content = Content {
        operations: vec![
            // Updated_at label
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F2".into(), 10.into()]),
            Operation::new("Td", vec![400.into(), 820.into()]),
            Operation::new("Tj", vec![Object::string_literal("Updated at:")]),
            Operation::new("ET", vec![]),
            // Updated_at value
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F1".into(), 10.into()]),
            Operation::new("Td", vec![460.into(), 820.into()]),
            Operation::new("Tj", vec![Object::string_literal(ticket.updated_at)]),
            Operation::new("ET", vec![]),
            // Ticket_number label
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F1".into(), 16.into()]),
            Operation::new("Td", vec![50.into(), 785.into()]),
            Operation::new("Tj", vec![Object::string_literal("Ticket")]),
            Operation::new("ET", vec![]),
            // Ticket_number value
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F2".into(), 17.into()]),
            Operation::new("Td", vec![100.into(), 785.into()]),
            Operation::new("Tj", vec![Object::string_literal(ticket.id)]),
            Operation::new("ET", vec![]),
            // Requester label
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F2".into(), 12.into()]),
            Operation::new("Td", vec![50.into(), 750.into()]),
            Operation::new("Tj", vec![Object::string_literal("Requester:")]),
            Operation::new("ET", vec![]),
            // Requester value
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F1".into(), 12.into()]),
            Operation::new("Td", vec![120.into(), 750.into()]),
            Operation::new("Tj", vec![Object::string_literal(ticket.requester)]),
            Operation::new("ET", vec![]),
            // Created_at label
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F2".into(), 12.into()]),
            Operation::new("Td", vec![50.into(), 700.into()]),
            Operation::new("Tj", vec![Object::string_literal("Created at:")]),
            Operation::new("ET", vec![]),
            // Created_at value
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F1".into(), 12.into()]),
            Operation::new("Td", vec![120.into(), 700.into()]),
            Operation::new("Tj", vec![Object::string_literal(ticket.created_at)]),
            Operation::new("ET", vec![]),
            // Status label
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F2".into(), 12.into()]),
            Operation::new("Td", vec![375.into(), 750.into()]),
            Operation::new("Tj", vec![Object::string_literal("Status:")]),
            Operation::new("ET", vec![]),
            // Status value
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F1".into(), 12.into()]),
            Operation::new("Td", vec![420.into(), 750.into()]),
            Operation::new("Tj", vec![Object::string_literal(ticket.status)]),
            Operation::new("ET", vec![]),
            // Title label
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F2".into(), 12.into()]),
            Operation::new("Td", vec![50.into(), 720.into()]),
            Operation::new("Tj", vec![Object::string_literal("Title:")]),
            Operation::new("ET", vec![]),
            // Title value
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F1".into(), 12.into()]),
            Operation::new("Td", vec![86.into(), 720.into()]),
            Operation::new("Tj", vec![Object::string_literal(ticket.title)]),
            Operation::new("ET", vec![]),
            // Description label
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F2".into(), 12.into()]),
            Operation::new("Td", vec![50.into(), 675.into()]),
            Operation::new("Tj", vec![Object::string_literal("Description:")]),
            Operation::new("ET", vec![]),
            // Description value
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F1".into(), 11.into()]),
            Operation::new("Td", vec![50.into(), 660.into()]),
            Operation::new("Tj", vec![Object::string_literal(ticket.description)]),
            Operation::new("ET", vec![]),
            // Closed_by label
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F2".into(), 12.into()]),
            Operation::new("Td", vec![50.into(), 560.into()]),
            Operation::new("Tj", vec![Object::string_literal("Closed by:")]),
            Operation::new("ET", vec![]),
            // Closed_by value
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F1".into(), 12.into()]),
            Operation::new("Td", vec![120.into(), 560.into()]),
            Operation::new("Tj", vec![Object::string_literal(ticket.closed_by)]),
            Operation::new("ET", vec![]),
            // Closed_at label
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F2".into(), 12.into()]),
            Operation::new("Td", vec![375.into(), 560.into()]),
            Operation::new("Tj", vec![Object::string_literal("Closed at:")]),
            Operation::new("ET", vec![]),
            // Closed_at value
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F1".into(), 12.into()]),
            Operation::new("Td", vec![440.into(), 560.into()]),
            Operation::new("Tj", vec![Object::string_literal(ticket.closed_at)]),
            Operation::new("ET", vec![]),
            // Solution label
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F2".into(), 12.into()]),
            Operation::new("Td", vec![50.into(), 540.into()]),
            Operation::new("Tj", vec![Object::string_literal("Solution:")]),
            Operation::new("ET", vec![]),
            // Solution value
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F1".into(), 11.into()]),
            Operation::new("Td", vec![110.into(), 540.into()]),
            Operation::new("Tj", vec![Object::string_literal(ticket.solution)]),
            Operation::new("ET", vec![]),
        ],
    };

    let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode()?));

    let page_id = doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content_id,
    });

    let pages = dictionary! {
        "Type" => "Pages",
        "Kids" => vec![page_id.into()],
        "Count" => 1,
        "Resources" => resources_id,
        "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
    };

    doc.objects.insert(pages_id, Object::Dictionary(pages));

    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });

    doc.trailer.set("Root", catalog_id);
    doc.compress();

    let mut pdf = Vec::new();
    doc.save_to(&mut pdf).unwrap();

    Ok(pdf)
}
