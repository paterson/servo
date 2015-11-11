/* This Source Code Form is subject to the terms of the Mozilla Public
* License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::attr::Attr;
use dom::attr::AttrValue;
use dom::bindings::cell::DOMRefCell;
use dom::bindings::codegen::Bindings::HTMLImageElementBinding;
use dom::bindings::codegen::Bindings::HTMLImageElementBinding::HTMLImageElementMethods;
use dom::bindings::codegen::Bindings::WindowBinding::WindowMethods;
use dom::bindings::codegen::InheritTypes::{ElementCast, EventTargetCast, NodeCast};
use dom::bindings::codegen::InheritTypes::{HTMLElementCast, HTMLImageElementDerived};
use dom::bindings::error::Fallible;
use dom::bindings::js::{LayoutJS, Root};
use dom::bindings::refcounted::Trusted;
use dom::document::Document;
use dom::element::{AttributeMutation, ElementTypeId};
use dom::event::{Event, EventBubbles, EventCancelable};
use dom::eventtarget::{EventTarget, EventTargetTypeId};
use dom::htmlelement::{HTMLElement, HTMLElementTypeId};
use dom::node::{Node, NodeDamage, NodeTypeId, document_from_node, window_from_node};
use dom::virtualmethods::VirtualMethods;
use ipc_channel::ipc;
use ipc_channel::router::ROUTER;
use net_traits::image::base::Image;
use net_traits::image_cache_task::{ImageResponder, ImageResponse};
use script_task::ScriptTaskEventCategory::UpdateReplacedElement;
use script_task::{CommonScriptMsg, Runnable, ScriptChan};
use std::borrow::ToOwned;
use std::sync::Arc;
use string_cache::Atom;
use url::{Url, UrlParser};
use util::str::DOMString;
use dom::imagedata::ImageData;
use dom::bindings::global::GlobalRef;


// Unavailable
//     The user agent hasn't obtained any image data, or has obtained some or all of the 
//     image data but hasn't yet decoded enough of the image to get the image dimensions.
// Partially available
//     The user agent has obtained some of the image data and at least the image dimensions are available.
// Completely available
//     The user agent has obtained all of the image data and at least the image dimensions are available.
// Broken
//     The user agent has obtained all of the image data that it can, but it cannot even 
//     decode the image enough to get the image dimensions (e.g. the image is corrupted, or the format 
//         is not supported, or no data could be obtained).

#[must_root]
#[derive(JSTraceable, Clone, HeapSizeOf)]
pub enum ImageState {
   Unavailable,
   PartiallyAvailable,
   CompletelyAvailable,
   Broken,
}

#[dom_struct]
pub struct ImageRequest {
   state : ImageState,
   currentUrl : Url,
   imageData : ImageData,
}


impl ImageRequest {
   fn new(imageData : ImageData, currentUrl : Url) -> ImageRequest {
       ImageRequest {
           state : ImageState { Unavailable },
           currentUrl : currentUrl,
           imageData : imageData,
       }
   }
}


impl ImageRequestMethods for ImageRequest {
   
   fn State(&self) -> ImageState {
       self.state
   }

   fn CurrentUrl(&self) -> String {
       self.currentUrl
   }

   fn ImageData(&self) -> ImageData {
       self.imageData
   }

   fn SetStateToUnavailable() {
       self.state = ImageState { Unavailable }
   }

   fn SetStateToPartiallyAvailable() {
       self.state = ImageState { PartiallyAvailable }
   }

   fn SetStateToCompletelyAvailable() {
       self.state = ImageState { CompletelyAvailable }
   }

   fn SetStateToBroken() {
       self.state = ImageState { Broken }
   }

   fn SetCurrentUrl(newUrl : String) {
       self.currentUrl = newUrl
   }
}