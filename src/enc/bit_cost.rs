#![allow(dead_code)]
use super::super::alloc::SliceWrapper;

use super::util::{brotli_max_uint32_t, FastLog2};
use super::histogram::{CostAccessors};


static mut kCopyBase: [u32; 24] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 14, 18, 22, 30, 38, 54, 70,
                                   102, 134, 198, 326, 582, 1094, 2118];

static mut kCopyExtra: [u32; 24] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 7, 8,
                                    9, 10, 24];

static kBrotliMinWindowBits: i32 = 10i32;

static kBrotliMaxWindowBits: i32 = 24i32;



fn ShannonEntropy(mut population: &[u32], size: usize, mut total: &mut usize) -> f64 {
  let mut sum: usize = 0usize;
  let mut retval: f64 = 0i32 as (f64);
  population = &population[..(size as usize)];
  let mut p: usize;
  let mut odd_number_of_elements_left: i32 = 0i32;
  if size & 1usize != 0 {
    odd_number_of_elements_left = 1i32;
  }
  while population.len() != 0 {
    if odd_number_of_elements_left == 0 {
      p = population[0] as usize;
      population = &population[1..];
      sum = sum.wrapping_add(p);
      retval = retval - p as (f64) * FastLog2(p);
    }
    odd_number_of_elements_left = 0i32;
    p = population[0] as usize;
    population = &population[1..];
    sum = sum.wrapping_add(p);
    retval = retval - p as (f64) * FastLog2(p);
  }
  if sum != 0 {
    retval = retval + sum as (f64) * FastLog2(sum);
  }
  *total = sum;
  retval
}

fn BitsEntropy(population: &[u32], size: usize) -> f64 {
  let mut sum: usize = 0;
  let mut retval: f64 = ShannonEntropy(population, size, &mut sum);
  if retval < sum as (f64) {
    retval = sum as (f64);
  }
  retval
}

pub fn BrotliPopulationCost<HistogramType:SliceWrapper<u32>+CostAccessors>(
    histogram : &HistogramType
) -> f64{
  static kOneSymbolHistogramCost: f64 = 12i32 as (f64);
  static kTwoSymbolHistogramCost: f64 = 20i32 as (f64);
  static kThreeSymbolHistogramCost: f64 = 28i32 as (f64);
  static kFourSymbolHistogramCost: f64 = 37i32 as (f64);
  let data_size: usize = (*histogram).slice().len();
  let mut count: i32 = 0i32;
  let mut s: [usize; 5] = [0; 5];
  let mut bits: f64 = 0.0f64;
  let mut i: usize;
    if (*histogram).total_count() == 0usize {
    return kOneSymbolHistogramCost;
  }
  i = 0usize;
  'break1: while i < data_size {
    {
      if (*histogram).slice()[i] > 0u32 {
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
    return kTwoSymbolHistogramCost + (*histogram).total_count() as (f64);
  }
  if count == 3i32 {
    let histo0: u32 = (*histogram).slice()[s[0usize]];
    let histo1: u32 = (*histogram).slice()[s[1usize]];
    let histo2: u32 = (*histogram).slice()[s[2usize]];
    let histomax: u32 = brotli_max_uint32_t(histo0, brotli_max_uint32_t(histo1, histo2));
    return kThreeSymbolHistogramCost +
           (2u32).wrapping_mul(histo0.wrapping_add(histo1).wrapping_add(histo2)) as (f64) -
           histomax as (f64);
  }
  if count == 4i32 {
    let mut histo: [u32; 4] = [0;4];
    let h23: u32;
    let histomax: u32;
    i = 0usize;
    while i < 4usize {
      {
        histo[i] = (*histogram).slice()[s[i]];
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
    let log2total: f64 = FastLog2((*histogram).total_count());
    i = 0usize;
    while i < data_size {
      if (*histogram).slice()[i] > 0u32 {
        let log2p: f64 = log2total - FastLog2((*histogram).slice()[i] as (usize));
        let mut depth: usize = (log2p + 0.5f64) as (usize);
        bits = bits + (*histogram).slice()[i] as (f64) * log2p;
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
        while k < data_size && ((*histogram).slice()[k] == 0u32) {
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
    bits = bits + BitsEntropy(&depth_histo[..], 18usize);
  }
  bits
}
/*
fn HistogramDataSizeCommand() -> usize {
    704i32 as (usize)
}*/


/*
fn HistogramDataSizeDistance() -> usize {
    520i32 as (usize)
}
*/
