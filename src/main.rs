use std::{
    iter::repeat,
    fs::{File, metadata},
    io::{Read, Write, BufReader, BufWriter, BufRead},
    env,
    time::Instant,
    path::Path,
};

pub trait BufferedRead {
    fn read_byte(&mut self) -> u8;
    fn read_byte_checked(&mut self) -> Option<u8>;
}
impl BufferedRead for BufReader<File> {
    fn read_byte(&mut self) -> u8 {
        let mut byte = [0u8; 1];

        if self.read(&mut byte).is_ok() {
            if self.buffer().is_empty() {
                self.consume(self.capacity());
                self.fill_buf().unwrap();
            }
        }
        else {
            println!("Function read_byte failed.");
        }
        u8::from_le_bytes(byte)
    }

    fn read_byte_checked(&mut self) -> Option<u8> {
        let mut byte = [0u8; 1];

        let bytes_read = self.read(&mut byte).unwrap();
        if self.buffer().len() <= 0 { 
            self.consume(self.capacity()); 
            self.fill_buf().unwrap();
        }
        if bytes_read == 0 {
            return None;
        }
        Some(u8::from_le_bytes(byte))
    }
}

pub trait BufferedWrite {
    fn write_byte(&mut self, output: u8);
    fn flush_buffer(&mut self);
}
impl BufferedWrite for BufWriter<File> {
    fn write_byte(&mut self, output: u8) {
        self.write(&[output]).unwrap();
        
        if self.buffer().len() >= self.capacity() {
            self.flush().unwrap();
        }
    }

    fn flush_buffer(&mut self) {
        self.flush().unwrap(); 
    }
}


fn new_input_file(capacity: usize, file_name: &str) -> BufReader<File> {
    BufReader::with_capacity(
        capacity, File::open(file_name).unwrap()
    )
}

fn new_output_file(capacity: usize, file_name: &str) -> BufWriter<File> {
    BufWriter::with_capacity(
        capacity, File::create(file_name).unwrap()
    )
}

const fn squash(d: i32) -> i32 {
    const SQ_T: [i32; 33] = [
    1,2,3,6,10,16,27,45,73,120,194,310,488,747,1101,
    1546,2047,2549,2994,3348,3607,3785,3901,3975,4022,
    4050,4068,4079,4085,4089,4092,4093,4094];
    if d > 2047  { return 4095; }
    if d < -2047 { return 0;    }
    let i_w = d & 127;
    let d = ((d >> 7) + 16) as usize;
    (SQ_T[d] * (128 - i_w) + SQ_T[d+1] * i_w + 64) >> 7
}

const STRETCH: [i16; 4096] = build_stretch_table();

const fn build_stretch_table() -> [i16; 4096] {
    let mut table = [0i16; 4096];
    let mut pi = 0;
    let mut x = -2047;
    while x <= 2047 {
        let i = squash(x);
        let mut j = pi;
        while j <= i {
            table[j as usize] = x as i16;
            j += 1;
        }
        pi = i + 1;
        x += 1;
    }
    table[4095] = 2047;
    table
}

fn stretch(p: i32) -> i32 {
    STRETCH[p as usize] as i32
}


const STATE_TABLE: [[u8; 2]; 256] = [
[  1,  2],[  3,  5],[  4,  6],[  7, 10],[  8, 12],[  9, 13],[ 11, 14], // 0
[ 15, 19],[ 16, 23],[ 17, 24],[ 18, 25],[ 20, 27],[ 21, 28],[ 22, 29], // 7
[ 26, 30],[ 31, 33],[ 32, 35],[ 32, 35],[ 32, 35],[ 32, 35],[ 34, 37], // 14
[ 34, 37],[ 34, 37],[ 34, 37],[ 34, 37],[ 34, 37],[ 36, 39],[ 36, 39], // 21
[ 36, 39],[ 36, 39],[ 38, 40],[ 41, 43],[ 42, 45],[ 42, 45],[ 44, 47], // 28
[ 44, 47],[ 46, 49],[ 46, 49],[ 48, 51],[ 48, 51],[ 50, 52],[ 53, 43], // 35
[ 54, 57],[ 54, 57],[ 56, 59],[ 56, 59],[ 58, 61],[ 58, 61],[ 60, 63], // 42
[ 60, 63],[ 62, 65],[ 62, 65],[ 50, 66],[ 67, 55],[ 68, 57],[ 68, 57], // 49
[ 70, 73],[ 70, 73],[ 72, 75],[ 72, 75],[ 74, 77],[ 74, 77],[ 76, 79], // 56
[ 76, 79],[ 62, 81],[ 62, 81],[ 64, 82],[ 83, 69],[ 84, 71],[ 84, 71], // 63
[ 86, 73],[ 86, 73],[ 44, 59],[ 44, 59],[ 58, 61],[ 58, 61],[ 60, 49], // 70
[ 60, 49],[ 76, 89],[ 76, 89],[ 78, 91],[ 78, 91],[ 80, 92],[ 93, 69], // 77
[ 94, 87],[ 94, 87],[ 96, 45],[ 96, 45],[ 48, 99],[ 48, 99],[ 88,101], // 84
[ 88,101],[ 80,102],[103, 69],[104, 87],[104, 87],[106, 57],[106, 57], // 91
[ 62,109],[ 62,109],[ 88,111],[ 88,111],[ 80,112],[113, 85],[114, 87], // 98
[114, 87],[116, 57],[116, 57],[ 62,119],[ 62,119],[ 88,121],[ 88,121], // 105
[ 90,122],[123, 85],[124, 97],[124, 97],[126, 57],[126, 57],[ 62,129], // 112
[ 62,129],[ 98,131],[ 98,131],[ 90,132],[133, 85],[134, 97],[134, 97], // 119
[136, 57],[136, 57],[ 62,139],[ 62,139],[ 98,141],[ 98,141],[ 90,142], // 126
[143, 95],[144, 97],[144, 97],[ 68, 57],[ 68, 57],[ 62, 81],[ 62, 81], // 133
[ 98,147],[ 98,147],[100,148],[149, 95],[150,107],[150,107],[108,151], // 140
[108,151],[100,152],[153, 95],[154,107],[108,155],[100,156],[157, 95], // 147
[158,107],[108,159],[100,160],[161,105],[162,107],[108,163],[110,164], // 154
[165,105],[166,117],[118,167],[110,168],[169,105],[170,117],[118,171], // 161
[110,172],[173,105],[174,117],[118,175],[110,176],[177,105],[178,117], // 168
[118,179],[110,180],[181,115],[182,117],[118,183],[120,184],[185,115], // 175
[186,127],[128,187],[120,188],[189,115],[190,127],[128,191],[120,192], // 182
[193,115],[194,127],[128,195],[120,196],[197,115],[198,127],[128,199], // 189
[120,200],[201,115],[202,127],[128,203],[120,204],[205,115],[206,127], // 196
[128,207],[120,208],[209,125],[210,127],[128,211],[130,212],[213,125], // 203
[214,137],[138,215],[130,216],[217,125],[218,137],[138,219],[130,220], // 210
[221,125],[222,137],[138,223],[130,224],[225,125],[226,137],[138,227], // 217
[130,228],[229,125],[230,137],[138,231],[130,232],[233,125],[234,137], // 224
[138,235],[130,236],[237,125],[238,137],[138,239],[130,240],[241,125], // 231
[242,137],[138,243],[130,244],[245,135],[246,137],[138,247],[140,248], // 238
[249,135],[250, 69],[ 80,251],[140,252],[249,135],[250, 69],[ 80,251], // 245
[140,252],[  0,  0],[  0,  0],[  0,  0]];  // 252

fn next_state(state: u8, bit: i32) -> u8 {
    STATE_TABLE[state as usize][bit as usize]
}

#[allow(overflowing_literals)]
const PR_MSK: i32 = 0xFFFFFE00; // High 23 bit mask
const LIMIT: usize = 127; // Controls rate of adaptation (higher = slower) (0..512)

// StateMap --------------------------------------------------------
struct StateMap {
    cxt:     usize,         
    cxt_map: Vec<u32>,  // Maps a context to a prediction and a count 
    rec:     Vec<u16>,  // Controls adjustment to cxt_map
}
impl StateMap {
    fn new(n: usize) -> Self {
        Self { 
            cxt:     0,
            cxt_map: vec![1 << 31; n],
            rec:     (0..512).map(|i| 32768/(i+i+5)).collect(),
        }
    }
    fn p(&mut self, bit: i32, cxt: usize) -> i32 {
        assert!(bit == 0 || bit == 1);
        self.update(bit);                      
        self.cxt = cxt;
        (self.cxt_map[self.cxt] >> 20) as i32  
    }
    fn update(&mut self, bit: i32) {
        let count = (self.cxt_map[self.cxt] & 511) as usize; // Low 9 bits
        let pr    = (self.cxt_map[self.cxt] >> 14) as i32;   // High 18 bits

        if count < LIMIT {
            self.cxt_map[self.cxt] += 1; 
        }

        let pr_err = (bit << 18) - pr; // Prediction error
        let rec_v = self.rec[count] as i32; // Reciprocal value
        let update = (pr_err * rec_v & PR_MSK) as u32;
        self.cxt_map[self.cxt] = self.cxt_map[self.cxt].wrapping_add(update); 
    }
}

struct Apm {
    bin:  usize,    
    cxts: usize, 
    bins: Vec<u16>, // maps each bin to a squashed value
}
impl Apm {
    fn new(n: usize) -> Self {
        let bins = repeat(
            (0..33)
            .map(|i| (squash((i - 16) * 128) * 16) as u16)
            .collect::<Vec<u16>>().into_iter() 
        )
        .take(n)
        .flatten()
        .collect();

        Self {
            bin:  0,
            cxts: n,
            bins,
        }
    }

    fn p(&mut self, bit: i32, rate: i32, mut pr: i32, cxt: usize) -> i32 {
        assert!(bit == 0 || bit == 1);
        assert!(pr >= 0 && pr < 4096);
        assert!(cxt < self.cxts);

        self.update(bit, rate);
        
        pr = stretch(pr); // -2047 to 2047
        let i_w = pr & 127; // Interpolation weight (33 points)
        
        self.bin = (((pr + 2048) >> 7) + ((cxt as i32) * 33)) as usize;

        let a = self.bins[self.bin] as i32;
        let b = self.bins[self.bin+1] as i32;
        ((a * (128 - i_w)) + (b * i_w)) >> 11
    }

    fn update(&mut self, bit: i32, rate: i32) {
        assert!(bit == 0 || bit == 1);
        assert!(rate > 0 && rate < 32);
        
        // Positive update if bit is 0, negative if 1
        let g = (bit << 16) + (bit << rate) - bit - bit;
        let a = self.bins[self.bin] as i32;
        let b = self.bins[self.bin+1] as i32;
        self.bins[self.bin]   = (a + ((g - a) >> rate)) as u16;
        self.bins[self.bin+1] = (b + ((g - b) >> rate)) as u16;
    }
}

struct Predictor {
    cxt:   usize,         
    cxt4:  usize,        
    pr:    i32,         
    state: [u8; 256],  
    sm:    StateMap, 
    apm:   [Apm; 5],  
}
impl Predictor {
    fn new() -> Self {
        let apm = [
            Apm::new(256),    
            Apm::new(256),   
            Apm::new(65536), 
            Apm::new(8192),
            Apm::new(16384), 
        ];

        Self {
            cxt:   0,                    
            cxt4:  0,                    
            pr:    2048,                 
            state: [0; 256],             
            sm:    StateMap::new(65536), 
            apm,
        }
    }

    fn p(&mut self) -> i32 { 
        assert!(self.pr >= 0 && self.pr < 4096);
        self.pr 
    } 

    fn update(&mut self, bit: i32) {
        assert!(bit == 0 || bit == 1);
        self.state[self.cxt] = next_state(self.state[self.cxt], bit);

        self.cxt += self.cxt + bit as usize;
        if self.cxt >= 256 {
            self.cxt4 = (self.cxt4 << 8) | (self.cxt - 256);
            self.cxt = 0;
        }

        self.pr = self.sm.p(bit, self.state[self.cxt] as usize);

        // SSE
        let cxt = self.cxt;
        self.pr = self.apm[0].p(bit, 5, self.pr, cxt) + 
                  self.apm[1].p(bit, 9, self.pr, cxt) + 1 >> 1;
        
        let cxt = self.cxt | (self.cxt4 << 8) & 0xFF00;
        self.pr = self.apm[2].p(bit, 7, self.pr, cxt);
        
        let cxt = self.cxt | (self.cxt4 & 0x1F00);
        self.pr = self.apm[3].p(bit, 7, self.pr, cxt) * 3 + self.pr + 2 >> 2;

        let hash = (((self.cxt4 as u32) & 0xFFFFFF).wrapping_mul(123456791)) >> 18;
        let cxt = ((self.cxt as u32) ^ hash) as usize;
        self.pr = self.apm[4].p(bit, 7, self.pr, cxt) + self.pr + 1 >> 1;
    }   
}

struct Encoder {
    predictor: Predictor,
    high:      u32,
    low:       u32,
    file_out:  BufWriter<File>,
}
impl Encoder {
    fn new(file_out: BufWriter<File>) -> Self {
        Self {
            predictor: Predictor::new(), 
            high: 0xFFFFFFFF, 
            low: 0,  
            file_out,
        }
    }
    fn encode(&mut self, bit: i32) {
        let p = self.predictor.p() as u32;
        let range = self.high - self.low;
        let mid = self.low + (range >> 12) * p + ((range & 0x0FFF) * p >> 12);

        if bit == 1 { 
            self.high = mid;    
        } 
        else {        
            self.low = mid + 1; 
        }
        self.predictor.update(bit);

        while ( (self.high ^ self.low) & 0xFF000000) == 0 {
            self.file_out.write_byte((self.high >> 24) as u8);
            self.high = (self.high << 8) + 255;
            self.low <<= 8;  
        }
    }
    fn flush(&mut self) {
        while ( (self.high ^ self.low) & 0xFF000000) == 0 {
            self.file_out.write_byte((self.high >> 24) as u8);
            self.high = (self.high << 8) + 255;
            self.low <<= 8; 
        }
        self.file_out.write_byte((self.high >> 24) as u8);
        self.file_out.flush_buffer();
    }
}

struct Decoder {
    predictor: Predictor,
    high:      u32,
    low:       u32,
    x:         u32,
    file_in:   BufReader<File>,   
}
impl Decoder {
    fn new(file_in: BufReader<File>) -> Self {
        let mut dec = Self {
            predictor: Predictor::new(), 
            high: 0xFFFFFFFF, 
            low: 0, 
            x: 0, 
            file_in, 
        };
        for _ in 0..4 {
            dec.x = (dec.x << 8) + dec.file_in.read_byte() as u32;
        }
        dec
    }
    fn decode(&mut self) -> u8 {
        let p = self.predictor.p() as u32;
        let range = self.high - self.low;
        let mid = self.low + (range >> 12) * p + ((range & 0x0FFF) * p >> 12);

        let mut bit = 0;
        if self.x <= mid {
            bit = 1;
            self.high = mid;
        } 
        else {
            self.low = mid + 1;
        }
        self.predictor.update(bit);
        
        while ( (self.high ^ self.low) & 0xFF000000) == 0 {
            self.high = (self.high << 8) + 255;
            self.low <<= 8;
            self.x = (self.x << 8) + self.file_in.read_byte() as u32; 
        }
        bit as u8
    }
}

fn main() {
    let start_time = Instant::now();
    let args: Vec<String> = env::args().collect();
    
    let mut file_in  = new_input_file(4096, &args[2]);
    let mut file_out = new_output_file(4096, &args[3]);
    match (&args[1]).as_str() {
        "c" => {  
            let mut enc = Encoder::new(file_out);

            while let Some(byte) = file_in.read_byte_checked() { 
                enc.encode(1);
                for i in (0..8).rev() {
                    enc.encode(((byte >> i) & 1).into());
                } 
            }   
            enc.encode(0);
            enc.flush(); 
            println!("Finished Compressing.");     
        }
        "d" => {
            let mut dec = Decoder::new(file_in);
            
            while dec.decode() != 0 { 
                let byte = (0..8).fold(1, |acc, _| (acc << 1) + dec.decode());
                file_out.write_byte(byte);
            }
            file_out.flush_buffer();
            println!("Finished Decompressing.");   
        }
        _ => {  
            println!("Enter 'c input output' to compress");
            println!("Enter 'd input output' to decompress"); 
        } 
    } 
    println!("{} bytes -> {} bytes in {:.2?}", 
        metadata(Path::new(&args[2])).unwrap().len(), 
        metadata(Path::new(&args[3])).unwrap().len(), 
        start_time.elapsed()
    );  
}