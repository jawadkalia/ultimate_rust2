// 1. Thank you for volunteering to document our pumpkin library! Let's start by grabbing the first
// paragraph from https://en.wikipedia.org/wiki/Pumpkin and pasting it as our module-level
// documentation. Hint: Use inner-documentation comments.
//
// Once you've got the documentation here, run `cargo doc --no-deps --open` and take a look!

/*!
A pumpkin is a cultivar of winter squash that is round with smooth, slightly ribbed skin, and is most often deep yellow to orange in coloration. The thick shell contains the seeds and pulp. The name is most commonly used for cultivars of Cucurbita pepo, but some cultivars of Cucurbita maxima, C. argyrosperma, and C. moschata with similar appearance are also sometimes called "pumpkins".

Native to North America (northeastern Mexico and the southern United States), pumpkins are one of the oldest domesticated plants, having been used as early as 7,000 to 5,500 BC. Pumpkins are widely grown for food, as well as for aesthetic and recreational purposes. Pumpkin pie, for instance, is a traditional part of Thanksgiving meals in Canada and the United States, and pumpkins are frequently carved as jack-o'-lanterns for decoration around Halloween, although commercially canned pumpkin purée and pumpkin pie fillings are usually made from varieties of winter squash different from the ones used for jack-o'-lanterns.
!*/

// 2. What about an image!? Add an image of a pumpkin to the end of the module-level documentation.
// The markdown format is ![some alt text](https://url-to-the-image.png)
// Here's the image to link to: https://upload.wikimedia.org/wikipedia/commons/thumb/5/5c/FrenchMarketPumpkinsB.jpg/700px-FrenchMarketPumpkinsB.jpg

// 3. Document the Pumpkin struct.
// - The description on the index page should be "Big orange thing"
// - Make a section header called "Recipes"
// - Explain that recipes will be coming soon.
// - Document the "roundness" field, explaining that it is a percentage
// - Document the "orangeness" field, explaining that it is a number from 8 to 27
pub mod check;
pub struct Pumpkin {
    pub roundness: f32,
    pub orangeness: i32,
}

// 4. Document the "smash" method. Explain that if you smash the pumpkin, it will be gone. Then it
// can't be used for pie. :'-(

impl Pumpkin {
    pub fn smash(self) {}
}

// 5. Document that BURNT_ORANGE is for the "orangeness" field in the Pumpkin struct.
// - Link to the Pumpkin struct in your description

pub const BURNT_ORANGE: i32 = 13;

// Challenge: Find the option to pass to `cargo doc` so that documentation for this private item
// gets generated as well.  Hint: `cargo doc -h` will show you all the relevant options.

/// For internal use only. In fact, this documentation is so private that it won't be generated.
/// At least not by default. But if you pass the correct option in, it will magically appear!
#[allow(dead_code)] // to silence the warning
enum PrivateEnum {
    /// For Halloween. To be lit by candlelight.
    JackOLantern,
    /// For dessert during North American winter holidays.
    PumpkinPie,
}
