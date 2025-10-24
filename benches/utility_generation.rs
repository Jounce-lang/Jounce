// Benchmark for utility class generation performance
// Target: < 10ms for 100 utilities

use jounce_compiler::utility_config::UtilityConfig;
use jounce_compiler::utility_generator::UtilityGenerator;
use std::time::Instant;

fn benchmark_utility_generation(num_utilities: usize) -> (std::time::Duration, usize, usize) {
    let config = UtilityConfig::default();
    let mut generator = UtilityGenerator::new(config);

    // Generate diverse utility class names
    let mut class_names = Vec::new();
    for i in 0..num_utilities {
        match i % 12 {
            0 => class_names.push(format!("p-{}", (i % 16) * 4)),
            1 => class_names.push(format!("m-{}", (i % 16) * 4)),
            2 => class_names.push(format!("bg-blue-{}", (i % 9 + 1) * 100)),
            3 => class_names.push(format!("text-gray-{}", (i % 9 + 1) * 100)),
            4 => class_names.push("flex".to_string()),
            5 => class_names.push("grid".to_string()),
            6 => class_names.push(format!("w-{}", (i % 16) * 4)),
            7 => class_names.push(format!("h-{}", (i % 16) * 4)),
            8 => class_names.push(format!("md:p-{}", (i % 16) * 4)),
            9 => class_names.push(format!("hover:bg-blue-{}", (i % 9 + 1) * 100)),
            10 => class_names.push("shadow-lg".to_string()),
            11 => class_names.push("rounded-xl".to_string()),
            _ => class_names.push("hidden".to_string()),
        }
    }

    // Simulate direct insertion (as if scanned from AST)
    for class_name in &class_names {
        generator.used_utilities.insert(class_name.clone());
    }

    // Benchmark CSS generation
    let start = Instant::now();
    let css = generator.generate_css();
    let duration = start.elapsed();

    let metrics = generator.metrics();
    (duration, css.len(), metrics.utilities_generated)
}

fn main() {
    println!("🔥 RavensOne Utility Generation Benchmark\n");

    let test_sizes = vec![10, 50, 100, 200, 500];

    for size in test_sizes {
        let (duration, css_size, utilities_generated) = benchmark_utility_generation(size);
        let ms = duration.as_secs_f64() * 1000.0;

        println!("📊 Test with {} utility classes:", size);
        println!("   ├─ Duration: {:.3}ms", ms);
        println!("   ├─ Utilities generated: {}", utilities_generated);
        println!("   ├─ CSS output: {} bytes", css_size);
        println!("   └─ Performance: {:.2} utilities/ms\n", utilities_generated as f64 / ms);
    }

    // Final benchmark: 100 utilities target
    println!("🎯 Target Performance Check: 100 utilities");
    let (duration, _css_size, _utilities_generated) = benchmark_utility_generation(100);

    let target_ms = 10.0;
    let actual_ms = duration.as_secs_f64() * 1000.0;

    if actual_ms < target_ms {
        println!("   ✅ PASS: {:.3}ms < {:.2}ms target", actual_ms, target_ms);
    } else {
        println!("   ❌ FAIL: {:.3}ms > {:.2}ms target", actual_ms, target_ms);
    }
}
