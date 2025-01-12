use std::collections::HashMap;
use std::hash::Hash;

// 定义一个结构体 PageCache，用于缓存页面。
struct PageCache<U,A,F>
where
    U: Eq + Hash, // 泛型 U 表示用户 ID，必须实现 Eq 和 Hash trait（用于键的比较和哈希计算）。
    A: Eq + Hash, // 泛型 A 表示文章 ID，同样需要实现 Eq 和 Hash。
    F: Fn(&U,&A) -> String // 泛型 F 是一个函数闭包类型，接收用户 ID 和文章 ID 的引用并返回渲染后的页面内容。
{
    cache: HashMap<(U,A),String>, // 用于存储缓存的渲染页面，键是 (用户 ID, 文章 ID) 的组合。
    renderer: F, // 渲染器，负责生成页面内容。
}

// 为 PageCache 实现方法。
impl<U,A,F> PageCache<U,A,F>
where
    U: Eq + Hash + Clone, // U 类型需要实现 Eq, Hash, 和 Clone（用于复制键值）。
    A: Eq + Hash + Clone, // A 类型同样需要实现 Eq, Hash, 和 Clone。
    F: Fn(&U,&A) -> String, // 渲染器类型需要能够接收引用并返回页面内容。
{
    // 构造函数：初始化 PageCache 实例。
    fn new(renderer: F) -> Self {
        Self{
            cache: HashMap::new(), // 初始化空的缓存。
            renderer, // 存储传入的渲染器函数。
        }
    }

    // 获取页面的方法：根据用户 ID 和文章 ID 返回渲染后的页面内容。
    fn get_page(&mut self, user_id:U,article_id:A) -> String {
        // 构造键，表示特定用户和文章。
        let key = (user_id.clone(),article_id.clone());
        // 构造键，表示特定用户和文章。
        if let Some(cached_page) = self.cache.get(&key) {
            // 构造键，表示特定用户和文章。
            return cached_page.clone()
        }
        // 如果没有缓存，调用渲染器渲染页面
        let rendered_page = (self.renderer)(&user_id, &article_id);
        // 将生成的页面内容存入缓存中。
        self.cache.insert(key, rendered_page.clone());
        // 返回渲染后的页面内容。
        rendered_page
    }
}

fn main() {
    // 创建一个 PageCache 实例，并提供一个闭包作为渲染器。
    // 渲染器接收用户 ID 和文章 ID 的引用，返回 HTML 格式的页面内容。
    let mut page_cache = PageCache::new(|user_id: &String, article_id: &u32| -> String {
        println!("Rendering page for user {} and article {}", user_id, article_id);
        format!(
            "Rendered HTML content for user {} and article {}",
            user_id, article_id
        )
    });

    // 第一次调用，会执行页面渲染
    // println!("{}", page_cache.get_page("user1".to_string(), 42));
    // 第二次调用，直接返回缓存结果
    // println!("{}", page_cache.get_page("user1".to_string(), 42));
    // 不同用户查看同一文章，会重新渲染
    println!("{}", page_cache.get_page("user2".to_string(), 42));

}
