use leptos::document;

use self::dom_position::DomPosition;
pub mod dom_position;

pub(crate) fn find_element_pos(element_id: impl AsRef<str>) -> Option<DomPosition> {
    let element = document().get_element_by_id(element_id.as_ref())?;
    let dom_rect = element.get_bounding_client_rect();
    let (top, left, bottom, right) = (
        dom_rect.top(),
        dom_rect.left(),
        dom_rect.bottom(),
        dom_rect.right(),
    );
    Some(DomPosition::new(top, left, bottom, right))
}
