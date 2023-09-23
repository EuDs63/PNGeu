# PNGeu

*PNGeu is an implementation of [PNGme: An Intermediate Rust Project](https://picklenerd.github.io/pngme_book/) for study purposes.*


## Reference
- [pngme_book](https://picklenerd.github.io/pngme_book/)
- [gabebw's pngme](https://github.com/gabebw/pngme)
- [makisevon's pngme](https://github.com/makisevon/pngme)
- [clap](https://docs.rs/clap/latest/clap/_cookbook/index.html)

## 踩坑
1. 
  ```Rust
    if let read_crc= cal_crc {
        Ok()
    }else{
        Err("invalid chunk".into())
    }
  ```
  
  - 我这里误解了`if let`的用法，写出了这样一段代码。但这无法实现我的需求：判断两个crc是否一致。

  - `if let`主要用于匹配和解构 Option 或 Result 类型的枚举变量，并根据匹配结果执行不同的代码分支
  