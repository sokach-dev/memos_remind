# memo remind
这个是用来提醒我回顾笔记的小服务，每天都会给我发送一篇我的笔记作为邮件，来帮我加深思考。

目前部署在oracle的机器上，上面集成了邮件的发送[lettre](https://crates.io/crates/lettre)和markdown到html的转化[pulldown-cmark](https://crates.io/crates/pulldown-cmark)

其中lettre在使用中需要注意，我们发送的是html，一定要在header中加入标识如下：
```rust
let email = Message::builder()
       .from(from.parse()?)
       .to(to.parse()?)
       .subject(subject)
       .header(ContentType::TEXT_HTML) // !!! import !!!
       .body(body)?;
```

而对于pulldown-cmark默认是不支持一些markdown语法的如表格等，如果需要支持需要添加标识，如：
```rust
Options::ENABLE_STRIKETHROUGH
    | Options::ENABLE_TABLES
    | Options::ENABLE_MATH
    | Options::ENABLE_FOOTNOTES
    | Options::ENABLE_TASKLISTS
    | Options::ENABLE_SMART_PUNCTUATION
    | Options::ENABLE_TASKLISTS
    | Options::ENABLE_GFM,
);
```