use easy_rss::RssParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "https://www.zhihu.com/rss";
    let mut parser = RssParser::from_url(address, "utf8")?;
    parser.author_tag = String::from("dc:creator");
}