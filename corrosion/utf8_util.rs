static kMinUTF8Ratio : f64 = 0.75f64;

unsafe extern fn BrotliParseAsUTF8(
    mut symbol : *mut i32, mut input : *const u8, mut size : usize
) -> usize {
    if *input.offset(0i32 as (isize)) as (i32) & 0x80i32 == 0i32 {
        *symbol = *input.offset(0i32 as (isize)) as (i32);
        if *symbol > 0i32 {
            return 1i32 as (usize);
        }
    }
    if size > 1u32 as (usize) && (*input.offset(
                                       0i32 as (isize)
                                   ) as (i32) & 0xe0i32 == 0xc0i32) && (*input.offset(
                                                                             1i32 as (isize)
                                                                         ) as (i32) & 0xc0i32 == 0x80i32) {
        *symbol = (*input.offset(
                        0i32 as (isize)
                    ) as (i32) & 0x1fi32) << 6i32 | *input.offset(
                                                         1i32 as (isize)
                                                     ) as (i32) & 0x3fi32;
        if *symbol > 0x7fi32 {
            return 2i32 as (usize);
        }
    }
    if size > 2u32 as (usize) && (*input.offset(
                                       0i32 as (isize)
                                   ) as (i32) & 0xf0i32 == 0xe0i32) && (*input.offset(
                                                                             1i32 as (isize)
                                                                         ) as (i32) & 0xc0i32 == 0x80i32) && (*input.offset(
                                                                                                                   2i32 as (isize)
                                                                                                               ) as (i32) & 0xc0i32 == 0x80i32) {
        *symbol = (*input.offset(
                        0i32 as (isize)
                    ) as (i32) & 0xfi32) << 12i32 | (*input.offset(
                                                          1i32 as (isize)
                                                      ) as (i32) & 0x3fi32) << 6i32 | *input.offset(
                                                                                           2i32 as (isize)
                                                                                       ) as (i32) & 0x3fi32;
        if *symbol > 0x7ffi32 {
            return 3i32 as (usize);
        }
    }
    if size > 3u32 as (usize) && (*input.offset(
                                       0i32 as (isize)
                                   ) as (i32) & 0xf8i32 == 0xf0i32) && (*input.offset(
                                                                             1i32 as (isize)
                                                                         ) as (i32) & 0xc0i32 == 0x80i32) && (*input.offset(
                                                                                                                   2i32 as (isize)
                                                                                                               ) as (i32) & 0xc0i32 == 0x80i32) && (*input.offset(
                                                                                                                                                         3i32 as (isize)
                                                                                                                                                     ) as (i32) & 0xc0i32 == 0x80i32) {
        *symbol = (*input.offset(
                        0i32 as (isize)
                    ) as (i32) & 0x7i32) << 18i32 | (*input.offset(
                                                          1i32 as (isize)
                                                      ) as (i32) & 0x3fi32) << 12i32 | (*input.offset(
                                                                                             2i32 as (isize)
                                                                                         ) as (i32) & 0x3fi32) << 6i32 | *input.offset(
                                                                                                                              3i32 as (isize)
                                                                                                                          ) as (i32) & 0x3fi32;
        if *symbol > 0xffffi32 && (*symbol <= 0x10ffffi32) {
            return 4i32 as (usize);
        }
    }
    *symbol = 0x110000i32 | *input.offset(0i32 as (isize)) as (i32);
    1i32 as (usize)
}

#[no_mangle]
pub unsafe extern fn BrotliIsMostlyUTF8(
    mut data : *const u8,
    pos : usize,
    mask : usize,
    length : usize,
    min_fraction : f64
) -> i32 {
    let mut size_utf8 : usize = 0i32 as (usize);
    let mut i : usize = 0i32 as (usize);
    while i < length {
        let mut symbol : i32;
        let mut bytes_read
            : usize
            = BrotliParseAsUTF8(
                  &mut symbol as (*mut i32),
                  &*data.offset(
                        (pos.wrapping_add(i) & mask) as (isize)
                    ) as (*const u8),
                  length.wrapping_sub(i)
              );
        i = i.wrapping_add(bytes_read);
        if symbol < 0x110000i32 {
            size_utf8 = size_utf8.wrapping_add(bytes_read);
        }
    }
    if !!(size_utf8 as (f64) > min_fraction * length as (f64)) {
        1i32
    } else {
        0i32
    }
}
