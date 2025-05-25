use std::thread;

fn main() {
    let urls = vec![
        "https://botanicusboutique.com.au/products/birthday-wishes-gift-card",
        "https://www.bottrellaccounting.com.au/roster-template-2/",
        "https://bornprimitive.com.au/products/tidal-bikini-bottom-summer-fields",
        "https://www.bosshardmedical.com.au/shop/bedroom/accessories/manchester/",
        "https://bottega1995.com.au/product/fourmage-gouda/",
        "https://bostonclothing.com.au/products/pink-clover-paisley-print-silk-pocket-square",
        "https://us.boteh.com.au/collections/ss22-la-esencia/products/la-ponche-minimal-short-1",
        "https://botanex.com.au/products/haws-childrens-watering-can",
        "https://borderchronicle.com.au/sport/2022/01/14/report-from-the-dolphins-amateur-swimming-club/",
        "https://borderwatch.com.au/all-digital-editions/tbw-border-watch-friday-11th-april-2025/",
    ];

    let mut handles = vec![];

    for url in urls.into_iter() {
        handles.push(thread::spawn(move || ureq::get(url).call().unwrap()));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
