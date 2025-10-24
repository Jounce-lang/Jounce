/**
 * Test suite for Jounce Reactivity System
 *
 * Run with: node runtime/test_reactivity.js
 */

const { signal, computed, effect, batch, untrack, _internals } = require('./reactivity.js');

// Simple test framework
let testsPassed = 0;
let testsFailed = 0;

function test(name, fn) {
    try {
        fn();
        console.log(`✓ ${name}`);
        testsPassed++;
    } catch (error) {
        console.error(`✗ ${name}`);
        console.error(`  ${error.message}`);
        testsFailed++;
    }
}

function assertEqual(actual, expected, message) {
    if (actual !== expected) {
        throw new Error(`${message}\n  Expected: ${expected}\n  Actual: ${actual}`);
    }
}

function assertThrows(fn, message) {
    try {
        fn();
        throw new Error(`${message}\n  Expected function to throw but it didn't`);
    } catch (error) {
        // Success - function threw as expected
        if (error.message.includes('Expected function to throw')) {
            throw error;
        }
    }
}

console.log('\n=== Testing Jounce Reactivity System ===\n');

// ============================================================================
// Signal Tests
// ============================================================================

console.log('--- Signal Tests ---\n');

test('Signal: Create and read', () => {
    const count = signal(0);
    assertEqual(count.value, 0, 'Initial value should be 0');
});

test('Signal: Write updates value', () => {
    const count = signal(0);
    count.value = 5;
    assertEqual(count.value, 5, 'Value should be updated to 5');
});

test('Signal: Notifies effects on change', () => {
    const count = signal(0);
    let effectRuns = 0;
    let lastValue = null;

    effect(() => {
        lastValue = count.value;
        effectRuns++;
    });

    assertEqual(effectRuns, 1, 'Effect should run immediately');
    assertEqual(lastValue, 0, 'Effect should see initial value');

    count.value = 5;
    assertEqual(effectRuns, 2, 'Effect should run again after change');
    assertEqual(lastValue, 5, 'Effect should see new value');
});

test('Signal: Skips notification if value unchanged', () => {
    const count = signal(0);
    let effectRuns = 0;

    effect(() => {
        count.value; // Read to track
        effectRuns++;
    });

    assertEqual(effectRuns, 1, 'Effect should run initially');

    count.value = 0; // Same value
    assertEqual(effectRuns, 1, 'Effect should NOT run if value unchanged');

    count.value = 5; // Different value
    assertEqual(effectRuns, 2, 'Effect should run when value changes');
});

// ============================================================================
// Computed Tests
// ============================================================================

console.log('\n--- Computed Tests ---\n');

test('Computed: Derives from signal', () => {
    const count = signal(0);
    const doubled = computed(() => count.value * 2);

    assertEqual(doubled.value, 0, 'Initial computed value should be 0');

    count.value = 5;
    assertEqual(doubled.value, 10, 'Computed should auto-update to 10');
});

test('Computed: Is lazy (only runs when accessed)', () => {
    let computeRuns = 0;
    const count = signal(0);
    const doubled = computed(() => {
        computeRuns++;
        return count.value * 2;
    });

    assertEqual(computeRuns, 0, 'Computed should not run until accessed');

    doubled.value;
    assertEqual(computeRuns, 1, 'Computed should run on first access');

    doubled.value;
    assertEqual(computeRuns, 1, 'Computed should be memoized');

    count.value = 5;
    assertEqual(computeRuns, 1, 'Computed should not run until accessed again');

    doubled.value;
    assertEqual(computeRuns, 2, 'Computed should run after dependency changes');
});

test('Computed: Is read-only', () => {
    const count = signal(0);
    const doubled = computed(() => count.value * 2);

    assertThrows(
        () => { doubled.value = 10; },
        'Assigning to computed should throw'
    );
});

test('Computed: Can depend on multiple signals', () => {
    const a = signal(2);
    const b = signal(3);
    const sum = computed(() => a.value + b.value);

    assertEqual(sum.value, 5, 'Sum should be 2 + 3 = 5');

    a.value = 10;
    assertEqual(sum.value, 13, 'Sum should update to 10 + 3 = 13');

    b.value = 7;
    assertEqual(sum.value, 17, 'Sum should update to 10 + 7 = 17');
});

test('Computed: Can chain multiple computed values', () => {
    const count = signal(2);
    const doubled = computed(() => count.value * 2);
    const quadrupled = computed(() => doubled.value * 2);

    assertEqual(quadrupled.value, 8, 'Initial: 2 * 2 * 2 = 8');

    count.value = 3;
    assertEqual(quadrupled.value, 12, 'After change: 3 * 2 * 2 = 12');
});

// ============================================================================
// Effect Tests
// ============================================================================

console.log('\n--- Effect Tests ---\n');

test('Effect: Runs immediately', () => {
    let ran = false;
    effect(() => {
        ran = true;
    });
    assertEqual(ran, true, 'Effect should run immediately');
});

test('Effect: Tracks dependencies automatically', () => {
    const count = signal(0);
    let effectRuns = 0;

    effect(() => {
        count.value; // Read to track
        effectRuns++;
    });

    assertEqual(effectRuns, 1, 'Effect runs immediately');

    count.value = 1;
    assertEqual(effectRuns, 2, 'Effect re-runs when dependency changes');
});

test('Effect: Only tracks signals actually read', () => {
    const a = signal(0);
    const b = signal(0);
    let effectRuns = 0;

    effect(() => {
        a.value; // Only read 'a'
        effectRuns++;
    });

    assertEqual(effectRuns, 1, 'Effect runs initially');

    b.value = 1; // Change 'b' (not tracked)
    assertEqual(effectRuns, 1, 'Effect should NOT re-run');

    a.value = 1; // Change 'a' (tracked)
    assertEqual(effectRuns, 2, 'Effect SHOULD re-run');
});

test('Effect: Can be disposed', () => {
    const count = signal(0);
    let effectRuns = 0;

    const eff = effect(() => {
        count.value;
        effectRuns++;
    });

    assertEqual(effectRuns, 1, 'Effect runs initially');

    count.value = 1;
    assertEqual(effectRuns, 2, 'Effect runs after change');

    eff.dispose();

    count.value = 2;
    assertEqual(effectRuns, 2, 'Effect should NOT run after disposal');
});

test('Effect: Works with computed dependencies', () => {
    const count = signal(0);
    const doubled = computed(() => count.value * 2);
    let effectValue = null;

    effect(() => {
        effectValue = doubled.value;
    });

    assertEqual(effectValue, 0, 'Initial: doubled = 0');

    count.value = 5;
    assertEqual(effectValue, 10, 'After change: doubled = 10');
});

// ============================================================================
// Batch Tests
// ============================================================================

console.log('\n--- Batch Tests ---\n');

test('Batch: Defers effect execution', () => {
    const count = signal(0);
    let effectRuns = 0;

    effect(() => {
        count.value;
        effectRuns++;
    });

    assertEqual(effectRuns, 1, 'Effect runs initially');

    batch(() => {
        count.value = 1;
        assertEqual(effectRuns, 1, 'Effect should NOT run inside batch');

        count.value = 2;
        assertEqual(effectRuns, 1, 'Effect still should NOT run');

        count.value = 3;
        assertEqual(effectRuns, 1, 'Effect still should NOT run');
    });

    assertEqual(effectRuns, 2, 'Effect should run ONCE after batch');
});

test('Batch: Works with multiple signals', () => {
    const a = signal(0);
    const b = signal(0);
    let effectRuns = 0;

    effect(() => {
        a.value;
        b.value;
        effectRuns++;
    });

    assertEqual(effectRuns, 1, 'Effect runs initially');

    batch(() => {
        a.value = 1;
        b.value = 2;
    });

    assertEqual(effectRuns, 2, 'Effect runs once after batch (not twice)');
});

test('Batch: Can be nested', () => {
    const count = signal(0);
    let effectRuns = 0;

    effect(() => {
        count.value;
        effectRuns++;
    });

    assertEqual(effectRuns, 1, 'Effect runs initially');

    batch(() => {
        count.value = 1;

        batch(() => {
            count.value = 2;
        });

        count.value = 3;
    });

    assertEqual(effectRuns, 2, 'Effect runs once after nested batches');
});

test('Batch: Returns function result', () => {
    const result = batch(() => {
        return 42;
    });

    assertEqual(result, 42, 'Batch should return function result');
});

// ============================================================================
// Untrack Tests
// ============================================================================

console.log('\n--- Untrack Tests ---\n');

test('Untrack: Prevents dependency tracking', () => {
    const a = signal(0);
    const b = signal(0);
    let effectRuns = 0;

    effect(() => {
        a.value; // Tracked

        untrack(() => {
            b.value; // NOT tracked
        });

        effectRuns++;
    });

    assertEqual(effectRuns, 1, 'Effect runs initially');

    b.value = 1; // Should NOT trigger re-run
    assertEqual(effectRuns, 1, 'Effect should not run (b was untracked)');

    a.value = 1; // SHOULD trigger re-run
    assertEqual(effectRuns, 2, 'Effect should run (a was tracked)');
});

test('Untrack: Returns function result', () => {
    const count = signal(5);
    const result = untrack(() => {
        return count.value * 2;
    });

    assertEqual(result, 10, 'Untrack should return function result');
});

// ============================================================================
// Edge Cases
// ============================================================================

console.log('\n--- Edge Case Tests ---\n');

test('Edge: Circular dependency in computed throws', () => {
    const a = signal(0);
    const b = computed(() => {
        return c.value + 1; // References c
    });
    const c = computed(() => {
        return b.value + 1; // References b (circular!)
    });

    assertThrows(
        () => { b.value; },
        'Circular dependency should throw'
    );
});

test('Edge: Circular dependency in effect throws', () => {
    const a = signal(0);

    assertThrows(() => {
        effect(() => {
            a.value; // Read a
            a.value = a.value + 1; // Write a (creates circular dependency)
        });
    }, 'Circular dependency in effect should throw');
});

test('Edge: Effect can read without triggering re-run using untrack', () => {
    const count = signal(0);
    let effectRuns = 0;

    effect(() => {
        const current = count.value; // Tracked

        untrack(() => {
            if (current > 0) {
                count.value; // NOT tracked (prevents infinite loop)
            }
        });

        effectRuns++;
    });

    assertEqual(effectRuns, 1, 'Effect runs initially');

    count.value = 5;
    assertEqual(effectRuns, 2, 'Effect runs once more');
});

test('Edge: Multiple effects on same signal', () => {
    const count = signal(0);
    let effect1Runs = 0;
    let effect2Runs = 0;

    effect(() => {
        count.value;
        effect1Runs++;
    });

    effect(() => {
        count.value;
        effect2Runs++;
    });

    assertEqual(effect1Runs, 1, 'Effect 1 runs initially');
    assertEqual(effect2Runs, 1, 'Effect 2 runs initially');

    count.value = 5;
    assertEqual(effect1Runs, 2, 'Effect 1 re-runs');
    assertEqual(effect2Runs, 2, 'Effect 2 re-runs');
});

test('Edge: Computed with no dependencies', () => {
    const constant = computed(() => 42);
    assertEqual(constant.value, 42, 'Constant computed works');
});

test('Edge: Effect with no dependencies', () => {
    let runs = 0;
    effect(() => {
        runs++;
    });
    assertEqual(runs, 1, 'Effect runs once even with no dependencies');
});

// ============================================================================
// Type Checking Tests
// ============================================================================

console.log('\n--- Type Checking Tests ---\n');

test('Type: signal() accepts any value', () => {
    const num = signal(42);
    const str = signal('hello');
    const obj = signal({ a: 1 });
    const arr = signal([1, 2, 3]);

    assertEqual(num.value, 42, 'Number signal');
    assertEqual(str.value, 'hello', 'String signal');
    assertEqual(obj.value.a, 1, 'Object signal');
    assertEqual(arr.value.length, 3, 'Array signal');
});

test('Type: computed() requires function', () => {
    assertThrows(
        () => { computed(42); },
        'computed() should throw if not given a function'
    );
});

test('Type: effect() requires function', () => {
    assertThrows(
        () => { effect(42); },
        'effect() should throw if not given a function'
    );
});

// ============================================================================
// Results
// ============================================================================

console.log('\n=== Test Results ===\n');
console.log(`✓ Passed: ${testsPassed}`);
console.log(`✗ Failed: ${testsFailed}`);
console.log(`Total: ${testsPassed + testsFailed}`);

if (testsFailed > 0) {
    console.log('\n⚠️  Some tests failed!');
    process.exit(1);
} else {
    console.log('\n🎉 All tests passed!');
    process.exit(0);
}
