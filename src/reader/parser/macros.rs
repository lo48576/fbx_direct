#![macro_use]

macro_rules! try_with_pos {
    ($pos:expr, $expr:expr) => (match $expr {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(err) => {
            return ::std::result::Result::Err($crate::error::Error::new($pos, err));
        },
    })
}

// TODO: Is it possible to simplify `try_read_le_*` using another macros such as `concat_idents!`?

macro_rules! try_read_le_u8 {
    ($pos:expr, $reader:expr) => ({
        use self::byteorder::ReadBytesExt;
        let val = try_with_pos!($pos, $reader.read_u8());
        $pos += 1;
        val
    })
}

macro_rules! try_read_le_u32 {
    ($pos:expr, $reader:expr) => ({
        use self::byteorder::ReadBytesExt;
        let val = try_with_pos!($pos, $reader.read_u32::<byteorder::LittleEndian>());
        $pos += 4;
        val
    })
}

macro_rules! try_read_le_i16 {
    ($pos:expr, $reader:expr) => ({
        use self::byteorder::ReadBytesExt;
        let val = try_with_pos!($pos, $reader.read_i16::<byteorder::LittleEndian>());
        $pos += 2;
        val
    })
}

macro_rules! try_read_le_i32 {
    ($pos:expr, $reader:expr) => ({
        use self::byteorder::ReadBytesExt;
        let val = try_with_pos!($pos, $reader.read_i32::<byteorder::LittleEndian>());
        $pos += 4;
        val
    })
}

macro_rules! try_read_le_i64 {
    ($pos:expr, $reader:expr) => ({
        use self::byteorder::ReadBytesExt;
        let val = try_with_pos!($pos, $reader.read_i64::<byteorder::LittleEndian>());
        $pos += 8;
        val
    })
}

macro_rules! try_read_le_f32 {
    ($pos:expr, $reader:expr) => ({
        use self::byteorder::ReadBytesExt;
        let val = try_with_pos!($pos, $reader.read_f32::<byteorder::LittleEndian>());
        $pos += 4;
        val
    })
}

macro_rules! try_read_le_f64 {
    ($pos:expr, $reader:expr) => ({
        use self::byteorder::ReadBytesExt;
        let val = try_with_pos!($pos, $reader.read_f64::<byteorder::LittleEndian>());
        $pos += 8;
        val
    })
}

macro_rules! try_read_fixstr {
    ($pos:expr, $reader:expr, $len:expr) => ({
        let mut buffer = String::with_capacity($len as usize);
        let len = try_with_pos!($pos, $reader.by_ref().take($len as u64).read_to_string(&mut buffer)) as u64;
        if len != ($len as u64) {
            return Err(Error::new($pos, ErrorKind::UnexpectedEof));
        }
        $pos += len;
        buffer
    })
}

macro_rules! try_read_exact {
    ($pos:expr, $reader:expr, $len:expr) => ({
        let mut buffer = Vec::<u8>::with_capacity($len as usize);
        let len = try_with_pos!($pos, $reader.by_ref().take($len as u64).read_to_end(&mut buffer)) as u64;
        if len != ($len as u64) {
            return Err(Error::new($pos, ErrorKind::UnexpectedEof));
        }
        $pos += len;
        buffer
    })
}
