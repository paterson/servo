/* This Source Code Form is subject to the terms of the Mozilla Public
* License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */


use url::Url;
use dom::imagedata::ImageData;


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
