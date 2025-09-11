mod chumsky_basic;
mod chumsky_optimized;
mod manual_parser;

fn generate_input(len: usize) -> String {
    // 使用简单的线性同余生成器作为伪随机数生成器
    struct SimpleRng {
        state: u64,
    }

    impl SimpleRng {
        fn new(seed: u64) -> Self {
            Self { state: seed }
        }

        fn next(&mut self) -> u64 {
            self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
            self.state
        }

        fn gen_range(&mut self, min: usize, max: usize) -> usize {
            min + (self.next() as usize) % (max - min)
        }

        fn gen_bool(&mut self, probability: f64) -> bool {
            (self.next() % 100) < (probability * 100.0) as u64
        }
    }

    // 创建随机数生成器，使用长度作为种子
    let mut rng = SimpleRng::new(len as u64);

    // 定义转义字符映射
    let escape_chars = [
        ("\\\\", "\\"),  // 反斜杠
        ("\\t", "\t"),   // 制表符
        ("\\n", "\n"),   // 换行符
        ("\\r", "\r"),   // 回车符
        ("\\0", "\0"),   // 空字符
        ("\\\"", "\""),  // 双引号
        ("\\'", "'"),    // 单引号
        ("\\b", "\x08"), // 退格符
        ("\\f", "\x0C"), // 换页符
        ("\\v", "\x0B"), // 垂直制表符
        ("\\a", "\x07"), // 响铃符
    ];

    // 普通字符集合
    let normal_chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 !@#$%^&*()_+-={}[]|:;<>?,./";

    let mut result = String::with_capacity(len * 2); // 预分配更大容量
    let mut current_len = 0;

    while current_len < len {
        // 30% 概率生成转义字符，70% 概率生成普通字符
        if rng.gen_bool(0.05) && current_len + 2 <= len {
            // 生成转义字符
            let idx = rng.gen_range(0, escape_chars.len());
            let (escape_seq, _) = escape_chars[idx];
            result.push_str(escape_seq);
            current_len += escape_seq.len();
        } else if current_len < len {
            // 生成普通字符
            let idx = rng.gen_range(0, normal_chars.len());
            let ch = normal_chars.chars().nth(idx).unwrap();
            result.push(ch);
            current_len += 1;
        }
    }

    // 确保不超过指定长度
    if result.len() > len {
        result.truncate(len);
    }

    result
}

// 测试函数，比较两种算法的性能和正确性
fn benchmark_algorithms() {
    println!("开始测试算法性能和正确性...\n");

    let test_sizes = [10, 100, 1000, 5000];

    for &size in &test_sizes {
        println!("测试大小: {} 字符", size);

        // 生成测试输入
        let input = generate_input(size);
        println!(
            "生成的测试输入: {}",
            if input.len() > 50 {
                format!("{}...(共{}字符)", &input[..50], input.len())
            } else {
                input.clone()
            }
        );

        // 测试 chumsky_basic 实现
        let start = std::time::Instant::now();
        let result_chumsky_basic = chumsky_basic::parse(&input);
        let time_chumsky_basic = start.elapsed();

        // 测试 manual_parser 实现
        let start = std::time::Instant::now();
        let result_manual_parser = manual_parser::parse(&input);
        let time_manual_parser = start.elapsed();

        // 测试 chumsky_optimized 实现
        let start = std::time::Instant::now();
        let result_chumsky_optimized = chumsky_optimized::parse(&input);
        let time_chumsky_optimized = start.elapsed();

        // 比较结果
        match (
            result_chumsky_basic,
            result_manual_parser,
            result_chumsky_optimized,
        ) {
            (Ok(res1), Ok(res2), Ok(res3)) => {
                if res1 == res2 && res2 == res3 {
                    println!("✓ 三种实现结果一致");
                    println!("  chumsky_basic 实现耗时:     {:?}", time_chumsky_basic);
                    println!("  chumsky_optimized 实现耗时: {:?}", time_chumsky_optimized);
                    println!("  manual_parser 实现耗时:     {:?}", time_manual_parser);

                    let manual_nanos = time_manual_parser.as_nanos() as f64;
                    println!(
                        "  速度比较: manual_parser 比 chumsky_basic 快 {:.2}x",
                        time_chumsky_basic.as_nanos() as f64 / manual_nanos
                    );
                    println!(
                        "  速度比较: manual_parser 比 chumsky_optimized 快 {:.2}x",
                        time_chumsky_optimized.as_nanos() as f64 / manual_nanos
                    );
                } else {
                    println!("✗ 结果不一致!");
                    println!("  chumsky_basic 结果:     {:?}", res1);
                    println!("  manual_parser 结果:     {:?}", res2);
                    println!("  chumsky_optimized 结果: {:?}", res3);
                }
            }
            (Err(_), Err(_), Err(_)) => {
                println!("✓ 三种实现都返回错误（一致）");
                println!("  chumsky_basic 实现耗时:     {:?}", time_chumsky_basic);
                println!("  chumsky_optimized 实现耗时: {:?}", time_chumsky_optimized);
                println!("  manual_parser 实现耗时:     {:?}", time_manual_parser);
            }
            (res1, res2, res3) => {
                println!("✗ 实现结果不一致（成功/失败混合）");
                println!(
                    "  chumsky_basic:     {}",
                    if res1.is_ok() { "Ok" } else { "Err" }
                );
                println!(
                    "  manual_parser:     {}",
                    if res2.is_ok() { "Ok" } else { "Err" }
                );
                println!(
                    "  chumsky_optimized: {}",
                    if res3.is_ok() { "Ok" } else { "Err" }
                );
            }
        }
        println!();
    }
}

fn main() {
    // 演示生成函数
    println!("=== 测试生成函数演示 ===");
    for len in [20, 50, 100] {
        let input = generate_input(len);
        println!("长度 {}: {}", len, input);
    }
    println!();

    // 运行基准测试
    println!("=== 算法性能对比 ===");
    benchmark_algorithms();
}
