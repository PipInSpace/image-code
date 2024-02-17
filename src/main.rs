use image::io::Reader as ImgReader;

mod rand;

const STARTING_POS: (usize, usize) = (4, 4);
const BITS_PER_CELL: u8 = 3; // Max pixel baseline diff: 7
const DATA: &str = "Dolores amet minima autem doloremque perferendis. Et exercitationem nulla repellendus nobis voluptatem. Soluta iusto consequatur autem. Similique asperiores porro natus et. Pariatur amet eum corrupti alias et expedita provident placeat. Facere ipsam incidunt nostrum delectus et tempora et corrupti.

Et tenetur vero quis tempore quibusdam. Fuga facilis asperiores placeat rem recusandae hic velit. Error expedita beatae odio veritatis et saepe soluta.

Excepturi quia beatae sed veniam quia. In qui aut optio suscipit laudantium. Maxime nam voluptatem velit incidunt alias possimus dolor laudantium. Iste mollitia ex consectetur ut et. Ab dolores excdasdepdturi ullam omnis tenetur non eveniet dolorum.

Id illo aut cumque consectetur dolores. Quod quis sunt nihil quas et. Cupiditate omnis doloremque vero optio eos et temporibus. Ipsum asperiores repellendus voluptas qui veritatis. Dolore non sunt labore. Commodi voluptatem quae qui et repellendus in odit dolores.

Eum sed eum voluptatibus at rem sunt doloremque aut. Et dolores vel nihil a et. Voluptatem molestiae molestiae laboriosam et placeat nobis sequi eum. Ab itaque ea qui similique vitae. Culpa possimus atque et perferendis aperiam.
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please specify a file");
        std::process::exit(0)
    }

    encode(&args);
    decode(&args);
}

fn decode(args: &Vec<String>) {
    let img = ImgReader::open("ion_enc.png").expect(" ").decode().unwrap().into_rgb8();
    let img_bytes = img.clone().into_raw();
    let dims = (img.width() as usize, img.height() as usize);
    let xs = dims.0;
    let bytes_per_px = img_bytes.len() / (dims.0 * dims.1);
    println!("Image dims: {:?}\nBuffer len: {}\nBytes per pixel: {}", dims, img_bytes.len(), bytes_per_px);

    let mut bitstream: Vec<bool> = vec![];

    let mut pxs = img_bytes.get_pixels(dims, STARTING_POS, xs);
    let mut rgb_d: [i32; 3] = u8_3_to_i8_3(pxs[0]);
    let mut cur_pos: (usize, usize) = STARTING_POS;
    let mut i = 0;
    while !(rgb_d[1] == 0 && rgb_d[2] == 0) {
        rgb_d = difference(pxs);
        if rgb_d[1] == 0 && rgb_d[2] == 0 {
            // Termination pixel, break
            break;
        }
        //println!("RGB baseline diff: {:?} at {:?}", rgb_d, cur_pos);
        cur_pos = wrapping_coords(cur_pos, dims, (rgb_d[1], rgb_d[2]));
        pxs = img_bytes.get_pixels(dims, cur_pos, xs);

        for i in 0..BITS_PER_CELL {
            bitstream.push((rgb_d[0] & 4>>i) != 0);
        }
    }
    //println!("Ended sequence at: {:?}", cur_pos);
    //println!("Bit stream: {:?}", bitstream);
    let bytes: Vec<u8> = bitstream_to_bytes(bitstream);
    let string = String::from_utf8_lossy(&bytes);
    println!("Data string: \"{}\"", string)
}

fn encode(args: &Vec<String>) {
    let img = ImgReader::open(&args[1]).expect(" ").decode().unwrap().into_rgb8();
    let mut img_bytes = img.clone().into_raw();
    let dims = (img.width() as usize, img.height() as usize);
    let xs = dims.0;
    let bytes_per_px = img_bytes.len() / (dims.0 * dims.1);
    println!("Image dims: {:?}\nBuffer len: {}\nBytes per pixel: {}", dims, img_bytes.len(), bytes_per_px);

    let bytes: Vec<u8> = DATA.to_string().as_bytes().to_vec();
    let mut bitstream: Vec<bool> = bytes_to_bitstream(&bytes);
    //println!("Data \"{:?}\" as bitstream: {:?}", bytes, bitstream);

    
    let mut cur_pos: (usize, usize) = STARTING_POS;
    let mut rng = rand::RandU64::new(Some(0));
    let mut blocked_px: Vec<bool> = vec![false; dims.0*dims.1];

    while bitstream.len() > 0 {
        // Prep data channel
        let mut bits: Vec<bool> = vec![];
        for _ in 0..BITS_PER_CELL {
            if bitstream.len() > 0 {
                bits.push(bitstream.remove(0));
            }
        }
        let mut px_bits = 0;
        for (i, bit) in bits.iter().enumerate() {
            px_bits += (*bit as u8) << (BITS_PER_CELL-1-i as u8);
        }

        // Set new pixel
        let pxs = img_bytes.get_pixels(dims, cur_pos, xs);
        let baseline = get_baseline(pxs);
        let mut px_r = baseline[0];
        if (px_r as u32 + px_bits as u32) > 255 {
            px_r -= px_bits-1;
        } else {
            px_r += px_bits;
        }
        let mut px_g = baseline[1];
        let mut px_b = baseline[2];
        let mut px_g_off = 0;
        let mut px_b_off = 0;
        let mut valid_px = false;
        while !valid_px {
            px_g = baseline[1];
            px_b = baseline[2];
            let (r1, r2) = (rng.next() as u8 & 0b_0000_0111, rng.next() as u8 & 0b_0000_0111);
            //println!("{:#05b}, {:#05b}", r1, r2);
            px_g_off = std::cmp::max(r1, 1) as i32;
            px_b_off = std::cmp::max(r2, 1) as i32;
            if (px_g as u32 + px_g_off as u32) > 255 {
                px_g -= px_g_off as u8;
                px_g_off = -px_g_off;
            } else {
                px_g += px_g_off as u8;
            }
            if (px_b as u32 + px_b_off as u32) > 255 {
                px_b -= px_b_off as u8;
                px_b_off = -px_b_off;
            } else {
                px_b += px_b_off as u8;
            }
            valid_px = check_diff_validity(cur_pos, dims, (px_g_off, px_b_off), xs, &mut blocked_px)
        }
        
        
        img_bytes.set_pixel([px_r, px_g, px_b], cur_pos, xs);

        //println!("Set pixel at {:?} to {}", cur_pos, (px_r as i32 - baseline[0] as i32));
        cur_pos = wrapping_coords(cur_pos, dims, (px_g_off as i32, px_b_off as i32));
        //println!("new pixel at {:?}", cur_pos)
    }

    let pxs = img_bytes.get_pixels(dims, cur_pos, xs);
    let baseline = get_baseline(pxs);
    img_bytes.set_pixel(baseline, cur_pos, xs);

    let img = image::RgbImage::from_raw(dims.0 as u32, dims.1 as u32, img_bytes).unwrap();
    img.save("ion_enc.png").unwrap();

}

pub trait ImageBytes {
    fn get_pixels(&self, dims: (usize, usize), crd: (usize, usize), xs: usize) -> [[u8; 3]; 5];
    fn set_pixel(&mut self, px: [u8; 3], crd: (usize, usize), xs: usize);
}

impl ImageBytes for Vec<u8> {
    fn get_pixels(&self, dims: (usize, usize), crd: (usize, usize), xs: usize) -> [[u8; 3]; 5] {
        let scrds = surrounding_coords(dims, crd);
        let x0y0 = crd_to_ind3(scrds[0].0, scrds[0].1, xs);
        let x1y0 = crd_to_ind3(scrds[1].0, scrds[1].1, xs);
        let x2y0 = crd_to_ind3(scrds[2].0, scrds[2].1, xs);
        let x0y1 = crd_to_ind3(scrds[3].0, scrds[3].1, xs);
        let x0y2 = crd_to_ind3(scrds[4].0, scrds[4].1, xs);
        [
            [self[x0y0], self[x0y0+1], self[x0y0+2]],
            [self[x1y0], self[x1y0+1], self[x1y0+2]],
            [self[x2y0], self[x2y0+1], self[x2y0+2]],
            [self[x0y1], self[x0y1+1], self[x0y1+2]],
            [self[x0y2], self[x0y2+1], self[x0y2+2]],
        ]
    }

    fn set_pixel(&mut self, px: [u8; 3], crd: (usize, usize), xs: usize) {
        let ind = crd_to_ind3(crd.0, crd.1, xs);
        self[ind] = px[0];
        self[ind+1] = px[1];
        self[ind+2] = px[2];
    }
}

fn difference(pxs: [[u8; 3]; 5]) -> [i32; 3] {
    let baseline = get_baseline(pxs);
    [
        pxs[0][0] as i32 - baseline[0] as i32,
        pxs[0][1] as i32 - baseline[1] as i32,
        pxs[0][2] as i32 - baseline[2] as i32,
    ]
}

fn get_baseline(pxs: [[u8; 3]; 5]) -> [u8; 3] {
    let mut r: u32 = 0;
    let mut g: u32 = 0;
    let mut b: u32 = 0;
    for i in 1..5 {
        r += pxs[i][0] as u32;
        g += pxs[i][1] as u32;
        b += pxs[i][2] as u32;
    }
    [(r/4) as u8, (g/4) as u8, (b/4) as u8]
}

fn wrapping_coords(pos: (usize, usize), dims: (usize, usize), diff: (i32, i32)) -> (usize, usize) {
    let mut pos: (i32, i32) = (pos.0 as i32, pos.1 as i32);
    pos.0 += diff.0 * 2;
    pos.1 += diff.1 * 2;
    if pos.0 < 0 {
        pos.0 += dims.0 as i32
    }
    if pos.1 < 0 {
        pos.1 += dims.1 as i32
    }

    ((pos.0 % dims.0 as i32) as usize, (pos.1 % dims.1 as i32) as usize)
}

fn surrounding_coords(dims: (usize, usize), crd: (usize, usize)) -> [(usize, usize); 5] {
    let x0 = crd.0 == 0;
    let y0 = crd.1 == 0;
    let xm = crd.0 == dims.0-1;
    let ym = crd.1 == dims.1-1;
    [
        (crd.0, crd.1),
        (if xm {0} else {crd.0+1}, crd.1),
        (if x0 {dims.0-1} else {crd.0-1}, crd.1),
        (crd.0, if ym {0} else {crd.1+1}),
        (crd.0, if y0 {dims.1-1} else {crd.1-1}),
    ]
}

fn check_diff_validity(pos: (usize, usize), dims: (usize, usize), diff: (i32, i32), xs: usize, blocked: &mut Vec<bool>) -> bool {
    let crd = wrapping_coords(pos, dims, diff);
    let scrds = surrounding_coords(dims, crd);
    let ind0 = crd_to_ind1(scrds[0].0, scrds[0].1, xs);
    let ind1 = crd_to_ind1(scrds[1].0, scrds[1].1, xs);
    let ind2 = crd_to_ind1(scrds[2].0, scrds[2].1, xs);
    let ind3 = crd_to_ind1(scrds[3].0, scrds[3].1, xs);
    let ind4 = crd_to_ind1(scrds[4].0, scrds[4].1, xs);
    let is_blocked = blocked[ind0]; //|| blocked[ind1] || blocked[ind2] || blocked[ind3] || blocked[ind4];
    if is_blocked {
        println!("Pixel blocked")
    } else {
        blocked[ind0] = true;
        blocked[ind1] = true;
        blocked[ind2] = true;
        blocked[ind3] = true;
        blocked[ind4] = true;
    }
    !is_blocked
}

fn u8_3_to_i8_3(x: [u8; 3]) -> [i32; 3] {
    [
        x[0] as i32,
        x[1] as i32,
        x[2] as i32,
    ]
}


fn bitstream_to_bytes(bits: Vec<bool>) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![];
    let mut byte: u8 = 0;
    let mut pos = 7;

    for bit in bits {
        byte += (bit as u8) << pos;
        pos -= 1;
        if pos == -1 {
            pos = 7;
            bytes.push(byte);
            byte = 0;
        }
    }
    if pos != 7 {
        bytes.push(byte);
    }
    bytes
}

fn bytes_to_bitstream(bytes: &Vec<u8>) -> Vec<bool> {
    let mut bits: Vec<bool> = vec![];
    for byte in bytes {
        for i in 0..8 {
            bits.push((byte >> (7-i) & 1) != 0)
        }
    }
    bits
}


fn crd_to_ind3(x: usize, y: usize, xs: usize) -> usize {
    (y * xs + x) * 3
}

fn crd_to_ind1(x: usize, y: usize, xs: usize) -> usize {
    y * xs + x
}

fn ind_to_crd(n: usize, xs: usize) -> (usize, usize) {
    (n / 3 % xs, n / 3 / xs)
}
