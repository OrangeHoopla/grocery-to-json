use image::{DynamicImage, ImageDecoder, ImageReader};
use distance::*;

use grocery_to_json::image_processors::{self, tesseract_processor};

fn main() {
    // Run registered benchmarks.
    divan::main();

    // right now the bench mark for performance is being used against the 
    // iphones images scanning capability of the reciept
    // using the Levenshtein distance to determine the performance
    // tldr: lower number = better
}

// Register a `fibonacci` function and benchmark it over multiple cases.
#[divan::bench(args = [1, 2, 4, 8, 16, 32])]
fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
}


#[divan::bench(sample_size = 1,sample_count=1)]
fn standard(){

        let result = tesseract_processor::result("../images/sample.jpg");
        let distance = levenshtein(&result,IPHONE_ANSWER);
        println!("levenshtein distance: {}", distance);
    }

const IPHONE_ANSWER: &str = r#"
ALDI
Store #15
801 H ST NE
washington, DC
https://help.aldi.us
Your cashier today was William
383804 Org Tenderloin
384651 Pasture Raised Egg
382260 Plain NF Greek Yog
201364 Organic Creamy PB
382589 Sliced Sourdough
634778 Org Roma Tomatoes
356574 Org. Yellow Potato
356454 Organic Grape Tom
365992 Tortilla Chips
AMEX
***************3786 OTHER
08/10/25 10:02 Ref/Sea # 657558
Trace # 657558
Auth # 804385
AID A000000025010901
TVR 0000000000
IAD 06030103A20102
TSI A800
ARC 000
EntryMode 07
++APPROVED++
8.60 FA
4
. 85 FA
3.19 FA
3.79 FA
3.29 FA
2.39 FA
3.99 FA
2.79 FA
1.85 FA
34.74
SUBTOTAL
A-Taxable @0.00%
AMOUNT DUE
T
34.74
0.00
34.74
95 34-14
9 ITEMS
Credit Card
$ 34.74
*2285 FJ13/004/012 08/10/25 10:02AM
******
*****************************
Like ALDI? Tell ALDI!
Tell us how we did at www.tellaldi.us
Enter the drawing for a chance to win a $100 ALDI gift card.
Must be 18 years old to enter.
No purchase necessary.
Sign up for ALDI emails
for a sneak peek on the weekly ad!
www.aldi.us/signup
"#;