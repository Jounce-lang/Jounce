// Test the generated for loop code
function main() {
  console.log("Exclusive range 1..5:");
  for (let i = 1; i < 5; i++) {
    console.log(`  i = ${i}`);
  }
  console.log("Inclusive range 1..=5:");
  for (let i = 1; i <= 5; i++) {
    console.log(`  i = ${i}`);
  }
  let start = 10;
  let end = 15;
  console.log(`Range with variables ${start}..${end}:`);
  for (let j = start; j < end; j++) {
    console.log(`  j = ${j}`);
  }
}

main();
