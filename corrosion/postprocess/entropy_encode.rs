extern "C" {
  fn memset(__b: &mut [::std::os::raw::c_void],
            __c: i32,
            __len: usize)
            -> *mut ::std::os::raw::c_void;
}



pub struct HuffmanTree {
  pub total_count_: u32,
  pub index_left_: i16,
  pub index_right_or_value_: i16,
}


pub fn BrotliSetDepth(mut p0: i32,
                      mut pool: &mut [HuffmanTree],
                      mut depth: &mut [u8],
                      mut max_depth: i32)
                      -> i32 {
  let mut stack: [i32; 16];
  let mut level: i32 = 0i32;
  let mut p: i32 = p0;
  0i32;
  stack[0usize] = -1i32;
  while 1i32 != 0 {
    if (pool[(p as (usize))]).index_left_ as (i32) >= 0i32 {
      level = level + 1;
      if level > max_depth {
        return 0i32;
      }
      stack[level as (usize)] = (pool[(p as (usize))]).index_right_or_value_ as (i32);
      p = (pool[(p as (usize))]).index_left_ as (i32);
      {
        {
          continue;
        }
      }
    } else {
      depth[((pool[(p as (usize))]).index_right_or_value_ as (usize))] = level as (u8);
    }
    while level >= 0i32 && (stack[level as (usize)] == -1i32) {
      level = level - 1;
    }
    if level < 0i32 {
      return 1i32;
    }
    p = stack[level as (usize)];
    stack[level as (usize)] = -1i32;
  }
}

fn InitHuffmanTree(mut xself: &mut HuffmanTree, mut count: u32, mut left: i16, mut right: i16) {
  (*xself).total_count_ = count;
  (*xself).index_left_ = left;
  (*xself).index_right_or_value_ = right;
}

fn brotli_max_uint32_t(mut a: u32, mut b: u32) -> u32 {
  if a > b { a } else { b }
}

fn SortHuffmanTreeItems(mut items: &mut [HuffmanTree],
                        n: usize,
                        mut comparator: fn(&[HuffmanTree], &[HuffmanTree]) -> i32) {
  static mut gaps: [usize; 6] = [132usize, 57usize, 23usize, 10usize, 4usize, 1usize];
  if n < 13usize {
    let mut i: usize;
    i = 1usize;
    while i < n {
      {
        let mut tmp: HuffmanTree = items[(i as (usize))];
        let mut k: usize = i;
        let mut j: usize = i.wrapping_sub(1usize);
        while comparator(&mut tmp, &mut items[(j as (usize))]) != 0 {
          items[(k as (usize))] = items[(j as (usize))];
          k = j;
          if {
               let _old = j;
               j = j.wrapping_sub(1 as (usize));
               _old
             } == 0 {
            {
              break;
            }
          }
        }
        items[(k as (usize))] = tmp;
      }
      i = i.wrapping_add(1 as (usize));
    }
  } else {
    let mut g: i32 = if n < 57usize { 2i32 } else { 0i32 };
    while g < 6i32 {
      {
        let mut gap: usize = gaps[g as (usize)];
        let mut i: usize;
        i = gap;
        while i < n {
          {
            let mut j: usize = i;
            let mut tmp: HuffmanTree = items[(i as (usize))];
            while j >= gap &&
                  (comparator(&mut tmp, &mut items[(j.wrapping_sub(gap) as (usize))]) != 0) {
              {
                items[(j as (usize))] = items[(j.wrapping_sub(gap) as (usize))];
              }
              j = j.wrapping_sub(gap);
            }
            items[(j as (usize))] = tmp;
          }
          i = i.wrapping_add(1 as (usize));
        }
      }
      g = g + 1;
    }
  }
}

fn SortHuffmanTree(mut v0: &[HuffmanTree], mut v1: &[HuffmanTree]) -> i32 {
  if (*v0).total_count_ != (*v1).total_count_ {
    return if !!((*v0).total_count_ < (*v1).total_count_) {
             1i32
           } else {
             0i32
           };
  }
  if !!((*v0).index_right_or_value_ as (i32) > (*v1).index_right_or_value_ as (i32)) {
    1i32
  } else {
    0i32
  }
}


pub fn BrotliCreateHuffmanTree(mut data: &[u32],
                               length: usize,
                               tree_limit: i32,
                               mut tree: &mut [HuffmanTree],
                               mut depth: &mut [u8]) {
  let mut count_limit: u32;
  let mut sentinel: HuffmanTree;
  InitHuffmanTree(&mut sentinel, !(0u32), -1i32 as (i16), -1i32 as (i16));
  count_limit = 1u32;
  'break1: loop {
    {
      let mut n: usize = 0usize;
      let mut i: usize;
      let mut j: usize;
      let mut k: usize;
      i = length;
      while i != 0usize {
        i = i.wrapping_sub(1 as (usize));
        if data[(i as (usize))] != 0 {
          let count: u32 = brotli_max_uint32_t(data[(i as (usize))], count_limit);
          InitHuffmanTree(&mut tree[({
                                  let _old = n;
                                  n = n.wrapping_add(1 as (usize));
                                  _old
                                } as (usize))],
                          count,
                          -1i32 as (i16),
                          i as (i16));
        }
      }
      if n == 1usize {
        depth[((tree[(0usize)]).index_right_or_value_ as (usize))] = 1i32 as (u8);
        {
          {
            break 'break1;
          }
        }
      }
      SortHuffmanTreeItems(tree, n, SortHuffmanTree);
      tree[(n as (usize))] = sentinel;
      tree[(n.wrapping_add(1usize) as (usize))] = sentinel;
      i = 0usize;
      j = n.wrapping_add(1usize);
      k = n.wrapping_sub(1usize);
      while k != 0usize {
        {
          let mut left: usize;
          let mut right: usize;
          if (tree[(i as (usize))]).total_count_ <= (tree[(j as (usize))]).total_count_ {
            left = i;
            i = i.wrapping_add(1 as (usize));
          } else {
            left = j;
            j = j.wrapping_add(1 as (usize));
          }
          if (tree[(i as (usize))]).total_count_ <= (tree[(j as (usize))]).total_count_ {
            right = i;
            i = i.wrapping_add(1 as (usize));
          } else {
            right = j;
            j = j.wrapping_add(1 as (usize));
          }
          {
            let mut j_end: usize = (2usize).wrapping_mul(n).wrapping_sub(k);
            (tree[(j_end as (usize))]).total_count_ =
              (tree[(left as (usize))])
                .total_count_
                .wrapping_add((tree[(right as (usize))]).total_count_);
            (tree[(j_end as (usize))]).index_left_ = left as (i16);
            (tree[(j_end as (usize))]).index_right_or_value_ = right as (i16);
            tree[(j_end.wrapping_add(1usize) as (usize))] = sentinel;
          }
        }
        k = k.wrapping_sub(1 as (usize));
      }
      if BrotliSetDepth((2usize).wrapping_mul(n).wrapping_sub(1usize) as (i32),
                        &mut tree[(0usize)],
                        depth,
                        tree_limit) != 0 {
        {
          break 'break1;
        }
      }
    }
    count_limit = count_limit.wrapping_mul(2u32);
  }
}


pub fn BrotliOptimizeHuffmanCountsForRle(mut length: usize,
                                         mut counts: &mut [u32],
                                         mut good_for_rle: &mut [u8]) {
  let mut nonzero_count: usize = 0usize;
  let mut stride: usize;
  let mut limit: usize;
  let mut sum: usize;
  let streak_limit: usize = 1240usize;
  let mut i: usize;
  i = 0usize;
  while i < length {
    {
      if counts[(i as (usize))] != 0 {
        nonzero_count = nonzero_count.wrapping_add(1 as (usize));
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
  if nonzero_count < 16usize {
    return;
  }
  while length != 0usize && (counts[(length.wrapping_sub(1usize) as (usize))] == 0u32) {
    length = length.wrapping_sub(1 as (usize));
  }
  if length == 0usize {
    return;
  }
  {
    let mut nonzeros: usize = 0usize;
    let mut smallest_nonzero: u32 = (1i32 << 30i32) as (u32);
    i = 0usize;
    while i < length {
      {
        if counts[(i as (usize))] != 0u32 {
          nonzeros = nonzeros.wrapping_add(1 as (usize));
          if smallest_nonzero > counts[(i as (usize))] {
            smallest_nonzero = counts[(i as (usize))];
          }
        }
      }
      i = i.wrapping_add(1 as (usize));
    }
    if nonzeros < 5usize {
      return;
    }
    if smallest_nonzero < 4u32 {
      let mut zeros: usize = length.wrapping_sub(nonzeros);
      if zeros < 6usize {
        i = 1usize;
        while i < length.wrapping_sub(1usize) {
          {
            if counts[(i.wrapping_sub(1usize) as (usize))] != 0u32 &&
               (counts[(i as (usize))] == 0u32) &&
               (counts[(i.wrapping_add(1usize) as (usize))] != 0u32) {
              counts[(i as (usize))] = 1u32;
            }
          }
          i = i.wrapping_add(1 as (usize));
        }
      }
    }
    if nonzeros < 28usize {
      return;
    }
  }
  memset(good_for_rle, 0i32, length);
  {
    let mut symbol: u32 = counts[(0usize)];
    let mut step: usize = 0usize;
    i = 0usize;
    while i <= length {
      {
        if i == length || counts[(i as (usize))] != symbol {
          if symbol == 0u32 && (step >= 5usize) || symbol != 0u32 && (step >= 7usize) {
            let mut k: usize;
            k = 0usize;
            while k < step {
              {
                good_for_rle[(i.wrapping_sub(k).wrapping_sub(1usize) as (usize))] = 1i32 as (u8);
              }
              k = k.wrapping_add(1 as (usize));
            }
          }
          step = 1usize;
          if i != length {
            symbol = counts[(i as (usize))];
          }
        } else {
          step = step.wrapping_add(1 as (usize));
        }
      }
      i = i.wrapping_add(1 as (usize));
    }
  }
  stride = 0usize;
  limit = (256u32)
    .wrapping_mul((counts[(0usize)]).wrapping_add(counts[(1usize)]).wrapping_add(counts
                                                                                   [(2usize)]))
    .wrapping_div(3u32)
    .wrapping_add(420u32) as (usize);
  sum = 0usize;
  i = 0usize;
  while i <= length {
    {
      if i == length || good_for_rle[(i as (usize))] != 0 ||
         i != 0usize && (good_for_rle[(i.wrapping_sub(1usize) as (usize))] != 0) ||
         ((256u32).wrapping_mul(counts[(i as (usize))]) as (usize))
           .wrapping_sub(limit)
           .wrapping_add(streak_limit) >= (2usize).wrapping_mul(streak_limit) {
        if stride >= 4usize || stride >= 3usize && (sum == 0usize) {
          let mut k: usize;
          let mut count: usize = sum.wrapping_add(stride.wrapping_div(2usize)).wrapping_div(stride);
          if count == 0usize {
            count = 1usize;
          }
          if sum == 0usize {
            count = 0usize;
          }
          k = 0usize;
          while k < stride {
            {
              counts[(i.wrapping_sub(k).wrapping_sub(1usize) as (usize))] = count as (u32);
            }
            k = k.wrapping_add(1 as (usize));
          }
        }
        stride = 0usize;
        sum = 0usize;
        if i < length.wrapping_sub(2usize) {
          limit = (256u32)
            .wrapping_mul((counts[(i as (usize))])
                            .wrapping_add(counts[(i.wrapping_add(1usize) as (usize))])
                            .wrapping_add(counts[(i.wrapping_add(2usize) as (usize))]))
            .wrapping_div(3u32)
            .wrapping_add(420u32) as (usize);
        } else if i < length {
          limit = (256u32).wrapping_mul(counts[(i as (usize))]) as (usize);
        } else {
          limit = 0usize;
        }
      }
      stride = stride.wrapping_add(1 as (usize));
      if i != length {
        sum = sum.wrapping_add(counts[(i as (usize))] as (usize));
        if stride >= 4usize {
          limit = (256usize)
            .wrapping_mul(sum)
            .wrapping_add(stride.wrapping_div(2usize))
            .wrapping_div(stride);
        }
        if stride == 4usize {
          limit = limit.wrapping_add(120usize);
        }
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
}

fn DecideOverRleUse(mut depth: &[u8],
                    length: usize,
                    mut use_rle_for_non_zero: &mut i32,
                    mut use_rle_for_zero: &mut i32) {
  let mut total_reps_zero: usize = 0usize;
  let mut total_reps_non_zero: usize = 0usize;
  let mut count_reps_zero: usize = 1usize;
  let mut count_reps_non_zero: usize = 1usize;
  let mut i: usize;
  i = 0usize;
  while i < length {
    let value: u8 = depth[(i as (usize))];
    let mut reps: usize = 1usize;
    let mut k: usize;
    k = i.wrapping_add(1usize);
    while k < length && (depth[(k as (usize))] as (i32) == value as (i32)) {
      {
        reps = reps.wrapping_add(1 as (usize));
      }
      k = k.wrapping_add(1 as (usize));
    }
    if reps >= 3usize && (value as (i32) == 0i32) {
      total_reps_zero = total_reps_zero.wrapping_add(reps);
      count_reps_zero = count_reps_zero.wrapping_add(1 as (usize));
    }
    if reps >= 4usize && (value as (i32) != 0i32) {
      total_reps_non_zero = total_reps_non_zero.wrapping_add(reps);
      count_reps_non_zero = count_reps_non_zero.wrapping_add(1 as (usize));
    }
    i = i.wrapping_add(reps);
  }
  *use_rle_for_non_zero = if !!(total_reps_non_zero > count_reps_non_zero.wrapping_mul(2usize)) {
    1i32
  } else {
    0i32
  };
  *use_rle_for_zero = if !!(total_reps_zero > count_reps_zero.wrapping_mul(2usize)) {
    1i32
  } else {
    0i32
  };
}

fn Reverse(mut v: &mut [u8], mut start: usize, mut end: usize) {
  end = end.wrapping_sub(1 as (usize));
  while start < end {
    let mut tmp: u8 = v[(start as (usize))];
    v[(start as (usize))] = v[(end as (usize))];
    v[(end as (usize))] = tmp;
    start = start.wrapping_add(1 as (usize));
    end = end.wrapping_sub(1 as (usize));
  }
}

fn BrotliWriteHuffmanTreeRepetitionsZeros(mut repetitions: usize,
                                          mut tree_size: &mut [usize],
                                          mut tree: &mut [u8],
                                          mut extra_bits_data: &mut [u8]) {
  if repetitions == 11usize {
    tree[(*tree_size as (usize))] = 0i32 as (u8);
    extra_bits_data[(*tree_size as (usize))] = 0i32 as (u8);
    *tree_size = (*tree_size).wrapping_add(1 as (usize));
    repetitions = repetitions.wrapping_sub(1 as (usize));
  }
  if repetitions < 3usize {
    let mut i: usize;
    i = 0usize;
    while i < repetitions {
      {
        tree[(*tree_size as (usize))] = 0i32 as (u8);
        extra_bits_data[(*tree_size as (usize))] = 0i32 as (u8);
        *tree_size = (*tree_size).wrapping_add(1 as (usize));
      }
      i = i.wrapping_add(1 as (usize));
    }
  } else {
    let mut start: usize = *tree_size;
    repetitions = repetitions.wrapping_sub(3usize);
    while 1i32 != 0 {
      tree[(*tree_size as (usize))] = 17i32 as (u8);
      extra_bits_data[(*tree_size as (usize))] = (repetitions & 0x7usize) as (u8);
      *tree_size = (*tree_size).wrapping_add(1 as (usize));
      repetitions = repetitions >> 3i32;
      if repetitions == 0usize {
        {
          break;
        }
      }
      repetitions = repetitions.wrapping_sub(1 as (usize));
    }
    Reverse(tree, start, *tree_size);
    Reverse(extra_bits_data, start, *tree_size);
  }
}

fn BrotliWriteHuffmanTreeRepetitions(previous_value: u8,
                                     value: u8,
                                     mut repetitions: usize,
                                     mut tree_size: &mut [usize],
                                     mut tree: &mut [u8],
                                     mut extra_bits_data: &mut [u8]) {
  0i32;
  if previous_value as (i32) != value as (i32) {
    tree[(*tree_size as (usize))] = value;
    extra_bits_data[(*tree_size as (usize))] = 0i32 as (u8);
    *tree_size = (*tree_size).wrapping_add(1 as (usize));
    repetitions = repetitions.wrapping_sub(1 as (usize));
  }
  if repetitions == 7usize {
    tree[(*tree_size as (usize))] = value;
    extra_bits_data[(*tree_size as (usize))] = 0i32 as (u8);
    *tree_size = (*tree_size).wrapping_add(1 as (usize));
    repetitions = repetitions.wrapping_sub(1 as (usize));
  }
  if repetitions < 3usize {
    let mut i: usize;
    i = 0usize;
    while i < repetitions {
      {
        tree[(*tree_size as (usize))] = value;
        extra_bits_data[(*tree_size as (usize))] = 0i32 as (u8);
        *tree_size = (*tree_size).wrapping_add(1 as (usize));
      }
      i = i.wrapping_add(1 as (usize));
    }
  } else {
    let mut start: usize = *tree_size;
    repetitions = repetitions.wrapping_sub(3usize);
    while 1i32 != 0 {
      tree[(*tree_size as (usize))] = 16i32 as (u8);
      extra_bits_data[(*tree_size as (usize))] = (repetitions & 0x3usize) as (u8);
      *tree_size = (*tree_size).wrapping_add(1 as (usize));
      repetitions = repetitions >> 2i32;
      if repetitions == 0usize {
        {
          break;
        }
      }
      repetitions = repetitions.wrapping_sub(1 as (usize));
    }
    Reverse(tree, start, *tree_size);
    Reverse(extra_bits_data, start, *tree_size);
  }
}


pub fn BrotliWriteHuffmanTree(mut depth: &[u8],
                              mut length: usize,
                              mut tree_size: &mut [usize],
                              mut tree: &mut [u8],
                              mut extra_bits_data: &mut [u8]) {
  let mut previous_value: u8 = 8i32 as (u8);
  let mut i: usize;
  let mut use_rle_for_non_zero: i32 = 0i32;
  let mut use_rle_for_zero: i32 = 0i32;
  let mut new_length: usize = length;
  i = 0usize;
  'break27: while i < length {
    {
      if depth[(length.wrapping_sub(i).wrapping_sub(1usize) as (usize))] as (i32) == 0i32 {
        new_length = new_length.wrapping_sub(1 as (usize));
      } else {
        break 'break27;
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
  if length > 50usize {
    DecideOverRleUse(depth,
                     new_length,
                     &mut use_rle_for_non_zero,
                     &mut use_rle_for_zero);
  }
  i = 0usize;
  while i < new_length {
    let value: u8 = depth[(i as (usize))];
    let mut reps: usize = 1usize;
    if value as (i32) != 0i32 && (use_rle_for_non_zero != 0) ||
       value as (i32) == 0i32 && (use_rle_for_zero != 0) {
      let mut k: usize;
      k = i.wrapping_add(1usize);
      while k < new_length && (depth[(k as (usize))] as (i32) == value as (i32)) {
        {
          reps = reps.wrapping_add(1 as (usize));
        }
        k = k.wrapping_add(1 as (usize));
      }
    }
    if value as (i32) == 0i32 {
      BrotliWriteHuffmanTreeRepetitionsZeros(reps, tree_size, tree, extra_bits_data);
    } else {
      BrotliWriteHuffmanTreeRepetitions(previous_value,
                                        value,
                                        reps,
                                        tree_size,
                                        tree,
                                        extra_bits_data);
      previous_value = value;
    }
    i = i.wrapping_add(reps);
  }
}

fn BrotliReverseBits(mut num_bits: usize, mut bits: u16) -> u16 {
  static mut kLut: [usize; 16] = [0x0usize, 0x8usize, 0x4usize, 0xcusize, 0x2usize, 0xausize,
                                  0x6usize, 0xeusize, 0x1usize, 0x9usize, 0x5usize, 0xdusize,
                                  0x3usize, 0xbusize, 0x7usize, 0xfusize];
  let mut retval: usize = kLut[(bits as (i32) & 0xfi32) as (usize)];
  let mut i: usize;
  i = 4usize;
  while i < num_bits {
    {
      retval = retval << 4i32;
      bits = (bits as (i32) >> 4i32) as (u16);
      retval = retval | kLut[(bits as (i32) & 0xfi32) as (usize)];
    }
    i = i.wrapping_add(4usize);
  }
  retval = retval >> ((0usize).wrapping_sub(num_bits) & 0x3usize);
  retval as (u16)
}


pub fn BrotliConvertBitDepthsToSymbols(mut depth: &[u8], mut len: usize, mut bits: &mut [u16]) {
  let mut bl_count: [u16; 16] = [0i32 as (u16),
                                 0i32 as (u16),
                                 0i32 as (u16),
                                 0i32 as (u16),
                                 0i32 as (u16),
                                 0i32 as (u16),
                                 0i32 as (u16),
                                 0i32 as (u16),
                                 0i32 as (u16),
                                 0i32 as (u16),
                                 0i32 as (u16),
                                 0i32 as (u16),
                                 0i32 as (u16),
                                 0i32 as (u16),
                                 0i32 as (u16),
                                 0i32 as (u16)];
  let mut next_code: [u16; 16];
  let mut i: usize;
  let mut code: i32 = 0i32;
  i = 0usize;
  while i < len {
    {
      let _rhs = 1;
      let _lhs = &mut bl_count[depth[(i as (usize))] as (usize)];
      *_lhs = (*_lhs as (i32) + _rhs) as (u16);
    }
    i = i.wrapping_add(1 as (usize));
  }
  bl_count[0usize] = 0i32 as (u16);
  next_code[0usize] = 0i32 as (u16);
  i = 1usize;
  while i < 16usize {
    {
      code = code + bl_count[i.wrapping_sub(1usize)] as (i32) << 1i32;
      next_code[i] = code as (u16);
    }
    i = i.wrapping_add(1 as (usize));
  }
  i = 0usize;
  while i < len {
    {
      if depth[(i as (usize))] != 0 {
        bits[(i as (usize))] = BrotliReverseBits(depth[(i as (usize))] as (usize), {
          let _rhs = 1;
          let _lhs = &mut next_code[depth[(i as (usize))] as (usize)];
          let _old = *_lhs;
          *_lhs = (*_lhs as (i32) + _rhs) as (u16);
          _old
        });
      }
    }
    i = i.wrapping_add(1 as (usize));
  }
}
