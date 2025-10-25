// Test the generated match OR pattern code
function classify_number(n) {
  return (() => {
    const __match_value = n;
    if (__match_value === 1) {
      return "one";
    }
    else if (__match_value === 2) {
      return "two";
    }
    else if ((__match_value === 3 || __match_value === 4 || __match_value === 5)) {
      return "three to five";
    }
    else if ((__match_value === 6 || __match_value === 7 || __match_value === 8 || __match_value === 9)) {
      return "six to nine";
    }
    else {
      return "other";
    }
  })();
}

function main() {
  console.log(`1 is ${classify_number(1)}`);
  console.log(`3 is ${classify_number(3)}`);
  console.log(`4 is ${classify_number(4)}`);
  console.log(`7 is ${classify_number(7)}`);
  console.log(`10 is ${classify_number(10)}`);
}

main();
