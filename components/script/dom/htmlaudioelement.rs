/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::HTMLAudioElementBinding;
use dom::bindings::codegen::InheritTypes::HTMLAudioElementDerived;
use dom::bindings::js::{JSRef, Temporary};
use dom::bindings::utils::{Reflectable, Reflector};
use dom::document::Document;
use dom::element::HTMLAudioElementTypeId;
use dom::eventtarget::{EventTarget, NodeTargetTypeId};
use dom::htmlmediaelement::HTMLMediaElement;
use dom::node::{Node, ElementNodeTypeId};
use servo_util::str::DOMString;

#[dom_struct]
pub struct HTMLAudioElement {
    htmlmediaelement: HTMLMediaElement
}

impl HTMLAudioElementDerived for EventTarget {
    fn is_htmlaudioelement(&self) -> bool {
        *self.type_id() == NodeTargetTypeId(ElementNodeTypeId(HTMLAudioElementTypeId))
    }
}

impl HTMLAudioElement {
    fn new_inherited(localName: DOMString, prefix: Option<DOMString>, document: JSRef<Document>) -> HTMLAudioElement {
        HTMLAudioElement {
            htmlmediaelement: HTMLMediaElement::new_inherited(HTMLAudioElementTypeId, localName, prefix, document)
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(localName: DOMString, prefix: Option<DOMString>, document: JSRef<Document>) -> Temporary<HTMLAudioElement> {
        let element = HTMLAudioElement::new_inherited(localName, prefix, document);
        Node::reflect_node(box element, document, HTMLAudioElementBinding::Wrap)
    }
}

impl Reflectable for HTMLAudioElement {
    fn reflector<'a>(&'a self) -> &'a Reflector {
        self.htmlmediaelement.reflector()
    }
}
