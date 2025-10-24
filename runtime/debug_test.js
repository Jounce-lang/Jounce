/**
 * Debug test to identify the hanging issue
 */

const { signal, computed, effect } = require('./reactivity.js');

console.log('Test 1: Simple signal...');
const count = signal(0);
console.log('✓ Signal created');
console.log('Value:', count.value);
console.log('✓ Read signal');

console.log('\nTest 2: Write to signal...');
count.value = 5;
console.log('✓ Written to signal');
console.log('Value:', count.value);
console.log('✓ Read signal again');

console.log('\nTest 3: Effect (might hang here)...');
let runs = 0;
effect(() => {
    console.log('  Effect running, count.value =', count.value);
    runs++;
    console.log('  runs =', runs);
});

console.log('✓ Effect created');
console.log('runs after effect =', runs);

console.log('\nTest 4: Update signal (might hang here)...');
count.value = 10;
console.log('✓ Signal updated');
console.log('runs after update =', runs);

console.log('\n✓ All tests passed!');
