
static mut kInsBase: [u32; 24] = [0u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 8u32, 10u32, 14u32,
                                  18u32, 26u32, 34u32, 50u32, 66u32, 98u32, 130u32, 194u32,
                                  322u32, 578u32, 1090u32, 2114u32, 6210u32, 22594u32];

static mut kInsExtra: [u32; 24] = [0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 1u32, 2u32, 2u32,
                                   3u32, 3u32, 4u32, 4u32, 5u32, 5u32, 6u32, 7u32, 8u32, 9u32,
                                   10u32, 12u32, 14u32, 24u32];

static mut kCopyBase: [u32; 24] = [2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32, 12u32,
                                   14u32, 18u32, 22u32, 30u32, 38u32, 54u32, 70u32, 102u32,
                                   134u32, 198u32, 326u32, 582u32, 1094u32, 2118u32];

static mut kCopyExtra: [u32; 24] = [0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 1u32,
                                    2u32, 2u32, 3u32, 3u32, 4u32, 4u32, 5u32, 5u32, 6u32, 7u32,
                                    8u32, 9u32, 10u32, 24u32];

static kBrotliMinWindowBits: i32 = 10i32;

static kBrotliMaxWindowBits: i32 = 24i32;



fn BitsEntropy(mut population: &[u32], mut size: usize) -> f64 {
  let mut sum: usize;
  let mut retval: f64 = ShannonEntropy(population, size, &mut sum);
  if retval < sum as (f64) {
    retval = sum as (f64);
  }
  retval
}


pub fn BrotliPopulationCostLiteral(mut histogram: &[HistogramLiteral]) -> f64 {
  static kOneSymbolHistogramCost: f64 = 12i32 as (f64);
  static kTwoSymbolHistogramCost: f64 = 20i32 as (f64);
  static kThreeSymbolHistogramCost: f64 = 28i32 as (f64);
  static kFourSymbolHistogramCost: f64 = 37i32 as (f64);
  let data_size: usize = HistogramDataSizeLiteral();
  let mut count: i32 = 0i32;
  let mut s: [usize; 5];
  let mut bits: f64 = 0.0f64;
  let mut i: usize;
  if (*histogram).total_count_ == 0usize {
    return kOneSymbolHistogramCost;
  }
  i = 0usize;
  'break1: while i < data_size {
    {
      if (*histogram).data_[i] > 0u32 {
        s[count as (usize)] = i;
        count = count + 1;
        if count > 4i32 {
          {
            break 'break1;
          }
        }
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
  if count == 1i32 {
    return kOneSymbolHistogramCost;
  }
  if count == 2i32 {
    return kTwoSymbolHistogramCost + (*histogram).total_count_ as (f64);
  }
  if count == 3i32 {
    let histo0: u32 = (*histogram).data_[s[0usize]];
    let histo1: u32 = (*histogram).data_[s[1usize]];
    let histo2: u32 = (*histogram).data_[s[2usize]];
    let histomax: u32 = brotli_max_uint32_t(histo0, brotli_max_uint32_t(histo1, histo2));
    return kThreeSymbolHistogramCost +
           (2u32).wrapping_mul(histo0.wrapping_add(histo1).wrapping_add(histo2)) as (f64) -
           histomax as (f64);
  }
  if count == 4i32 {
    let mut histo: [u32; 4];
    let mut h23: u32;
    let mut histomax: u32;
    i = 0usize;
    while i < 4usize {
      {
        histo[i] = (*histogram).data_[s[i]];
      }
      i = i.wrapping_add(1 as (usize));
    }
    i = 0usize;
    while i < 4usize {
      {
        let mut j: usize;
        j = i.wrapping_add(1usize);
        while j < 4usize {
          {
            if histo[j] > histo[i] {
              let mut __brotli_swap_tmp: u32 = histo[j];
              histo[j] = histo[i];
              histo[i] = __brotli_swap_tmp;
            }
          }
          j = j.wrapping_add(1 as (usize));
        }
      }
      i = i.wrapping_add(1 as (usize));
    }
    h23 = histo[2usize].wrapping_add(histo[3usize]);
    histomax = brotli_max_uint32_t(h23, histo[0usize]);
    return kFourSymbolHistogramCost + (3u32).wrapping_mul(h23) as (f64) +
           (2u32).wrapping_mul(histo[0usize].wrapping_add(histo[1usize])) as (f64) -
           histomax as (f64);
  }
  {
    let mut max_depth: usize = 1usize;
    let mut depth_histo: [u32; 18] = [0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
                                      0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32];
    let log2total: f64 = FastLog2((*histogram).total_count_);
    i = 0usize;
    while i < data_size {
      if (*histogram).data_[i] > 0u32 {
        let mut log2p: f64 = log2total - FastLog2((*histogram).data_[i] as (usize));
        let mut depth: usize = (log2p + 0.5f64) as (usize);
        bits = bits + (*histogram).data_[i] as (f64) * log2p;
        if depth > 15usize {
          depth = 15usize;
        }
        if depth > max_depth {
          max_depth = depth;
        }
        {
          let _rhs = 1;
          let _lhs = &mut depth_histo[depth];
          *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
        i = i.wrapping_add(1 as (usize));
      } else {
        let mut reps: u32 = 1u32;
        let mut k: usize;
        k = i.wrapping_add(1usize);
        while k < data_size && ((*histogram).data_[k] == 0u32) {
          {
            reps = reps.wrapping_add(1 as (u32));
          }
          k = k.wrapping_add(1 as (usize));
        }
        i = i.wrapping_add(reps as (usize));
        if i == data_size {
          {
            break;
          }
        }
        if reps < 3u32 {
          let _rhs = reps;
          let _lhs = &mut depth_histo[0usize];
          *_lhs = (*_lhs).wrapping_add(_rhs);
        } else {
          reps = reps.wrapping_sub(2u32);
          while reps > 0u32 {
            {
              let _rhs = 1;
              let _lhs = &mut depth_histo[17usize];
              *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
            }
            bits = bits + 3i32 as (f64);
            reps = reps >> 3i32;
          }
        }
      }
    }
    bits = bits + (18usize).wrapping_add((2usize).wrapping_mul(max_depth)) as (f64);
    bits = bits + BitsEntropy(depth_histo.as_mut_ptr(), 18usize);
  }
  bits
}



pub struct HistogramCommand {
  pub data_: [u32; 704],
  pub total_count_: usize,
  pub bit_cost_: f64,
}

fn HistogramDataSizeCommand() -> usize {
  704usize
}


pub fn BrotliPopulationCostCommand(mut histogram: &[HistogramCommand]) -> f64 {
  static kOneSymbolHistogramCost: f64 = 12i32 as (f64);
  static kTwoSymbolHistogramCost: f64 = 20i32 as (f64);
  static kThreeSymbolHistogramCost: f64 = 28i32 as (f64);
  static kFourSymbolHistogramCost: f64 = 37i32 as (f64);
  let data_size: usize = HistogramDataSizeCommand();
  let mut count: i32 = 0i32;
  let mut s: [usize; 5];
  let mut bits: f64 = 0.0f64;
  let mut i: usize;
  if (*histogram).total_count_ == 0usize {
    return kOneSymbolHistogramCost;
  }
  i = 0usize;
  'break11: while i < data_size {
    {
      if (*histogram).data_[i] > 0u32 {
        s[count as (usize)] = i;
        count = count + 1;
        if count > 4i32 {
          {
            break 'break11;
          }
        }
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
  if count == 1i32 {
    return kOneSymbolHistogramCost;
  }
  if count == 2i32 {
    return kTwoSymbolHistogramCost + (*histogram).total_count_ as (f64);
  }
  if count == 3i32 {
    let histo0: u32 = (*histogram).data_[s[0usize]];
    let histo1: u32 = (*histogram).data_[s[1usize]];
    let histo2: u32 = (*histogram).data_[s[2usize]];
    let histomax: u32 = brotli_max_uint32_t(histo0, brotli_max_uint32_t(histo1, histo2));
    return kThreeSymbolHistogramCost +
           (2u32).wrapping_mul(histo0.wrapping_add(histo1).wrapping_add(histo2)) as (f64) -
           histomax as (f64);
  }
  if count == 4i32 {
    let mut histo: [u32; 4];
    let mut h23: u32;
    let mut histomax: u32;
    i = 0usize;
    while i < 4usize {
      {
        histo[i] = (*histogram).data_[s[i]];
      }
      i = i.wrapping_add(1 as (usize));
    }
    i = 0usize;
    while i < 4usize {
      {
        let mut j: usize;
        j = i.wrapping_add(1usize);
        while j < 4usize {
          {
            if histo[j] > histo[i] {
              let mut __brotli_swap_tmp: u32 = histo[j];
              histo[j] = histo[i];
              histo[i] = __brotli_swap_tmp;
            }
          }
          j = j.wrapping_add(1 as (usize));
        }
      }
      i = i.wrapping_add(1 as (usize));
    }
    h23 = histo[2usize].wrapping_add(histo[3usize]);
    histomax = brotli_max_uint32_t(h23, histo[0usize]);
    return kFourSymbolHistogramCost + (3u32).wrapping_mul(h23) as (f64) +
           (2u32).wrapping_mul(histo[0usize].wrapping_add(histo[1usize])) as (f64) -
           histomax as (f64);
  }
  {
    let mut max_depth: usize = 1usize;
    let mut depth_histo: [u32; 18] = [0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
                                      0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32];
    let log2total: f64 = FastLog2((*histogram).total_count_);
    i = 0usize;
    while i < data_size {
      if (*histogram).data_[i] > 0u32 {
        let mut log2p: f64 = log2total - FastLog2((*histogram).data_[i] as (usize));
        let mut depth: usize = (log2p + 0.5f64) as (usize);
        bits = bits + (*histogram).data_[i] as (f64) * log2p;
        if depth > 15usize {
          depth = 15usize;
        }
        if depth > max_depth {
          max_depth = depth;
        }
        {
          let _rhs = 1;
          let _lhs = &mut depth_histo[depth];
          *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
        i = i.wrapping_add(1 as (usize));
      } else {
        let mut reps: u32 = 1u32;
        let mut k: usize;
        k = i.wrapping_add(1usize);
        while k < data_size && ((*histogram).data_[k] == 0u32) {
          {
            reps = reps.wrapping_add(1 as (u32));
          }
          k = k.wrapping_add(1 as (usize));
        }
        i = i.wrapping_add(reps as (usize));
        if i == data_size {
          {
            break;
          }
        }
        if reps < 3u32 {
          let _rhs = reps;
          let _lhs = &mut depth_histo[0usize];
          *_lhs = (*_lhs).wrapping_add(_rhs);
        } else {
          reps = reps.wrapping_sub(2u32);
          while reps > 0u32 {
            {
              let _rhs = 1;
              let _lhs = &mut depth_histo[17usize];
              *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
            }
            bits = bits + 3i32 as (f64);
            reps = reps >> 3i32;
          }
        }
      }
    }
    bits = bits + (18usize).wrapping_add((2usize).wrapping_mul(max_depth)) as (f64);
    bits = bits + BitsEntropy(depth_histo.as_mut_ptr(), 18usize);
  }
  bits
}



pub struct HistogramDistance {
  pub data_: [u32; 520],
  pub total_count_: usize,
  pub bit_cost_: f64,
}

fn HistogramDataSizeDistance() -> usize {
  520usize
}


pub fn BrotliPopulationCostDistance(mut histogram: &[HistogramDistance]) -> f64 {
  static kOneSymbolHistogramCost: f64 = 12i32 as (f64);
  static kTwoSymbolHistogramCost: f64 = 20i32 as (f64);
  static kThreeSymbolHistogramCost: f64 = 28i32 as (f64);
  static kFourSymbolHistogramCost: f64 = 37i32 as (f64);
  let data_size: usize = HistogramDataSizeDistance();
  let mut count: i32 = 0i32;
  let mut s: [usize; 5];
  let mut bits: f64 = 0.0f64;
  let mut i: usize;
  if (*histogram).total_count_ == 0usize {
    return kOneSymbolHistogramCost;
  }
  i = 0usize;
  'break21: while i < data_size {
    {
      if (*histogram).data_[i] > 0u32 {
        s[count as (usize)] = i;
        count = count + 1;
        if count > 4i32 {
          {
            break 'break21;
          }
        }
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
  if count == 1i32 {
    return kOneSymbolHistogramCost;
  }
  if count == 2i32 {
    return kTwoSymbolHistogramCost + (*histogram).total_count_ as (f64);
  }
  if count == 3i32 {
    let histo0: u32 = (*histogram).data_[s[0usize]];
    let histo1: u32 = (*histogram).data_[s[1usize]];
    let histo2: u32 = (*histogram).data_[s[2usize]];
    let histomax: u32 = brotli_max_uint32_t(histo0, brotli_max_uint32_t(histo1, histo2));
    return kThreeSymbolHistogramCost +
           (2u32).wrapping_mul(histo0.wrapping_add(histo1).wrapping_add(histo2)) as (f64) -
           histomax as (f64);
  }
  if count == 4i32 {
    let mut histo: [u32; 4];
    let mut h23: u32;
    let mut histomax: u32;
    i = 0usize;
    while i < 4usize {
      {
        histo[i] = (*histogram).data_[s[i]];
      }
      i = i.wrapping_add(1 as (usize));
    }
    i = 0usize;
    while i < 4usize {
      {
        let mut j: usize;
        j = i.wrapping_add(1usize);
        while j < 4usize {
          {
            if histo[j] > histo[i] {
              let mut __brotli_swap_tmp: u32 = histo[j];
              histo[j] = histo[i];
              histo[i] = __brotli_swap_tmp;
            }
          }
          j = j.wrapping_add(1 as (usize));
        }
      }
      i = i.wrapping_add(1 as (usize));
    }
    h23 = histo[2usize].wrapping_add(histo[3usize]);
    histomax = brotli_max_uint32_t(h23, histo[0usize]);
    return kFourSymbolHistogramCost + (3u32).wrapping_mul(h23) as (f64) +
           (2u32).wrapping_mul(histo[0usize].wrapping_add(histo[1usize])) as (f64) -
           histomax as (f64);
  }
  {
    let mut max_depth: usize = 1usize;
    let mut depth_histo: [u32; 18] = [0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
                                      0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32];
    let log2total: f64 = FastLog2((*histogram).total_count_);
    i = 0usize;
    while i < data_size {
      if (*histogram).data_[i] > 0u32 {
        let mut log2p: f64 = log2total - FastLog2((*histogram).data_[i] as (usize));
        let mut depth: usize = (log2p + 0.5f64) as (usize);
        bits = bits + (*histogram).data_[i] as (f64) * log2p;
        if depth > 15usize {
          depth = 15usize;
        }
        if depth > max_depth {
          max_depth = depth;
        }
        {
          let _rhs = 1;
          let _lhs = &mut depth_histo[depth];
          *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
        i = i.wrapping_add(1 as (usize));
      } else {
        let mut reps: u32 = 1u32;
        let mut k: usize;
        k = i.wrapping_add(1usize);
        while k < data_size && ((*histogram).data_[k] == 0u32) {
          {
            reps = reps.wrapping_add(1 as (u32));
          }
          k = k.wrapping_add(1 as (usize));
        }
        i = i.wrapping_add(reps as (usize));
        if i == data_size {
          {
            break;
          }
        }
        if reps < 3u32 {
          let _rhs = reps;
          let _lhs = &mut depth_histo[0usize];
          *_lhs = (*_lhs).wrapping_add(_rhs);
        } else {
          reps = reps.wrapping_sub(2u32);
          while reps > 0u32 {
            {
              let _rhs = 1;
              let _lhs = &mut depth_histo[17usize];
              *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
            }
            bits = bits + 3i32 as (f64);
            reps = reps >> 3i32;
          }
        }
      }
    }
    bits = bits + (18usize).wrapping_add((2usize).wrapping_mul(max_depth)) as (f64);
    bits = bits + BitsEntropy(depth_histo.as_mut_ptr(), 18usize);
  }
  bits
}
