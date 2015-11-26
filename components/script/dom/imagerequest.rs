/* This Source Code Form is subject to the terms of the Mozilla Public
* License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */


use dom::url::URL;
use dom::bindings::global::GlobalRef;
use dom::imagedata::ImageData;
use dom::bindings::codegen::Bindings::ImageDataBinding::ImageDataMethods;
use dom::bindings::reflector::{Reflector, reflect_dom_object};
use dom::bindings::codegen::Bindings::ImageRequestBinding;
use dom::bindings::codegen::Bindings::ImageRequestBinding::ImageRequestMethods;

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

#[dom_struct]
pub struct ImageRequest {
  reflector_: Reflector,
  state : ImageState,
  currentUrl : URL,
  imageData : ImageData,
}

//#[derive(JSTraceable, Clone, HeapSizeOf)]
pub enum ImageState {
   Unavailable,
   PartiallyAvailable,
   CompletelyAvailable,
   Broken,
}

impl ImageRequest {
   fn new(global: GlobalRef, imageData : ImageData, currentUrl : DOMString) -> ImageRequest {
       ImageRequest {
          reflector_: Reflector::new(),
          state : ImageState::Unavailable,
          currentUrl : currentUrl,
          imageData : imageData,
       }
   }
}


impl ImageRequestMethods for ImageRequest {
   
   fn State(&self) -> ImageState {
       self.state
   }

   fn CurrentUrl(&self) -> URL {
       self.currentUrl
   }

   fn ImageData(&self) -> ImageData {
       self.imageData
   }
}
