pub struct XP3File {
    tag1: u32,
    // 标志1，"File" 0x656c6946
    fileSize: u64,
    // 文件信息数据大小
    tag2: u32,
    // 标志2，"info" 0x6f666e69
    infoSize: u64,
    // 文件基本数据大小
    protect: u32,
    // 估计是表示此文件是否加过密
    rsize: u64,
    // 文件原始大小
    psize: u64,
    // 文件包中大小
    nameLen: u16,
    // 文件名长度（指的是UTF-16字符个数）
    fileName: Vec<i16>,
    // 文件名(UTF-16LE编码，无0结尾) nameLen wchar_t
    tag3: u32,
    // 标志3，"segm" 0x6d676573
    segmSize: u64,
    // 文件段数据大小
    compress: u32,
    // 文件是否用zlib压缩过
    offset: u64,
    // 文件开始的位置
    tag4: u32,
    // 标志4，"adlr" 0x726c6461
    adlrSize: u64,
    // 文件附加数据大小，一般是4
    key: u32,
    // 附加数据，用于加密
}
