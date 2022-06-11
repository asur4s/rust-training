use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // 函数通常使用 Result 作为返回类型，如果成功就返回 OK，如果失败就返回 Err。
    // Result 左边是 OK 的类型，右边是 Err 的类型。不能改变顺序
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // if args.len() < 3 {
        //     return Err("Not enough arguments");
        // }

        // 由于 args 是借用，所以必须使用 clone 获取所有权。
        // let query = args[1].clone();
        // let filename = args[2].clone();

        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        // is_err 可以判断是否出错。如果存在该变量（忽略大小写），就为 False，如果不存在就为 True。
        // case sensitive：区分大小写
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        // println!("{:?}", case_sensitive);

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// 这里的 Error 采用特征对象（Box<dyn Error>）
// todo: 为什么不能使用 &dyn Error
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? 是一个宏，主要用于传播错误。
    // ? 会调用 From 特质的 from 方法，自动进行类型提升
    // ? 还可以自动转换 Option 返回类型。
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        // 传入引用
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// 返回引用 需要标注生命周期。
// 字符串的内容来源于 content，所以生命周期和 content 相同。
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     // 判断字符串中是否包含查询内容
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     // 改变内容的大小写再查询。
    //     if line.to_lowercase().contains(&query.clone().to_lowercase()) {
    //         results.push(line);
    //     }
    // }
    // results
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()[..]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
Safe, fast, productive.
Pick three.";
        assert_eq!(vec!["Safe, fast, productive."], search(query, contents))
    }

    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
Safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
